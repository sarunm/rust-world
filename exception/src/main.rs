fn main() {
    let tressures = vec!["gold", "silver", "diamond"];
    let treasure = tressures.get(3);

    match treasure {
        Some(t) => println!("Found a tressure: {}", t),
        None => println!("No treasure found"),
    }

    // println!("{}",try_open_chest(true).)

    let chest_result = match open_chest(true) {
        Some(tressure) => tressure,
        None => "The chest is empty".to_string(),
    };
    let door_result = match open_door(false){
        Ok(door) => door,
        Err(e) => panic!("{}", e),
    };

    println!("{}", chest_result.to_string());
    println!("{}", door_result.to_string());
}

// fn try_open_chest(is_locked: bool) -> Result<(), String> {
//     let result = open_chest1(is_locked)?;
//     Ok(())
// }
//
// fn open_chest1(is_locked: bool) -> Result<String, String> {
//     if is_locked {
//         Err(String::from("Locked!"))
//     } else {
//         Ok(String::from("ok"))
//     }
// }

fn open_chest(is_empty: bool) -> Option<String> {
    if is_empty {
        None
    } else {
        Some(String::from("ok"))
    }
}

fn open_door(is_danger: bool) -> Result<String, String> {
    if is_danger {
        Err(String::from("danger"))
    } else {
        Ok(String::from("ok"))
    }
}
