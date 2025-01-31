use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Hello, world!");

    let crabby_gold = Arc::new(Mutex::new(10));

    let loot1 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_gold);
        move || {
            let mut gold =crabby_gold_artifact.lock().unwrap();
            *gold += 100;
        }
    });
    let loot2 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_gold);
        move || {
            let mut gold =crabby_gold_artifact.lock().unwrap();
            *gold += 200;
        }
    });
    let loot3 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_gold);
        move || {
            let mut gold =crabby_gold_artifact.lock().unwrap();
            *gold += 80;
        }
    });

    loot1.join().unwrap();
    loot2.join().unwrap();
    loot3.join().unwrap();
    println!("Gold {}",crabby_gold.lock().unwrap());
}
