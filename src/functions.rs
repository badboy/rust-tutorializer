// What would be a suitable type for addition?
fn add(first: (), second: ()) -> () {
    first + second
}

fn main() {
    // Call add with some input and check that it is correct
    assert_eq!(RESULT, add(_, _));
}
