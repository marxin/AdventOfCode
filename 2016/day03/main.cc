
#include <iostream>
#include <unordered_set>
#include <fstream>
#include <sstream>

using namespace std;

int main(int argc, char **argv)
{
    unsigned int triangles = 0;

    ifstream ifile("input.txt");
    for (string line; getline(ifile, line);)
    {
        unsigned int a, b, c;

        stringstream ss{line};
        ss >> a >> b >> c;

        if ((a + b) > c && (a + c) > b && (b + c) > a)
            triangles++;
    }

    cout << triangles << endl;

    return 0;
}