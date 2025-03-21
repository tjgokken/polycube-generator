use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use std::sync::{atomic::{AtomicUsize, Ordering}, Mutex};

use rustc_hash::FxHashSet;
use rayon::prelude::*;

use crate::polycube::Polycube;

// Generate all polycubes of size n
pub fn generate_polycubes(n: u8, use_cache: bool) -> Vec<Polycube> {
    if n < 1 {
        return Vec::new();
    } else if n == 1 {
        return vec![Polycube::unit_cube()];
    } else if n == 2 {
        return vec![Polycube::domino()];
    }

    // Check cache file
    let cache_path = format!("cubes_{}.zst", n);
    if use_cache && Path::new(&cache_path).exists() {
        println!("Loading polycubes n={} from cache", n);
        match load_from_cache(&cache_path) {
            Ok(polycubes) => {
                println!("Loaded {} shapes", polycubes.len());
                return polycubes;
            }
            Err(e) => {
                println!("Error loading cache: {}", e);
            }
        }
    }

    // Get base polycubes (n-1)
    let base_cubes = generate_polycubes(n - 1, use_cache);
    
    // Empty list of new n-polycubes
    let unique_forms = Mutex::new(FxHashSet::default());
    
    let total = base_cubes.len();
    println!("Processing {} base polycubes of size {}", total, n-1);
    
    // Use rayon for parallel processing
    let progress = AtomicUsize::new(0);
    
    // Generate new shapes in parallel
    let results: Vec<_> = base_cubes.par_iter().flat_map(|base_cube| {
        // Get expansion positions
        let positions = base_cube.get_expansion_positions();
        let mut local_polycubes = Vec::new();
        
        for position in positions {
            // Create expanded shape
            let expanded_shape = base_cube.expand(position);
            
            // Skip if not face-connected
            if !expanded_shape.is_face_connected() {
                continue;
            }
            
            // Normalize
            let normalized = expanded_shape.normalize();
            
            // Get canonical form and check for uniqueness
            let canonical = normalized.get_canonical_form();
            
            // Try to add to global uniqueness set
            let mut unique_forms_guard = unique_forms.lock().unwrap();
            if unique_forms_guard.insert(canonical) {
                local_polycubes.push(normalized);
            }
        }
        
        // Update progress
        let idx = progress.fetch_add(1, Ordering::SeqCst);
        if idx % 100 == 0 {
            print!("\rGenerating polycubes n={}: {:.1}%", n, (idx as f32 / total as f32) * 100.0);
            let _ = std::io::stdout().flush();
        }
        
        local_polycubes
    }).collect();
    
    // Combine results
    let polycubes: Vec<Polycube> = results;
    
    println!("\rGenerating polycubes n={}: 100%", n);
    println!("Found {} unique polycubes", polycubes.len());
    
    // Cache results
    if use_cache {
        println!("Saving to cache...");
        match save_to_cache(&polycubes, &cache_path) {
            Ok(_) => println!("Saved to cache successfully"),
            Err(e) => println!("Error saving to cache: {}", e)
        }
    }
    
    polycubes
}

// Save polycubes to compressed cache
fn save_to_cache(polycubes: &[Polycube], path: &str) -> Result<(), std::io::Error> {
    let serialized = bincode::serialize(polycubes)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    // Create a file with zstd encoder
    let file = File::create(path)?;
    let mut encoder = zstd::Encoder::new(file, 3)?;
    
    // Write the serialized data
    encoder.write_all(&serialized)?;
    
    // Finish the compression
    encoder.finish()?;
    
    Ok(())
}

// Load polycubes from compressed cache
fn load_from_cache(path: &str) -> Result<Vec<Polycube>, std::io::Error> {
    let file = File::open(path)?;
    let mut decoder = zstd::Decoder::new(file)?;
    
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    
    bincode::deserialize(&decompressed)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

// Known counts for validation
pub fn get_known_count(n: u8) -> Option<u64> {
    match n {
        1 => Some(1),
        2 => Some(1),
        3 => Some(2),
        4 => Some(8),
        5 => Some(29),
        6 => Some(166),
        7 => Some(1023),
        8 => Some(6922),
        9 => Some(48311),
        10 => Some(346543),
        11 => Some(2522522),
        12 => Some(18598427),
        13 => Some(139333147),
        14 => Some(1056657611),
        15 => Some(8107839447),
        16 => Some(62709211271),
        17 => Some(489997729602),
        18 => Some(3847265309118),
        _ => None,
    }
}

// Generate summary statistics
pub fn generate_summary(polycubes: &[Polycube]) {
    if polycubes.is_empty() {
        println!("Summary: No polycubes to analyze");
        return;
    }
    
    // Count 1D, 2D, and 3D shapes
    let mut flat_count = 0;
    let mut linear_count = 0;
    
    for polycube in polycubes {
        if polycube.is_flat() {
            flat_count += 1;
            
            if polycube.is_linear() {
                linear_count += 1;
            }
        } else if polycube.is_linear() {
            linear_count += 1;
        }
    }
    
    let three_d_count = polycubes.len() - flat_count;
    
    println!("\nSummary:");
    println!("  1D Linear shapes: {}", linear_count);
    println!("  2D Flat shapes: {}", flat_count - linear_count);
    println!("  3D shapes: {}", three_d_count);
    
    // Calculate max dimension
    let max_dim = polycubes.iter().map(|p| {
        let (max_x, max_y, max_z) = p.get_dimensions();
        std::cmp::max(max_x as i32, std::cmp::max(max_y as i32, max_z as i32))
    }).max().unwrap();
    
    println!("\n  Maximum dimension: {}", max_dim);
    
    // Distribution by max dimension
    let mut dim_counts = vec![0; max_dim as usize + 1];
    
    for polycube in polycubes {
        let (max_x, max_y, max_z) = polycube.get_dimensions();
        let dim = std::cmp::max(max_x as i32, std::cmp::max(max_y as i32, max_z as i32));
        
        dim_counts[dim as usize] += 1;
    }
    
    println!("  Distribution by maximum dimension:");
    for (dim, count) in dim_counts.iter().enumerate() {
        if *count > 0 {
            println!("    Max dim {}: {} shapes", dim, count);
        }
    }
}