#include "Cube.hpp"

Cube::Cube()
{
    for (int i = 0; i < 8; ++i) {
        cp[i] = i;
        co[i] = 0;
    }
    for (int i = 0; i < 12; ++i) {
        ep[i] = i;
        eo[i] = 0;
    }
}

Cube::Cube(const Cube& cube) {
    for (int i = 0; i < 8; ++i) {
        cp[i] = cube.cp[i];
        co[i] = cube.co[i];
    }
    for (int i = 0; i < 12; ++i) {
        ep[i] = cube.ep[i];
        eo[i] = cube.eo[i];
    }
}

Cube& Cube::operator=(const Cube& cube) {
    for (int i = 0; i < 8; ++i) {
        cp[i] = cube.cp[i];
        co[i] = cube.co[i];
    }
    for (int i = 0; i < 12; ++i) {
        ep[i] = cube.ep[i];
        eo[i] = cube.eo[i];
    }

    return *this;
}


void Cube::applyMove(const int move, const char modifier)
{
    int left = 0;

    if (modifier == '\'')
        left = 3;
    else if (modifier == '2')
        left = 2;
    else
        left = 1;
    do
    {
        const MoveTable& m = MOVE_TABLES[move];

        int new_cp[8], new_co[8];
        int new_ep[12], new_eo[12];

        // Apply corner permutation and orientation
        for (int i = 0; i < 8; i++) {
            new_cp[i] = cp[m.cp[i]];

            // Corner orientation addition mod 3
            int ori = co[m.cp[i]] + m.co[i];
            new_co[i] = ori % 3;
        }

        // Apply edge permutation and orientation
        for (int i = 0; i < 12; i++) {
            new_ep[i] = ep[m.ep[i]];

            // Edge orientation flips (XOR)
            int ori = eo[m.ep[i]] ^ m.eo[i];
            new_eo[i] = ori;
        }

        // Copy new state back
        for (int i = 0; i < 8; i++) {
            cp[i] = new_cp[i];
            co[i] = new_co[i];
        }
        for (int i = 0; i < 12; i++) {
            ep[i] = new_ep[i];
            eo[i] = new_eo[i];
        }

        left--;
    } while (left != 0);
}

bool Cube::isSolved() const
{
    for (int i = 0; i < 8; ++i) {
        if (this->cp[i] != i)
            return false;
        if (this->co[i] != 0)
            return false;
    }

    for (int i = 0; i < 12; ++i) {
        if (this->ep[i] != i)
            return false;
        if (this->eo[i] != 0)
            return false;
    }

    return true;
}

bool Cube::operator==(const Cube& other) const
{
    for (int i = 0; i < 8; ++i) {
        if (this->cp[i] != other.cp[i])
            return false;
        if (this->co[i] != other.co[i])
            return false;
    }

    for (int i = 0; i < 12; ++i) {
        if (this->ep[i] != other.ep[i])
            return false;
        if (this->eo[i] != other.eo[i])
            return false;
    }

    return true;
}


std::ostream& operator<<(std::ostream& os, const Cube& cube)
{

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