mod lib;
use lib::two_pointer::is_palindrome;
fn main() {
    let mut s = "race a car".to_string();
    is_palindrome(s);
}
