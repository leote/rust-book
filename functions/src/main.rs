
fn main() {
    another_function(5);
    third_function(5, 62);
    print_labeled_measurement(5, 'h');

    let number = number();

    println!("The value of number is: {number}");


    let plus1 = plus_one(66);

    println!("The value of plus1 is {plus1}");

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}


fn third_function(y: i32, x: i32) {
    let sum = x+y;
    println!("The value of x + y is: {sum}");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}





fn number() -> f32 {
    41.67
}

fn plus_one(plus1: i32) -> i32 {
    plus1 + 1
}