use crate::cube::Cube;
use crate::cube::Move;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const CORNER_ORI_SIZE: usize = 3usize.pow(7);
const EDGE_ORI_SIZE: usize = 1 << 11;
const TABLE_SIZE: usize = CORNER_ORI_SIZE * EDGE_ORI_SIZE;

pub struct PruningTable {
    table: Vec<i8>,
}

impl PruningTable {
    pub fn new() -> Self {
        Self {
            table: vec![-1; TABLE_SIZE],
        }
    }

    fn index(corner_ori_idx: usize, edge_ori_idx: usize) -> usize {
        corner_ori_idx * EDGE_ORI_SIZE + edge_ori_idx
    }

    fn generate(&mut self) {
        let mut queue = VecDeque::new();

        // Start at solved state
        let solved_corner_ori = 0;
        let solved_edge_ori = 0;
        let start_index = Self::index(solved_corner_ori, solved_edge_ori);

        self.table[start_index] = 0;
        queue.push_back((solved_corner_ori, solved_edge_ori));

        while let Some((c_idx, e_idx)) = queue.pop_front() {
            let dist = self.table[Self::index(c_idx, e_idx)];
            // Optional: set max depth cutoff to limit table generation
            if dist == 15 {
                continue;
            }

            // Decode indices into cube orientations
            let mut cube = Cube::new_solved();
            cube.set_corner_orientation_from_index(c_idx);
            cube.set_edge_orientation_from_index(e_idx);

            // All 18 moves (U, U', U2, R, R', R2, etc.)
            for mv in [
                Move::U, Move::UPrime, Move::U2,
                Move::R, Move::RPrime, Move::R2,
                Move::F, Move::FPrime, Move::F2,
                Move::D, Move::DPrime, Move::D2,
                Move::L, Move::LPrime, Move::L2,
                Move::B, Move::BPrime, Move::B2,
            ].iter() {
                let mut new_cube = cube.clone();
                new_cube.apply_move(mv);

                let new_c_idx = new_cube.corner_orientation_index();
                let new_e_idx = new_cube.edge_orientation_index();
                let new_index = Self::index(new_c_idx, new_e_idx);

                if self.table[new_index] == -1 {
                    self.table[new_index] = dist + 1;
                    queue.push_back((new_c_idx, new_e_idx));
                }
            }
        }
    }

    pub fn load_or_generate(path: &str) -> std::io::Result<Self> {
        if Path::new(path).exists() {
            // Try loading
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            // Validate size
            if buffer.len() != TABLE_SIZE {
                println!("File size mismatch, regenerating pruning table...");
                return Self::generate_and_save(path);
            }

            let table = buffer.into_iter().map(|b| b as i8).collect();
            
            println!("File found and loaded");

            Ok(Self { table })
        } else {
            // File not found, generate & save
            Self::generate_and_save(path)
        }
    }

    fn generate_and_save(path: &str) -> std::io::Result<Self> {
        println!("Generating pruning table, please wait...");
        let mut pruning_table = Self::new();
        pruning_table.generate();
        println!("Generation complete! Saving to file...");

        let mut file = File::create(path)?;
        // Save as bytes (cast i8 -> u8)
        let bytes: Vec<u8> = pruning_table.table.iter().map(|&v| v as u8).collect();
        file.write_all(&bytes)?;

        Ok(pruning_table)
    }
}
