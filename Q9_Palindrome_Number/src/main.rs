
pub fn is_palindrome(x: i32) -> bool {
    if x < 0{
        return false;
    }

    let xs =  x.to_string();
    let reversed: String = xs.chars().rev().collect();

    xs == reversed
}



fn main() {
    let test1 = 12321;
    let test2 = 1002;
    println!("{}", is_palindrome(test1));
    println!("{}", is_palindrome(test2));
}
