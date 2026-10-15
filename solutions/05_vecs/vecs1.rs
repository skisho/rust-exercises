// vecs1.rs - solution
//
// `Vec::new()` allocates nothing until the first push. `insert(0, ..)`
// and `remove(0)` shift every other element along, which is fine for a
// small queue. For big queues, `std::collections::VecDeque` does both ends
// cheaply.

fn new_queue() -> Vec<String> {
    Vec::new()
}

fn add_song(queue: &mut Vec<String>, song: &str) {
    queue.push(song.to_string());
}

fn play_next_now(queue: &mut Vec<String>, song: &str) {
    queue.insert(0, song.to_string());
}

fn play(queue: &mut Vec<String>) -> Option<String> {
    if queue.is_empty() {
        None
    } else {
        Some(queue.remove(0))
    }
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
