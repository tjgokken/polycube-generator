use std::time::Instant;
use crate::{generate_polycubes, get_known_count};

// Benchmark results structure
#[derive(Debug)]
pub struct BenchmarkResult {
    pub size: u8,
    pub count: usize,
    pub expected: Option<u64>,
    pub time_ms: u128,
    pub matches_expected: bool,
}

pub fn run_benchmarks(max_size: u8, use_cache: bool) -> Vec<BenchmarkResult> {
    let mut results = Vec::new();
    
    println!("\nRunning benchmarks up to size {}:", max_size);
    println!("----------------------------------");
    println!("| Size | Count     | Time (ms) | Match |");
    println!("----------------------------------");
    
    for n in 1..=max_size {
        let start = Instant::now();
        let polycubes = generate_polycubes(n, use_cache);
        let duration = start.elapsed();
        
        let expected = get_known_count(n);
        let matches_expected = match expected {
            Some(count) => polycubes.len() as u64 == count,
            None => true, // No reference count available
        };
        
        let result = BenchmarkResult {
            size: n,
            count: polycubes.len(),
            expected,
            time_ms: duration.as_millis(),
            matches_expected,
        };
        
        println!(
            "| {:4} | {:9} | {:9} | {:5} |",
            n,
            polycubes.len(),
            duration.as_millis(),
            if matches_expected { "✓" } else { "✗" }
        );
        
        results.push(result);
    }
    
    println!("----------------------------------");
    results
}