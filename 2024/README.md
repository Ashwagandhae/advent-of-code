# Advent of code 2023

## Prerequisites

- [Rust](https://www.rust-lang.org/learn/get-started) — used to write solutions and for the runner
- [Python](https://www.python.org/downloads/) — used to write solutions
- [unbuffer](https://linux.die.net/man/1/unbuffer) utility (part of the `expect` package), `brew install expect` on macOS — used to show realtime output of solutions
- optional: [funky](https://github.com/bbugyi200/funky) — you can type `a` instead of `./adv` when running the runner

## Usage

### Structure

**Main**

- `_adventer/`: The runner's source code.
- `_cache/`: Cache for inputs and problem htmls.
- `_solutions/`: Solutions for each day and part.

**Workspaces**

- `python/`: Python workspace.
- `rust/`: Rust workspace.

**Important files**

- `adv`: The runner's executable. You can also use `funky` to run it with `a`.
- `.funky`: Configuration for `funky`, allowing you to run `a` instead of `./adv`.
- `state.json`: The runner's state.
- `secrets.json`: Your [session cookie](https://github.com/wimglenn/advent-of-code-wim/issues/1) for adventofcode.com, in the format `{"session": "your cookie"}`—used to download inputs and problem htmls.

### Basics

```bash
# see all available commands (you can also do this on subcommands)
a --help

# update runner after editing its source code
a update
```

### Running old solutions

```bash
# run solution for day 1 part 1 on input
a run 1.0

# run solution for day 1 part 2 on example and input
a run 1.1 all
```

### Working on new solutions

```bash
# set workspace to day 1 part 1 in rust
a work set -p 1.0 -l rs

# get workspace's status
a work status

# clear code in workspace
a work clear

# open code editor in workspace
a work open

# set workspace, clear code, and open code editor
a work init -p 1.0 -l rs

# run code in workspace on example and then input if example passes
a work run

# run code in workspace on input
a work run input

# save solution
a work save

# save current solution, and go to next part of day or next day
a work next
```
