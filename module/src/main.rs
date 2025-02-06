mod potions {
    pub fn use_items() {
        println!("I'm using potions!");
    }
}

mod weapons {
    pub fn use_items() {
        println!("I'm using weapons!");
    }
}

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
