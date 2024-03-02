fn string_length(s: &String) -> usize {

    s.len()
}

fn str_length(s: &str) -> usize {

    s.len()
}

fn test_str_length(input: &str, expected: usize) {

    let result = str_length(input);
    
    if result == expected 
    {
        println!("test passed.");
    }
    else 
    {
        println!("test failed on input {}", input);
    }
}

fn test_string_length(input: &String, expected: usize) {

    let result = string_length(input);
    
    if result == expected 
    {
        println!("test passed.");
    }
    else 
    {
        println!("test failed on input {}", input);
    }
}

fn main() 
{
    let test_count = 3;

    // str
    let a = "one";
    let b = "two";
    let c = "three";
    let d = "four";

    test_str_length(a, 3);
    test_str_length(b, 3);
    test_str_length(c, 5);
    test_str_length(d, 4);

    // Strings
    let s1 = String::from("one");

    test_string_length(&s1, 3);


}








