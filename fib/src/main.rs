use std::io;

// -------------------------------------------------------------

fn main() {
    println!("\n----------------------");
    println!("Pick a mode:");
    println!("\tA. give a specified number of fibonacci numbers");
    println!("\tB. give a specified range to print (note that all numbers up to the starting point will be calculated any way but not printed)");

    let mut mode = String::new();

    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read");
    
        println!("mode = {}", mode);

    if mode.eq("A\n"){
        length_mode();
    }
    else if mode.eq("B\n"){
        range_mode();
    }
    else {
        println!("invalid input, please input the letter 'A' or 'B'")
    }

    println!("\n----------------------");
}

// -----------------------------------------------------

fn length_mode() {
    let length: i32 = get_length();

    let mut count = 1;
    let mut x = 0;
    let mut y = 1;

    loop {
        if count > length {
            break;
        }

        print!(" {} ", x+y);
        if x <= y{
            x = x+y;
        } else {
            y = x+y;
        }

        count += 1;
    }
}

fn get_length() -> i32{
    println!("Enter a number of fibonacci numbers");

    let mut length = String::new();

    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read");
    
    let length: i32 = length.trim().parse().expect("Please enter a number");
    println!("length: {}", length);
    length
}

// ---------------------------------------------------------------

fn range_mode(){
    let start: i32 = get_start();
    let end: i32 = get_end();

    let mut x = 0;
    let mut y = 1;

    loop{
        
        if x+y > end {
            break;
        }

        if x+y >= start{
            print!(" {} ", x+y);
        }
        
        if x <= y{
            x = x+y;
        } else {
            y = x+y;
        }       
    }
}

fn get_start() -> i32{
    println!("Enter start number in the range: ");

    let mut length = String::new();

    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read");
    
    let length: i32 = length.trim().parse().expect("Please enter a number");
    println!("start: {}", length);
    length
}

fn get_end() -> i32{
    println!("Enter end number in the range: ");

    let mut length = String::new();

    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read");
    
    let length: i32 = length.trim().parse().expect("Please enter a number");
    println!("end: {}", length);
    length
}

// -------------------------------------------------------