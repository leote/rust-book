fn main() {

    
    let number = 69;

    println!("Number = {number}");

    if number <41 {
        println!("Condition true");
    }
    else{
        println!("Condition false");
    }


    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!("\n");

    let condition = true;
    let number2 = if condition { 5 } else { 6 };

    println!("The value of number2 is: {number2}");


    println!("\n");

    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break (counter * 2) + 1;
        }
    };

    println!("The result is {result}");

    println!("\n");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    

    println!("\n");


    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    if number == 0{
        println!("Get ready!");
    }

    println!("LIFTOFF!!!");


    println!("\n");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    

    let a = [10, 20, 30, 40, 50]; // loop through array
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("\n"); 

    let a = [10, 20, 30, 40, 50]; // loop through array 1 by 1

    for element in a {
        println!("the value is: {element}");
    }
    
    println!("\n"); 


    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
