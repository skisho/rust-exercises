// vecs1.rs
//
// A `Vec` is a growable list. Build a tiny music queue with it:
//   - `new_queue` returns an empty queue
//   - `add_song` puts a song at the back
//   - `play_next_now` puts a song at the *front*
//   - `play` removes and returns the song at the front, or None if empty
//
// Look up `push`, `insert` and `remove` in the Vec docs. (`remove` panics
// on an empty Vec, so check first.)

// I AM NOT DONE

fn new_queue() -> Vec<String> {
    todo!()
}

fn add_song(queue: &mut Vec<String>, song: &str) {
    todo!()
}

fn play_next_now(queue: &mut Vec<String>, song: &str) {
    todo!()
}

fn play(queue: &mut Vec<String>) -> Option<String> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_empty() {
        let mut q = new_queue();
        assert!(q.is_empty());
        assert_eq!(play(&mut q), None);
    }

    #[test]
    fn first_in_first_out() {
        let mut q = new_queue();
        add_song(&mut q, "Yellow Submarine");
        add_song(&mut q, "Bohemian Rhapsody");
        assert_eq!(q.len(), 2);
        assert_eq!(play(&mut q).as_deref(), Some("Yellow Submarine"));
        assert_eq!(play(&mut q).as_deref(), Some("Bohemian Rhapsody"));
        assert_eq!(play(&mut q), None);
    }

    #[test]
    fn jump_the_queue() {
        let mut q = new_queue();
        add_song(&mut q, "Track A");
        add_song(&mut q, "Track B");
        play_next_now(&mut q, "Urgent Track");
        assert_eq!(q, ["Urgent Track", "Track A", "Track B"]);
    }
}
