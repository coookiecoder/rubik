#ifndef CUBE_H
#define CUBE_H

#include <unordered_map>
#include <string>
#include <iostream>

enum Move {
    U       = 0,
    U2      = 1,
    U_PRIME = 2,
    R       = 3,
    R2      = 4,
    R_PRIME = 5,
    F       = 6,
    F2      = 7,
    F_PRIME = 8,
    D       = 9,
    D2      = 10,
    D_PRIME = 11,
    L       = 12,
    L2      = 13,
    L_PRIME = 14,
    B       = 15,
    B2      = 16,
    B_PRIME = 17
};

const std::unordered_map<std::string, Move> moveMap = {
    {"U",   U},
    {"U2",  U2},
    {"U'",  U_PRIME},
    {"U’",  U_PRIME},
    {"R",   R},
    {"R2",  R2},
    {"R'",  R_PRIME},
    {"R’",  R_PRIME},
    {"F",   F},
    {"F2",  F2},
    {"F'",  F_PRIME},
    {"F’",  F_PRIME},
    {"D",   D},
    {"D2",  D2},
    {"D'",  D_PRIME},
    {"D’",  D_PRIME},
    {"L",   L},
    {"L2",  L2},
    {"L'",  L_PRIME},
    {"L’",  L_PRIME},
    {"B",   B},
    {"B2",  B2},
    {"B'",  B_PRIME},
    {"B’",  B_PRIME},
};

class Cube {
    public:
        // Corner permutation (0–7), and orientation (0–2)
        int cp[8] = {}; // corner permutation
        int co[8] = {}; // corner orientation

        // Edge permutation (0–11), and orientation (0–1)
        int ep[12] = {}; // edge permutation
        int eo[12] = {}; // edge orientation

        Cube();
        void applyMove(int move);
};

std::ostream& operator<<(std::ostream& os, const Cube& cube);

#endif