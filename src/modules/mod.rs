/*
--------------------------------------------
modules/mod.rs
This file lists which modules (Rust files) are part of the "modules" folder.

This file just tells Rust what’s available. It’s like a directory of all the tools.

Example:
pub mod grid;

Once listed here, you can import from main.rs:
use crate::modules::grid::draw_grid;
--------------------------------------------
*/
// Add modules below
pub mod still_image;
pub mod text_button;
pub mod grid;
pub mod image_button;
pub mod scale;
pub mod animated_image;
pub mod preload_image;
pub mod label;