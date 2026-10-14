// iterators3.rs - solution
//
// `flat_map` flattens nested collections into one stream. `max_by_key`
// returns an Option, which `map` transforms without unwrapping.

struct Player {
    name: &'static str,
    scores: Vec<u32>,
}

fn total_points(players: &[Player]) -> u32 {
    players.iter().flat_map(|p| p.scores.iter()).sum()
}

fn best_player(players: &[Player]) -> Option<&str> {
    players
        .iter()
        .max_by_key(|p| p.scores.iter().sum::<u32>())
        .map(|p| p.name)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn players() -> Vec<Player> {
        vec![
            Player { name: "Ada", scores: vec![10, 20, 5] },
            Player { name: "Grace", scores: vec![30, 15] },
            Player { name: "Linus", scores: vec![] },
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
