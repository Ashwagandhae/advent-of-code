# Advent of Code Solutions

These are my [Advent of Code](https://adventofcode.com/) solutions, starting from 2022. They are written in Rust, Java, and Python, with no particular pattern to which language I use for which problem.

The code is optimized for nothing. I'm just trying to get the right answer as fast as possible, and learn a little bit about the languages I'm using.

## Organization

Solutions are in `/done` and are organized by problem number. Part 1 and part 2 of each problem are in different files, denoted by `.1` and `.2` respectively. They have different file extensions based on the languages they're written in. For example, the solution to problem 4 part 2, written in rust, is `/done/4.2.rs`.

Edited solutions are in `/done/edited`. These are solutions that I've edited after the fact, either as a challenge or to make them more readable. The different versions of the solutions are in different files, denoted by `.1`, `.2`, etc. For example, first edited version of problem 4 part 1, written in rust, is in `/done/edited/4.1.1.rs`.

## Running the code

To run the code, first clone the repository.

```bash
git clone https://github.com/Ashwagandhae/advent-of-code.git
cd advent-of-code
```

Then, for each language, do the following:

### Rust (.rs)

1. Download [rust](https://www.rust-lang.org/tools/install)
2. Copy the code of the file you want to run into `/rust/src/main.rs`
3. Enter the rust folder and run the program

```bash
cd rust
cargo run
# add the --release flag if the code is too slow
```

### Java (.java)

1. Download [java](https://www.java.com/)
2. Copy the code of the file you want to run into `/java/Main.java`
3. Enter the java folder and run the program

```bash
cd java
javac Main.java
java Main
```

### Python (.py)

1. Download [python](https://www.python.org/downloads/)
2. Copy the code of the file you want to run into `/python/main.py`
3. Enter the python folder and run the program

```bash
cd python
python3 main.py
```

### C++ (.cpp)

1. Copy the code of the file you want to run into `/cpp/main.cpp`
2. Enter the cpp folder and run the program

```bash
cd cpp
clang++ -std=c++17 -stdlib=libc++ main.cpp -o main
./main
```

### JavaScript (.js)

1. Download [node](https://nodejs.org/en/download/) (or [deno](https://deno.land/) or [bun](https://bun.sh/))
2. Copy the code of the file you want to run into `/js/main.js`
3. Enter the js folder and run the program

```bash
cd js
node main.js
# for deno
deno run main.js
# for bun
bun run main.js
```

## Working folders

The folders `/rust`, `/java`, `/python`, `/c++`, and `javascript` are for working on and running the solutions, and do not contain the final solutions. The final solutions are in `/done`.
