fn double(input: u32) -> u32
{
    input * 2
}

fn test_double(input: u32, expected: u32)
{
    let result = double(input);

    if result == expected
    {
        println!("test passed.");
    }
    else
    {
        println!("test failed with input {} and expected {}", input, expected);
    }
}

fn main() {

    let values = [3, 5, 4, 8, 5, 1, 45, 70, 12];

    test_double(values[0], 6);
    test_double(values[1], 10);
    test_double(values[2], 8);
    test_double(values[3], 16);
    test_double(values[4], 10);
    test_double(values[5], 2);

}
