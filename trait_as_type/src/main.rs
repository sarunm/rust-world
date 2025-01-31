trait Gear {
    fn use_gear(&self);
}

struct Sword;
struct Bow;
struct Potion;

impl Gear for Sword {
    fn use_gear(&self) {
        println!("Sword gear!");
    }
}
impl Gear for Bow {
    fn use_gear(&self) {
        println!("Bow gear!");
    }
}
impl Gear for Potion {
    fn use_gear(&self) {
        println!("Potion gear!");
    }
}

fn use_gear<T: Gear>(item:T){
    item.use_gear();
}

fn main() {
    let crabby_sword = Sword;
    let crabby_bow = Bow;
    let crabby_potion = Potion;

    use_gear(crabby_sword);
    use_gear(crabby_bow);
    use_gear(crabby_potion);
}
