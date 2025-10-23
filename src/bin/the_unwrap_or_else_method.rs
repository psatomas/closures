fn main() {
    let option = Some("Salami");
    let closure = || "pizza";
    let food = option.unwrap_or_else(closure);
    println!("{food}");

    let option: Option<&str> = None;
    let pizza_fan = false;
    let closure = || if pizza_fan { "Pizza" } else { "Hot Pockets" };
    let food = option.unwrap_or_else(closure);
    println!("{food}");
}