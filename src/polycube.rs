use std::collections::HashSet;
use serde::{Serialize, Deserialize};

// 3D coordinate type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pos {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Pos {
    pub fn new(x: i8, y: i8, z: i8) -> Self {
        Pos { x, y, z }
    }

    // Get face-adjacent positions
    pub fn adjacent_positions(&self) -> Vec<Pos> {
        vec![
            Pos::new(self.x + 1, self.y, self.z),
            Pos::new(self.x - 1, self.y, self.z),
            Pos::new(self.x, self.y + 1, self.z),
            Pos::new(self.x, self.y - 1, self.z),
            Pos::new(self.x, self.y, self.z + 1),
            Pos::new(self.x, self.y, self.z - 1),
        ]
    }
}

// Polycube representation as a set of positions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Polycube {
    pub cubes: Vec<Pos>,
}

impl Polycube {
    pub fn new(cubes: Vec<Pos>) -> Self {
        Polycube { cubes }
    }

    // Get all possible positions to expand this polycube
    pub fn get_expansion_positions(&self) -> HashSet<Pos> {
        let mut expansion_positions = HashSet::new();
        let current_positions: HashSet<Pos> = self.cubes.iter().cloned().collect();

        for &cube in &self.cubes {
            for adj in cube.adjacent_positions() {
                if !current_positions.contains(&adj) {
                    expansion_positions.insert(adj);
                }
            }
        }

        expansion_positions
    }

    // Expand by adding a cube at the specified position
    pub fn expand(&self, position: Pos) -> Self {
        let mut new_cubes = self.cubes.clone();
        new_cubes.push(position);
        Self::new(new_cubes)
    }

    // Normalize so minimum coordinates are at origin
    pub fn normalize(&self) -> Self {
        if self.cubes.is_empty() {
            return self.clone();
        }

        let min_x = self.cubes.iter().map(|p| p.x).min().unwrap();
        let min_y = self.cubes.iter().map(|p| p.y).min().unwrap();
        let min_z = self.cubes.iter().map(|p| p.z).min().unwrap();

        let new_cubes = self.cubes.iter()
            .map(|p| Pos::new(p.x - min_x, p.y - min_y, p.z - min_z))
            .collect();

        Self::new(new_cubes)
    }

    // Check if polycube is face-connected
    pub fn is_face_connected(&self) -> bool {
        if self.cubes.len() <= 1 {
            return true;
        }

        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        let positions: HashSet<Pos> = self.cubes.iter().cloned().collect();

        // Start with first cube
        queue.push(self.cubes[0]);
        visited.insert(self.cubes[0]);

        // BFS traversal
        while let Some(current) = queue.pop() {
            for adj in current.adjacent_positions() {
                if positions.contains(&adj) && !visited.contains(&adj) {
                    visited.insert(adj);
                    queue.push(adj);
                }
            }
        }

        // Check if all cubes were visited
        visited.len() == self.cubes.len()
    }

    // Create base polycubes
    pub fn unit_cube() -> Self {
        Self::new(vec![Pos::new(0, 0, 0)])
    }

    pub fn domino() -> Self {
        Self::new(vec![Pos::new(0, 0, 0), Pos::new(1, 0, 0)])
    }

    // Calculate shape metrics for reporting
    pub fn get_dimensions(&self) -> (i8, i8, i8) {
        if self.cubes.is_empty() {
            return (0, 0, 0);
        }
        
        let max_x = self.cubes.iter().map(|p| p.x).max().unwrap();
        let max_y = self.cubes.iter().map(|p| p.y).max().unwrap();
        let max_z = self.cubes.iter().map(|p| p.z).max().unwrap();
        
        (max_x + 1, max_y + 1, max_z + 1)
    }

    pub fn is_linear(&self) -> bool {
        let (width, height, depth) = self.get_dimensions();
        (width == 1 && height == 1) || 
        (width == 1 && depth == 1) || 
        (height == 1 && depth == 1)
    }

    pub fn is_flat(&self) -> bool {
        let (width, height, depth) = self.get_dimensions();
        width == 1 || height == 1 || depth == 1
    }
}