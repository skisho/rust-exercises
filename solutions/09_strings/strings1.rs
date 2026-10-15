// strings1.rs - solution
//
// - `shout` wants a `&str`, so borrow the String with `&name`. (A `&String`
//   converts to `&str` automatically.)
// - `store` wants an owned `String`, so a literal must be converted with
//   `.to_string()` or `String::from`. Passing `name` itself is fine: it's
//   a String, and we don't use it afterwards.
// - A literal is an immutable `&str` that lives in the program binary;
//   only a `String` can grow, so `motto` must be one.

fn shout(text: &str) -> String {
    text.to_uppercase() + "!"
}

fn store(names: &mut Vec<String>, name: String) {
    names.push(name);
}

fn main() {
    let name = String::from("ferris");
    println!("{}", shout(&name));

    let mut names = Vec::new();
    store(&mut names, "corro".to_string());
    store(&mut names, name);

    let mut motto = String::from("fast");
    motto.push_str(" and fearless");

    println!("{names:?} {motto}");
}
