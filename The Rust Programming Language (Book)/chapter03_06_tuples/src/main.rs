fn main() {
    let tup: (char, char, char, u32) = ('C', 'V', 'G', 23);

    println!("{}.{}.{}. - {} years old", tup.0, tup.1, tup.2, tup.3);

    let (a, b, c, years) = tup;

    println!("Hey! Same information hahaha:");
    println!("{}.{}.{}. - {} years old", a, b, c, years);
}
