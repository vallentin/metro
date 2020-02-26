use std::io::{self, Write};
use std::iter;

/// `Event`s are produced automatically by using [`Metro`],
/// but can also be created and used manually.
///
/// An `Event` specifies an action and is used when rendering
/// the metro lines graph.
///
/// [`Metro`]: struct.Metro.html
///
/// # Example
///
/// ```no_run
/// use metro::Event::*;
///
/// let events = [
///     Station(0, "Station 1"),
///     Station(0, "Station 2"),
///     Station(0, "Station 3"),
///     SplitTrack(0, 1),
///     Station(1, "Station 4"),
///     SplitTrack(1, 2),
///     Station(1, "Station 5"),
///     Station(2, "Station 6"),
///     Station(0, "Station 7"),
///     Station(1, "Station 8"),
///     Station(2, "Station 9"),
///     SplitTrack(2, 3),
///     SplitTrack(3, 4),
///     Station(5, "Station 10 (Detached)"),
///     JoinTrack(4, 0),
///     Station(3, "Station 11"),
///     StopTrack(1),
///     Station(0, "Station 12"),
///     Station(2, "Station 13"),
///     Station(3, "Station 14"),
///     JoinTrack(3, 0),
///     Station(2, "Station 15"),
///     StopTrack(2),
///     Station(0, "Station 16"),
/// ];
///
/// let string = metro::to_string(&events).unwrap();
///
/// println!("{}", string);
/// ```
///
/// This will output the following:
///
/// ```text
/// * Station 1
/// * Station 2
/// * Station 3
/// |\
/// | * Station 4
/// | |\
/// | * | Station 5
/// | | * Station 6
/// * | | Station 7
/// | * | Station 8
/// | | * Station 9
/// | | |\
/// | | | |\
/// | | | | | Station 10 (Detached)
/// | |_|_|/
/// |/| | |
/// | | | * Station 11
/// | " | |
/// |  / /
/// * | | Station 12
/// | * | Station 13
/// | | * Station 14
/// | |/
/// |/|
/// | * Station 15
/// | "
/// * Station 16
/// ```
#[derive(Clone, Debug)]
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
    /// Text with multiple lines is also allowed.
    /// Given 3 tracks `0, 1, 2` then `Station(1, "Hello\nWorld")` would render as:
    ///
    /// ```text
    /// | | |
    /// | * | Hello
    /// | | | World
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
    /// - If `from_track_id` does not exist, then this event does nothing.
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
/// *[See also `Metro::to_writer`.][`Metro::to_writer`]*
///
/// *See also [`to_string`] and [`to_vec`].*
///
/// [`to_vec`]: fn.to_vec.html
/// [`to_string`]: fn.to_string.html
///
/// [`Event`]: enum.Event.html
///
/// [`Metro::to_writer`]: struct.Metro.html#method.to_writer
///
/// [`<W: io::Write>`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html
pub fn to_writer<W: Write>(mut writer: W, events: &[Event]) -> io::Result<()> {
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
                let mut line = tracks
                    .iter()
                    .map(|&id| if id == track_id { "*" } else { "|" })
                    .collect::<Vec<_>>()
                    .join(" ");

                for (i, station_name) in station_name.lines().enumerate() {
                    if i == 1 {
                        line = iter::repeat("|")
                            .take(tracks.len())
                            .collect::<Vec<_>>()
                            .join(" ");
                    }

                    writeln!(&mut writer, "{} {}", line, station_name)?;
                }
            }

            &SplitTrack(from_track_id, new_track_id) => {
                if !tracks.contains(&new_track_id) {
                    let from_track_index = tracks.iter().position(|&id| id == from_track_id);

                    if let Some(from_track_index) = from_track_index {
                        let line = (0..tracks.len())
                            .map(|i| {
                                use std::cmp::Ordering::*;
                                match i.cmp(&from_track_index) {
                                    Greater => "\\",
                                    Equal => "|\\",
                                    Less => "|",
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" ");

                        writeln!(&mut writer, "{}", line)?;

                        tracks.insert(from_track_index + 1, new_track_id);
                    } else {
                        tracks.push(new_track_id);

                        let line = iter::repeat("|")
                            .take(tracks.len())
                            .collect::<Vec<_>>()
                            .join(" ");

                        writeln!(&mut writer, "{}", line)?;
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

                            let track_count = tracks.len() - 1;
                            let line = (0..track_count)
                                .map(|i| {
                                    if i == left_index {
                                        "|/"
                                    } else if i == (track_count - 1) {
                                        "|"
                                    } else {
                                        "| "
                                    }
                                })
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

fn stop_track<W: Write>(mut writer: W, tracks: &mut Vec<usize>, track_id: usize) -> io::Result<()> {
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
/// *[See also `Metro::to_vec`.][`Metro::to_vec`]*
///
/// *See also [`to_string`] and [`to_writer`].*
///
/// [`to_writer`]: fn.to_writer.html
/// [`to_string`]: fn.to_string.html
///
/// [`Event`]: enum.Event.html
///
/// [`Metro::to_vec`]: struct.Metro.html#method.to_vec
///
/// [`Vec<u8>`]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
#[inline]
pub fn to_vec(events: &[Event]) -> io::Result<Vec<u8>> {
    let mut vec = Vec::new();
    to_writer(&mut vec, events)?;
    Ok(vec)
}

/// Write `&[`[`Event`]`]` to [`String`].
/// Defines a default track with `track_id` of `0`.
///
/// *[See also `Metro::to_string`.][`Metro::to_string`]*
///
/// *See also [`to_vec`] and [`to_writer`].*
///
/// [`to_writer`]: fn.to_writer.html
/// [`to_vec`]: fn.to_vec.html
///
/// [`Event`]: enum.Event.html
///
/// [`Metro::to_string`]: struct.Metro.html#method.to_string
///
/// [`String`]: https://doc.rust-lang.org/stable/std/string/struct.String.html
#[inline]
pub fn to_string(events: &[Event]) -> io::Result<String> {
    let vec = to_vec(events)?;
    // Ok(String::from_utf8(vec)?)
    // Metro only writes `str`s and `String`s to the `vec`
    // which are always valid UTF-8, so this is safe.
    #[allow(unsafe_code)]
    unsafe {
        Ok(String::from_utf8_unchecked(vec))
    }
}

/*
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
// */

#[cfg(test)]
mod tests {
    use super::to_string;
    use super::Event::*;

    #[test]
    fn start_track() {
        let events = [StartTrack(1)];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "| |\n");
    }

    #[test]
    fn start_track_already_exists() {
        let events = [StartTrack(0)];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "");
    }

    #[test]
    fn start_track_already_exists2() {
        #[rustfmt::skip]
        let events = [
            StartTrack(1),
            StartTrack(2),
            StartTrack(1),
            StartTrack(2),
        ];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "| |\n| | |\n");
    }

    #[test]
    fn start_track_default() {
        let events = [StartTrack(0)];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "");
    }

    #[test]
    fn event_start_track_default2() {
        let events = [StartTrack(1)];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "| |\n");
    }

    #[test]
    fn event_start_tracks() {
        let events = [StartTracks(&[1, 2, 3])];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "| | | |\n");
    }

    #[test]
    fn start_tracks_some_already_exist() {
        let events = [StartTracks(&[0, 1, 2])];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "| | |\n");
    }

    #[test]
    fn start_tracks_all_already_exist() {
        let events = [StartTracks(&[0, 0, 0])];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "");
    }

    #[test]
    fn stop_track() {
        let events = [StopTrack(0)];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "\"\n");
    }

    #[test]
    fn stop_track_does_not_exist() {
        let events = [StopTrack(1)];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "");
    }

    #[test]
    fn stop_track_left() {
        #[rustfmt::skip]
        let events = [
            StartTracks(&[0, 1, 2, 3, 4]),
            StopTrack(0),
            NoEvent,
        ];
        let string = to_string(&events).unwrap();

        assert_eq!(
            string,
            r#"| | | | |
" | | | |
 / / / /
| | | |
"#
        );
    }

    #[test]
    fn stop_track_middle() {
        #[rustfmt::skip]
        let events = [
            StartTracks(&[0, 1, 2, 3, 4]),
            StopTrack(2),
            NoEvent,
        ];
        let string = to_string(&events).unwrap();

        assert_eq!(
            string,
            r#"| | | | |
| | " | |
| |  / /
| | | |
"#
        );
    }

    #[test]
    fn stop_track_right() {
        #[rustfmt::skip]
        let events = [
            StartTracks(&[0, 1, 2, 3, 4]),
            StopTrack(4),
            NoEvent,
        ];
        let string = to_string(&events).unwrap();

        assert_eq!(
            string,
            r#"| | | | |
| | | | "
| | | |
"#
        );
    }

    #[test]
    fn station() {
        let events = [
            StartTracks(&[0, 1, 2]),
            Station(0, "Station 1"),
            Station(1, "Station 2"),
            Station(2, "Station 3"),
            Station(0, "Station 4"),
            Station(1, "Station 5"),
            Station(2, "Station 6"),
        ];
        let string = to_string(&events).unwrap();

        assert_eq!(
            string,
            r#"| | |
* | | Station 1
| * | Station 2
| | * Station 3
* | | Station 4
| * | Station 5
| | * Station 6
"#
        );
    }

    #[test]
    fn station_non_existing_track() {
        let events = [
            StartTracks(&[0, 1, 2]),
            Station(0, "Station 1"),
            Station(1, "Station 2"),
            Station(2, "Station 3"),
            Station(3, "Station 4"),
            Station(4, "Station 5"),
            Station(5, "Station 6"),
            Station(std::usize::MAX, "Station 7"),
        ];
        let string = to_string(&events).unwrap();

        assert_eq!(
            string,
            r#"| | |
* | | Station 1
| * | Station 2
| | * Station 3
| | | Station 4
| | | Station 5
| | | Station 6
| | | Station 7
"#
        );
    }

    #[test]
    fn split_track() {
        let events = [
            SplitTrack(0, 1),
            NoEvent,
            SplitTrack(0, 2),
            SplitTrack(1, 3),
            SplitTrack(3, 4),
        ];
        let string = to_string(&events).unwrap();

        assert_eq!(
            string,
            r#"|\
| |
|\ \
| | |\
| | | |\
"#
        );
    }

    #[test]
    fn split_track_non_existing_from_track() {
        let events1 = [
            SplitTrack(1, 2),
            SplitTrack(3, 4),
            Station(2, "2"),
            Station(4, "4"),
        ];
        let events2 = [
            StartTrack(2),
            StartTrack(4),
            Station(2, "2"),
            Station(4, "4"),
        ];

        let string1 = to_string(&events1).unwrap();
        let string2 = to_string(&events2).unwrap();

        assert_eq!(string1, string2);
    }

    #[test]
    fn split_track_already_existing_new_track() {
        let events1 = [
            StartTracks(&[0, 1, 2]),
            SplitTrack(0, 1),
            SplitTrack(0, 2),
            SplitTrack(3, 4),
            Station(0, "0"),
            Station(1, "1"),
            Station(2, "2"),
            Station(3, "3"),
            Station(4, "4"),
        ];
        let events2 = [
            StartTracks(&[0, 1, 2]),
            StartTrack(4),
            Station(0, "0"),
            Station(1, "1"),
            Station(2, "2"),
            Station(3, "3"),
            Station(4, "4"),
        ];

        let string1 = to_string(&events1).unwrap();
        let string2 = to_string(&events2).unwrap();

        assert_eq!(string1, string2);
    }

    #[test]
    fn split_track_same_from_and_new_track() {
        let events = [SplitTrack(0, 0)];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "");

        #[rustfmt::skip]
        let events1 = [
            StartTracks(&[0, 1, 2]),
            SplitTrack(1, 1),
            SplitTrack(0, 2),
        ];
        let events2 = [StartTracks(&[0, 1, 2])];

        let string1 = to_string(&events1).unwrap();
        let string2 = to_string(&events2).unwrap();

        assert_eq!(string1, string2);
    }

    #[test]
    fn join_track_zero_between() {
        #[rustfmt::skip]
        let events = [
            StartTracks(&[0, 1, 2]),
            JoinTrack(1, 0),
            NoEvent,
        ];
        let string = to_string(&events).unwrap();

        assert_eq!(
            string,
            r#"| | |
|/ /
| |
"#
        );
    }

    #[test]
    fn join_track_one_between() {
        #[rustfmt::skip]
        let events = [
            StartTracks(&[0, 1, 2]),
            JoinTrack(2, 0),
            NoEvent,
        ];
        let string = to_string(&events).unwrap();

        assert_eq!(
            string,
            r#"| | |
| |/
|/|
| |
"#
        );
    }

    #[test]
    fn join_track_many_between() {
        #[rustfmt::skip]
        let events = [
            StartTracks(&[0, 1, 2, 3, 4]),
            JoinTrack(4, 0),
            NoEvent,
        ];
        let string = to_string(&events).unwrap();

        assert_eq!(
            string,
            r#"| | | | |
| |_|_|/
|/| | |
| | | |
"#
        );
    }

    #[test]
    fn join_track_always_leftmost() {
        let events1 = [
            StartTracks(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            JoinTrack(4, 1),
            JoinTrack(5, 0),
            JoinTrack(8, 7),
            JoinTrack(9, 6),
            NoEvent,
        ];

        let events2 = events1
            .iter()
            .cloned()
            .map(|event| match event {
                JoinTrack(from, to) => JoinTrack(to, from),
                _ => event,
            })
            .collect::<Vec<_>>();

        let string1 = to_string(&events1).unwrap();
        let string2 = to_string(&events2).unwrap();

        // Note these only visually looks the same,
        // the actual deleted track is not the same.
        assert_eq!(string1, string2);
    }

    // The above `join_track` tests are all `existing_from_track_existing_to_track`

    #[test]
    fn join_track_non_existing_from_track_existing_to_track() {
        let events1 = [
            StartTracks(&[0, 1, 2, 3, 4]),
            JoinTrack(5, 0),
            JoinTrack(10, 1),
        ];
        let events2 = [StartTracks(&[0, 1, 2, 3, 4])];

        let string1 = to_string(&events1).unwrap();
        let string2 = to_string(&events2).unwrap();

        assert_eq!(string1, string2);
    }

    #[test]
    fn join_track_non_existing_from_track_non_existing_to_track() {
        let events1 = [
            StartTracks(&[0, 1, 2, 3, 4]),
            JoinTrack(5, 6),
            JoinTrack(10, 11),
        ];
        let events2 = [StartTracks(&[0, 1, 2, 3, 4])];

        let string1 = to_string(&events1).unwrap();
        let string2 = to_string(&events2).unwrap();

        assert_eq!(string1, string2);
    }

    #[test]
    fn join_track_existing_from_track_non_existing_to_track() {
        let events1 = [
            StartTracks(&[0, 1, 2, 3, 4]),
            JoinTrack(0, 5),
            JoinTrack(2, 10),
        ];
        #[rustfmt::skip]
        let events2 = [
            StartTracks(&[0, 1, 2, 3, 4]),
            StopTrack(0),
            StopTrack(2),
        ];

        let string1 = to_string(&events1).unwrap();
        let string2 = to_string(&events2).unwrap();

        assert_eq!(string1, string2);
    }

    #[test]
    fn join_track_same_from_and_to_track() {
        let events1 = [
            StartTracks(&[0, 1, 2, 3, 4]),
            JoinTrack(0, 0),
            JoinTrack(2, 2),
            JoinTrack(10, 10),
        ];
        #[rustfmt::skip]
        let events2 = [
            StartTracks(&[0, 1, 2, 3, 4]),
            StopTrack(0),
            StopTrack(2),
        ];

        let string1 = to_string(&events1).unwrap();
        let string2 = to_string(&events2).unwrap();

        assert_eq!(string1, string2);
    }

    #[test]
    fn no_event() {
        let events = [NoEvent, NoEvent, NoEvent];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "|\n|\n|\n");

        #[rustfmt::skip]
        let events = [
            StartTracks(&[0, 1, 2]),
            NoEvent,
            NoEvent,
            NoEvent,
        ];
        let string = to_string(&events).unwrap();

        assert_eq!(string, "| | |\n| | |\n| | |\n| | |\n");
    }
}
