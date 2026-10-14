// modules1.rs - solution
//
// `pub` makes an item visible outside its module. The secret recipe stays
// private: code inside the module can still call it.

mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        let _recipe = get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
