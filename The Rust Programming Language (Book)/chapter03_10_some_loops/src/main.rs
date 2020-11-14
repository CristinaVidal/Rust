fn main() {
    let array: [i32; 4] = [17, 18, 19, 20];

    normal_for(array);
    reverse_for(array);
}

fn normal_for(array: [i32; 4]) {
    println!("Standar for loop: ");
    for number in array.iter() {
        println!("{}", number);
    }
    println!("___");
    for number in 1..4 {
        println!("{}", number);
    }
    println!("________________________________");
}

fn reverse_for(array: [i32; 4]) {
    println!("Reverse for loop: ");
    for number in array.iter().rev() {
        println!("{}", number);
    }
    println!("___");
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("________________________________");
}
