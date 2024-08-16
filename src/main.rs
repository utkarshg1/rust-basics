use std::io;
fn main() {
    let p:f64 = prompt_for_float("Please provide Principal in INR : ");
    let n:f64 = prompt_for_float("Please provide Period in Years : ");
    let r:f64 = prompt_for_float("Please provide Rate of Intrest in %p.a. : ");
    let(intr, amt) = simple_intrest(p, n, r);
    println!("Given Principal = {} INR", p);
    println!("Period = {} Years", n);
    println!("Rate of Intrest = {} %p.a",r);
    println!("Simple Intrest is : {} INR", intr);
    println!("Total Amount is {} INR", amt);
}

fn simple_intrest(p:f64, n:f64, r:f64) -> (f64, f64) {
    let intr:f64 = (p*n*r)/100.0;
    let amt:f64 = p + intr;
    return (intr, amt);
}

fn prompt_for_float(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Please enter a valid number")
}
