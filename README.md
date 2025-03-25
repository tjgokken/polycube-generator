# Polycube Generator and Counter

A high-performance polycube toolkit written in Rust, with comprehensive generation, counting, analysis, and visualization capabilities.

## What are Polycubes?

Polycubes are three-dimensional shapes formed by connecting unit cubes face-to-face. They are the 3D equivalent of polyominoes (like Tetris pieces). 

![Polycubes Example](https://upload.wikimedia.org/wikipedia/commons/thumb/b/b1/Tetracube_categories.svg/170px-Tetracube_categories.svg.png)

*Image: The 29 free pentacubes (polycubes with 5 unit cubes). Source: Wikipedia*


This project efficiently generates and counts all unique polycubes of a given size, using parallel processing and advanced algorithms for rotation checking and canonicalization.

## Features

- **Polycube Generation**: Fast generation of all unique polycubes using parallel processing
- **Fixed vs. Free Counting**: Support for counting both fixed polycubes (distinct orientations) and free polycubes (distinct shapes)
- **Efficient Algorithms**: Advanced techniques for detecting unique shapes (handles all 24 possible rotations)
- **Caching Mechanism**: Save and load previously generated polycubes for faster processing
- **Export Options**: CSV and text file exports compatible with the included web viewer
- **Interactive 3D Visualization**: Web-based visualization for exploring generated polycubes
- **Detailed Analysis**: Shape metrics and classification tools

## Installation

### Prerequisites

- Rust (1.58 or later)
- A modern web browser (for the web viewer)

### Build from Source

1. Clone this repository
2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Basic Usage

```bash
# Generate polycubes of size 5
cargo run --release -- 5

# Count polycubes of size 11 (without generating them)
cargo run --release -- 11 --count-only

# Count fixed polycubes (no symmetry consideration)
cargo run --release -- 8 --count-only --no-symmetry
```

### Export and Analysis

```bash
# Generate and export to CSV (for web viewer)
cargo run --release -- 5 --export-csv

# Generate and export detailed text report
cargo run --release -- 5 --export-text

# Generate without using cache
cargo run --release -- 5 --no-cache
```
The console application provides user friendly options.

### Performance Considerations

- For n ≤ 6: Generation is fast and practical and viewing generated objects is feasible
- For n ≤ 10: Generation is fast and practical
- For n = 11-12: Generation is possible but may take several minutes
- For n > 12: Counting-only approach is recommended although at this point I am not sure if it is better. Best to use Dodds' or Thompson's applications
- For n > 16: Specialized algorithms are required (see Phil Thompson's implementation of Dodds's algorithm)

## Understanding Polycube Types

This toolkit works with two types of polycubes:

- **Fixed Polycubes**: Considered different if they have different orientations (OEIS A000162)
- **Free Polycubes**: Considered the same if one can be transformed into the other through rotations or reflections (OEIS A001931)

The toolkit can count both types, with free polycubes requiring symmetry considerations.

## Algorithm Design

The project implements two main approaches:

1. **Generation Algorithm**: For n ≤ 12, a full generation approach enumerates all unique polycubes and can export them for visualization and analysis

2. **Counting Algorithm**: For larger n values, specialized counting algorithms avoid generating the shapes explicitly

My tests showed that the generation algorithm is often more practical for n ≤ 12, while specialized counting algorithms (like Dodds's algorithm) are necessary for larger values.

## Known Polycube Counts

| Size (n) | Fixed Polycubes | Free Polycubes | Size (n) | Fixed Polycubes | Free Polycubes |
|----------|----------------|---------------|----------|----------------|---------------|
| 1        | 1              | 1             | 10       | 346,543        | 346,543       |
| 2        | 1              | 1             | 11       | 2,522,522      | 2,522,522     |
| 3        | 2              | 2             | 12       | 18,598,427     | 18,598,427    |
| 4        | 8              | 8             | 13       | 139,333,147    | 138,462,649   |
| 5        | 29             | 29            | 14       | 1,056,657,611  | 1,039,496,297 |
| 6        | 166            | 166           | 15       | 8,107,839,447  | 7,859,514,470 |
| 7        | 1,023          | 1,023         | 16       | 62,709,211,271 | 59,795,121,480|
| 8        | 6,922          | 6,922         | 17       | 489,997,729,602| 457,409,613,979|
| 9        | 48,311         | 48,311        | 18       | 3,847,265,309,118| 3,516,009,200,564|

## Performance Insights

Our exploration into polycube counting algorithms revealed several interesting insights:

1. **Generation vs. Counting**: While counting should theoretically be faster than generating, the counting apps needed highly sophisticated unsafe coding.

2. **Memory Safety Trade-offs**: The highest-performance algorithms (like Dodds's) use unsafe pointer manipulation and specialized memory layouts for maximum efficiency

3. **Scaling Challenges**: The exponential growth in polycube counts (roughly 7-8x for each increment in n) makes even small inefficiencies compound dramatically

For a deeper discussion, see the algorithm notes in the documentation folder and the blog article at https://tjgokken.com.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the MIT License.

## Credits

Special thanks to:
- Stanley Dodds for his pioneering work on efficient polycube counting algorithms
- Phil Thompson for his Rust port of Dodds's algorithm
- OEIS (Online Encyclopedia of Integer Sequences) for sequences A000162 and A001931