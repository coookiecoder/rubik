#include "Cube.hpp"

Cube::Cube() {
    for (int i = 0; i < 8; ++i) {
        cp[i] = i;
        co[i] = 0;
    }
    for (int i = 0; i < 12; ++i) {
        ep[i] = i;
        eo[i] = 0;
    }
}

void Cube::applyMove(int move) {
    // Face indices: 0=U, 1=R, 2=F, 3=D, 4=L, 5=B
    const int cornerCycle[6][4] = {
        {0, 1, 2, 3},  // U
        {0, 3, 7, 4},  // R
        {0, 4, 5, 1},  // F
        {4, 7, 6, 5},  // D
        {2, 1, 5, 6},  // L
        {3, 2, 6, 7}   // B
    };
    const int edgeCycle[6][4] = {
        {0, 1, 2, 3},   // U
        {0, 8, 4, 11},  // R
        {1, 9, 5, 8},   // F
        {4, 5, 6, 7},   // D
        {2, 10, 6, 9},  // L
        {3, 11, 7, 10}  // B
    };
    const int cornerOrientChange[6] = {0, 2, 1, 0, 1, 2}; // For R/F/L/B
    const int edgeOrientChange[6] = {0, 0, 1, 0, 1, 1};   // For F/L/B

    int face = move / 3;      // 0–5
    int turns = move % 3 + 1; // 1=90, 2=180, 3=270 (prime)

    for (int t = 0; t < turns; ++t) {
        // Corners
        int temp_cp[4], temp_co[4];
        for (int i = 0; i < 4; ++i) {
            temp_cp[i] = cp[cornerCycle[face][i]];
            temp_co[i] = co[cornerCycle[face][i]];
        }
        for (int i = 0; i < 4; ++i) {
            cp[cornerCycle[face][i]] = temp_cp[(i + 3) % 4];
            int ori = temp_co[(i + 3) % 4];
            if (cornerOrientChange[face] != 0) {
                ori = (ori + cornerOrientChange[face]) % 3;
            }
            co[cornerCycle[face][i]] = ori;
        }

        // Edges
        int temp_ep[4], temp_eo[4];
        for (int i = 0; i < 4; ++i) {
            temp_ep[i] = ep[edgeCycle[face][i]];
            temp_eo[i] = eo[edgeCycle[face][i]];
        }
        for (int i = 0; i < 4; ++i) {
            ep[edgeCycle[face][i]] = temp_ep[(i + 3) % 4];
            int ori = temp_eo[(i + 3) % 4];
            if (edgeOrientChange[face] != 0) {
                ori ^= 1;
            }
            eo[edgeCycle[face][i]] = ori;
        }
    }
}


std::ostream& operator<<(std::ostream& os, const Cube& cube) {

    os << "cp : ";

    for (int i : cube.cp) {
        os << i << " ";

    }

    os << std::endl << "co : ";

    for (int i : cube.co) {
        os << i << " ";
    }

    os << std::endl << "ep : ";

    for (int i : cube.ep) {
        os << i << " ";
    }

    os << std::endl << "eo : ";

    for (int i : cube.eo) {
        os << i << " ";
    }

    return os;
}
