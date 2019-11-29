// Problem 7: 10001st prime
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
// we can see that the 6th prime is 13.
// What is the 10001st prime number?
fn main() {
    const N: i64 = 10001;
    let want = 104743;
    let mut got = 0;
    let mut c = 0;
    while c < N {
        got += 1;
        if is_prime(got) {
            c += 1;
        }
    }
    println!("want:{} == got:{} = {}", want, got, want == got);
}

fn is_prime(n: i64) -> bool {
    if n == 2 {
        return true;
    } else if n < 2 {
        return false;
    } else if n % 2 == 0 {
        return false;
    }
    for i in (3..(n as f64).sqrt() as i64 + 1).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
