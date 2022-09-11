# k-mer-benchmark
I was inspired by an article I found on Medium called "[How Fast Is C++ Compared to Python?](https://medium.com/towards-data-science/how-fast-is-c-compared-to-python-978f18f474c7)". In the article, the author compares Python to C++ by benchmarking how quickly basic programs can generate all possible [DNA k-mers](https://en.wikipedia.org/wiki/K-mer) for a fixed value of “k”. Like the article, the only goal was to compare these languages when they are going through exactly the same algorithm and instructions.

# Test Machine
I ran the code in this repo on a MacBook Pro with the following specs:
* Chip: Apple M1 Pro
* Memeory: 32 GB
* OS Version: 12.5.1

The versions of the languages that were used are:
* Rust: `rustc 1.63.0 (4b91a6ea7 2022-08-08)`
* C++: `Apple clang version 13.1.6 (clang-1316.0.21.2.5)`
* Golang: `go version go1.19.1 darwin/amd64`

# Build Processes
I wanted to benchmark the binary each language would output. Below are the build commands I used for each language
* Rust: `cargo build --release` 
* C++: `g++ -o ./k_mer ./k_mer.cpp`
* Golang: `Go build -ldflags "-s -w"`

Each binary was timed using the [time](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/time.html) utility.

# Findings
| K-mer | Number of Combinations | Rust Runtime (sec) | C++ Runtime (sec) | Golang Runtime (sec) |
|-------|------------------------|--------------------|-------------------|----------------------|
| 13    | 67,108,864             | 6.438              | 6.334             | 7.480                |
| 14    | 268,435,456            | 26.102             | 25.308            | 30.196               |
| 15    | 1,073,741,824          | 104.963            | 101.485           | 119.602              |
