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
        "Just delete the `// I AM NOT DONE` line and save the file."
    ),
    ex!(
        "intro2",
        "00_intro",
        Run,
        "There are two `{}` placeholders but no arguments after the format string. \
         Add a second variable and pass both: `println!(\"...\", a, b)`."
    ),
    // 01 variables
    ex!(
        "variables1",
        "01_variables",
        Run,
        "Declaring a variable needs a keyword in front of the name."
    ),
    ex!(
        "variables2",
        "01_variables",
        Run,
        "`x` is declared but never given a value. Assign one in the `let`."
    ),
    ex!(
        "variables3",
        "01_variables",
        Run,
        "Variables are immutable by default. Which keyword after `let` makes them mutable?"
    ),
    ex!(
        "variables4",
        "01_variables",
        Run,
        "You can declare a *new* variable with the same name using another `let`. \
         This is called shadowing, and the new variable can have a different type."
    ),
    ex!(
        "variables5",
        "01_variables",
        Run,
        "Unlike `let`, constants always need an explicit type: `const NAME: Type = value;`."
    ),
    // 02 functions
    ex!(
        "functions1",
        "02_functions",
        Run,
        "Define `fn greet() { ... }` anywhere in the file."
    ),
    ex!(
        "functions2",
        "02_functions",
        Run,
        "Every function parameter needs a type, like `from: u32`."
    ),
    ex!(
        "functions3",
        "02_functions",
        Test,
        "Declare the return type with `-> i32`. Then look at the semicolon after \
         `n * n`: a block's last expression is only returned if it has no semicolon."
    ),
    // 03 if
    ex!(
        "if1",
        "03_if",
        Test,
        "`if` is an expression: `if cond { a } else { b }` evaluates to `a` or `b`."
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
        "Repeat a value with `[value; count]`. Slice with `&array[start..end]`, \
         where `end` is exclusive."
    ),
    ex!(
        "primitive_types2",
        "04_primitive_types",
        Test,
        "`let (a, b) = tuple;` destructures. Tuple fields are accessed with `.0`, `.1`, ..."
    ),
    // 05 vecs
    ex!(
        "vecs1",
        "05_vecs",
        Test,
        "`vec![1, 2, 3]` creates a Vec. Don't forget the semicolon."
    ),
    ex!(
        "vecs2",
        "05_vecs",
        Test,
        "In the loop, `element` is a `&mut i32`. Use `*element` to reach the value. \
         For the second function: `v.iter().map(|n| ...).collect()`."
    ),
    // 06 move semantics
    ex!(
        "move_semantics1",
        "06_move_semantics",
        Test,
        "The function owns `vec`, but the binding is immutable. Parameters can be \
         declared `mut` too: `fn f(mut x: T)`."
    ),
    ex!(
        "move_semantics2",
        "06_move_semantics",
        Test,
        "`fill_vec(v)` moves `v`, so it can't be used afterwards. Give the function \
         a copy instead with `.clone()`."
    ),
    ex!(
        "move_semantics3",
        "06_move_semantics",
        Test,
        "Take a `&mut String` parameter, and pass `&mut greeting` at the call site."
    ),
    ex!(
        "move_semantics4",
        "06_move_semantics",
        Test,
        "`push` needs a mutable borrow while `first` (a shared borrow) is still in use. Move the line that uses `first` so it comes before the push."
    ),
    ex!(
        "move_semantics5",
        "06_move_semantics",
        Test,
        "Return `&str` instead of `String`, and take `&str` so literals work too. `s.find(' ')` gives the index of the first space; slice with `&s[..i]`."
    ),
    // 07 structs
    ex!(
        "structs1",
        "07_structs",
        Test,
        "Fields look like `name: Type,`. Tuple struct fields are just types: \
         `struct Point(f64, f64);`."
    ),
    ex!(
        "structs2",
        "07_structs",
        Test,
        "Write `impl Rectangle { ... }`. `new` takes no `self`; `area` and \
         `can_hold` take `&self`."
    ),
    // 08 enums
    ex!(
        "enums1",
        "08_enums",
        Run,
        "Variants can be plain (`Quit`), hold a tuple (`Echo(String)`), or have \
         named fields (`Move { x: i32, y: i32 }`)."
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
        "A string literal is a `&str`. Convert it to an owned String with \
         `.to_string()` or `String::from(...)`."
    ),
    ex!(
        "strings2",
        "09_strings",
        Test,
        "Look for `trim`, `format!` and `replace` in the docs."
    ),
    // 10 modules
    ex!(
        "modules1",
        "10_modules",
        Run,
        "Only `make_sausage` needs to be callable from outside the module. Mark it `pub`."
    ),
    ex!(
        "modules2",
        "10_modules",
        Run,
        "`use a::b::{self, C};` brings both module `b` and item `C` into scope. \
         Is `veggies` public?"
    ),
    // 11 hashmaps
    ex!(
        "hashmaps1",
        "11_hashmaps",
        Test,
        "`HashMap::new()` makes an empty map, and `insert(key, value)` adds to it."
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
        "The syntax is `if let Some(x) = opt { ... }` and `while let Some(x) = expr { ... }`."
    ),
    // 13 error handling
    ex!(
        "errors1",
        "13_error_handling",
        Test,
        "Change the return type to `Result<String, String>`, then use `Ok(...)` \
         and `Err(...)` instead of `Some(...)` and `None`."
    ),
    ex!(
        "errors2",
        "13_error_handling",
        Test,
        "`parse` returns a `Result`. Put `?` after it to get the number out, or \
         return early with the error."
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
        "`struct Wrapper<T> { value: T }` and `impl<T> Wrapper<T> { ... }`."
    ),
    ex!(
        "generics2",
        "14_generics",
        Test,
        "The `>` operator comes from the `PartialOrd` trait: `fn largest<T: PartialOrd>`."
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
        "Write `fn append_bar(mut self) -> Self { ... }` inside the impl. \
         `push_str` will help."
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
        "Declare a lifetime `<'a>` on the function and use `&'a str` for both \
         inputs and the output."
    ),
    ex!(
        "lifetimes2",
        "16_lifetimes",
        Test,
        "`struct Excerpt<'a> { part: &'a str }`, then `impl<'a> Excerpt<'a>` and \
         tie `text` to the same lifetime."
    ),
    ex!(
        "lifetimes3",
        "16_lifetimes",
        Test,
        "Both inputs must outlive every use of `result`. Move `string2` out of the inner block, or move the assertion into it."
    ),
    // 17 tests
    ex!(
        "tests1",
        "17_tests",
        Test,
        "Call `is_even` with an even number in one test, and use `!is_even(...)` \
         with an odd number in the other."
    ),
    ex!(
        "tests2",
        "17_tests",
        Test,
        "Put `#[should_panic]` on the tests that are supposed to panic."
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
        "`first.to_uppercase()` yields chars; `collect::<String>()` them, then \
         append `chars.as_str()`. `collect()` can produce a Vec or a String."
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
        "Change the variant to `Cons(i32, Box<List>)`, then build the list with \
         nested `Box::new(...)` calls."
    ),
    ex!(
        "rc1",
        "19_smart_pointers",
        Test,
        "Use `Rc::clone(&sun)` for each planet, and `drop(earth)` to release one."
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
        "`handle.join()` waits for the thread and returns a Result containing the \
         closure's return value."
    ),
    ex!(
        "threads2",
        "20_threads",
        Test,
        "Start with `Arc::new(Mutex::new(0))`. Clone the Arc before each spawn, \
         and use `*counter.lock().unwrap() += 1` inside the thread."
    ),
    ex!(
        "threads3",
        "20_threads",
        Test,
        "Add `let tx = tx.clone();` inside the loop, before `spawn`. Then call `drop(tx);` after the loop, or `rx.iter()` will wait forever for the original sender."
    ),
    // 21 macros
    ex!(
        "macros1",
        "21_macros",
        Run,
        "Macro calls need a `!`. Also, `macro_rules!` macros must be defined above \
         where they're used."
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
        "`s.split_once(',')` returns `Option<(&str, &str)>`. Use `let ... else` or \
         `match` to bail out early with `Person::default()`."
    ),
    ex!(
        "try_from",
        "22_conversions",
        Test,
        "`u8::try_from(n)` fails for anything outside 0..=255. Use \
         `.map_err(|_| IntoColorError::OutOfRange)?` on each component."
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
