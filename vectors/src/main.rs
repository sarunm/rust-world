fn main() {
    len_vs_cap();
    new_vec();
}

fn len_vs_cap() {
    let mut v = vec!["a", "b", "c", "d", "e"];

    println!("len: {}, capacity: {}", v.len(), v.capacity());

    v.pop();
    println!("len: {}, capacity: {}", v.len(), v.capacity());

    // let v1 = v.pop();
    // let l1 = v1.len();
    // let c1 = v1.capacity();

    // println!("len: {}, capacity: {}", l1, c1);
}

fn new_vec() {
    let mut v: Vec<String> = Vec::new();
    v.push("a".to_string());
    v.push("b".to_string());

    v.push(String::from("c"));
    v.push(String::from("d"));

    println!("{:?}", v);
}
