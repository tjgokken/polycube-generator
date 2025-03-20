use std::env;
use std::io::{self, Write};
use std::time::Instant;

mod polycube;
mod rotation;
mod generator;
mod polycube_exporter;

use generator::{generate_polycubes, get_known_count, generate_summary};

fn main() -> io::Result<()> {
    println!("High-Performance Polycube Generator (Rust)");
    println!("==========================================");
    
    // Parse command line args
    let args: Vec<String> = env::args().collect();
    
    let mut n = 0;
    let mut use_cache = true;
    let mut export_csv = false;
    let mut export_text = false;
    
    if args.len() > 1 {
        n = args[1].parse::<u8>().unwrap_or(0);
        
        for arg in &args {
            if arg == "--no-cache" {
                use_cache = false;
            } else if arg == "--export-csv" {
                export_csv = true;
            } else if arg == "--export-text" {
                export_text = true;
            }
        }
    }
    
    if n == 0 {
        print!("Enter the size of polycubes to generate (1-18): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        n = input.trim().parse::<u8>().unwrap_or(0);
        
        if n < 1 || n > 18 {
            println!("Invalid input. Using default size 5.");
            n = 5;
        }
    }
    
    // Start timing
    let start_time = Instant::now();
    
    // Generate polycubes
    let polycubes = generate_polycubes(n, use_cache);
    
    // Stop timing
    let duration = start_time.elapsed();
    
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
    
    // Show available export options
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
    
    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(())
}