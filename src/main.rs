pub mod algorithms;

use crate::algorithms::fibonacci::FibonacciCalculator;

fn main() {
    let mut fib = FibonacciCalculator::new();
    for i in 0..65 {
        print!("{} ", fib.calculate(i));
    }
    println!();
}
