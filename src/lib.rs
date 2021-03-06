//! Metro is a crate for creating and rendering graphs
//! similar to `git log --graph`.
//!
//! # Example Output
//!
//! *The code for creating the following example, can be found
//! after the graph.*
//!
//! ```text
//! * Station 1
//! * Station 2
//! * Station 3
//! |\
//! | * Station 4
//! | |\
//! | * | Station 5
//! | | * Station 6
//! * | | Station 7
//! | * | Station 8
//! | | * Station 9
//! | | |\
//! | | | |\
//! | | | | | Station 10 (Detached)
//! | |_|_|/
//! |/| | |
//! | | | * Station 11
//! | " | |
//! |  / /
//! * | | Station 12
//! | * | Station 13
//! | | * Station 14
//! | |/
//! |/|
//! | * Station 15
//! | "
//! * Station 16
//! ```
//!
//! # Example Using `Metro`
//!
//! *The following example outputs the graph above.*
//!
//! ```no_run
//! use metro::Metro;
//!
//! let mut metro = Metro::new();
//!
//! let mut track1 = metro.new_track();
//! track1.add_station("Station 1");
//! track1.add_station("Station 2");
//! track1.add_station("Station 3");
//!
//! let mut track2 = track1.split();
//! track2.add_station("Station 4");
//!
//! let mut track3 = track2.split();
//! track2.add_station("Station 5");
//! track3.add_station("Station 6");
//!
//! track1.add_station("Station 7");
//! track2.add_station("Station 8");
//! track3.add_station("Station 9");
//!
//! let mut track4 = track3.split();
//! let track5 = track4.split();
//!
//! metro.add_station("Station 10 (Detached)");
//!
//! track5.join(&track1);
//!
//! track4.add_station("Station 11");
//!
//! track2.stop();
//!
//! track1.add_station("Station 12");
//! track3.add_station("Station 13");
//! track4.add_station("Station 14");
//!
//! track4.join(&track1);
//!
//! track3.add_station("Station 15");
//!
//! track3.stop();
//!
//! track1.add_station("Station 16");
//!
//! let string = metro.to_string().unwrap();
//!
//! println!("{}", string);
//! ```
//!
//! # Example Using `Event`
//!
//! *The following example outputs the graph above.*
//!
//! ```no_run
//! use metro::Event;
//!
//! let events = [
//!     Event::station(0, "Station 1"),
//!     Event::station(0, "Station 2"),
//!     Event::station(0, "Station 3"),
//!     Event::SplitTrack(0, 1),
//!     Event::station(1, "Station 4"),
//!     Event::SplitTrack(1, 2),
//!     Event::station(1, "Station 5"),
//!     Event::station(2, "Station 6"),
//!     Event::station(0, "Station 7"),
//!     Event::station(1, "Station 8"),
//!     Event::station(2, "Station 9"),
//!     Event::SplitTrack(2, 3),
//!     Event::SplitTrack(3, 4),
//!     Event::station(5, "Station 10 (Detached)"),
//!     Event::JoinTrack(4, 0),
//!     Event::station(3, "Station 11"),
//!     Event::StopTrack(1),
//!     Event::station(0, "Station 12"),
//!     Event::station(2, "Station 13"),
//!     Event::station(3, "Station 14"),
//!     Event::JoinTrack(3, 0),
//!     Event::station(2, "Station 15"),
//!     Event::StopTrack(2),
//!     Event::station(0, "Station 16"),
//! ];
//!
//! let string = metro::to_string(&events).unwrap();
//!
//! println!("{}", string);
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
// #![deny(missing_doc_code_examples)]
#![deny(missing_debug_implementations)]
#![warn(clippy::all)]

mod events;
mod metro;

pub use crate::metro::{Metro, Track};
pub use events::*;
