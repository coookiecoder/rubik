use crate::move_table::*;
use crate::pruning_table;

#[derive(PartialEq)]
pub enum Move {
    U, UPrime, U2,
    R, RPrime, R2,
    F, FPrime, F2,
    D, DPrime, D2,
    L, LPrime, L2,
    B, BPrime, B2,
    None
}

impl Move {
    pub fn from_str(s: String) -> Self {
        match s.as_str() {
            "U" => Move::U,
            "U'" | "U’" => Move::UPrime,
            "R" => Move::R,
            "R'" | "R’" => Move::RPrime,
            "F" => Move::F,
            "F'" | "F’" => Move::FPrime,
            "D" => Move::D,
            "D'" | "D’" => Move::DPrime,
            "L" => Move::L,
            "L'" | "L’" => Move::LPrime,
            "B" => Move::B,
            "B'" | "B’" => Move::BPrime,
            _ => Move::None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Cube {
    pub corners_perm: [usize; 8],
    pub corners_orient: [u8; 8],

    pub edges_perm: [usize; 12],
    pub edges_orient: [u8; 12],
}

impl Cube {
    pub fn new_solved() -> Self {
        Self {
            corners_perm: [0,1,2,3,4,5,6,7],
            corners_orient: [0; 8],
            edges_perm: [0,1,2,3,4,5,6,7,8,9,10,11],
            edges_orient: [0; 12],
        }
    }

    pub fn apply_move_table(&mut self, table: &MoveTable) {
        let old_c_perm = self.corners_perm;
        let old_c_orient = self.corners_orient;
        let old_e_perm = self.edges_perm;
        let old_e_orient = self.edges_orient;

        // Update corners
        for i in 0..8 {
            let old_pos = table.corner_perm[i];
            self.corners_perm[i] = old_c_perm[old_pos];
            // Add orientation delta mod 3 (corner twists)
            self.corners_orient[i] = (old_c_orient[old_pos] + table.corner_orient_delta[i]) % 3;
        }

        // Update edges
        for i in 0..12 {
            let old_pos = table.edge_perm[i];
            self.edges_perm[i] = old_e_perm[old_pos];
            // Add orientation delta mod 2 (edge flips)
            self.edges_orient[i] = (old_e_orient[old_pos] + table.edge_orient_delta[i]) % 2;
        }
    }

    fn apply_move_n(&mut self, table: &MoveTable, n: usize) {
        for _ in 0..n {
            self.apply_move_table(table);
        }
    }

    pub fn apply_move(&mut self, mv: &Move) {
        match mv {
            Move::U => self.apply_move_table(&U_MOVE),
            Move::UPrime => self.apply_move_n(&U_MOVE, 3),
            Move::U2 => self.apply_move_n(&U_MOVE, 2),

            Move::R => self.apply_move_table(&R_MOVE),
            Move::RPrime => self.apply_move_n(&R_MOVE, 3),
            Move::R2 => self.apply_move_n(&R_MOVE, 2),

            Move::F => self.apply_move_table(&F_MOVE),
            Move::FPrime => self.apply_move_n(&F_MOVE, 3),
            Move::F2 => self.apply_move_n(&F_MOVE, 2),

            Move::D => self.apply_move_table(&D_MOVE),
            Move::DPrime => self.apply_move_n(&D_MOVE, 3),
            Move::D2 => self.apply_move_n(&D_MOVE, 2),

            Move::L => self.apply_move_table(&L_MOVE),
            Move::LPrime => self.apply_move_n(&L_MOVE, 3),
            Move::L2 => self.apply_move_n(&L_MOVE, 2),

            Move::B => self.apply_move_table(&B_MOVE),
            Move::BPrime => self.apply_move_n(&B_MOVE, 3),
            Move::B2 => self.apply_move_n(&B_MOVE, 2),
            
            Move::None => unreachable!(),
        }
    }

    pub fn is_solved(&self) -> bool {
        self.corners_perm == [0,1,2,3,4,5,6,7]
            && self.corners_orient.iter().all(|&o| o == 0)
            && self.edges_perm == [0,1,2,3,4,5,6,7,8,9,10,11]
            && self.edges_orient.iter().all(|&o| o == 0)
    }

    fn solve(&self) -> Option<Vec<Move>> {
        // 1. Encode the current cube state into coordinates
        let corner_ori_idx = self.corner_orientation_index();
        let edge_ori_idx = self.edge_orientation_index();
        let corner_perm_idx = self.corner_permutation_index();
        let edge_perm_idx = self.edge_permutation_index();

        // 2. Initialize or load pruning tables (not shown here)
        let pruning_table = pruning_table::PruningTable::load_or_generate("pruning_table.bin").unwrap();

        // 3. Use IDA* or other search:
        //    - Start from (corner_ori_idx, edge_ori_idx, corner_perm_idx, edge_perm_idx)
        //    - Use pruning tables to guide and prune search space
        //    - Explore sequences of moves until solved state is found

        // 4. Return the found move sequence
        None // placeholder
    }

}