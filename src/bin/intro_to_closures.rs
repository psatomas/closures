fn main() {
    let multiplier = 5;

    //fn multiply_by(value: i32) -> i32 {
    //    value * multiplier
    //}

    let multiply_by = |value: i32| -> i32 {
        return value * multiplier;
    };

    println!("{}", multiply_by(2));

    let product = |a:i32, b: i32| -> i32 {
        println!("Calculating the product for you");
        return a * b;
    };

    println!("{}", product(3, 10));
    println!("{}", product(5, 8));
}