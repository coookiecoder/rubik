#include "ida.hpp"

#include <algorithm>

constexpr int FOUND = -1;
constexpr int INF = 999;
std::vector<int> solutionPathLetter;
std::vector<int> solutionPathModifier;

struct CubeHash {
    std::size_t operator()(const Cube& c) const {
        std::size_t h = 0;
        for (int i = 0; i < 8; ++i) {
            h ^= std::hash<int>{}(c.cp[i]) + 0x9e3779b9 + (h << 6) + (h >> 2);
            h ^= std::hash<int>{}(c.co[i]) + 0x9e3779b9 + (h << 6) + (h >> 2);
        }
        for (int i = 0; i < 12; ++i) {
            h ^= std::hash<int>{}(c.ep[i]) + 0x9e3779b9 + (h << 6) + (h >> 2);
            h ^= std::hash<int>{}(c.eo[i]) + 0x9e3779b9 + (h << 6) + (h >> 2);
        }
        return h;
    }
};

std::unordered_map<Cube, int, CubeHash> transpositionTable;

static
int cornerOrientationCost(const Cube& cube)
{
    int sum = 0;

    for (const int i : cube.co) {
        if (i != 0) sum++;
    }

    return (sum + 2) / 3; // min number of moves to fix corner orientation
}

static
int edgeOrientationCost(const Cube& cube)
{
    int sum = 0;

    for (const int i : cube.eo) {
        if (i != 0) sum++;
    }

    return (sum + 1) / 2; // min number of moves to fix edge orientation
}

static
int permutationCost(const Cube& cube)
{
    int misplaced = 0;

    for (int i = 0; i < 8; ++i)
        if (cube.cp[i] != i) misplaced++;

    for (int i = 0; i < 12; ++i)
        if (cube.ep[i] != i) misplaced++;

    return (misplaced + 7) / 8; // Very rough but admissible
}

static
int heuristic(const Cube& cube)
{
    int max = std::max(cornerOrientationCost(cube),edgeOrientationCost(cube));
    max = std::max(max,permutationCost(cube));

    return max;
}

static
int dfs(const Cube& node, const int g, const int threshold, const int prevMove, const int mix_size)
{
    const int f = g + std::min(heuristic(node), mix_size);

    if (f > threshold)
        return f;

    if (node.isSolved())
        return FOUND;

    auto it = transpositionTable.find(node);
    if (it != transpositionTable.end() && it->second <= g) {
        return INF; // Already visited with better or equal cost
    }

    transpositionTable[node] = g;

    int min = INF;

    for (int modifier = 0; modifier < 3; ++modifier) {
        for (int move = 0; move < 6; ++move) {

            std::cout << "move : " << moveNames[move] << " " << modifier << " " << threshold << std::endl;

            if (move == prevMove)
                continue;

            Cube next = node;
            if (modifier == 0)
                next.applyMove(move, '\0');
            if (modifier == 1)
                next.applyMove(move, '\'');
            if (modifier == 2)
                next.applyMove(move, '2');

            solutionPathLetter.push_back(move);
            solutionPathModifier.push_back(modifier);

            if (node.isSolved())
                return FOUND;

            const int t = dfs(next, g + 1, threshold, move, mix_size);

            if (t == FOUND)
                return FOUND;

            if (t < min)
                min = t;

            solutionPathLetter.pop_back();
            solutionPathModifier.pop_back();
        }
    }

    return min;
}

static
void printSolution(Cube& start) {


    for (unsigned long index = 0; index < solutionPathLetter.size(); ++index) {
        if (solutionPathModifier[index] == 0)
            start.applyMove(solutionPathLetter[index], '\0');
        if (solutionPathModifier[index] == 1)
            start.applyMove(solutionPathLetter[index], '\'');
        if (solutionPathModifier[index] == 2)
            start.applyMove(solutionPathLetter[index], '2');

        std::cout << moveNames[solutionPathLetter[index]];

        if (solutionPathModifier[index] == 1)
            std::cout << "'";
        if (solutionPathModifier[index] == 2)
            std::cout << "2";

        std::cout << " ";
    }
    std::cout << std::endl;
}

void ida_start(Cube& start, int mix_size) {
    solutionPathLetter.clear();
    solutionPathModifier.clear();
    int threshold = std::min(heuristic(start), mix_size);

    std::cout << threshold << std::endl;

    while (true) {
        transpositionTable.clear();
        const int t = dfs(start, 0, threshold, -1, mix_size);

        if (t == FOUND)
            return printSolution(start);

        if (t == INF)
            return;

        threshold = t;
    }
}
