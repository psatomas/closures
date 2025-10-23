fn main() {
    let mut numbers = vec![4, 8, 15, 16, 23, 42];
    let mut add_number = || numbers.push(100);
    add_number();
    add_number();
    println!("{:?}", numbers);
}