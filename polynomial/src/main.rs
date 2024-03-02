// ax^2 + bx + c
fn polynomial(a:f64, b:f64, c:f64, x:f64) -> f64
{
    (a * (x*x)) + (b * x) + c
}

fn test_polynomial(a:f64, b:f64, c:f64, x:f64, expected:f64) -> bool
{
    let result = polynomial(a, b, c, x);
    if result == expected
    {
        println!("passed: {result} == {expected}");
        true
    }
    else
    {
        println!("failed: {result} != {expected}");
        false
    }
}


fn main() 
{
    let a = [0.0;5];
    let b = [1.0, 0.0, 0.0, 0.0, 0.0];
    let polynomialList = [a, b];
    for poly in polynomialList
    {
        test_polynomial(poly[0], poly[1], poly[2], poly[3], poly[4]);
    }

    let mut x = 0.0;
    while x < 10.0
    {
        println!("({x}, {})", polynomial(1.0, 0.0, 0.0, x));
        x += 1.0;
    }
}
