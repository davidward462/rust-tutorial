fn main()
{
    let limit = 10;

    for value in 0..limit
    {
        if value % 3 == 0
        {
            println!("fizz");
        }
        else
        {
            println!("{value}");
        }
    }
}
