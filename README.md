# Blocksworld Puzzle Solver
## Description
### Introduction
The blocksworld puzzle solver is a project created for a coursework at the University of Southampton, computer science course as part of the _Intelligent Systems_ module. It aims to explore different search algorithms within the field by implementing them for a simple block-based grid puzzle.

[Report written for project](report.pdf)

### Scenario
>An agent moves in a simulated NxN grid world with the goal of building towers of blocks. Each grid space contains either a block with a letter on it, the agent or is empty.
    
>The agent moves up/down/left/right (except where borders prevent it). If the agent moves onto a block, the block moves to fill the previous position of the agent.

```
* * * * * *  A, B and C represent blocks.
*         *  @ represents the agent.
*         *
*         *
* A B C @ *
* * * * * *

```
_Above: an example start state for the puzzle._

### Task
The task was to create an implementation of the scenario using different search algorithms (breadth first, depth first, iterative deepening and A*) to guide the agent's path to the goal state.

## Implementation
The program (written in [Rust](http://rust-lang.org)) in its given form runs each search algorithm on the start state given above to reach the goal state given below (the agent can be located anywhere).

```
* * * * * * 
*         * 
*   A     *
*   B     *
*   C   @ *
* * * * * *
```
_Above: the goal state for the puzzle._

Code has been written to perform searches from different goal states to increase the complexity of the problem - these are iteratively generated (reusing the same search algorithm code!) to be of a target complexity. _However_ there is no interface when the program is run for this functionality to be used without minor edits, which may be made at a later date.

## Usage
First, install rust and cargo, the rust package manager by following the instructions at <https://doc.rust-lang.org/stable/book/getting-started.html>

Finally, in a terminal, change directory to this project and run `cargo run --release`. This command will build the project and then run it directly after.