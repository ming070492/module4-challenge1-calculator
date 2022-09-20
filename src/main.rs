//Sorry for adding some colors >T_T<

use std::io;

fn main() {
    println!("\x1b[93m*******************************************\x1b[0m");
    println!("\x1b[93m*\x1b[0m  SELECT OPERATION                       \x1b[93m*\x1b[0m");
    println!("\x1b[93m*******************************************\x1b[0m");
    println!("\x1b[93m*\x1b[0m  ADDITION : +                           \x1b[93m*\x1b[0m");
    println!("\x1b[93m*\x1b[0m  SUBTRACTION : -                        \x1b[93m*\x1b[0m");
    println!("\x1b[93m*\x1b[0m  MULTIPLICATION : *                     \x1b[93m*\x1b[0m");
    println!("\x1b[93m*\x1b[0m  DIVISION : /                           \x1b[93m*\x1b[0m");
    println!("\x1b[93m*******************************************\x1b[0m");

    fn math_add(num_1: i64, num_2: i64) -> i64 {
            let ans = num_1 + num_2;
            ans
    }
    fn math_sub(num_1: i64, num_2: i64) -> i64 {
        let ans = num_1 - num_2;
        ans
    }
    fn math_mul(num_1: i64, num_2: i64) -> i64 {
        let ans = num_1 * num_2;
        ans
    }
    fn math_div(num_1: i64, num_2: i64) -> f64 {
        if num_2 == 0 {
            println!("\x1b[91mERR:\x1b[0m DIVIDING BY ZERO IS NOT ALLOWED.");
            0.0
        }else{
            let ans = num_1 as f64 / num_2 as f64;
            ans
        }
    }

    fn get_two_numbers() -> Vec<i64> {
        let mut num_vec: Vec<i64> = Vec::new();
        let mut num_1 = String::new();
        let mut num_2 = String::new();
        println!("ENTER FIRST NUMBER: ");
        io::stdin().read_line(&mut num_1).expect("\x1b[91mERR:\x1b[0m INVALID INPUT");
        println!("ENTER SECOND NUMBER: ");
        io::stdin().read_line(&mut num_2).expect("\x1b[91mERR:\x1b[0m INVALID INPUT");
        num_vec.push(num_1.trim().parse().unwrap());
        num_vec.push(num_2.trim().parse().unwrap());
        num_vec
    }

    let mut not_done = true;

    while not_done {
        println!("\x1b[93m*******************************************\x1b[0m");
        let mut operation = String::new();
        println!("ENTER \x1b[36mOPERATION SYMBOL\x1b[0m: ");
        io::stdin().read_line(&mut operation).expect("\x1b[91mERR:\x1b[0m INVALID OPERATION");

        if operation.trim() == "+" {
            println!("\x1b[93m-------------------------------------------\x1b[0m");
            println!("  \x1b[92mADDITION SELECTED\x1b[0m");
            println!("\x1b[93m-------------------------------------------\x1b[0m");
            let numbers = get_two_numbers();
            println!("ANSWER: \x1b[92m\x1b[1m{}\x1b[0m", math_add(numbers[0], numbers[1]));
        } else if operation.trim() == "-" {
            println!("\x1b[93m-------------------------------------------\x1b[0m");
            println!("  \x1b[92mSUBTRACTION SELECTED\x1b[0m");
            println!("\x1b[93m-------------------------------------------\x1b[0m");
            let numbers = get_two_numbers();
            println!("ANSWER: \x1b[92m\x1b[1m{}\x1b[0m", math_sub(numbers[0], numbers[1]));
        } else if operation.trim() == "*" {
            println!("\x1b[93m-------------------------------------------\x1b[0m");
            println!("  \x1b[92mMULTIPLICATION SELECTED\x1b[0m");
            println!("\x1b[93m-------------------------------------------\x1b[0m");
            let numbers = get_two_numbers();
            println!("ANSWER: \x1b[92m\x1b[1m{}\x1b[0m", math_mul(numbers[0], numbers[1]));
        } else if operation.trim() == "/" {
            println!("\x1b[93m-------------------------------------------\x1b[0m");
            println!("  \x1b[92mDIVISION SELECTED\x1b[0m");
            println!("\x1b[93m-------------------------------------------\x1b[0m");
            let numbers = get_two_numbers();
            let ans: f64 = math_div(numbers[0], numbers[1]);
            if ans != 0.0 {
                println!("ANSWER: \x1b[92m\x1b[1m{}\x1b[0m", ans);
            }
        } else {
            println!("\x1b[91mERR:\x1b[0m INVALID OPERATION. PLEASE TRY AGAIN.\n");
        }

        println!("--------------------");
        println!("NEW COMPUTATION? Y/N");
        let mut res = String::new();
        io::stdin().read_line(&mut res).expect("\x1b[91mERR:\x1b[0m INVALID RESPONSE");
        if res.trim() == "N" || res.trim() == "n" {
            not_done = false;
        } else {
            not_done = true;
        }
        println!("\x1b[93m*******************************************\x1b[0m");
    }
}
