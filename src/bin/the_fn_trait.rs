fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(),
{
    procedure();
    procedure();
    procedure();
}

fn main() {
    let mut bosses = vec!["Boris"];
    let closure = || {
        bosses.push("Alexandra")
    };
    execute_thrice(closure);
}