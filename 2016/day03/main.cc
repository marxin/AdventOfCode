
#include <iostream>
#include <unordered_set>
#include <fstream>
#include <sstream>
#include <vector>
#include <tuple>

using namespace std;

int main(int argc, char **argv)
{
    unsigned int triangles = 0;
    vector<vector<unsigned int>> values;

    auto is_triangle = [](unsigned int a, unsigned int b, unsigned int c)
    {
        return (a + b > c && a + c > b && b + c > a);
    };

    ifstream ifile("input.txt");
    for (string line; getline(ifile, line);)
    {
        unsigned int a, b, c;

        stringstream ss{line};
        ss >> a >> b >> c;
        values.push_back({a, b, c});

        if(is_triangle(a, b, c))
            triangles++;
    }

    cout << "part 1: " << triangles << endl;

    // Part 2
    unsigned int triangles2 = 0;

    for (unsigned int c = 0; c < 3; c++)
        for (unsigned i = 0; i < values.size() / 3; i++)
            if(is_triangle(values[3 * i][c], values[3 * i + 1][c], values[3 * i + 2][c]))
                triangles2++;

    cout << "part 2: " << triangles2 << endl;

    return 0;
}