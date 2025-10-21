fn main() {
    let multiplier = 5;

    let multiply_by = |value| value * multiplier;
    println!("{}", multiply_by(3 as u8));

    let mirror = |value| value;
    println!("{}", mirror("Why"));
    //println!("{}", mirror(10));
}