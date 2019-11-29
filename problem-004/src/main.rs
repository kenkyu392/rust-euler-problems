// Problem 4: Largest palindrome product
// A palindromic number reads the same both ways. The largest palindrome
// made from the product of two 2-digit numbers is 9009 = 91 Ã— 99.
// Find the largest palindrome made from the product of two 3-digit numbers.
fn main() {
    let want = 906609;
    let mut got = 0;
    let mut z;
    for x in (100..1000).rev() {
        for y in (100..1000).rev() {
            z = x * y;
            if is_palindrome(z.to_string()) {
                got = z;
            }
            if z < got {
                break;
            }
        }
    }
    println!("want:{} == got:{} = {}", want, got, want == got)
}

fn is_palindrome(s: String) -> bool {
    let len = s.len();
    return len % 2 == 0
        && s[..len / 2] == s.chars().rev().collect::<String>()[..len / 2].to_string();
}
