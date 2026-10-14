// iterators3.rs
//
// Each player has a list of scores. Using iterator adapters:
//   - `total_points` adds up every score of every player
//   - `best_player` returns the name of the player with the highest total
//     (or None if there are no players)

// I AM NOT DONE

struct Player {
    name: &'static str,
    scores: Vec<u32>,
}

fn total_points(players: &[Player]) -> u32 {
    // TODO: try `flat_map` or `map` + `sum` twice.
    todo!()
}

fn best_player(players: &[Player]) -> Option<&str> {
    // TODO: try `max_by_key`.
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn players() -> Vec<Player> {
        vec![
            Player {
                name: "Ada",
                scores: vec![10, 20, 5],
            },
            Player {
                name: "Grace",
                scores: vec![30, 15],
            },
            Player {
                name: "Linus",
                scores: vec![],
            },
        ]
    }

    #[test]
    fn totals() {
        assert_eq!(total_points(&players()), 80);
    }

    #[test]
    fn best() {
        assert_eq!(best_player(&players()), Some("Grace"));
    }

    #[test]
    fn nobody() {
        assert_eq!(best_player(&[]), None);
    }
}
