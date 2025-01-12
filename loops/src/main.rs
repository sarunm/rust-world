fn main() {
    println!("Hello, world!");
}

fn infinity_loop() {
    let mut counter = 0;

    loop {
        counter += 1;

        println!("counter: {}", counter);

        if counter == 10 {
            break;
        }
    }
}

fn while_loop() {
    let mut counter = 0;

    while counter < 10 {
        counter += 1;
        println!("counter: {}", counter);
    }
}

fn for_loop() {
    let array = [10, 20, 30, 40, 50];

    for element in array.iter() {
        println!("element: {}", element);
    }
}
