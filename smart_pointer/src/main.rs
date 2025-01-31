use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // let sword = String::from("sword");
    // let epic_loot = Rc::new(sword);
    //
    // let loot1= Rc::clone(&epic_loot);
    // let loot2= Rc::clone(&epic_loot);

    let chest = Box::new(10);

    let shared_chest = Rc::new(RefCell::new(chest));

    **shared_chest.borrow_mut() += 20;
    **shared_chest.borrow_mut() += 10;

    println!("Chest value: {}", shared_chest.borrow());

}
