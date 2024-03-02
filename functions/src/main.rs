fn increment(a:i32) -> i32
{
    a + 1
}

fn decrement(a:i32) -> i32
{
    a - 1
}

fn main()
{
    let x = 10;
    let y = increment(x);
    let z = decrement(x);
    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");
}
