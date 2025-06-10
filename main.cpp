#include <string>
#include <sstream>
#include <vector>
#include <iostream>

#include "Class/Cube/Cube.hpp"

void convert(const std::string& string, std::vector<std::string>& moves) {
    std::stringstream moves_stream{string};
    std::string move;

    while (moves_stream >> move) {
        std::cout << "Move found : \"" << move << "\"" << std::endl;

        if (move.size() > 2) {
            throw std::invalid_argument("Invalid move : " + move);
        }

        moves.push_back(move);
    }
}

int main(const int argc, const char **argv) {
    if (argc != 2) {
        std::cerr << "usage : " << argv[0] << " [mix]" << std::endl;
        return 1;
    }

    std::vector<std::string> moves(argc - 1);

    try {
        convert(argv[1], moves);
    } catch (std::invalid_argument& error) {
        std::cerr << error.what() << std::endl;
        return 1;
    }

    Cube cube;

    for (auto move : moves) {
        auto it = moveMap.find(move.c_str());

        if (it != moveMap.end())
            cube.applyMove(it->second);
        else {
            std::cerr << "Invalid move : " << move << std::endl;
            return 1;
        }
    }

    std::cout << cube << std::endl;

    return 0;
}