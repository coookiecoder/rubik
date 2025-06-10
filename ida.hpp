#ifndef IDA_HPP
#define IDA_HPP

#include "Class/Cube/Cube.hpp"

inline const char* moveNames[] = {"U","D","L","R","F","B"};

void ida_start(Cube& start, int mix_size);

#endif
