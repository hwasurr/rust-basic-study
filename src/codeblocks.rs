pub fn run() {
    // codeblocks

    let x = 3;

    {
        // isoldated
        let a = 1;
        println!("{}", x);
    }

    // println!("{}", a); // can't access variable "a"


    let mut y = 1;
    let z = 1;
    {
        y = 15;
        let z = 3;
        println!("z in the code block {}", z); // 3
    }
    println!("y is {}", y); // now y is 15. overwrittened.
    println!("y is {}", y); // now z is 1.

}