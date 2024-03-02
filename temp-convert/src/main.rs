fn c_to_f(tempC: f64) -> f64
{
    (tempC * 9.0/5.0) + 32.0
}

fn f_to_c(tempF: f64) -> f64
{
    (tempF - 32.0) * 5.0/9.0
}

fn test_f_to_c(tempF: f64)
{
    let result: f64 = f_to_c(tempF);
    println!("{tempF} F -> {result} C");
}

fn test_c_to_f(tempC: f64)
{
    let result: f64 = c_to_f(tempC);
    println!("{tempC} C -> {result} F");
}

fn main() 
{
    let c1 = [-34.1, -4.6, 0.0, 3.4, 12.7, 22.4, 32.9];
    let c2 = [25.0, 30.0, 33.0, 35.0, 40.0, 180.0];
    let f1 = [-10.0, 0.0, 10.5, 42.9, 57.1, 79.0, 99.9];
    for temp in c2
    {
        test_c_to_f(temp);
    }

    for temp in f1
    {
        test_f_to_c(temp);
    }
}
