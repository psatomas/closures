fn main() {
    let first_name = String::from("Alice");
    let last_name = String::from("Wonder");
    let capture_string = move || {
        println!("{first_name} {last_name}");
    };
    capture_string();
    capture_string();
    capture_string();
}
