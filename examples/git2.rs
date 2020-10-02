use std::collections::HashMap;

use git2::Repository;

use metro::Metro;

const MAX_COMMITS: usize = 100;

fn main() {
    let path = std::env::args().nth(1);
    let path = path.as_deref().unwrap_or(".");

    let repo = Repository::open(path).unwrap();

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let revwalk = revwalk.take(MAX_COMMITS).filter_map(Result::ok);

    let mut metro = Metro::new();
    let mut tracks = HashMap::new();

    for id in revwalk {
        let commit = if let Ok(commit) = repo.find_commit(id) {
            commit
        } else {
            break;
        };

        let mut cur_track = if tracks.is_empty() {
            Some(metro.new_track())
        } else {
            tracks.remove(&id)
        };

        if let Some(track) = cur_track.as_mut() {
            if let Some(msg) = commit.summary() {
                track.add_station(msg.to_owned());
            }
        }

        for i in 0..commit.parent_count() {
            let par_id = commit.parent(i).unwrap().id();

            if (i == 0) && cur_track.is_some() {
                continue;
            }

            let track = if let Some(cur_track) = &cur_track {
                cur_track.split()
            } else {
                metro.new_track()
            };

            tracks.insert(par_id, track);
        }

        if let Some(cur_track) = cur_track {
            if let Ok(par) = commit.parent(0) {
                if let Some(old_track) = tracks.remove(&par.id()) {
                    old_track.join(&cur_track);
                }

                tracks.insert(par.id(), cur_track);
            }
        }
    }

    let string = metro.to_string().unwrap();

    println!("{}", string);
}
