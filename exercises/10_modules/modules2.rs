// modules2.rs
//
// Make this compile. There are three problems, all about *paths*:
//
//   1. `game::score` calls a function in a *sibling* module. A plain name
//      is looked up relative to the current module, so you need `super::`
//      (the parent module) or `crate::` (the root).
//   2. `ui` has its own `score` function, so importing `game::score`
//      into `main` would clash. Import it under a different name with `as`.
//   3. One item that `main` uses isn't visible from outside its module.
//
// Don't change any function bodies except the path in `game::score`.

// I AM NOT DONE

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
        clears.iter().map(|&n| rules::points_for(n)).sum()
    }
}

mod ui {
    pub fn score(points: u32) -> String {
        format!("SCORE: {points:06}")
    }

    fn banner() -> &'static str {
        "== BLOCK DROP =="
    }
}

// TODO: add a `use` for `game::score`, renamed so it doesn't clash.

fn main() {
    let total = game_score(&[1, 0, 4, 2]);
    println!("{}", ui::banner());
    println!("{}", ui::score(total));
}
