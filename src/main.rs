use std::io;

fn main() {

    println!("Rust Calculator 1.4");
    println!();

    let num1 = get_num1();
    let op = get_op();
    let num2 = get_num2();

    op_checker(op, num1, num2);

}

fn get_num1() -> f64 {
    let mut num1 = String::new();
    eprint!("Enter your first number: ");
    io::stdin().read_line(&mut num1).expect("failed to read line");
    println!();
    let num1:f64 = num1.trim().parse().unwrap();
    return num1;
}

fn get_op() -> String {
    let mut op = String::new();
    eprint!("Select an Operator [+, -, *, /]: ");
    io::stdin().read_line(&mut op).expect("failed to read line");
    println!();
    let op: String = op.trim().to_string();
    return op;
}

fn get_num2() -> f64 {
    let mut num2 = String::new();
    eprint!("Enter your second number: ");
    io::stdin().read_line(&mut num2).expect("failed to read line");
    println!();
    let num2:f64 = num2.trim().parse().unwrap();
    return num2;
}

fn op_checker(op: String, num1: f64, num2: f64) {
    if op == "+" {
        println!("Result: {}", num1 + num2);
    }

    else if op == "-" {
        println!("Result: {}", num1 - num2);
    }

    else if op == "*" {
        println!("Result: {}", num1 * num2);
    }

    else if op == "/" {
        println!("Result: {}", num1 / num2);
    }

    else {
        println!("Please enter a valid operator!");
        println!();
        let new_op = get_op();
        op_checker(new_op, num1, num2);

    }

    println!();
    let mut restart = String::new();
    eprint!("Enter 'r' to restart or press enter to close: ");
    io::stdin().read_line(&mut restart).expect("failed to read line");
    let restart = restart.trim();

    if restart == "r" {
        println!();
        println!();
        main();
        return;
    }

    else {
        return;
    }

}
