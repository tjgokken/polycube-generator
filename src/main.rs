use std::env;
use std::io::{self, Write};
use std::time::Instant;

mod polycube;
mod rotation;
mod generator;
mod polycube_exporter;
mod safe_counter;

use generator::{generate_polycubes, get_known_count, generate_summary};
use safe_counter::count_polycubes;

fn main() -> io::Result<()> {
    println!("Polycube Generator and Counter (Rust)");
    println!("=====================================");
    
    // Parse command line args
    let args: Vec<String> = env::args().collect();
    
    let mut n = 0;
    let mut use_cache = true;
    let mut export_csv = false;
    let mut export_text = false;
    let mut count_only = false;
    let mut operation_selected = false;
    let mut use_symmetry = true;
    
    if args.len() > 1 {
        n = args[1].parse::<u8>().unwrap_or(0);
        
        for arg in &args {
            if arg == "--no-cache" {
                use_cache = false;
            } else if arg == "--export-csv" {
                export_csv = true;
            } else if arg == "--export-text" {
                export_text = true;
            } else if arg == "--count-only" {
                count_only = true;
                operation_selected = true;
            } else if arg == "--generate" {
                count_only = false;
                operation_selected = true;
            } else if arg == "--no-symmetry" {
                use_symmetry = false;
            }
        }
    }
    
    if n == 0 {
        print!("Enter the size of polycubes (1-18): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        n = input.trim().parse::<u8>().unwrap_or(0);
        
        if n < 1 || n > 18 {
            println!("Invalid input. Using default size 5.");
            n = 5;
        }
    }
    
    if !operation_selected {
        println!("\nSelect operation:");
        println!("  1. Generate and analyze polycubes (best for n ≤ 10)");
        println!("  2. Count polycubes without generating them (fast for large n)");
        
        print!("Enter your choice (1-2): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u8>() {
            Ok(2) => count_only = true,
            _ => count_only = false,
        }
        
        // For large n values, warn the user if they chose to generate
        if !count_only && n > 10 {
            println!("\nWARNING: Generating polycubes for n > 10 may take a long time and use substantial memory.");
            print!("Continue with generation? (y/n, default: n): ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if !input.trim().eq_ignore_ascii_case("y") {
                count_only = true;
                println!("Switching to count-only mode for better performance.");
            }
        }
    }
    
    // Start timing
    let start_time = Instant::now();
    
    // For count-only mode, use the simplified counting algorithm
    if count_only {
        println!("\nUsing simplified counting algorithm for n={}", n);
        
        if use_symmetry {
            println!("Counting free polycubes (accounting for symmetry)");
        } else {
            println!("Counting fixed polycubes (no symmetry consideration)");
        }
        
        let count = count_polycubes(n as usize, use_symmetry);
        
        // Stop timing
        let duration = start_time.elapsed();
        
        println!("\nResults:");
        println!("=========");
        if use_symmetry {
            println!("Count of free polycubes of size {}: {}", n, count);
        } else {
            println!("Count of fixed polycubes of size {}: {}", n, count);
        }
        println!("Time taken: {:.2} seconds", duration.as_secs_f32());
        
        // Check against known count if available
        if let Some(expected) = get_known_count(n) {
            println!("Expected count for size {}: {}", n, expected);
            
            if count != expected {
                println!("WARNING: Count does not match expected value!");
                println!("Difference: {}", if count > expected { 
                    format!("+{}", count - expected) 
                } else { 
                    format!("-{}", expected - count) 
                });
            } else {
                println!("Generated count matches expected count!");
            }
        }
    } else {
        // Generate full polycubes using the original algorithm
        println!("\nGenerating polycubes of size {}...", n);
        let polycubes = generate_polycubes(n, use_cache);
        
        // Stop timing
        let duration = start_time.elapsed();
        
        println!("\nResults:");
        println!("=========");
        println!("Generated {} unique polycubes of size {}", polycubes.len(), n);
        println!("Time taken: {:.2} seconds", duration.as_secs_f32());
        
        // Check against known count
        if let Some(expected) = get_known_count(n) {
            println!("Expected count for size {}: {}", n, expected);
            
            if polycubes.len() < expected as usize {
                println!("WARNING: Missing {} polycubes!", expected as usize - polycubes.len());
            } else if polycubes.len() > expected as usize {
                println!("WARNING: Found {} extra polycubes!", polycubes.len() - expected as usize);
            } else {
                println!("Generated count matches expected count!");
            }
        }
        
        // Generate summary statistics
        generate_summary(&polycubes);
        
        // Export data if requested
        if export_csv {
            match polycube_exporter::export_to_csv(&polycubes, n) {
                Ok(filename) => println!("Exported to CSV file: {}", filename),
                Err(e) => println!("Error exporting to CSV: {}", e),
            }
        }
        
        // Export to text file if requested
        if export_text {
            match polycube_exporter::export_to_text_file(&polycubes, n) {
                Ok(_) => {},
                Err(e) => println!("Error exporting to text file: {}", e),
            }
        }
        
        // Show available export options if not already specified
        if !export_csv && !export_text {
            println!("\nExport options:");
            println!("  1. Export to CSV (for web viewer)");
            println!("  2. Export to text file");
            println!("  3. Skip export");
            
            print!("Choose an option (1-3): ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            match input.trim().parse::<u8>() {
                Ok(1) => {
                    match polycube_exporter::export_to_csv(&polycubes, n) {
                        Ok(filename) => {
                            println!("Exported to CSV file: {}", filename);
                            println!("Use the 'polycube_viewer.html' file to visualize this data.");
                        },
                        Err(e) => println!("Error exporting to CSV: {}", e),
                    }
                },
                Ok(2) => {
                    match polycube_exporter::export_to_text_file(&polycubes, n) {
                        Ok(_) => {},
                        Err(e) => println!("Error exporting to text file: {}", e),
                    }
                },
                _ => {
                    println!("Skipping export.");
                }
            }
        }
    }
    
    println!("\nPress Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(())
}