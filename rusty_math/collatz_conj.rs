use std::os;

mod collatz;

fn main() {
    if os::args().len() < 2 {
        println!("Error: please provide a number as argument.");
        
        return;
    }

    let i = from_str::<int>(os::args()[1].unwrap();
    println!("{:d} has {:d} Collatz steps", i, collatz(i));
}
