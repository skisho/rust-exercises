// move_semantics3.rs
//
// `record` should add an entry to the caller's log. But it takes the log
// by value, so the loop moves it on the first iteration and the compiler
// complains on the second.
//
// Change `record`'s signature and the call inside the loop. Don't change
// anything else.

// I AM NOT DONE

fn record(mut log: Vec<String>, event: &str) {
    log.push(format!("[event] {event}"));
}

fn run_events(events: &[&str]) -> Vec<String> {
    let log = Vec::new();
    for event in events {
        record(log, event);
    }
    log
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_every_event() {
        let log = run_events(&["boot", "login", "logout"]);
        assert_eq!(log, ["[event] boot", "[event] login", "[event] logout"]);
    }

    #[test]
    fn nothing_happened() {
        assert!(run_events(&[]).is_empty());
    }
}
