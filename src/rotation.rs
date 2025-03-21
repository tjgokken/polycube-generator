use crate::polycube::{Polycube, Pos};

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

    // Get canonical form for uniqueness testing
    pub fn get_canonical_form(&self) -> String {
        let rotations = all_rotations(self);
        
        // Find the lexicographically smallest representation
        let mut smallest: Option<String> = None;
        
        for rotation in &rotations {
            // Generate string representation
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
            
            let representation: String = positions.iter()
                .map(|pos| format!("{},{},{};", pos.x, pos.y, pos.z))
                .collect();
            
            if smallest.is_none() || representation < smallest.clone().unwrap() {
                smallest = Some(representation);
            }
        }
        
        smallest.unwrap()
    }
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