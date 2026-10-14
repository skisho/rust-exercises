// refcell1.rs
//
// Two services share one `Logger` through `Rc`. But `Rc` only hands out
// shared (`&`) references, so `log` takes `&self`, and then it can't
// push to the Vec.
//
// Use `RefCell` for "interior mutability": mutation through a shared
// reference, with the borrow rules checked at runtime instead of compile
// time. Change the `messages` field and fix `log` and `count`.

// I AM NOT DONE

use std::cell::RefCell;
use std::rc::Rc;

struct Logger {
    messages: Vec<String>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            messages: Vec::new(),
        }
    }

    fn log(&self, message: &str) {
        self.messages.push(message.to_string());
    }

    fn count(&self) -> usize {
        self.messages.len()
    }
}

struct Service {
    name: &'static str,
    logger: Rc<Logger>,
}

impl Service {
    fn run(&self) {
        self.logger.log(&format!("{} ran", self.name));
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_logger() {
        let logger = Rc::new(Logger::new());
        let db = Service {
            name: "db",
            logger: Rc::clone(&logger),
        };
        let web = Service {
            name: "web",
            logger: Rc::clone(&logger),
        };

        db.run();
        web.run();
        web.run();

        assert_eq!(logger.count(), 3);
    }
}
