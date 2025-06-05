use crate::cube::Cube;

impl Cube {
    // Corner Orientation Index (base 3 for first 7 corners)
    pub fn corner_orientation_index(&self) -> usize {
        let mut index = 0;
        for i in 0..7 {
            index = index * 3 + (self.corners_orient[i] as usize);
        }
        index
    }

    pub fn set_corner_orientation_from_index(&mut self, mut index: usize) {
        let mut sum = 0u8;
        for i in (0..7).rev() {
            self.corners_orient[i] = (index % 3) as u8;
            sum += self.corners_orient[i];
            index /= 3;
        }
        self.corners_orient[7] = (3 - (sum % 3)) % 3;
    }

    // Edge Orientation Index (binary for first 11 edges)
    pub fn edge_orientation_index(&self) -> usize {
        let mut index = 0;
        for i in 0..11 {
            index = (index << 1) | (self.edges_orient[i] as usize);
        }
        index
    }

    pub fn set_edge_orientation_from_index(&mut self, mut index: usize) {
        let mut sum = 0u8;
        for i in (0..11).rev() {
            self.edges_orient[i] = (index & 1) as u8;
            sum += self.edges_orient[i];
            index >>= 1;
        }
        self.edges_orient[11] = (2 - (sum % 2)) % 2;
    }

    // Corner Permutation Index using Lehmer code
    pub fn corner_permutation_index(&self) -> usize {
        let mut index = 0;
        let mut used = [false; 8];
        for i in 0..8 {
            let mut smaller = 0;
            for j in 0..self.corners_perm[i] {
                if !used[j] {
                    smaller += 1;
                }
            }
            index = index * (8 - i) + (smaller - 1);
            used[self.corners_perm[i]] = true;
        }
        index
    }

    pub fn set_corner_permutation_from_index(&mut self, mut index: usize) {
        let mut available: Vec<usize> = (0..8).collect();
        for i in (0..8).rev() {
            let fact = factorial(i);
            let pos = index / fact;
            index %= fact;
            self.corners_perm[7 - i] = available.remove(pos);
        }
    }

    // Edge Permutation Index using Lehmer code
    pub fn edge_permutation_index(&self) -> usize {
        let mut index = 0;
        let mut used = [false; 12];
        for i in 0..12 {
            let mut smaller = 0;
            for j in 0..self.edges_perm[i] {
                if !used[j] {
                    smaller += 1;
                }
            }
            index = index * (12 - i) + (smaller - 1);
            used[self.edges_perm[i]] = true;
        }
        index
    }

    pub fn set_edge_permutation_from_index(&mut self, mut index: usize) {
        let mut available: Vec<usize> = (0..12).collect();
        for i in (0..12).rev() {
            let fact = factorial(i);
            let pos = index / fact;
            index %= fact;
            self.edges_perm[11 - i] = available.remove(pos);
        }
    }
}

// Helper factorial function outside the impl
fn factorial(n: usize) -> usize {
    (1..=n).product::<usize>().max(1)
}
