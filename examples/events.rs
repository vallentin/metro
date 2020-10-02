use metro::Event::*;

fn main() {
    let events = [
        Station(0, "Station 1"),
        Station(0, "Station 2"),
        Station(0, "Station 3"),
        SplitTrack(0, 1),
        Station(1, "Station 4"),
        SplitTrack(1, 2),
        Station(1, "Station 5"),
        Station(2, "Station 6"),
        Station(0, "Station 7"),
        Station(1, "Station 8"),
        Station(2, "Station 9"),
        SplitTrack(2, 3),
        SplitTrack(3, 4),
        Station(5, "Station 10 (Detached)"),
        JoinTrack(4, 0),
        Station(3, "Station 11"),
        StopTrack(1),
        Station(0, "Station 12"),
        Station(2, "Station 13"),
        Station(3, "Station 14"),
        JoinTrack(3, 0),
        Station(2, "Station 15"),
        StopTrack(2),
        Station(0, "Station 16"),
    ];

    let string = metro::to_string(&events).unwrap();

    println!("{}", string);
}
