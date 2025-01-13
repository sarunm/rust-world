fn main() {
    let add = |a, b| a + b; // this closure takes two arguments and returns their sum

    let result = add(1, 2);

    println!("The sum is: {}", result);
    Lesson();
}

fn Lesson() {
    let tressure = vec![100, 200, 300, 400, 500];
    let double = |x| x * 2;
    let result: Vec<i32> = tressure.iter().map(double).collect();
    print!("{:?}", result);
}
