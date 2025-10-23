fn main () {
    let number = 13;
    let capture_number = || number;
    let a = capture_number();
    let b = capture_number();
    println!("{a} {b} {number}");

    let first_name = String::from("Alice");
    let capture_string = || {
        let person = first_name;
        println!("{person}");
    };
    capture_string();
}