// modules1.rs
//
// Make this compile. Everything in a module is private by default.

// I AM NOT DONE

mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        let _recipe = get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
