// rc1.rs - solution
//
// `anthem` has three owners: the original `Rc`, `party` and `chill`.
// `ballad` has two. Dropping `party` drops its Vec, which drops each `Rc`
// in it, so each count falls by one. The songs themselves are freed only
// at the end of the test, when their counts reach zero.
//
// `Rc::clone(&x)` (rather than `x.clone()`) is the usual style: it makes
// clear that only a pointer is copied, not the data.

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
        chill.songs.push(Rc::clone(&anthem));

        assert_eq!(Rc::strong_count(&anthem), 3);
        assert_eq!(Rc::strong_count(&ballad), 2);

        assert!(Rc::ptr_eq(&party.songs[0], &chill.songs[0]));
        assert_eq!(chill.songs[0].title, "Crab Rave");

        drop(party);
        assert_eq!(Rc::strong_count(&anthem), 2);
        assert_eq!(Rc::strong_count(&ballad), 1);
    }
}
