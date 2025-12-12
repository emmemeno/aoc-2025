#![allow(dead_code)]

// Some random thought about this problem
// I could assign a score potential to a grid configuration based on empty positions '.' and its neighbour
// calculate the sum of empty neihbours for each empty space. For example a grid:
// ..
// ..
// has a score of 8 becuase each space have 2 empty neighbours. The grid:
// ...
// ...
// ...
// has a score of 24 (from top left to bottom right: ( 2 + 3 + 2 + 3 + 4 + 3 + 2 + 3 + 2)
// // Again:
// ..#
// ...
// .#.
// has a score of 13 (2 + 2 + 0 + 3 + 3 + 1 + 1 + 0 + 1)
// 
