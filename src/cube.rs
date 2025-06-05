use crate::color::Color;

use std::fmt;
use std::fmt::Formatter;

pub struct Cube {
    pub front: Vec<Color>,
    pub back: Vec<Color>,
    pub right: Vec<Color>,
    pub left: Vec<Color>,
    pub top: Vec<Color>,
    pub bottom: Vec<Color>
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Front: {:?}", self.front).expect("write failed");
        writeln!(f, "Back: {:?}", self.back).expect("write failed");
        writeln!(f, "Right: {:?}", self.right).expect("write failed");
        writeln!(f, "Left: {:?}", self.left).expect("write failed");
        writeln!(f, "Top: {:?}", self.top).expect("write failed");
        writeln!(f, "Bottom: {:?}", self.bottom).expect("write failed");

        Ok(())
    }
}

fn rotate_face_cw(face: &mut Vec<Color>) {
    let old = face.clone();
    face[0] = old[6];
    face[1] = old[3];
    face[2] = old[0];
    face[3] = old[7];
    face[4] = old[4];
    face[5] = old[1];
    face[6] = old[8];
    face[7] = old[5];
    face[8] = old[2];
}

impl Cube {
    pub fn up(&mut self) {
        rotate_face_cw(&mut self.top);

        let temp = self.front[0..3].to_vec();
        self.front[0..3].copy_from_slice(&self.right[0..3]);
        self.right[0..3].copy_from_slice(&self.back[0..3]);
        self.back[0..3].copy_from_slice(&self.left[0..3]);
        self.left[0..3].copy_from_slice(&temp);
    }

    pub fn up_prime(&mut self) {
        self.up();
        self.up();
        self.up();
    }

    pub fn down(&mut self) {
        rotate_face_cw(&mut self.bottom);

        let temp = self.front[6..9].to_vec();
        self.front[6..9].copy_from_slice(&self.left[6..9]);
        self.left[6..9].copy_from_slice(&self.back[6..9]);
        self.back[6..9].copy_from_slice(&self.right[6..9]);
        self.right[6..9].copy_from_slice(&temp);
    }

    pub fn down_prime(&mut self) {
        self.down();
        self.down();
        self.down();
    }

    pub fn right(&mut self) {
        rotate_face_cw(&mut self.right);

        let temp = vec![self.top[2], self.top[5], self.top[8]];
        self.top[2] = self.front[2];
        self.top[5] = self.front[5];
        self.top[8] = self.front[8];

        self.front[2] = self.bottom[2];
        self.front[5] = self.bottom[5];
        self.front[8] = self.bottom[8];

        self.bottom[2] = self.back[6];
        self.bottom[5] = self.back[3];
        self.bottom[8] = self.back[0];

        self.back[6] = temp[0];
        self.back[3] = temp[1];
        self.back[0] = temp[2];
    }

    pub fn right_prime(&mut self) {
        self.right();
        self.right();
        self.right();
    }

    pub fn left(&mut self) {
        rotate_face_cw(&mut self.left);

        let temp = vec![self.top[0], self.top[3], self.top[6]];
        self.top[0] = self.back[8];
        self.top[3] = self.back[5];
        self.top[6] = self.back[2];

        self.back[8] = self.bottom[0];
        self.back[5] = self.bottom[3];
        self.back[2] = self.bottom[6];

        self.bottom[0] = self.front[0];
        self.bottom[3] = self.front[3];
        self.bottom[6] = self.front[6];

        self.front[0] = temp[0];
        self.front[3] = temp[1];
        self.front[6] = temp[2];
    }

    pub fn left_prime(&mut self) {
        self.left();
        self.left();
        self.left();
    }

    pub fn front(&mut self) {
        rotate_face_cw(&mut self.front);

        let temp = vec![self.top[6], self.top[7], self.top[8]];
        self.top[6] = self.left[8];
        self.top[7] = self.left[5];
        self.top[8] = self.left[2];

        self.left[8] = self.bottom[2];
        self.left[5] = self.bottom[1];
        self.left[2] = self.bottom[0];

        self.bottom[0] = self.right[6];
        self.bottom[1] = self.right[3];
        self.bottom[2] = self.right[0];

        self.right[0] = temp[2];
        self.right[3] = temp[1];
        self.right[6] = temp[0];
    }

    pub fn front_prime(&mut self) {
        self.front();
        self.front();
        self.front();
    }

    pub fn back(&mut self) {
        rotate_face_cw(&mut self.back);

        let temp = vec![self.top[0], self.top[1], self.top[2]];
        self.top[0] = self.right[2];
        self.top[1] = self.right[5];
        self.top[2] = self.right[8];

        self.right[2] = self.bottom[8];
        self.right[5] = self.bottom[7];
        self.right[8] = self.bottom[6];

        self.bottom[6] = self.left[0];
        self.bottom[7] = self.left[3];
        self.bottom[8] = self.left[6];

        self.left[0] = temp[2];
        self.left[3] = temp[1];
        self.left[6] = temp[0];
    }

    pub fn back_prime(&mut self) {
        self.back();
        self.back();
        self.back();
    }

    pub fn solved(&mut self) -> bool {
        let mut buffer:Color = self.front[0];

        for face in &self.front {
            if buffer != *face {
                return false;
            }
        }

        buffer = self.back[0];
        for face in &self.back {
            if buffer != *face {
                return false;
            }
        }

        buffer = self.right[0];
        for face in &self.right {
            if buffer != *face {
                return false;
            }
        }

        buffer = self.left[0];
        for face in &self.left {
            if buffer != *face {
                return false;
            }
        }

        buffer = self.top[0];
        for face in &self.top {
            if buffer != *face {
                return false;
            }
        }

        buffer = self.bottom[0];
        for face in &self.bottom {
            if buffer != *face {
                return false;
            }
        }

        return true;
    }
}
