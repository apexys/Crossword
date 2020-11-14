# Crossword
Generator for Crossword Puzzles / Word Grids

Currently uses a backtracking approach

## Usage

Run example by issuing `cargo run --release biology.txt 12`

First argument controls what wordlist (one word per line, please no special characters) to read, second is the minimum board size to start with.

This currently takes an eternity for more than about 10 words.

## Output
```
SBOGCELLBVYF
YCMRCBPLANTT
HBEVOLUTIONF
ACLECOSYSTEM
VATYPFUNGUSY
ATATRUNKLEAF
VMUAEDBPIBVL
RBCNDORRFTVO
FRCIADAEELSU
URPMTYNYDSNE
YBCAOECYRCGA
NLSLRNHPNTEF

Find in this: FUNGUS, EVOLUTION, ANIMAL, PLANT, CELL, LEAF, ECOSYSTEM, PREDATOR, PREY, LIFE, TRUNK, BRANCH

Solution:
____CELL____
______PLANT_
__EVOLUTION_
___ECOSYSTEM
____PFUNGUS_
___TRUNKLEAF
___AE_BPI___
___ND_RRF___
___IA_AEE___
___MT_NY____
___AO_C_____
___LR_H_____
```
## Optimization

I think these are places where we could optimize:

- [x] Limit allowable letters to A-Z and use u8 to store them (normal cmp instead of memcmp)
- [x] Add a 2dim-Array to the board and cache where which letter is to make collision detection faster
- [x] Kick out PlacedWord, inline it completely into board
- [ ] Maybe use an intelligent search algorithm where positions in the board are tried first if they have the same letter as one of the letters in the current word

All in all, this is already pretty optimized.

I tried randomizing the position first and then going only to board_size - word_length for the given direction, but that didn't make it a whole lot faster (iterating over things where we know we're oob is cheap) and made the crossword quality worse (most words ended up pointing in the same direction).

Maybe one could investigate acceleration structures? Would a quadtree help?