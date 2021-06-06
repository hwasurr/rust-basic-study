use std::mem;

pub fn run() {
    // Vectors Resizable Arrays

    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // 타입이 통일되어야 한다.

    println!("{:?}", numbers);

    // Get single val
    println!("single val[0]: {}", numbers[0]);

    // Mutable Vector - 타입에 맞추어서 변경만 가능
    let mut mutable_numbers: Vec<i32> = vec![6, 7, 8, 9, 0];
    mutable_numbers[2] = 20;
    println!("Mutated Vector {:?}", mutable_numbers);

    // Get Vector length
    println!("Length of Vector {:?}", mutable_numbers.len());

    // Arrays are stack allocated
    println!(
        "Vector occupies {} bytes",
        mem::size_of_val(&mutable_numbers)
    );
    // 이 경우, i32 = 4bytes가 5개이므로, 20bytes

    // Get Slice
    let slice: &[i32] = &mutable_numbers[0..2];
    println!("Slice {:?}", slice);

    // *******************
    // Vector only

    // Add on to vector
    mutable_numbers.push(5);
    mutable_numbers.push(6);
    mutable_numbers.pop();

    // Loop through vector values
    for x in mutable_numbers.iter() {
        println!("Vector numbers in for loop: {}", x);
    }

    // Loop & mutate values
    for x in mutable_numbers.iter_mut() {
        *x *= 2;
    }

    // each value multiply by 2
    println!("Vector numbers multiply: {:?}", mutable_numbers);
}
