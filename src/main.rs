fn open_store(x: &str) {
    println!("Opening my pizza store in {x}")
}

fn bake_pizza(number_of_minutes: i8, type_of_pizza: &str) {
    println!("Now we are baking a {type_of_pizza} pizza for {number_of_minutes} minutes")
}

fn swim_in_profit(x: &str) {
    println!("Making {x} fuckin stacks yo")
}

fn main() {
    open_store("Houston");

    bake_pizza(32, "Hawaiin");

    swim_in_profit("Tacos");
    swim_in_profit("Banana");
}
