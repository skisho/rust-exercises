// intro2.rs - solution
//
// `println!` checks at compile time that every `{}` placeholder has a
// matching argument. You can also name variables inline: `{name}`.

fn main() {
    let name = "Ferris";
    let species = "crab";
    println!("Hello, {}! You are a {}.", name, species);
    println!("Inline form works too: {name} the {species}.");
}
