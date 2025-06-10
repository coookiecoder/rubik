cmake_minimum_required(VERSION 3.31)
project(rubik)

set(CMAKE_CXX_STANDARD 20)

add_executable(rubik main.cpp
        Class/Cube/Cube.cpp
        Class/Cube/Cube.hpp)
