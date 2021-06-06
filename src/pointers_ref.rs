pub fn run() {
    // Reference Pointer - Point to a resource in memory
    
    // Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("Values: {:?}", (arr1, arr2));

    // with non-primitives, if you assign another variable to a piece of data,
    // the first variable will no longer hold that value.
    // You will need to use a reference (&) to point to resource

    // Non-primitive Array (Vector)
    let vec1 = vec![1,2,3];
    // let vec2 = vec1; // now, vec1 is unaccessible
    let vec2 = &vec1;
    println!("Vector Values: {:?}", (&vec1, vec2));

    // ******************************************
    // reference variable
    let x = 10;
    let xr = &x;
    let dom = &x;
    println!("(x, xr, dom) is {:?}", (x, xr, dom));

    if xr == dom {
        println!("xr == dom => {}", true);
    }

    let mut y = 10;
    let yr = &mut y;
    println!("y is {}", yr);
    *yr += 1;
    println!("yr is {}", yr);
    println!("y is {}", y);
}