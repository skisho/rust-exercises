// modules2.rs - solution
//
// Paths are relative to the current module. Inside `game`, `super` is the
// crate root, where `rules` lives. (`crate::rules::points_for` would work
// too and doesn't depend on where `game` sits.)
//
// `use path::item as new_name;` imports an item under another name, which
// is the usual fix when two modules export the same name.

mod rules {
    pub fn points_for(lines_cleared: u32) -> u32 {
        match lines_cleared {
            0 => 0,
            1 => 100,
            2 => 300,
            3 => 500,
            _ => 800,
        }
    }
}

mod game {
    pub fn score(clears: &[u32]) -> u32 {
        clears.iter().map(|&n| super::rules::points_for(n)).sum()
    }
}

mod ui {
    pub fn score(points: u32) -> String {
        format!("SCORE: {points:06}")
    }

    pub fn banner() -> &'static str {
        "== BLOCK DROP =="
    }
}

use game::score as game_score;

fn main() {
    let total = game_score(&[1, 0, 4, 2]);
    println!("{}", ui::banner());
    println!("{}", ui::score(total));
}
