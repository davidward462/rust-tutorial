
// GENERIC FUNCTION
// max() is generic over some type T.
// There is one parameter named list, which is a slice of values of type T.
// max() returns a reference to a value of type T.
fn max<T: PartialOrd>(list: &[T]) -> &T
{
    let mut current_max = &list[0];

    for item in list {
        if item > current_max {
            current_max = item;
        }
    }
    current_max
}

// GENERIC STRUCT
// Both values will have the same type T.
struct Point<T> {
    x: T,
    y: T,
}

// GENERIC METHOD for struct.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// GENERIC STRUCT
// The values can have different types.
struct Person<T, U> {
    age: T,
    height: U,
}

fn main()
{
    let number_list = vec![45, 76, 12, 0, 34, 8];
    let result = max(&number_list);
    println!("Maximum value: {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = max(&number_list);
    println!("Maximum value: {}", result);

}
