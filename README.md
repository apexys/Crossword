# Crossword
Generator for Crossword Puzzles / Word Grids

Currently uses a backtracking approach

Run example by issuing `cargo run --release few_more_words.txt`

This currently takes an eternity for more_words.txt.

I think these are places where we could optimize:

- [*] Limit allowable letters to A-Z and use u8 to store them (normal cmp instead of memcmp)
- [*] Add a 2dim-Array to the board and cache where which letter is to make collision detection faster
- [ ] Maybe use an intelligent search algorithm where positions in the board are tried first if they have the same letter as one of the letters in the current word
- [ ] Kick out PlacedWord, inline it completely into board
- [ ] We probably don't even need SmallVec outside RandomIter if we inline PlacedWord, this might speed things up a little more
