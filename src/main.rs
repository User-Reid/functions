fn square(number: i32) -> i32 {
    number * number
}

fn main() {
    let answer: i32 = square(5);
    println!("The result is {answer}");
    let answer: i32 = square(10);
    println!("The result to the second function is {answer}");
}
