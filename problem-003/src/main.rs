// Problem 3: Largest prime factor
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?
fn main() {
    let want = 6857;
    let mut got: i64 = 600851475143;
    let mut i = 3;
    while i * i <= got {
        while got % i == 0 {
            got = got / i;
        }
        i = i + 2;
    }
    println!("want:{} == got:{} = {}", want, got, want == got)
}
