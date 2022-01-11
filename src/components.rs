pub use crate::prelude::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
pub color : ColorPair,❶
pub glyph : FontCharType❷
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;