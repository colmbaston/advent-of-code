#![cfg_attr(feature = "permutations", feature(array_windows))]

#[cfg(feature = "bounds")]
pub mod bounds;

#[cfg(feature = "direction")]
pub mod direction;

#[cfg(feature = "pathfinding")]
pub mod pathfinding;

#[cfg(feature = "permutations")]
pub mod permutations;

#[cfg(feature = "transpose")]
pub mod transpose;
