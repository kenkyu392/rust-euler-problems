// Problem 3: Largest prime factor
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?
fn main() {
    let want = 6857;
    let mut got = 0;
    let mut n: i64 = 600851475143;
    while n % 2 == 0 {
        n = n / 2;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            got = i;
            n = n / i;
        }
        i = i + 2;
    }
    if n > 2 {
        got = n;
    }
    println!("want:{} == got:{} = {}", want, got, want == got)
}
