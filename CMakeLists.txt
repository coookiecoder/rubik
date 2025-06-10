cmake_minimum_required(VERSION 3.20)
project(rubik)

add_compile_options(-Wall -Wextra -g3)

set(CMAKE_CXX_STANDARD 20)

add_executable(rubik main.cpp
        Class/Cube/Cube.cpp
        Class/Cube/Cube.hpp)
