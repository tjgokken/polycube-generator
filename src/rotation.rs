use crate::polycube::{Polycube, Pos};
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

// Apply a rotation matrix to this polycube
impl Polycube {
    #[inline]
    pub fn apply_rotation(&self, rotation: &[[i8; 3]; 3]) -> Self {
        let new_cubes = self.cubes.iter().map(|p| {
            let x = rotation[0][0] * p.x + rotation[0][1] * p.y + rotation[0][2] * p.z;
            let y = rotation[1][0] * p.x + rotation[1][1] * p.y + rotation[1][2] * p.z;
            let z = rotation[2][0] * p.x + rotation[2][1] * p.y + rotation[2][2] * p.z;
            Pos::new(x, y, z)
        }).collect();

        Self::new(new_cubes)
    }

    // Get canonical form hash for uniqueness testing
    // Returns a 64-bit hash of the canonicalized polycube
    pub fn get_canonical_hash(&self) -> u64 {
        let rotations = all_rotations(self);
        
        // Find the lexicographically smallest rotation
        let mut smallest: Option<Vec<Pos>> = None;
        
        for rotation in &rotations {
            // Sort positions for consistent ordering
            let mut positions: Vec<_> = rotation.cubes.clone();
            positions.sort_by(|a, b| {
                match a.x.cmp(&b.x) {
                    std::cmp::Ordering::Equal => match a.y.cmp(&b.y) {
                        std::cmp::Ordering::Equal => a.z.cmp(&b.z),
                        other => other,
                    },
                    other => other,
                }
            });
            
            // If this is the first rotation or it's smaller than the current smallest
            if smallest.is_none() || lexicographically_smaller(&positions, smallest.as_ref().unwrap()) {
                smallest = Some(positions);
            }
        }
        
        // Compute a 64-bit hash of the canonical form
        let canonical_positions = smallest.unwrap();
        let mut hasher = FxHasher::default();
        canonical_positions.hash(&mut hasher);
        hasher.finish()
    }   
}

// Helper function to compare position vectors lexicographically
#[inline]
fn lexicographically_smaller(a: &[Pos], b: &[Pos]) -> bool {
    let len = a.len().min(b.len());
    
    for i in 0..len {
        // Compare x coordinates
        match a[i].x.cmp(&b[i].x) {
            std::cmp::Ordering::Less => return true,
            std::cmp::Ordering::Greater => return false,
            _ => {}
        }
        
        // If x is equal, compare y
        match a[i].y.cmp(&b[i].y) {
            std::cmp::Ordering::Less => return true,
            std::cmp::Ordering::Greater => return false,
            _ => {}
        }
        
        // If y is equal, compare z
        match a[i].z.cmp(&b[i].z) {
            std::cmp::Ordering::Less => return true,
            std::cmp::Ordering::Greater => return false,
            _ => {}
        }
    }
    
    // If all compared elements are equal, the shorter array is lexicographically smaller
    a.len() < b.len()
}

// Generate all 24 rotations of a polycube
pub fn all_rotations(polycube: &Polycube) -> Vec<Polycube> {
    let rotation_matrices = generate_rotation_matrices();
    let mut rotations = Vec::with_capacity(24);
    
    for rotation in &rotation_matrices {
        let rotated = polycube.apply_rotation(rotation);
        rotations.push(rotated.normalize());
    }
    
    rotations
}

// Generate all 24 rotation matrices
pub fn generate_rotation_matrices() -> Vec<[[i8; 3]; 3]> {
    let mut matrices = Vec::with_capacity(24);
    
    // +X face rotations
    matrices.push([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    matrices.push([[1, 0, 0], [0, 0, -1], [0, 1, 0]]);
    matrices.push([[1, 0, 0], [0, -1, 0], [0, 0, -1]]);
    matrices.push([[1, 0, 0], [0, 0, 1], [0, -1, 0]]);
    
    // -X face rotations
    matrices.push([[-1, 0, 0], [0, 1, 0], [0, 0, -1]]);
    matrices.push([[-1, 0, 0], [0, 0, 1], [0, 1, 0]]);
    matrices.push([[-1, 0, 0], [0, -1, 0], [0, 0, 1]]);
    matrices.push([[-1, 0, 0], [0, 0, -1], [0, -1, 0]]);
    
    // +Y face rotations
    matrices.push([[0, 1, 0], [-1, 0, 0], [0, 0, 1]]);
    matrices.push([[0, 1, 0], [0, 0, -1], [-1, 0, 0]]);
    matrices.push([[0, 1, 0], [1, 0, 0], [0, 0, -1]]);
    matrices.push([[0, 1, 0], [0, 0, 1], [1, 0, 0]]);
    
    // -Y face rotations
    matrices.push([[0, -1, 0], [1, 0, 0], [0, 0, 1]]);
    matrices.push([[0, -1, 0], [0, 0, -1], [1, 0, 0]]);
    matrices.push([[0, -1, 0], [-1, 0, 0], [0, 0, -1]]);
    matrices.push([[0, -1, 0], [0, 0, 1], [-1, 0, 0]]);
    
    // +Z face rotations
    matrices.push([[0, 0, 1], [0, 1, 0], [-1, 0, 0]]);
    matrices.push([[0, 0, 1], [1, 0, 0], [0, 1, 0]]);
    matrices.push([[0, 0, 1], [0, -1, 0], [1, 0, 0]]);
    matrices.push([[0, 0, 1], [-1, 0, 0], [0, -1, 0]]);
    
    // -Z face rotations
    matrices.push([[0, 0, -1], [0, 1, 0], [1, 0, 0]]);
    matrices.push([[0, 0, -1], [-1, 0, 0], [0, 1, 0]]);
    matrices.push([[0, 0, -1], [0, -1, 0], [-1, 0, 0]]);
    matrices.push([[0, 0, -1], [1, 0, 0], [0, -1, 0]]);
    
    matrices
}