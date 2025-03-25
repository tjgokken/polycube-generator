# Fast Polycube Counting: Algorithm Exploration and Insights

## Background

Based on Stanley Dodds' algorithm, this is the exploration of implementing a memory-safe counting algorithm that could efficiently calculate the number of polycubes without explicitly generating them. This document outlines the learnings from this exploration, including insights into why specialized algorithms like Dodds's are so effective.

## The Promise and Challenge of Counting Algorithms

### Theoretical Advantages

In theory, counting algorithms should be faster than generation algorithms because:

1. They don't need to store each polycube, only track their count
2. They can employ mathematical optimizations specific to counting
3. They can avoid redundant calculations and employ pruning strategies

### The Dodds Algorithm

Stanley Dodds's algorithm (and Phil Thompson's Rust port) achieves remarkable performance using several key techniques:

```rust
const X: i32 = (n as i32 + 5) / 4 * ((n as i32 + 5) / 4 * 3 - 2);
const Y: i32 = X + 1;
const Z: i32 = X + (n as i32 + 5) / 4 * 3;
```

These constants enable an elegant 1D encoding of 3D space, where any linear combination aX + bY + cZ = 0 implies either a = b = c = 0 or |a|+|b|+|c| > n.

This mathematical insight, combined with unsafe pointer manipulation and bit-level optimizations, allows Dodds's algorithm to count polycubes up to n=18 and beyond.

## Memory-Safe Implementation Attempt

I attempted to create a memory-safe alternative using Rust's type system and safe abstractions:

### Key Optimizations Employed

1. **FxHashSet**: Used a faster hashing algorithm for better performance
2. **SmallVec**: Implemented stack-allocated vectors to reduce heap allocations
3. **Canonicalization**: Developed efficient position normalization for deduplication
4. **In-place Operations**: Minimized memory allocations through in-place modifications
5. **Parallelization**: Distributed work across multiple cores for better performance

### Performance Results

This memory-safe implementation showed intriguing performance characteristics:

- **For n=11 (2.5 million polycubes)**: Completed almost instantly (<1 second)
- **For n=12 (18.5 million polycubes)**: Took over 11 minutes with high CPU usage

By comparison:
- My generation algorithm (based on the original Computer Philes Youtube video by Dr. Mike Pound ) processed n=12 in about 13 minutes while actually generating all polycubes
- Dodds's algorithm can process n=12 in seconds to minutes using unsafe optimizations

## Why Safety vs. Performance Is a Real Trade-off

### 1. Memory Access Patterns

Safe Rust enforces bounds checking and ownership rules, which can introduce overhead in tight loops. Dodds's algorithm uses direct pointer arithmetic to:
- Access memory with zero bounds-checking overhead
- Manipulate data structures with minimal indirection
- Implement bit-level operations that are difficult to express safely

### 2. Data Structure Efficiency

The safe implementation used:
- `FxHashSet` for uniqueness checking (still has overhead)
- `SmallVec` for stack allocation (still involves more complex logic)
- Canonical form calculations (requires sorting and transformations)

Dodds's algorithm uses:
- Flat byte arrays with direct manipulation
- Implicit representation of polycubes via positional encoding
- Mathematical shortcuts that avoid explicit data structure operations

### 3. Algorithm Structure

Safe code approach followed a more traditional recursive enumeration with pruning. Dodds's algorithm uses a hybrid approach that:
- Recursively builds partial polycubes to a certain depth
- Switches to mathematical counting formulas for batches of configurations
- Utilizes sophisticated bit manipulation for counting many cases at once

## Conclusion: The Right Tool for the Job

The key takeaway is that different approaches excel in different contexts:

1. **Generation Algorithm**: Best when you need the actual polycubes for visualization or analysis as was the original goal
2. **Memory-Safe Counting**: Theoretically appealing but faces practical performance limitations
3. **Specialized Unsafe Algorithms**: Necessary for pushing the boundaries of what's computationally feasible

This exploration didn't yield the faster alternative I hoped for, but it provided valuable insights into the nature of complex calculations and the real-world trade-offs between memory safety and performance.