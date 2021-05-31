use std::mem;

pub fn run() {
    // Array
    // Fixed list where elements are the same data types

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // 자릿수가 적거나 많아도 안된다.
    // 타입이 통일되어야 한다.

    println!("{:?}", numbers);

    // Get single val
    println!("single val[0]: {}", numbers[0]);

    // Mutable array - 자릿수, 타입에 맞추어서 변경만 가능
    let mut mutable_numbers: [i32; 5] = [6, 7, 8, 9, 0];
    mutable_numbers[2] = 20;
    println!("Mutated array {:?}", mutable_numbers);

    // Get Array length
    println!("Length of array {:?}", mutable_numbers.len());

    // Arrays are stack allocated
    println!(
        "Array occupies {} bytes",
        mem::size_of_val(&mutable_numbers)
    );
    // 이 경우, i32 = 4bytes가 5개이므로, 20bytes

    // Get Slice
    let slice: &[i32] = &mutable_numbers[0..2];
    println!("Slice {:?}", slice);
}
