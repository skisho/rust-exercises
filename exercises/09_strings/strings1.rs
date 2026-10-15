// strings1.rs
//
// Rust has two main string types: `String` (owned, growable) and `&str`
// (a borrowed view of some text). Make this compile. Don't change the
// function signatures; fix `main` instead.

// I AM NOT DONE

fn shout(text: &str) -> String {
    text.to_uppercase() + "!"
}

fn store(names: &mut Vec<String>, name: String) {
    names.push(name);
}

fn main() {
    let name = String::from("ferris");
    println!("{}", shout(name));

    let mut names = Vec::new();
    store(&mut names, "corro");
    store(&mut names, name);

    let mut motto = "fast";
    motto.push_str(" and fearless");

    println!("{names:?} {motto}");
}
