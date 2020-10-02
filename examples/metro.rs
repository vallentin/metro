use metro::Metro;

fn main() {
    let mut metro = Metro::new();

    let mut track1 = metro.new_track();
    track1.add_station("Station 1");
    track1.add_station("Station 2");
    track1.add_station("Station 3");

    let mut track2 = track1.split();
    track2.add_station("Station 4");

    let mut track3 = track2.split();
    track2.add_station("Station 5");
    track3.add_station("Station 6");

    track1.add_station("Station 7");
    track2.add_station("Station 8");
    track3.add_station("Station 9");

    let mut track4 = track3.split();
    let track5 = track4.split();

    metro.add_station("Station 10 (Detached)");

    track5.join(&track1);

    track4.add_station("Station 11");

    track2.stop();

    track1.add_station("Station 12");
    track3.add_station("Station 13");
    track4.add_station("Station 14");

    track4.join(&track1);

    track3.add_station("Station 15");

    track3.stop();

    track1.add_station("Station 16");

    let string = metro.to_string().unwrap();

    println!("{}", string);
}
