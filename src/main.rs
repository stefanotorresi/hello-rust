use std::time::Instant;
use hello_rust::algos::fibonacci::*;

fn benchmark<F: Fn()>(f: F) {
    let now = Instant::now();
    f();
    println!("{:?}", now.elapsed())
}

fn main() {
    benchmark(|| {
        println!("{}", fibonacci(50));
    });
    benchmark(|| {
        unsafe {
            println!("{}", fibonacci_iterative(150));
        }
    });
    benchmark(|| {
        unsafe {
            println!("{}", fibonacci_iterative(150));
        }
    });
}
