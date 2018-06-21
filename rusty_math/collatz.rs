fn collatz(N: int) -> int {
    if N == 1 { return 0; }
    match N % 2 {
        0 => { 1 + collatz(N/2) }
        _ => { 1 + collatz(N*3+1 }
    }
}
