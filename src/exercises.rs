//! The ordered list of exercises. To add one, create
//! `exercises/<dir>/<name>.rs` and `solutions/<dir>/<name>.rs`, add an entry
//! here, and register a `[[bin]]` for it in `exercises/Cargo.toml` and
//! `solutions/Cargo.toml`. The `consistency` tests check all of this.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// The exercise passes when its `main` compiles and runs successfully.
    Run,
    /// The exercise passes when its `#[test]`s compile and pass.
    Test,
}

pub struct Exercise {
    pub name: &'static str,
    pub dir: &'static str,
    pub mode: Mode,
    pub hint: &'static str,
}

impl Exercise {
    pub fn path(&self) -> String {
        format!("exercises/{}/{}.rs", self.dir, self.name)
    }

    pub fn solution_path(&self) -> String {
        format!("solutions/{}/{}.rs", self.dir, self.name)
    }
}

macro_rules! ex {
    ($name:literal, $dir:literal, $mode:ident, $hint:literal) => {
        Exercise {
            name: $name,
            dir: $dir,
            mode: Mode::$mode,
            hint: $hint,
        }
    };
}

pub const EXERCISES: &[Exercise] = &[
    // 00 intro
    ex!(
        "intro1",
        "00_intro",
        Run,
        "Nothing to fix here. Just delete the `// I AM NOT DONE` line and save the file."
    ),
    ex!(
        "intro2",
        "00_intro",
        Run,
        "`{}` needs the `Display` trait, which tuples and arrays don't have. Use the debug placeholder `{:?}` instead."
    ),
    // 01 variables
    ex!(
        "variables1",
        "01_variables",
        Run,
        "Each `let advice` disappears at its closing `}`. Declare `advice` once, before the `if`. Since `if` is an expression, you can write `let advice = if raining { ... } else { ... };`."
    ),
    ex!(
        "variables2",
        "01_variables",
        Run,
        "Whole numbers default to `i32`, which tops out around 2.1 billion. Annotate the big ones with a 64-bit type: `let x: u64 = ...;`."
    ),
    ex!(
        "variables3",
        "01_variables",
        Run,
        "Variables are immutable by default. Add `mut` after `let`, but only on the variables that actually change."
    ),
    ex!(
        "variables4",
        "01_variables",
        Test,
        "Work through it one `let` at a time. A `let` inside `{ ... }` only hides the outer variable until the closing brace. In the second test, `parse` turns \"17\" into the number 17."
    ),
    ex!(
        "variables5",
        "01_variables",
        Run,
        "`let` isn't allowed outside functions: use `const NAME: Type = value;` (constants always need a type). An array length must be a constant too, so store `board_area`'s result in a `const`. For that, the function has to be a `const fn`."
    ),
    // 02 functions
    ex!(
        "functions1",
        "02_functions",
        Test,
        "List both parameters, each with its own type: `fn rectangle_area(width: u32, height: u32) -> u32`."
    ),
    ex!(
        "functions2",
        "02_functions",
        Test,
        "Start with `if age < 3 { return 0; }`. Then compute a `base` price with an `if`/`else if` chain, and finish with `if is_tuesday { base / 2 } else { base }`, with no semicolon."
    ),
    ex!(
        "functions3",
        "02_functions",
        Test,
        "The parameter list is `f: fn(i32) -> i32, start: i32`, and the function returns `-> i32`. `negate` is an ordinary function returning `-n`."
    ),
    // 03 if
    ex!(
        "if1",
        "03_if",
        Test,
        "`if c <= 0.0 { \"freezing\" } else if c < 15.0 { ... } else if ... else { \"hot\" }`. Floats need a decimal point: write `0.0`, not `0`."
    ),
    ex!(
        "if2",
        "03_if",
        Test,
        "Check for \"divisible by both\" (i.e. by 15) first. Use `n.to_string()` \
         for the plain-number case."
    ),
    // 04 primitive types
    ex!(
        "primitive_types1",
        "04_primitive_types",
        Test,
        "`matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')`. For digits: `c as u32 - '0' as u32`. 'é' takes two bytes in UTF-8, and `300 as u8` keeps only the remainder after dividing by 256."
    ),
    ex!(
        "primitive_types2",
        "04_primitive_types",
        Test,
        "Slice the end with `&week[5..]`. For `extremes`, start `low` and `high` at `week[0]` and loop. `let (low, high) = extremes(&WEEK);` destructures, and `[-3; 7]` repeats a value."
    ),
    // 05 vecs
    ex!(
        "vecs1",
        "05_vecs",
        Test,
        "`Vec::new()`, `queue.push(..)`, `queue.insert(0, ..)`, and `queue.remove(0)`. Check `queue.is_empty()` before removing. Songs arrive as `&str`, so convert them with `.to_string()`."
    ),
    ex!(
        "vecs2",
        "05_vecs",
        Test,
        "`item.1 += amount;` updates the quantity through the `&mut`. `stock.retain(|(_, q)| *q > 0)` keeps the items in stock. `stock.get(index)` returns an Option instead of panicking."
    ),
    // 06 move semantics
    ex!(
        "move_semantics1",
        "06_move_semantics",
        Test,
        "Take `words: &[String]` instead of `Vec<String>`, and call it as `total_letters(&names)`."
    ),
    ex!(
        "move_semantics2",
        "06_move_semantics",
        Test,
        "Put `#[derive(Debug, Clone, Copy, PartialEq)]` on `Point`. `Copy` needs `Clone` too."
    ),
    ex!(
        "move_semantics3",
        "06_move_semantics",
        Test,
        "Take `log: &mut Vec<String>`, declare `let mut log`, and call `record(&mut log, event)`."
    ),
    ex!(
        "move_semantics4",
        "06_move_semantics",
        Test,
        "Collect the plurals into a new Vec inside the loop, then add them after the loop with `words.extend(plurals)`."
    ),
    ex!(
        "move_semantics5",
        "06_move_semantics",
        Test,
        "Return the owned `String` itself: change the return type to `String` and return `format!(...)` directly."
    ),
    // 07 structs
    ex!(
        "structs1",
        "07_structs",
        Test,
        "The fields are `title: String, author: String, pages: u32, edition: u32`. Shorthand: `Book { title, author, pages, edition: 1 }`. Update syntax: `Book { edition: 2, ..book.clone() }`."
    ),
    ex!(
        "structs2",
        "07_structs",
        Test,
        "`fn open(owner: &str) -> Self` takes no `self`. `balance` takes `&self`; `deposit` and `withdraw` take `&mut self`. In `withdraw`, check `amount > self.balance` before subtracting."
    ),
    // 08 enums
    ex!(
        "enums1",
        "08_enums",
        Test,
        "Add `Green, Yellow, Red,` to the enum. Then `match self { Light::Green => Light::Yellow, ... }`, with one arm per variant."
    ),
    ex!(
        "enums2",
        "08_enums",
        Test,
        "Add a match arm for each variant, destructuring the fields like the \
         `Circle` arm does. A triangle's area is half base times height."
    ),
    // 09 strings
    ex!(
        "strings1",
        "09_strings",
        Run,
        "`shout` wants a `&str`, so pass `&name`. `store` wants a `String`, so convert the literal with `.to_string()`. Only a `String` can grow, so `motto` must be `String::from(\"fast\")`."
    ),
    ex!(
        "strings2",
        "09_strings",
        Test,
        "`initials`: loop over `split_whitespace()`, take `word.chars().next()`, and push its `to_uppercase()` plus a '.'. `is_palindrome`: collect the lowercase alphanumeric chars into a Vec and compare it with its reverse. `censor`: `text.replace(word, &\"*\".repeat(...))`."
    ),
    // 10 modules
    ex!(
        "modules1",
        "10_modules",
        Run,
        "Make `Vault`, `unlock` and the `owner` field `pub`, but not `pin`. Code outside the module can't write a struct literal with a private field, so add a `pub fn new(owner: &str, pin: u32) -> Self` and call it from `main`."
    ),
    ex!(
        "modules2",
        "10_modules",
        Run,
        "In `game`, use `super::rules::points_for` (or `crate::rules::points_for`). Add `use game::score as game_score;`. And `banner` needs `pub`."
    ),
    // 11 hashmaps
    ex!(
        "hashmaps1",
        "11_hashmaps",
        Test,
        "`insert` replaces old values. `self.entries.get(name).map(|s| s.as_str())` gives `Option<&str>`. In `rename`, check `contains_key(new)` first, then `remove(old)` and re-`insert` its number under `new`."
    ),
    ex!(
        "hashmaps2",
        "11_hashmaps",
        Test,
        "`*counts.entry(key).or_insert(0) += 1;` and use `word.to_lowercase()` as the key."
    ),
    // 12 options
    ex!(
        "options1",
        "12_options",
        Test,
        "Loop and `return Some(n)` when you find one; end with `None`. \
         In the test, `.unwrap()` gets the value out of a `Some`."
    ),
    ex!(
        "options2",
        "12_options",
        Test,
        "`nickname.unwrap_or(real_name)`, `middle.map(|m| m.len())`, `setting.and_then(|s| s.parse().ok())`, and `score.filter(|&s| s > best)`."
    ),
    // 13 error handling
    ex!(
        "errors1",
        "13_error_handling",
        Test,
        "Change the return type to `Result<&str, String>`. Check each rule in order with `if` / `else if` and return `Err(\"...\".to_string())` for the first one that fails, or `Ok(name)` at the end."
    ),
    ex!(
        "errors2",
        "13_error_handling",
        Test,
        "Put `?` after each `parse::<u32>()`. If parsing fails, `?` returns the error from `parse_size` straight away."
    ),
    ex!(
        "errors3",
        "13_error_handling",
        Test,
        "`impl From<ParseIntError> for ParsePosError { fn from(e: ParseIntError) -> Self { ... } }`. \
         Then check `n <= 0` and return `Err(ParsePosError::NotPositive(n))`."
    ),
    ex!(
        "errors4",
        "13_error_handling",
        Test,
        "The error type is `Box<dyn Error>`. Any error type converts into it automatically with `?`."
    ),
    ex!(
        "errors5",
        "13_error_handling",
        Test,
        "`people.get(name)` gives an Option. `.ok_or_else(|| LookupError::Missing(...))?` turns None into your error. Then `text.parse::<u32>().map_err(|_| LookupError::BadAge(...))`."
    ),
    // 14 generics
    ex!(
        "generics1",
        "14_generics",
        Test,
        "`struct Pair<A, B> { first: A, second: B }`, then `impl<A, B> Pair<A, B>`. `swap` returns `Pair<B, A>`."
    ),
    ex!(
        "generics2",
        "14_generics",
        Test,
        "Comparing needs `PartialOrd` and `{}` needs `Display`. Add `where T: PartialOrd + Display` between the return type and the `{`."
    ),
    ex!(
        "generics3",
        "14_generics",
        Test,
        "The basic methods just call `Vec` methods: `push`, `pop`, `last`. Then add a second block, `impl<T: Display> Stack<T> { fn describe(&self) -> String { ... } }`, and use `to_string` plus `join(\", \")`."
    ),
    // 15 traits
    ex!(
        "traits1",
        "15_traits",
        Test,
        "`impl Emoji for Weather { fn emoji(&self) -> char { match self { ... } } }`. For bool, `if *self { '✅' } else { '❌' }`. In `forecast`, push each item's emoji onto a String in a loop, or `items.iter().map(|i| i.emoji()).collect()`."
    ),
    ex!(
        "traits2",
        "15_traits",
        Test,
        "A trait method can have a body; implementors may then omit it. \
         For `notify`, use `&impl Summary` as the parameter type."
    ),
    ex!(
        "traits3",
        "15_traits",
        Test,
        "A slice of trait objects is `&[Box<dyn Shape>]`. In the test, wrap each shape in `Box::new(...)` and annotate the Vec as `Vec<Box<dyn Shape>>`."
    ),
    ex!(
        "traits4",
        "15_traits",
        Test,
        "Add `#[derive(Debug, Clone, PartialEq, Default)]` above the struct. For Display: `impl fmt::Display for Point { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, ...) } }`."
    ),
    ex!(
        "traits5",
        "15_traits",
        Test,
        "`impl Add for Vec2 { type Output = Vec2; fn add(self, other: Vec2) -> Vec2 { ... } }`. `Mul<f64>` looks the same, except `fn mul(self, factor: f64)`."
    ),
    // 16 lifetimes
    ex!(
        "lifetimes1",
        "16_lifetimes",
        Test,
        "Put the lifetime only on `text` and the result: `fn strip_tag<'a>(text: &'a str, tag: &str) -> &'a str`. Giving `tag` the same lifetime breaks the second test."
    ),
    ex!(
        "lifetimes2",
        "16_lifetimes",
        Test,
        "`struct Words<'a> { rest: &'a str }`, `impl<'a> Words<'a>`, `fn new(text: &'a str)`, and `fn next_word(&mut self) -> Option<&'a str>`. Inside, trim the start, find the next space (or use the length), `split_at` there, and keep the rest in `self.rest`."
    ),
    ex!(
        "lifetimes3",
        "16_lifetimes",
        Test,
        "String literals have the type `&'static str`. Use that for the `reason` field."
    ),
    // 17 tests
    ex!(
        "tests1",
        "17_tests",
        Test,
        "Divide by `numbers.len()`, not `len() + 1`, and return `None` early when the slice is empty. The last test needs just one `assert_eq!`."
    ),
    ex!(
        "tests2",
        "17_tests",
        Test,
        "Add `-> Result<(), String>` to the first two tests and end each with `Ok(())`. For the third, copy part of the real panic message (\"would make the price negative\") into `expected`."
    ),
    // 18 iterators
    ex!(
        "iterators1",
        "18_iterators",
        Test,
        "`numbers.iter().filter(|n| ...).map(|n| ...).sum()`. Inside `filter` the \
         item is `&&i32`, so you may need `*n` or a `|&&n|` pattern."
    ),
    ex!(
        "iterators2",
        "18_iterators",
        Test,
        "`split_whitespace().rev().collect::<Vec<_>>().join(\" \")`. `words.iter().map(|&w| (w, w.len())).collect()`. `items.iter().map(|s| s.parse::<i32>()).collect()`: the return type tells `collect` to build a `Result<Vec<_>, _>`."
    ),
    ex!(
        "iterators3",
        "18_iterators",
        Test,
        "`players.iter().flat_map(|p| p.scores.iter()).sum()`. For the best player: \
         `max_by_key(|p| total)` gives an Option<&Player>; then `.map(|p| p.name)`."
    ),
    ex!(
        "iterators4",
        "18_iterators",
        Test,
        "Write `type Item = u64;` and `fn next(&mut self) -> Option<u64>`. Save `current`, move `next` into `current`, add the old value onto `next`, and return `Some(saved)`."
    ),
    ex!(
        "iterators5",
        "18_iterators",
        Test,
        "In order: `.enumerate().map(|(i, s)| ...)`, `.windows(2).map(|w| w[1] - w[0])`, `a.iter().zip(b).map(|(x, y)| x * y).sum()`, `.copied().take_while(|&n| n >= 0)`, and `.collect::<HashSet<_>>().len()` (import HashSet)."
    ),
    // 19 smart pointers
    ex!(
        "box1",
        "19_smart_pointers",
        Test,
        "Change the variants to `Add(Box<Expr>, Box<Expr>)` and so on. In `eval`, `match expr` and recurse: `Add(a, b) => eval(a) + eval(b)`. Build the example with `mul(add(..), neg(..))`."
    ),
    ex!(
        "rc1",
        "19_smart_pointers",
        Test,
        "Use `chill.songs.push(Rc::clone(&anthem));`. Then count the owners: each `Rc` (the original variable plus each clone stored in a playlist) counts as one."
    ),
    ex!(
        "refcell1",
        "19_smart_pointers",
        Test,
        "Make the field `RefCell<Vec<String>>` and create it with `RefCell::new(Vec::new())`. Use `self.messages.borrow_mut().push(...)` to write and `.borrow().len()` to read."
    ),
    ex!(
        "cow1",
        "19_smart_pointers",
        Test,
        "`if input.contains('\\t') { Cow::Owned(input.replace(...)) } else { Cow::Borrowed(input) }`"
    ),
    // 20 threads
    ex!(
        "threads1",
        "20_threads",
        Test,
        "Wrap the body in `thread::scope(|s| { ... })` and spawn with `s.spawn(...)` instead of `thread::spawn`. The scope's closure can return `total`."
    ),
    ex!(
        "threads2",
        "20_threads",
        Test,
        "`let tally = Arc::new(Mutex::new(HashMap::new()));`, and `let tally = Arc::clone(&tally);` before each spawn. Inside the thread, `let mut tally = tally.lock().unwrap();`. At the end: `Arc::try_unwrap(tally).unwrap().into_inner().unwrap()`."
    ),
    ex!(
        "threads3",
        "20_threads",
        Test,
        "Add `move` to both closures. Clone `report_tx` before `squared_tx` moves into the squarer thread. Then `drop(report_tx)` after sending the marker, or the channel never closes."
    ),
    // 21 macros
    ex!(
        "macros1",
        "21_macros",
        Test,
        "Move the macro above `main`. Add a second rule: `($name:expr) => { format!(\"Hello, {}!\", $name) };`."
    ),
    ex!(
        "macros2",
        "21_macros",
        Test,
        "Two rules: `($x:expr) => { $x };` and \
         `($x:expr, $($rest:expr),+) => { std::cmp::max($x, max!($($rest),+)) };`."
    ),
    // 22 conversions
    ex!(
        "from_into",
        "22_conversions",
        Test,
        "`impl From<Celsius> for Fahrenheit { fn from(c: Celsius) -> Self { Fahrenheit(c.0 * 9.0 / 5.0 + 32.0) } }`. A tuple parameter can be destructured: `fn from((r, g, b): (u8, u8, u8))`. For hex, use `format!(\"#{:02x}{:02x}{:02x}\", ...)`."
    ),
    ex!(
        "try_from",
        "22_conversions",
        Test,
        "`match n { 1 => Ok(Weekday::Monday), ..., _ => Err(WeekdayError::OutOfRange(n)) }`. For names, write `impl TryFrom<&str> for Weekday` with `type Error = WeekdayError;` and match on `name.to_ascii_lowercase().as_str()`."
    ),
    // 23 challenges: longer projects, each with a reading list in its header
    ex!(
        "rpn",
        "23_challenges",
        Test,
        "Loop over `input.split_whitespace()`. Match the four operator strings first, then fall back to `token.parse::<i64>()`. Pop `b` before `a`. `stack.pop().ok_or(RpnError::StackUnderflow)?` handles underflow neatly. For Display, `match self` and `write!` each message."
    ),
    ex!(
        "json",
        "23_challenges",
        Test,
        "Write a `write_escaped(f, s)` helper that matches each char. Use `write!(f, \"\\\\u{:04x}\", c as u32)` for control characters. Recurse with `write!(f, \"{item}\")`. For `pretty`, write a helper `fn pretty_into(&self, out: &mut String, depth: usize)` and fall back to `to_string()` for scalars and empty containers."
    ),
    ex!(
        "matrix",
        "23_challenges",
        Test,
        "Element (r, c) lives at `data[r * cols + c]`. Put the bounds check in one helper and use it from both `index` and `index_mut`. For `&a * &b`, write `impl Mul for &Matrix { type Output = Matrix; ... }`. Each result cell is `sum over k of a[(r, k)] * b[(k, c)]`."
    ),
    ex!(
        "linked_list",
        "23_challenges",
        Test,
        "Push: `Box::new(Node { value, next: self.head.take() })`. Pop: `self.head.take().map(|node| { self.head = node.next; node.value })`. Iter: start from `self.head.as_deref()`. Drop: loop, taking each node's `next` before the node goes out of scope. Chapter 2 of Too Many Linked Lists covers all of this."
    ),
    ex!(
        "lru_cache",
        "23_challenges",
        Test,
        "Fields: `capacity`, a `HashMap<K, V>` for values, and a `VecDeque<K>` ordered from least to most recently used. To \"use\" a key, find it with `iter().position`, `remove` it, and `push_back` it. To evict, use `pop_front`. In `get`, check `contains_key` before touching, then look the value up again."
    ),
    ex!(
        "calculator",
        "23_challenges",
        Test,
        "Lexer: `input.chars().peekable()`. Collect digits and dots while `peek()` matches, then parse. Parser: `struct Parser { tokens: Vec<Token>, pos: usize }` with `expr`, `term` and `factor` methods that mirror the grammar. After `expr()` returns, any leftover token is an `UnexpectedToken`."
    ),
    ex!(
        "thread_pool",
        "23_challenges",
        Test,
        "Store `Vec<JoinHandle<()>>` and `Option<Sender<Job>>`. Each worker loops on `let msg = receiver.lock().unwrap().recv();` and breaks on `Err`. In `drop`, `take()` the sender and drop it, then `join` every worker. The Rust book, ch. 21.2–21.3, builds this step by step."
    ),
];

#[cfg(test)]
mod consistency {
    use super::*;
    use std::collections::HashSet;
    use std::path::Path;

    const ROOT: &str = env!("CARGO_MANIFEST_DIR");

    #[test]
    fn names_are_unique() {
        let mut seen = HashSet::new();
        for ex in EXERCISES {
            assert!(seen.insert(ex.name), "duplicate exercise name {}", ex.name);
        }
    }

    #[test]
    fn files_exist() {
        for ex in EXERCISES {
            assert!(
                Path::new(ROOT).join(ex.path()).is_file(),
                "missing {}",
                ex.path()
            );
            assert!(
                Path::new(ROOT).join(ex.solution_path()).is_file(),
                "missing {}",
                ex.solution_path()
            );
        }
    }

    #[test]
    fn registered_as_bins() {
        for manifest in ["exercises/Cargo.toml", "solutions/Cargo.toml"] {
            let toml = std::fs::read_to_string(Path::new(ROOT).join(manifest)).unwrap();
            for ex in EXERCISES {
                let entry = format!(
                    "name = \"{}\"\npath = \"{}/{}.rs\"",
                    ex.name, ex.dir, ex.name
                );
                assert!(
                    toml.contains(&entry),
                    "{manifest} has no [[bin]] for {}",
                    ex.name
                );
            }
        }
    }

    #[test]
    fn solutions_have_no_marker() {
        for ex in EXERCISES {
            let src = std::fs::read_to_string(Path::new(ROOT).join(ex.solution_path())).unwrap();
            assert!(
                !crate::has_marker(&src),
                "solution {} has a marker",
                ex.name
            );
        }
    }
}
