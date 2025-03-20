use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::collections::HashSet;
use std::cmp::Ordering;

use crate::polycube::{Polycube, Pos};
use crate::generator::get_known_count;

#[derive(Clone)]
pub struct PolycubeMetrics {
    pub dimension_x: i8,
    pub dimension_y: i8,
    pub dimension_z: i8,
    pub is_linear: bool,
    pub is_flat: bool,
    pub surface_area: usize,
    pub volume: usize,
    pub shape_type: String,
    pub average_connectivity: f32,
}

#[derive(Clone)]
pub struct CatalogEntry {
    pub polycube: Polycube,
    pub metrics: PolycubeMetrics,
}

pub struct SummaryData {
    pub linear_count: usize,
    pub planar_count: usize,
    pub three_d_count: usize,
    pub single_layer_count: usize,
    pub multi_layer_count: usize,
}

// Export polycubes to CSV format
pub fn export_to_csv(polycubes: &[Polycube], n: u8) -> io::Result<String> {
    let filename = format!("polycubes_{}.csv", n);
    println!("Exporting {} polycubes to {}...", polycubes.len(), filename);
    
    // Verification against known counts
    if let Some(expected) = get_known_count(n) {
        println!("Expected count for n={}: {}", n, expected);
        println!("Found count: {}", polycubes.len());

        if polycubes.len() < expected as usize {
            println!("WARNING: Missing {} polycubes!", expected as usize - polycubes.len());
        } else if polycubes.len() > expected as usize {
            println!("WARNING: Found {} extra polycubes!", polycubes.len() - expected as usize);
        } else {
            println!("Count matches expected value.");
        }
    }
    
    // Calculate metrics for each polycube
    let catalog = create_catalog(polycubes);
    
    let file = File::create(&filename)?;
    let mut writer = BufWriter::new(file);
    
    // Write CSV header
    writeln!(writer, "ID,ShapeType,DimensionX,DimensionY,DimensionZ,SurfaceArea,Connectivity,CubeX,CubeY,CubeZ")?;
    
    // Write each polycube with its metrics
    for (i, entry) in catalog.iter().enumerate() {
        let metrics = &entry.metrics;
        
        for pos in &entry.polycube.cubes {
            writeln!(writer, 
                "{},{},{},{},{},{},{:.2},{},{},{}", 
                i + 1, 
                metrics.shape_type,
                metrics.dimension_x,
                metrics.dimension_y,
                metrics.dimension_z,
                metrics.surface_area,
                metrics.average_connectivity,
                pos.x, pos.y, pos.z
            )?;
        }
    }
    
    writer.flush()?;
    println!("Export to CSV complete!");
    Ok(filename)
}

pub fn export_to_text_file(polycubes: &[Polycube], n: u8) -> io::Result<()> {
    if n >= 7 {
        println!("Warning: Export to text file is only available for n < 7 due to the large number of shapes.");
        println!("Found {} polycubes of size {}.", polycubes.len(), n);
        return Ok(());
    }

    let filename = format!("polycubes_{}.txt", n);
    println!("Exporting {} polycubes to {}...", polycubes.len(), filename);

    // Verification against known counts
    if let Some(expected) = get_known_count(n) {
        println!("Expected count for n={}: {}", n, expected);
        println!("Found count: {}", polycubes.len());

        if polycubes.len() < expected as usize {
            println!("WARNING: Missing {} polycubes!", expected as usize - polycubes.len());
        } else if polycubes.len() > expected as usize {
            println!("WARNING: Found {} extra polycubes!", polycubes.len() - expected as usize);
        } else {
            println!("Count matches expected value.");
        }
    }

    let file = File::create(&filename)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "Polycubes of size {} - Total count: {}", n, polycubes.len())?;
    writeln!(writer, "{}", "=".repeat(50))?;

    // Calculate summary data
    let catalog_entries = create_catalog(polycubes);
    let summary = get_summary_data(&catalog_entries);

    // Write summary information
    writeln!(writer, "Summary Information:")?;
    writeln!(writer, "  1D Linear shapes: {}", summary.linear_count)?;
    writeln!(writer, "  2D Planar shapes: {}", summary.planar_count)?;
    writeln!(writer, "  3D shapes: {}", summary.three_d_count)?;
    writeln!(writer)?;
    writeln!(writer, "  Single-layer shapes: {}", summary.single_layer_count)?;
    writeln!(writer, "  Multi-layer shapes: {}", summary.multi_layer_count)?;
    writeln!(writer)?;

    let ordered_polycubes = order_polycubes(&catalog_entries);
    writeln!(writer, "Shapes organized systematically ({} total)", ordered_polycubes.len())?;
    writeln!(writer, "{}", "-".repeat(50))?;
    writeln!(writer)?;

    let mut current_dimension = -1;
    let mut current_shape_type = String::new();

    for (i, entry) in ordered_polycubes.iter().enumerate() {
        let dimensionality = get_dimensionality_order(&entry.metrics);

        // Print section headers
        if current_dimension != dimensionality {
            current_dimension = dimensionality;
            current_shape_type = String::new();

            writeln!(writer)?;
            match dimensionality {
                1 => writeln!(writer, "1D LINEAR SHAPES")?,
                2 => writeln!(writer, "2D PLANAR SHAPES")?,
                3 => writeln!(writer, "3D SHAPES")?,
                _ => {}
            }
            writeln!(writer, "{}", "-".repeat(30))?;
        }

        // Print shape type headers
        if current_shape_type != entry.metrics.shape_type {
            current_shape_type = entry.metrics.shape_type.clone();
            writeln!(writer)?;
            writeln!(writer, "-- {} Shapes --", entry.metrics.shape_type)?;
        }

        writeln!(writer)?;
        writeln!(writer, "Polycube #{}", i + 1)?;
        writeln!(writer, "Type: {}, Dimensions: {}×{}×{}", 
            entry.metrics.shape_type, 
            entry.metrics.dimension_x, 
            entry.metrics.dimension_y, 
            entry.metrics.dimension_z)?;
        
        writeln!(writer, "S/V Ratio: {:.2}, Connectivity: {:.1}", 
            entry.metrics.surface_area as f32 / entry.metrics.volume as f32,
            entry.metrics.average_connectivity)?;

        // Write cubes in the polycube
        let mut cube_strs = Vec::new();
        for pos in &entry.polycube.cubes {
            cube_strs.push(format!("({},{},{})", pos.x, pos.y, pos.z));
        }
        cube_strs.sort();
        writeln!(writer, "Cubes: {}", cube_strs.join(", "))?;

        // Write ASCII representation of the polycube
        writeln!(writer, "{}", polycube_to_string(&entry.polycube))?;
        writeln!(writer, "{}", "-".repeat(40))?;
    }

    writer.flush()?;
    println!("Export to text file complete!");
    Ok(())
}

fn create_catalog(polycubes: &[Polycube]) -> Vec<CatalogEntry> {
    polycubes.iter().map(|polycube| {
        let metrics = calculate_metrics(polycube);
        CatalogEntry {
            polycube: polycube.clone(),
            metrics,
        }
    }).collect()
}

fn calculate_metrics(polycube: &Polycube) -> PolycubeMetrics {
    let min_x = polycube.cubes.iter().map(|p| p.x).min().unwrap_or(0);
    let max_x = polycube.cubes.iter().map(|p| p.x).max().unwrap_or(0);
    let min_y = polycube.cubes.iter().map(|p| p.y).min().unwrap_or(0);
    let max_y = polycube.cubes.iter().map(|p| p.y).max().unwrap_or(0);
    let min_z = polycube.cubes.iter().map(|p| p.z).min().unwrap_or(0);
    let max_z = polycube.cubes.iter().map(|p| p.z).max().unwrap_or(0);
    
    let dimension_x = max_x - min_x + 1;
    let dimension_y = max_y - min_y + 1;
    let dimension_z = max_z - min_z + 1;
    
    // Check if it's a linear shape
    let is_linear = dimension_x == 1 && dimension_y == 1 || 
                    dimension_x == 1 && dimension_z == 1 || 
                    dimension_y == 1 && dimension_z == 1;
    
    // Check if it's a flat shape
    let is_flat = dimension_x == 1 || dimension_y == 1 || dimension_z == 1;
    
    // Calculate surface area (count of exposed faces)
    let positions: HashSet<_> = polycube.cubes.iter().cloned().collect();
    let mut surface_area = 0;
    
    for pos in &positions {
        // Check each of the 6 possible faces
        if !positions.contains(&Pos::new(pos.x + 1, pos.y, pos.z)) { surface_area += 1; }
        if !positions.contains(&Pos::new(pos.x - 1, pos.y, pos.z)) { surface_area += 1; }
        if !positions.contains(&Pos::new(pos.x, pos.y + 1, pos.z)) { surface_area += 1; }
        if !positions.contains(&Pos::new(pos.x, pos.y - 1, pos.z)) { surface_area += 1; }
        if !positions.contains(&Pos::new(pos.x, pos.y, pos.z + 1)) { surface_area += 1; }
        if !positions.contains(&Pos::new(pos.x, pos.y, pos.z - 1)) { surface_area += 1; }
    }
    
    // Calculate average connectivity
    let internal_connections = (polycube.cubes.len() * 6) - surface_area;
    let avg_connectivity = internal_connections as f32 / polycube.cubes.len() as f32;
    
    // Determine shape type
    let shape_type = if is_linear {
        "Linear".to_string()
    } else if is_flat {
        "Flat".to_string()
    } else {
        "3D".to_string()
    };
    
    PolycubeMetrics {
        dimension_x,
        dimension_y,
        dimension_z,
        is_linear,
        is_flat,
        surface_area,
        volume: polycube.cubes.len(),
        shape_type,
        average_connectivity: avg_connectivity,
    }
}

fn get_summary_data(catalog: &[CatalogEntry]) -> SummaryData {
    let mut linear_count = 0;
    let mut planar_count = 0;
    let mut single_layer_count = 0;
    
    for entry in catalog {
        if entry.metrics.is_linear {
            linear_count += 1;
        } else if entry.metrics.is_flat {
            planar_count += 1;
        }
        
        if entry.metrics.dimension_z == 1 || 
           entry.metrics.dimension_y == 1 || 
           entry.metrics.dimension_x == 1 {
            single_layer_count += 1;
        }
    }
    
    SummaryData {
        linear_count,
        planar_count,
        three_d_count: catalog.len() - linear_count - planar_count,
        single_layer_count,
        multi_layer_count: catalog.len() - single_layer_count,
    }
}

fn order_polycubes(catalog: &[CatalogEntry]) -> Vec<CatalogEntry> {
    let mut ordered = catalog.to_vec();
    ordered.sort_by(|a, b| {
        let dim_a = get_dimensionality_order(&a.metrics);
        let dim_b = get_dimensionality_order(&b.metrics);
        
        if dim_a != dim_b {
            return dim_a.cmp(&dim_b);
        }
        
        // Then sort by shape type
        let type_cmp = a.metrics.shape_type.cmp(&b.metrics.shape_type);
        if type_cmp != Ordering::Equal {
            return type_cmp;
        }
        
        // Finally sort by volume
        a.metrics.volume.cmp(&b.metrics.volume)
    });
    
    ordered
}

fn get_dimensionality_order(metrics: &PolycubeMetrics) -> i32 {
    if metrics.is_linear {
        return 1; // 1D
    }
    if metrics.is_flat {
        return 2; // 2D
    }
    3 // 3D
}

fn polycube_to_string(polycube: &Polycube) -> String {
    if polycube.cubes.is_empty() {
        return String::from("Empty polycube");
    }
    
    let min_x = polycube.cubes.iter().map(|p| p.x).min().unwrap();
    let max_x = polycube.cubes.iter().map(|p| p.x).max().unwrap();
    let min_y = polycube.cubes.iter().map(|p| p.y).min().unwrap();
    let max_y = polycube.cubes.iter().map(|p| p.y).max().unwrap();
    let min_z = polycube.cubes.iter().map(|p| p.z).min().unwrap();
    let max_z = polycube.cubes.iter().map(|p| p.z).max().unwrap();
    
    // Convert to a set for quick lookup
    let positions: HashSet<_> = polycube.cubes.iter().cloned().collect();
    
    // Create ASCII representation
    let mut result = String::new();
    
    // For a simple representation, we'll draw layers from top to bottom
    for z in min_z..=max_z {
        result.push_str(&format!("Layer z={}\n", z));
        
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if positions.contains(&Pos::new(x, y, z)) {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        result.push('\n');
    }
    
    result
}