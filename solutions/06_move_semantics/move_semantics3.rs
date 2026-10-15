// move_semantics3.rs - solution
//
// A loop body runs many times, so moving a value inside it would leave
// nothing for the next iteration. A mutable reference (`&mut`) lets
// `record` change the Vec while `run_events` keeps owning it. The caller
// must opt in by declaring `log` as `mut` and writing `&mut log`, so it's
// always clear at the call site that something may be modified.

fn record(log: &mut Vec<String>, event: &str) {
    log.push(format!("[event] {event}"));
}

fn run_events(events: &[&str]) -> Vec<String> {
    let mut log = Vec::new();
    for event in events {
        record(&mut log, event);
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
