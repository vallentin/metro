//! Metro

#![forbid(unsafe_code)]
#![deny(missing_docs)]
// #![deny(missing_doc_code_examples)]
#![deny(missing_debug_implementations)]
#![warn(clippy::all)]

use std::iter;

/// `Event`
#[derive(Debug)]
pub enum Event<'a> {
    /// `StartTrack(track_id)`
    ///
    /// - If `track_id` already exists, then this event does nothing.
    ///
    /// New `track_id`s are added rightmost.
    ///
    /// ## Output Example
    ///
    /// Given 3 tracks `0, 1, 2` then `StartTrack(4)` would render as:
    ///
    /// ```text
    /// | | |
    /// | | | |
    /// | | | |
    /// ```
    StartTrack(usize),

    /// `StartTracks(track_ids)`
    ///
    /// - If a `track_id` from `track_ids` already exists, then it is ignored.
    /// - If all `track_ids` already exists, then this event does nothing.
    ///
    /// New `track_id`s are added rightmost.
    ///
    /// ## Output Example
    ///
    /// Given 3 tracks `0, 1, 2` then `StartTracks(&[4, 5])` would render as:
    ///
    /// ```text
    /// | | |
    /// | | | | |
    /// | | | | |
    /// ```
    StartTracks(&'a [usize]),

    /// `Station(track_id, text)`
    ///
    /// - If the `track_id` does not exist, then `text` is still
    /// rendered, just not tied to any track.
    ///
    /// ## Output Example
    ///
    /// Given 3 tracks `0, 1, 2` then `Station(1, "Hello World")` would render as:
    ///
    /// ```text
    /// | | |
    /// | * | Hello World
    /// | | |
    /// ```
    ///
    /// If the `track_id` does not exist, then no rail is highlighted.
    /// Thus `Station(10, "Hello World")` would render as:
    ///
    /// ```text
    /// | | |
    /// | | | Hello World
    /// | | |
    /// ```
    Station(usize, &'a str),

    /// `SplitTrack(from_track_id, new_track_id)`
    ///
    /// Creates a new track diverging from `from_track_id` to the right.
    /// All rails to the right of `from_track_id`, are pushed to the
    /// right to make space for the new track.
    ///
    /// - If `from_track_id` does not exist, then this event is the
    /// same as `StartTrack(new_track_id)`.
    /// - If `new_track_id` already exists, then this event does nothing.
    ///
    /// ## Output Example
    ///
    /// Given 3 tracks `0, 1, 2` then `SplitTrack(1, 4)` would render as:
    ///
    /// ```text
    /// | | |
    /// | |\ \
    /// | | | |
    /// ```
    SplitTrack(usize, usize),

    /// `JoinTrack(from_track_id, to_track_id)`
    ///
    /// Joins `from_track_id` and `to_track_id`
    /// resulting in the `from_track_id` being removed.
    ///
    /// The rails are joined towards the leftmost rail.
    ///
    /// - If `from_track_id` does not exist, then it turns into a `NoEvent`.
    ///
    /// The track ID (`from_track_id`) can be reused for
    /// a new track after this event.
    ///
    /// ## Output Example
    ///
    /// Given 3 tracks `0, 1, 2` then `SplitTrack(1, 0)` would render as:
    ///
    /// ```text
    /// | | |
    /// |/ /
    /// | |
    /// ```
    ///
    /// Given 6 tracks `0, 1, 2, 3, 4, 5` then `JoinTrack(4, 0)` would render as:
    ///
    /// ```text
    /// | | | | | |
    /// | |_|_|/ /
    /// |/| | | |
    /// | | | | |
    /// ```
    JoinTrack(usize, usize),

    /// `NoEvent` produces one row of rails.
    ///
    /// ## Output Example
    ///
    /// Given 3 tracks `0, 1, 2` then `NoEvent` would render as:
    ///
    /// ```text
    /// | | |
    /// ```
    NoEvent,
}

/// *Test function that is being converted into an iterator.*
pub fn _print_events(events: &[Event<'_>]) {
    let mut tracks = vec![];

    for event in events {
        use Event::*;
        match event {
            StartTrack(track_id) => {
                if !tracks.contains(&track_id) {
                    tracks.push(track_id);

                    let line = iter::repeat("|")
                        .take(tracks.len())
                        .collect::<Vec<_>>()
                        .join(" ");

                    println!("{}", line);
                }
            }

            StartTracks(track_ids) => {
                let mut render = false;

                for track_id in track_ids.iter() {
                    if !tracks.contains(&track_id) {
                        tracks.push(track_id);

                        render = true;
                    }
                }

                if render {
                    let line = iter::repeat("|")
                        .take(tracks.len())
                        .collect::<Vec<_>>()
                        .join(" ");

                    println!("{}", line);
                }
            }

            Station(track_id, station_name) => {
                let line = tracks
                    .iter()
                    .map(|&id| if id == track_id { "*" } else { "|" })
                    .collect::<Vec<_>>()
                    .join(" ");

                println!("{} {}", line, station_name);
            }

            SplitTrack(from_track_id, new_track_id) => {
                if !tracks.contains(&new_track_id) {
                    let mut from_track_index = None;

                    let line = tracks
                        .iter()
                        .enumerate()
                        .map(|(i, &id)| {
                            if from_track_index.is_some() {
                                "\\"
                            } else if id == from_track_id {
                                from_track_index = Some(i);
                                "|\\"
                            } else {
                                "|"
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");

                    println!("{}", line);

                    if let Some(index) = from_track_index {
                        tracks.insert(index + 1, new_track_id);
                    } else {
                        tracks.push(new_track_id);
                    }
                }
            }

            JoinTrack(from_track_id, to_track_id) => {
                let from_track_index = tracks.iter().position(|&id| id == from_track_id);

                if let Some(from_track_index) = from_track_index {
                    let to_track_index = tracks.iter().position(|&id| id == to_track_id);

                    if let Some(to_track_index) = to_track_index {
                        let left_index = from_track_index.min(to_track_index);
                        let right_index = from_track_index.max(to_track_index);

                        if (right_index - left_index) == 1 {
                            let line = (0..tracks.len())
                                .filter_map(|i| {
                                    if i > right_index {
                                        Some("/")
                                    } else if i == left_index {
                                        Some("|/")
                                    } else if i != right_index {
                                        Some("|")
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(" ");

                            println!("{}", line);
                        } else {
                            let line = (0..tracks.len())
                                .filter_map(|i| {
                                    if i > right_index {
                                        Some(" /")
                                    } else if i == right_index {
                                        None
                                    } else if i >= (right_index - 1) {
                                        Some("|/")
                                    } else if i > left_index {
                                        Some("|_")
                                    } else {
                                        Some("| ")
                                    }
                                })
                                .collect::<Vec<_>>()
                                .concat();

                            println!("{}", line);

                            let line = (0..(tracks.len() - 1))
                                .map(|i| if i == left_index { "|/" } else { "| " })
                                .collect::<Vec<_>>()
                                .concat();

                            println!("{}", line);
                        }
                    }

                    tracks.remove(from_track_index);
                }
            }

            NoEvent => {
                let line = iter::repeat("|")
                    .take(tracks.len())
                    .collect::<Vec<_>>()
                    .join(" ");

                println!("{}", line);
            }
        }
    }
}
