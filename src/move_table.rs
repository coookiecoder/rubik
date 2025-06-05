pub struct MoveTable {
    pub corner_perm: [usize; 8],
    pub corner_orient_delta: [u8; 8],
    pub edge_perm: [usize; 12],
    pub edge_orient_delta: [u8; 12],
}

pub const U_MOVE: MoveTable = MoveTable {
    corner_perm: [3, 0, 1, 2, 4, 5, 6, 7],
    corner_orient_delta: [0; 8],
    edge_perm: [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
    edge_orient_delta: [0; 12],
};


pub const R_MOVE: MoveTable = MoveTable {
    corner_perm: [0, 5, 2, 3, 4, 6, 7, 1],
    corner_orient_delta: [0, 2, 0, 0, 0, 1, 2, 1],
    edge_perm: [0, 9, 2, 3, 4, 1, 6, 7, 8, 10, 5, 11],
    edge_orient_delta: [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
};

pub const F_MOVE: MoveTable = MoveTable {
    corner_perm: [1, 6, 2, 0, 4, 5, 3, 7],
    corner_orient_delta: [1, 2, 0, 2, 0, 0, 1, 0],
    edge_perm: [0, 1, 10, 3, 4, 5, 2, 7, 8, 9, 6, 11],
    edge_orient_delta: [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
};

pub const D_MOVE: MoveTable = MoveTable {
    corner_perm: [0, 1, 2, 3, 5, 6, 7, 4],
    corner_orient_delta: [0; 8],
    edge_perm: [0, 1, 2, 3, 5, 6, 7, 4, 8, 9, 10, 11],
    edge_orient_delta: [0; 12],
};

// L move: rotates the left face clockwise
pub const L_MOVE: MoveTable = MoveTable {
    corner_perm: [7, 1, 3, 4, 0, 5, 6, 2],
    corner_orient_delta: [2, 0, 1, 2, 1, 0, 0, 1],
    edge_perm: [11, 1, 2, 7, 0, 5, 6, 4, 8, 9, 10, 3],
    edge_orient_delta: [1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1],
};

// B move: rotates the back face clockwise
pub const B_MOVE: MoveTable = MoveTable {
    corner_perm: [0, 1, 7, 5, 4, 3, 2, 6],
    corner_orient_delta: [0, 0, 1, 2, 0, 1, 2, 1],
    edge_perm: [0, 1, 2, 11, 4, 10, 6, 7, 8, 5, 9, 3],
    edge_orient_delta: [0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1],
};