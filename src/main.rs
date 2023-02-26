use std::io;
use std::thread::sleep;
use std::time::Duration;

fn main() {

    println!("Rust Calculator 1.2");
    println!();

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut op = String::new();

    eprint!("Enter your first number: ");
    io::stdin().read_line(&mut num1).expect("failed to read line");
    println!();

    eprint!("Select an Operator [+, -, *, /]: ");
    io::stdin().read_line(&mut op).expect("failed to read line");
    println!();

    eprint!("Enter your second number: ");
    io::stdin().read_line(&mut num2).expect("failed to read line");
    println!();

    let num1:f64 = num1.trim().parse().unwrap();
    let num2:f64 = num2.trim().parse().unwrap();
    let op = op.trim();

    op_checker(op, num1, num2);

    let time = Duration::from_secs(5);

    sleep(time);

}

fn op_checker(op: &str, num1: f64, num2: f64) {
    if op == "+" {
        println!("{}", num1 + num2);
    }

    else if op == "-" {
        println!("{}", num1 - num2);
    }

    else if op == "*" {
        println!("{}", num1 * num2);
    }

    else if op == "/" {
        println!("{}", num1 / num2);
    }

    else {
        println!("Please enter a valid operator!");
    }

}
