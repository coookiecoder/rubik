#ifndef CUBE_H
#define CUBE_H

#include <unordered_map>
#include <string>
#include <iostream>
#include <vector>

#include "MoveTable.hpp"

enum Move {
    U = 0,
    D = 1,
    F = 4,
    B = 3,
    L = 4,
    R = 5,
};

const std::unordered_map<char, Move> moveMap = {
    {'U',   U},
    {'D',   D},
    {'F',   F},
    {'B',   B},
    {'L',   L},
    {'R',   R},
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
        Cube(const Cube& cube);
        Cube& operator=(const Cube& cube);

        void applyMove(int move, char modifier);

        bool isSolved() const;

        bool operator==(const Cube&) const;
};

std::ostream& operator<<(std::ostream& os, const Cube& cube);

#endif