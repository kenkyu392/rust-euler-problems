// Problem 5: Smallest multiple
// 2520 is the smallest number that can be divided by each of the numbers
// from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all
// of the numbers from 1 to 20?
fn main() {
    let want = 232792560;
    let mut got = 1;
    for i in 1..21 {
        got = lcm(got, i)
    }
    println!("want:{} == got:{} = {}", want, got, want == got)
}

fn lcm(a: i32, b: i32) -> i32 {
    return a / gcd(a, b) * b;
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
