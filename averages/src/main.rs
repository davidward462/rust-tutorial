fn main()
{
    //let v = vec![12.0, 16.0, 13.0, 1.0, 18.0, 19.0, 5.0, 20.0, 11.0, 16.0, 12.0, 1.0, 6.0, 10.0, 13.0, 20.0, 10.0, 13.0, 16.0, 11.0, 5.0, 6.0, 8.0, 20.0];

    let v = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0];

    let mut total = 0.0;
    let mut count = 0.0;
    let mut average = 0.0;

    for value in v
    {
        count += 1.0;
        total += value;
    }

    average = total / count;

    println!("{average}");
}
