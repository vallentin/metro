//! Metro is a crate for rendering graphs similar to `git log --graph`.
//!
//! # Example
//!
//! ```no_run
//! use metro::Event::*;
//! let events = [
//!     Station(0, "Station 1"),
//!     Station(0, "Station 2"),
//!     NoEvent,
//!     Station(0, "Station 3"),
//!     SplitTrack(0, 1),
//!     Station(1, "Station 4"),
//!     SplitTrack(1, 2),
//!     Station(1, "Station 5"),
//!     Station(2, "Station 6"),
//!     NoEvent,
//!     Station(0, "Station 7"),
//!     Station(1, "Station 8"),
//!     Station(2, "Station 9"),
//!     SplitTrack(2, 3),
//!     SplitTrack(3, 4),
//!     Station(5, "Station 10"),
//!     JoinTrack(4, 0),
//!     Station(3, "Station 11"),
//!     StopTrack(1),
//!     NoEvent,
//!     Station(0, "Station 12"),
//!     Station(2, "Station 13"),
//!     Station(3, "Station 14"),
//!     JoinTrack(5, 0),
//!     JoinTrack(3, 0),
//!     Station(2, "Station 15"),
//!     StopTrack(2),
//!     Station(0, "Station 16"),
//! ];
//!
//! println!("{}", metro::to_string(&events).unwrap());
//! ```
//!
//! This will output the following:
//!
//! ```text
//! * Station 1
//! * Station 2
//! |
//! * Station 3
//! |\
//! | * Station 4
//! | |\
//! | * | Station 5
//! | | * Station 6
//! | | |
//! * | | Station 7
//! | * | Station 8
//! | | * Station 9
//! | | |\
//! | | | |\
//! | | | | | Station 10
//! | |_|_|/
//! |/| | |
//! | | | * Station 11
//! | " | |
//! |  / /
//! | | |
//! * | | Station 12
//! | * | Station 13
//! | | * Station 14
//! | |/
//! |/|
//! | * Station 15
//! | "
//! * Station 16
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]
// #![deny(missing_doc_code_examples)]
#![deny(missing_debug_implementations)]
#![warn(clippy::all)]

mod events;
mod metro;

pub use crate::metro::{Metro, Track};
pub use events::*;
