use std::io;


fn main() {

    /* // variables

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("New value of x is: {x}");

    */

    /* // shadowing

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    */


    /* spaces

    let spaces = "          ";
    let spaces = spaces.len();

    println!("Length of spaces = {spaces}");

    */



    /*

    //data types
    

    /*
    8-bit	i8	u8
    16-bit	i16	u16
    32-bit	i32	u32
    64-bit	i64	u64
    128-bit	i128	u128
    Architecture-dependent	isize	usize

    Decimal	98_222
    Hex	0xff
    Octal	0o77
    Binary	0b1111_0000
    Byte (u8 only)	b'A'
    */


    //floating points

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32


    println!("x = {x} , y = {y}");


    /*

    //numeric operators

     // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder or mod
    let remainder = 43 % 5;

    

    let t = true;

    let f: bool = false; // with explicit type annotation

    */



    

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';



    


    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    */

    /*

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    */


}
