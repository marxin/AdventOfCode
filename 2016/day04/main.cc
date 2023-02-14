
#include <iostream>
#include <unordered_set>
#include <fstream>
#include <sstream>
#include <algorithm>
#include <unordered_map>
#include <vector>
#include <tuple>
#include <optional>

using namespace std;


optional<int> is_valid_room(string line)
{
    size_t numpos = line.find_first_of("0123456789");
    size_t endpos = line.find('[');
    string letters = line.substr(0, numpos);
    int nr = stoi(line.substr(numpos, endpos - numpos));

    string expected = line.substr(endpos + 1);
    expected.pop_back();

    unordered_map<char, size_t> frequency;

    for(char c: letters)
        if (c != '-')
            frequency[c] += 1;

    vector<tuple<size_t, int, char>> tuples;
    for (const auto& [key, value] : frequency)
    {
        tuples.push_back({value, -(int)key, key});
        // cout << key << " " << value << "x " << first_seen[key] << "" << endl;
    }

    sort(tuples.begin(), tuples.end(), greater<>());

    for (unsigned i = 0; i < expected.size(); i++)
        if (expected[i] != get<2>(tuples[i]))
            return {};

    return nr;
}

int main(int argc, char **argv)
{
    unsigned int triangles = 0;
    int total = 0;

    ifstream ifile("input.txt");
    for (string line; getline(ifile, line);)
    {
        auto value = is_valid_room(line);
        if (value.has_value())
            total += *value;

        cout << line << " valid: " << value.has_value() << endl;
    }

    cout << "total: " << total << endl;

    return 0;
}