// Problem 6: Sum square difference
// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... + 10^2 = 385
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)^2 = 55^2 = 3025
// Hence the difference between the sum of the squares of the first ten
// natural numbers and the square of the sum is 3025 − 385 = 2640.
// Find the difference between the sum of the squares of the first one
// hundred natural numbers and the square of the sum.
fn main() {
    const N: i64 = 100;
    let want = 25164150;
    let got = (N * (N + 1) / 2 as i64).pow(2) - (N * (N + 1) * (2 * N + 1)) / 6;
    println!("want:{} == got:{} = {}", want, got, want == got);
}
