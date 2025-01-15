use std::{collections::HashMap, string};

fn main() {
    let mut tressure = HashMap::new();
    tressure.insert("one", 100);
    tressure.insert("two", 200);

    println!("{:?}", tressure);

    if let Some(val) = tressure.get("one") {
        println!("Value of one is {}", val);
    }

    if let Some(val) = tressure.get_mut("two") {
        *val *= 2;
        println!("Value of two is {}", val);
    }

    for (tressure, count) in tressure.iter() {
        println!("{}: {}", tressure, count);
    }

    let three = tressure.get("three").unwrap_or(&0);
    println!("Value of three is {} when not have a key in hashmap", three);

    tressure.remove("one");
    for (tressure, count) in tressure.iter() {
        println!("{}: {}", tressure, count);
    }

    lesson();
}

fn lesson() {
    let mut treassure: HashMap<&str, i32> = HashMap::new();

    treassure.insert("Coins", 100);
    treassure.insert("Gems", 200);

    if let Some(val) = treassure.get_mut("Coins") {
        *val += 100;
        println!("Coins: {}", val);
    }

    print!("{:?}", treassure);
}
