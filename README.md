# Learning Rust 🦀

Small exercises for getting comfortable reading and writing Rust, in the
spirit of [rustlings](https://github.com/rust-lang/rustlings). Every exercise
is a tiny program that either doesn't compile or fails its tests. Fix it, and
the runner moves you on to the next one.

## Getting started

You need a Rust toolchain (1.85 or newer). Then, from the repository root:

```bash
cargo run
```

This starts **watch mode**: it checks the exercises in order and stops at the
first one that isn't finished yet. Open that file in your editor, fix it, and
save. The runner re-checks it automatically.

When an exercise passes, the runner waits for you to delete its
`// I AM NOT DONE` marker line. That gives you a chance to experiment before
moving on.

While watch mode is running, type one of these and press Enter:

| Key | Action                                |
|-----|---------------------------------------|
| `h` | Show a hint for the current exercise  |
| `l` | List all exercises                    |
| `c` | Re-check everything from the start    |
| `q` | Quit                                  |

## Commands

```text
cargo run -- watch              Re-check your progress whenever you save (default)
cargo run -- verify             Check every exercise in order, stopping at the first unfinished one
cargo run -- list               Show all exercises and whether they're done
cargo run -- run <name>         Compile and run a single exercise, showing its output
cargo run -- hint <name>        Show a hint for an exercise
cargo run -- solution <name>    Print the reference solution for an exercise
cargo run -- reset <name>       Restore an exercise to its original state (uses git)
cargo run -- verify-solutions   Check that every reference solution passes
```

## Topics

| #  | Topic           | Exercises                                  | Rust Book chapter |
|----|-----------------|--------------------------------------------|-------------------|
| 00 | intro           | intro1–2                                   | 1                 |
| 01 | variables       | variables1–5                               | 3.1               |
| 02 | functions       | functions1–3                               | 3.3               |
| 03 | if              | if1–2                                      | 3.5               |
| 04 | primitive types | primitive_types1–2                         | 3.2, 4.3          |
| 05 | vecs            | vecs1–2                                    | 8.1               |
| 06 | move semantics  | move_semantics1–5                          | 4.1–4.3           |
| 07 | structs         | structs1–2                                 | 5                 |
| 08 | enums           | enums1–2                                   | 6                 |
| 09 | strings         | strings1–2                                 | 8.2               |
| 10 | modules         | modules1–2                                 | 7                 |
| 11 | hashmaps        | hashmaps1–2                                | 8.3               |
| 12 | options         | options1–2                                 | 6.1–6.3           |
| 13 | error handling  | errors1–5                                  | 9                 |
| 14 | generics        | generics1–3                                | 10.1              |
| 15 | traits          | traits1–5                                  | 10.2, 18.2, 20.3  |
| 16 | lifetimes       | lifetimes1–3                               | 10.3              |
| 17 | tests           | tests1–2                                   | 11                |
| 18 | iterators       | iterators1–5                               | 13.2–13.4         |
| 19 | smart pointers  | box1, rc1, refcell1, cow1                  | 15                |
| 20 | threads         | threads1–3                                 | 16                |
| 21 | macros          | macros1–2                                  | 20.5              |
| 22 | conversions     | from_into, try_from                        | (std docs)        |
| 23 | **challenges**  | see below                                  | various           |

Reading the matching chapter of [The Rust Programming Language](https://doc.rust-lang.org/book/)
before each topic helps a lot.

## Challenges

The last topic contains bigger projects. Each one should take anywhere
from half an hour to an evening. They pull together several earlier topics,
and you'll probably need to read some documentation along the way. Every
challenge's header comment includes a **Further reading** list of links.

| Challenge     | You'll build                                             | Brings together                                      |
|---------------|----------------------------------------------------------|------------------------------------------------------|
| `rpn`         | A Reverse Polish Notation calculator                     | Error enums, `Display`, `std::error::Error`          |
| `json`        | A JSON serialiser with compact and pretty output         | Recursive enums, `fmt::Formatter`, string escaping   |
| `matrix`      | A matrix type with `m[(r, c)]` indexing and `&a * &b`    | `Index`/`IndexMut`, operator overloading on references |
| `linked_list` | A singly linked stack with iterators and in-place reverse | `Box`, `Option::take`, lifetimes, `Drop`, `FromIterator` |
| `lru_cache`   | A least-recently-used cache (you design the struct)      | `HashMap`, `VecDeque`, generics with trait bounds    |
| `calculator`  | A lexer and recursive-descent parser for `2 * (3 + 4)`   | Enums, `Peekable`, recursion, error handling         |
| `thread_pool` | A thread pool that shuts down gracefully                 | `Arc<Mutex<_>>`, channels, trait objects, `Drop`     |

Tips:

- `cargo run -- run <name>` shows the full test output for one challenge,
  which is handy while you work through a long list of failing tests.
- Read the tests. They're the specification.
- Try a challenge before reading its links in depth. Struggling first makes
  the reading stick.
- `linked_list` and `thread_pool` follow well-known write-ups (*Too Many
  Linked Lists* and the Rust book's final project). Have a go on your own,
  then compare your version with theirs.

## How it's laid out

```text
src/            The runner: CLI, watch mode, and the ordered exercise list with hints
exercises/      What you edit. One binary per exercise in the `exercises` package
solutions/      Reference solutions, with a comment explaining each one
```

Each exercise is registered as a `[[bin]]` in `exercises/Cargo.toml`, so
RustRover and rust-analyzer give you completion and inline errors as you work.
Don't run `cargo build` inside `exercises/`, though: unfinished exercises
won't compile. Use the runner.

Try not to peek at `solutions/` until you've had a real go. Hints
(`cargo run -- hint <name>`) are there for when you're stuck.

## Adding an exercise

1. Write `exercises/<topic>/<name>.rs` (with the `// I AM NOT DONE` marker)
   and `solutions/<topic>/<name>.rs` (without it).
2. Add an entry, with a hint, to `EXERCISES` in `src/exercises.rs`.
3. Add a `[[bin]]` for it to both `exercises/Cargo.toml` and `solutions/Cargo.toml`.
4. Run `cargo test` (checks the list, files, and manifests agree) and
   `cargo run -- verify-solutions`.
