// refcell1.rs - solution
//
// `borrow_mut()` returns a guard giving `&mut` access; `borrow()` gives
// `&`. RefCell tracks borrows at runtime and panics if you break the
// rules (e.g. two `borrow_mut`s alive at once). `Rc<RefCell<T>>` is the
// standard recipe for shared, mutable data on one thread.

use std::cell::RefCell;
use std::rc::Rc;

struct Logger {
    messages: RefCell<Vec<String>>,
}

impl Logger {
    fn new() -> Self {
        Logger { messages: RefCell::new(Vec::new()) }
    }

    fn log(&self, message: &str) {
        self.messages.borrow_mut().push(message.to_string());
    }

    fn count(&self) -> usize {
        self.messages.borrow().len()
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
        let db = Service { name: "db", logger: Rc::clone(&logger) };
        let web = Service { name: "web", logger: Rc::clone(&logger) };

        db.run();
        web.run();
        web.run();

        assert_eq!(logger.count(), 3);
    }
}
