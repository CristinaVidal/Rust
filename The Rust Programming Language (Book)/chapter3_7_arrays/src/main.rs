#![allow(non_snake_case)]

fn main() {
    let mut arrayAnyValues: [i32; 5] = [0, 1, 2, 3, 4];
    let arraySameValues = [3; 5];

    arrayAnyValues[0] = 3;

    println!(); // START

    println!("\nArray with any value: ");
    printArray(arrayAnyValues);

    println!("\nAnother array with same value: ");
    printArray(arraySameValues);

    println!(); // END

}

fn printArray(array: [i32; 5]) {
    for i in 0..array.len() {
        print!("{}, ", array[i]);
    }
    print!("{}.", array[4]);
}
