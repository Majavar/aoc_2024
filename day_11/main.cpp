#include <algorithm>
#include <cmath>
#include <iostream>
#include <string>
#include <vector>

typedef std::vector<std::pair<long, long>> stone_line;

void print_vector(stone_line &v) {
    for (auto s: v) {
        std::cout <<"("<<s.first<<" time "<<s.second<<")"<<" ";
    }
    std::cout << std::endl;
}

void add_stone(stone_line* stones, long stone, long count) {
    auto it = std::find_if(stones->begin(), stones->end(), [stone](std::pair<long, long> s) { return s.second == stone; });

    if (it != stones->end()) {
        it->first += count;
    } else {
        stones->push_back(std::pair(count, stone));
    }
}

long long count_stones(stone_line* stones) {
    long long total = 0;
    for (auto s: *stones) {
        total += s.first;
    }
    return total;
}

void blink(stone_line** stones, int times) {
    for (int i = 0; i < times; i++) {
        stone_line* new_stones = new stone_line();
        for (auto s: **stones) {
            if (s.second == 0) {
                add_stone(new_stones, 1, s.first);
            } else if ((int) log10(s.second) % 2 == 1) {
                long f = (int) std::pow(10, ((int) log10(s.second) + 1) / 2);
                add_stone(new_stones, s.second / f, s.first);
                add_stone(new_stones, s.second % f, s.first);
            } else {
                add_stone(new_stones, s.second * 2024, s.first);
            }
        }

        delete *stones;
        *stones = new_stones;
    }
}

int main(int argc, char** argv) {
    std::string input;

    auto stones = new stone_line();

    while(std::getline(std::cin, input, ' ')) {
        stones->push_back(std::pair(1, std::stol(input)));
    }

    blink(&stones, 25);
    std::cout<<"Part 1: "<<count_stones(stones)<<std::endl;

    blink(&stones, 75 - 25);
    std::cout<<"Part 2: "<<count_stones(stones)<<std::endl;

    delete stones;
    return 0;
}