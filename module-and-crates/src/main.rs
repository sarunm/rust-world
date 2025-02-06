use module_and_crates::potions;
use module_and_crates::weapons;
mod maps {
    pub fn use_items() {
        println!("I'm using maps!");
    }
}

fn main() {
    potions::use_items();
    weapons::use_items();
    maps::use_items();
}
