# Polycube Generator

A high-performance polycube generator written in Rust, with CSV export and interactive 3D visualization capabilities.

## What are Polycubes?

Polycubes are three-dimensional shapes formed by connecting unit cubes face-to-face. They are the 3D equivalent of polyominoes. This project efficiently generates all unique polycubes of a given size, using parallel processing and advanced algorithms for rotation checking and canonicalization.

## Features

- Fast generation of polycubes using parallel processing
- Efficient algorithms for detecting unique shapes (handles all 24 possible rotations)
- Caching mechanism to save and load previously generated polycubes
- CSV export format compatible with the included web viewer
- Interactive 3D visualization options (built-in and web-based)
- Detailed shape metrics and comparison tools

## Installation

### Prerequisites

- Rust (1.58 or later)
- A modern web browser (for the web viewer)

### Build from Source

1. Clone this repository
2. Run the build script:
   ```
   ./build.sh
   ```

## Usage

### Generating Polycubes

```bash
# Generate polycubes of size 5
./target/release/polycube-generator 5

# Generate and export to CSV (for web viewer)
./target/release/polycube-generator 5 --export-csv

# Generate and visualize with built-in 3D viewer
./target/release/polycube-generator 5 --visualize

# Generate without using cache
./target/release/polycube-generator 5 --no-cache
```

### Using the Web Viewer

1. Generate polycubes and export to CSV:
   ```bash
   ./target/release/polycube-generator 5 --export-csv
   ```

2. Open `polycube_viewer.html` in your web browser

3. Drag and drop the generated CSV file (`polycubes_5.csv`) onto the viewer, or use the "Choose File" button

4. Explore the shapes using the interactive controls:
   - Use mouse to rotate/zoom the 3D view
   - Filter shapes by category or search by name
   - Use the view controls to switch between preset perspectives
   - Compare shapes to identify potential duplicates

## Performance

The generator uses multiple performance optimization techniques:
- Parallel processing with Rayon
- Efficient bit-packed representation for polycubes
- Fast canonical form generation
- Caching of previously generated shapes
- Optimized rotation operations

## Known Polycube Counts

| Size (n) | Count      | Size (n) | Count        |
|----------|------------|----------|--------------|
| 1        | 1          | 10       | 346,543      |
| 2        | 1          | 11       | 2,522,522    |
| 3        | 2          | 12       | 18,598,427   |
| 4        | 8          | 13       | 139,333,147  |
| 5        | 29         | 14       | 1,056,657,611|
| 6        | 166        | 15       | 8,107,839,447|
| 7        | 1,023      | 16       | 62,709,211,271|
| 8        | 6,922      | 17       | 489,997,729,602|
| 9        | 48,311     | 18       | 3,847,265,309,118|

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the MIT License.