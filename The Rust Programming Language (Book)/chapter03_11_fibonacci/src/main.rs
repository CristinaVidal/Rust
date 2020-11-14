fn main() {
    print_fibonacci(17);
}

fn print_fibonacci(total: i32) {
    if total > 0 { print!("1") };
    if total > 1 {
        let mut previous_number: i32 = 1;
        let mut buffer: i32;
        let mut next_number: i32 = 1;
        for _number in 1..total {
            print!(", {}", next_number);
            buffer = next_number;
            next_number += previous_number;
            previous_number = buffer;
        }
    }
    println!("\n:)");
}
