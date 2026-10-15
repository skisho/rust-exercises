// rc1.rs
//
// Two playlists can contain the very same song. Instead of copying the
// song's data into each playlist, they share it with `Rc` ("reference
// counted"): every `Rc::clone` adds an owner, and the song is freed only
// when the last owner is gone.
//
// Replace each `???` and `todo!()` so the test passes. Don't clone the
// `Song` itself; clone the `Rc`.

// I AM NOT DONE

use std::rc::Rc;

#[derive(Debug)]
struct Song {
    title: String,
}

struct Playlist {
    songs: Vec<Rc<Song>>,
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_songs() {
        let anthem = Rc::new(Song {
            title: String::from("Crab Rave"),
        });
        let ballad = Rc::new(Song {
            title: String::from("Ferris' Lament"),
        });

        let mut party = Playlist { songs: Vec::new() };
        let mut chill = Playlist { songs: Vec::new() };

        party.songs.push(Rc::clone(&anthem));
        party.songs.push(Rc::clone(&ballad));
        // TODO: add `anthem` to `chill` as well.
        todo!();

        assert_eq!(Rc::strong_count(&anthem), ???);
        assert_eq!(Rc::strong_count(&ballad), ???);

        // Both playlists point at the *same* Song, not two copies.
        assert!(Rc::ptr_eq(&party.songs[0], &chill.songs[0]));
        assert_eq!(chill.songs[0].title, "Crab Rave");

        drop(party);
        assert_eq!(Rc::strong_count(&anthem), ???);
        assert_eq!(Rc::strong_count(&ballad), ???);
    }
}
