use std::error;
use std::fmt;
use std::io::{self, Write};
use std::iter;
use std::string::FromUtf8Error;

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

    /// `StopTrack(track_id)`
    ///
    /// - If `track_id` does not exist, then this event does nothing.
    ///
    /// All rails to the right of `track_id`, are pulled to the left.
    ///
    /// ## Output Example
    ///
    /// Given 3 tracks `0, 1, 2` then `StopTrack(1)` would render as:
    ///
    /// ```text
    /// | | |
    /// | " |
    /// |  /
    /// | |
    /// ```
    StopTrack(usize),

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
    /// - If `to_track_id` does not exist, then it turns into `StopTrack(from_track_id)`.
    /// - If `from_track_id` and `to_track_id` are the same, then it turns into `StopTrack(from_track_id)`
    ///
    /// The track ID (`from_track_id`) can be reused for
    /// a new track after this event.
    ///
    /// ## Output Example
    ///
    /// Given 3 tracks `0, 1, 2` then `JoinTrack(1, 0)` would render as:
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

/// Write `&[`[`Event`]`]` to [`<W: io::Write>`].
/// Defines a default track with `track_id` of `0`.
///
/// *See also [`to_string`] and [`to_vec`].*
///
/// [`to_vec`]: fn.to_vec.html
/// [`to_string`]: fn.to_string.html
///
/// [`Event`]: enum.Event.html
///
/// [`<W: io::Write>`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html
pub fn to_writer<W: Write>(mut writer: W, events: &[Event]) -> Result<(), Error> {
    let mut tracks = vec![0];

    for event in events {
        use Event::*;
        match event {
            &StartTrack(track_id) => {
                if !tracks.contains(&track_id) {
                    tracks.push(track_id);

                    let line = iter::repeat("|")
                        .take(tracks.len())
                        .collect::<Vec<_>>()
                        .join(" ");

                    writeln!(&mut writer, "{}", line)?;
                }
            }

            &StartTracks(track_ids) => {
                let mut render = false;

                for track_id in track_ids.iter() {
                    if !tracks.contains(track_id) {
                        tracks.push(*track_id);

                        render = true;
                    }
                }

                if render {
                    let line = iter::repeat("|")
                        .take(tracks.len())
                        .collect::<Vec<_>>()
                        .join(" ");

                    writeln!(&mut writer, "{}", line)?;
                }
            }

            &StopTrack(track_id) => stop_track(&mut writer, &mut tracks, track_id)?,

            &Station(track_id, station_name) => {
                let line = tracks
                    .iter()
                    .map(|&id| if id == track_id { "*" } else { "|" })
                    .collect::<Vec<_>>()
                    .join(" ");

                writeln!(&mut writer, "{} {}", line, station_name)?;
            }

            &SplitTrack(from_track_id, new_track_id) => {
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

                    writeln!(&mut writer, "{}", line)?;

                    if let Some(index) = from_track_index {
                        tracks.insert(index + 1, new_track_id);
                    } else {
                        tracks.push(new_track_id);
                    }
                }
            }

            &JoinTrack(from_track_id, to_track_id) => {
                let from_track_index = tracks.iter().position(|&id| id == from_track_id);

                if from_track_id == to_track_id {
                    stop_track(&mut writer, &mut tracks, from_track_id)?;
                    continue;
                }

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

                            writeln!(&mut writer, "{}", line)?;
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

                            writeln!(&mut writer, "{}", line)?;

                            let line = (0..(tracks.len() - 1))
                                .map(|i| if i == left_index { "|/" } else { "| " })
                                .collect::<Vec<_>>()
                                .concat();

                            writeln!(&mut writer, "{}", line)?;
                        }

                        tracks.remove(from_track_index);
                    } else {
                        stop_track(&mut writer, &mut tracks, from_track_id)?;
                    }
                }
            }

            NoEvent => {
                let line = iter::repeat("|")
                    .take(tracks.len())
                    .collect::<Vec<_>>()
                    .join(" ");

                writeln!(&mut writer, "{}", line)?;
            }
        }
    }

    Ok(())
}

fn stop_track<W: Write>(
    mut writer: W,
    tracks: &mut Vec<usize>,
    track_id: usize,
) -> Result<(), Error> {
    if let Some(index) = tracks.iter().position(|&id| id == track_id) {
        let line = (0..tracks.len())
            .map(|i| if i == index { "\"" } else { "|" })
            .collect::<Vec<_>>()
            .join(" ");

        writeln!(&mut writer, "{}", line)?;

        if index != (tracks.len() - 1) {
            let line = (0..tracks.len())
                .map(|i| {
                    use std::cmp::Ordering::*;
                    match i.cmp(&index) {
                        Greater => "/",
                        Equal => "",
                        Less => "|",
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");

            writeln!(&mut writer, "{}", line)?;
        }

        tracks.remove(index);
    }

    Ok(())
}

/// Write `&[`[`Event`]`]` to [`Vec<u8>`].
/// Defines a default track with `track_id` of `0`.
///
/// *See also [`to_string`] and [`to_writer`].*
///
/// [`to_writer`]: fn.to_writer.html
/// [`to_string`]: fn.to_string.html
///
/// [`Event`]: enum.Event.html
///
/// [`Vec<u8>`]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
#[inline]
pub fn to_vec(events: &[Event]) -> Result<Vec<u8>, Error> {
    let mut vec = Vec::new();
    to_writer(&mut vec, events)?;
    Ok(vec)
}

/// Write `&[`[`Event`]`]` to [`String`].
/// Defines a default track with `track_id` of `0`.
///
/// *See also [`to_vec`] and [`to_writer`].*
///
/// [`to_writer`]: fn.to_writer.html
/// [`to_vec`]: fn.to_vec.html
///
/// [`Event`]: enum.Event.html
///
/// [`String`]: https://doc.rust-lang.org/stable/std/string/struct.String.html
#[inline]
pub fn to_string(events: &[Event]) -> Result<String, Error> {
    let vec = to_vec(events)?;
    Ok(String::from_utf8(vec)?)
}

/// `Error` is an error that can be returned by
/// [`to_string`], [`to_vec`], and [`to_writer`].
///
/// [`to_writer`]: fn.to_writer.html
/// [`to_vec`]: fn.to_vec.html
/// [`to_string`]: fn.to_string.html
#[derive(Debug)]
pub enum Error {
    /// [See `std::io::Error`][io::Error].
    ///
    /// [io::Error]: https://doc.rust-lang.org/std/io/struct.Error.html
    IoError(io::Error),

    /// [See `std::string::FromUtf8Error`][FromUtf8Error].
    ///
    /// [FromUtf8Error]: https://doc.rust-lang.org/std/string/struct.FromUtf8Error.html
    FromUtf8Error(FromUtf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            IoError(err) => err.fmt(fmt),
            FromUtf8Error(err) => err.fmt(fmt),
        }
    }
}

impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            IoError(ref err) => Some(err),
            FromUtf8Error(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    #[inline]
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<FromUtf8Error> for Error {
    #[inline]
    fn from(err: FromUtf8Error) -> Self {
        Self::FromUtf8Error(err)
    }
}