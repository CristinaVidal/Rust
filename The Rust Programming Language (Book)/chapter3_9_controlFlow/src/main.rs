fn main() {
    test1();
    test2();
    test3();
    test4();
}

fn test1() {
    println!("Basic conditional test:");
    println!("___________________________");
    let number = 6;
    let result = if number % 2 == 0 { "even" } else { "odd" };

    println!("{} is {}", number, result);
    println!("___________________________");
}

fn test2() {
    println!("Basic loop test:");
    println!("___________________________");

    let mut counter = 1;
    let number = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 3;
        }
    };
    println!("Number after loop: {}", number);

    println!("___________________________");
}

fn test3() {
    println!("Basic while test:");
    println!("___________________________");

    let mut number = 5;
    while number > 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("END ! ");

    println!("___________________________");
}

fn test4() {
    println!("Basic for test:");
    println!("___________________________");

    let array = [17, 35, 74, 39, 66, 21];

    for item in array.iter() {
        println!("Item: {}", item);
    }

    println!("___________________________");
}
