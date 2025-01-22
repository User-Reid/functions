fn main() {
    let multiplier: i32 = 3;

    let calculation = {
        let value: i32 = 5 + 4;
        value * multiplier
    };

    println!("{calculation}")
}
