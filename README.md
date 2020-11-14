# Crossword
Generator for Crossword Puzzles / Word Grids

Currently uses a backtracking approach

Run example by issuing `cargo run --release few_more_words.txt`

This currently takes an eternity for more_words.txt.

I think these are places where we could optimize:

1. Limit allowable letters to A-Z and use u8 to store them (normal cmp instead of memcmp)
2. Add a 2dim-Array to the board and cache where which letter is to make collision detection faster
3. Maybe use an intelligent search algorithm where positions in the board are tried first if they have the same letter as one of the letters in the current word
