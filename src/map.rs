use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// Clone adds a clone() function to the type. Calling mytile.clone() makes a deep
// copy of the variable without affecting the original. If you clone a struct,
// everything the struct contains will also be cloned. This is useful when
// you want to safely work with a clone of some data with no risk of altering
// the original—or when you need to work around the borrow checker.
// Copy changes the default action when assigning aTileType from one variable
// to another. Instead of moving the value, it takes a copy. Smaller types are
// often faster when you copy them around. Clippy will warn you if you are
// borrowing a variable and it would be faster to copy it.
// PartialEq adds code that allows you to compare TileType values with the ==
// operator.
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}
