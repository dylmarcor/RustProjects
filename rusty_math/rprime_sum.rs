fn rprime_sum(x: int, y: int, m: int) {
    match (x+y)%m {
        0 => println!("Multiple");
        _ => println!("Relatively prime");
    }
}
