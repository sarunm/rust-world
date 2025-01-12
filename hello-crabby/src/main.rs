fn main() {
    // data_types();
    // poc_if();
    // poc_match();
    // poc_loops();
    // wrong_owner();
    // immutable_borrowing();
    // mutable_borrowing();
    lift_time();
}

fn lift_time() {
    let s1 = "hello";
    let s2 = "world";
    let result = longest(s1, s2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn wrong_owner() {
    let s = String::from("hello");
    let s2 = s;
    println!("s2 is {}", s2);
}

fn immutable_borrowing() {
    let s = String::from("hello");
    let s2 = &s;
    println!("s2 is {}", s2);
    println!("s is {}", s);
}

fn mutable_borrowing() {
    println!("mutable_borrowing");
    let mut s = String::from("hello");
    let s2 = &mut s;
    println!("s2 is {}", s2);

    s2.push_str(", world");
    println!("s is {}", s);
}

fn data_types() {
    let x = 5;
    let y = 10.0;
    let z = x + y as i32;

    println!("z is {}", z);

    // let msg1 = String::from("Hello, world!");
    // let msg2 = "Hello, world!".to_string();
    // let msg3 = "Hello, world!";
    let msg4 = format!("Point: {},{}", x, y);

    println!("msg4 is {}", msg4);
}

fn poc_if() {
    let check = 50;

    if check > 100 {
        println!("check is greater than 100");
    } else if check < 100 {
        println!("check is less than 100");
    } else {
        println!("check is equal to 100");
    }
}

fn poc_match() {
    let test_string = "test1";

    match test_string {
        "test1" => println!("test1"),
        "test2" => println!("test2"),
        _ => println!("default"),
    }
}

fn poc_loops() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count;
        }
    };
    println!("result is {}", result);
}
