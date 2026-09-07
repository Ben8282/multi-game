use roots::{find_roots_quadratic, Roots};
use std::io;
pub fn solve_quadratic() {
    println!("hello welcome to the quadratic equation solver");
    println!("please enter the values of a, b, and c of the quadratic equation ax^2 + bx + c = 0");
    println!("enter the value of a: ");
    let a_math: f64;
    loop {
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("failed to read input");
        match a.trim().parse::<f64>() {
            Ok(banana) => {
                a_math = banana.clone();
                break;
            }
            Err(_) => {
                println!("please enter a valid f64 number");
                continue;
            }
        }
    }
    println!("enter the value of b: ");
    let b_math: f64;
    loop {
        let mut b = String::new();
        io::stdin().read_line(&mut b).expect("failed to read input");
        match b.trim().parse::<f64>() {
            Ok(num) => {
                b_math = num.clone();
                break;
            }
            Err(_) => {
                println!("please enter a valid f64 number");
                continue;
            }
        }
    }
    println!("enter the value of c: ");
    let c_math: f64;
    loop {
        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("failed to read input");
        match c.trim().parse::<f64>() {
            Ok(num) => {
                c_math = num.clone();
                break;
            }
            Err(_) => {
                println!("please enter a valid f64 number");
                continue;
            }
        }
    }
    match find_roots_quadratic(a_math, b_math, c_math) {
        Roots::Two(roots) => {
            println!("the solutions of the quadratic equation are: {} and {}", roots[0], roots[1]);
        }
        Roots::One(roots) => {
            println!("the solution of the quadratic equation is: {}", roots[0]);
        }
        Roots::No(_) => {
            println!("the quadratic equation has no real solutions");
        }
        _ => unreachable!(), 
    }
}
