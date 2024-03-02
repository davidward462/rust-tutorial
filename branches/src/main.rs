fn main() 
{
    let positive: bool = false;
    
    let value: i32 = if positive { 1 } else { -1 };

    if value > 0
    {
        println!("value {value} is positive");
    }
    else if value == 0
    {
        println!("value {value} is equal to zero");
    }
    else
    {
        println!("value {value} is less than zero");
    }

}
