fn main(){
    //1st program function call
    println!("{}", is_even(2707));
}

//1st program function
fn is_even(num : i32) -> bool {
    if num % 2 == 0 {
        return true;
    }

    return false;
}