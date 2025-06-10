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

void Cube::applyMove(Move move) {

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