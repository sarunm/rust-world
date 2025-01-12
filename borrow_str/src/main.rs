fn main() {
    let map = String::from("old map");

    let t = "hello";
    println!("t: {}", t);

    let borrowed_map = map.as_str();

    let mut new_map = borrowed_map.to_string();

    new_map.push_str(" to new map");

    println!("new_map: {}", new_map);
}
