# squaring-the-square

This project, submitted in partial fulfillment of the requirements for the Computer Science Tripos, is an effort to reduce the computational complexity to exhaustively Squaring the Square (and rectangles).

Squaring Squares and Rectangles describes decomposing them into distinct integer squares. Ian Gambini's 1999 work presented the first algorithm to produce SPSRs and SPSSs exhaustively, and in this project significant refactoring of the algorithm lead to a significant exponential reduction in complexity. Additionally, the new algorithm was parallelised to take advantage of modern machines.

[The complete write-up is included, with intent to publish in the coming year.](./AC_MEng_Dissertation.pdf)

## Discoveries Made

As of June 2023, the algorithm has been run for 11 days across 9 cores, enumerating up to width 150, and produced the following discoveries:

- An exhaustive enumeration of all SPSSs, SPSRs, CPSSs, and CPSRs up to width 150, presented with notable properties such as order and boundary squares.
- 66 undiscovered SPSRs.
- Proof of the 10 smallest SPSRs of orders 17-26.
- Proof of the 8 smallest SPSRs with 5-12 boundary squares.
- An undiscovered and remarkable isomer:
  ![triple_readme drawio](https://github.com/anastasia-courtney/squaring-the-square/assets/60652829/37ea8882-6fa3-43d7-a9ad-f450277cb53b)
  - The smallest possible, and only the ninth to be discovered, isomer triple. 
  - The smallest possible, and first identified, isomer of order 22.
  - Demonstrates uncommon symmetry among isomers.
- A new bound on the number of 2-square edges a PSS can have.

All results so far processed are included in `Results`, including summaries, and the complete diagrams of every decomposition.

## Running the Project

To run the project, modify `main` such that the minimum and maximum sizes correspond to the search space you intend to enumerate. As this project is computationally expensive, I highly recommend running `--release`. Notably, the code parallelises significantly more efficiently on Apple silicon. It is significantly more efficient to run the program on multiple sizes consecutively, as it has been written to overlap threads and avoid trailing cores.

```bash
cargo run --release
```

`timings-*` contains details of when each size's last thread was taken from the queue, and finished. Primarily used for timing results, it is important to check this file if the program is terminated prematurely, to ascertain exactly which sizes were completely enumerated.

`OUTPUT` contains the results: in the `threading` branch, this describes the sets of squares which make up decompositions, ordered numerically. For each solution in the search space, a set describing it appears at least once, but not necessarily $2^n$ times, as some trivial transformations are excluded as part of the pruning of the search space.

To recover placement, including isomers and all transformations, the results must be processed through `solver`, which runs the same algorithm over the restricted search space in trivial time. The unordered sets can also make for surprisingly tricky puzzles!

## Future Work

The algorithm has only been run for 11 days, and feasibly could be run for a lot longer and continue to produce meaningful output.

Other additional work could include further bounds that allow us to prune the search space: most of which produce constant speedups in the exponential complexity. So far, only direct modification to the algorithm has produced exponential reductions in complexity, and should be the focus of future research.
