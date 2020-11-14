fn main() {
    test1();
    test2();
}

fn test1() {
    println!("_____________________");
    println!("Test 1, scope values\n");
    let x = 5;

    let y = {
        let x = 3;
        x - 1
    };
    println!("x: {}\ny: {}", x, y);

    println!("_____________________");
}

fn test2() {
    println!("_____________________");
    println!("Test 2, returned values\n");
    
    println!("17 hours --> {} minutes or {} seconds", hours_to_minutes(17), hours_to_seconds(17));

    println!("_____________________");
}

fn hours_to_minutes(hours: i32) -> i32 {
    return hours * 60; //**Same
}

fn hours_to_seconds(hours: i32) -> i32 {
    hours * 3600 //**Same
}
