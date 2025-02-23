Implement a new module called `MentalRotation`, for a game served at `/mental-rotation`:

Player must create a continuous pathway from square A (on left edge) to square B (on right edge) in the least moves possible.

Record the moves taken as well as other fun stats as long as they don't affect time or space complexity.

A is labelled using a rocket on the left on the start cell, and B with an Earth symbol on the right of the end cell (both outside the grid). When the arrows from different tiles form a path, with no gaps between tiles, from square A to square B *inclusive*, the rocket immediately but slowly travels smoothly through the path towards the Earth symbol, completing the level.

Tiles: The game uses polyomino tiles which each take up multiple square cells on a borderless square grid. Not all cells are occupied by a tile. Squares which are occupied by tiles have a `--bg-2` background colour. In each square, an arrow indicates the direction to the next junction within the tile. At junctions within a tile shape, arrows point at 45-degree angles between adjacent squares, but the ends of a tile must contain orthogonal arrows.

There should be no borders visible around the squares or around the entire polyomino. There should be no animations except the rocket one.

Use relative CSS units only. All arrows should be represented using `➔`, rotating if needed. Cells should be sized such that the arrows (which inherit the 1.5rem size) just touch the sides of the cell.

Tile Interaction:

Left-Click: Rotate the tile 90 degrees clockwise. If this would make it collide with another tile, or go out of bounds of the grid, do nothing.
Right-Click: Reverse the direction of all arrows on the tile selected. (the arrows should be visibly reversed).

A button is used to reset the tiles to their state at the start of the level.

 Use a 3-minute timer. The timer must be centred directly above the grid. The game state should be preserved after you close the tab. If the time has elapsed, a new game with the same level starts.

The tiles take up cells on a square grid with side length n, where n is the level starting at 1. Create a single valid path from A to B, then create tiles which can be interacted to create this path. Make the level as difficult as possible in O(n).

Do not add any JS; the linkage should be simply `<script src=index.js>`. Outsource whatever possible to other crates, but note that system entropy is not supported on WASM.

Link the game in [](../../static/index.html).

Ensure that individual code file sizes are small.

Make clear how to playtest.