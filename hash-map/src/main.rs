use std::collections::HashMap;

fn main()
{   
    // initialize hashmap
    let mut scores = HashMap::new();

    // insert values into hashmap 'scores'
    scores.insert(String::from("Red"), 19);
    scores.insert(String::from("Blue"), 14);

    let team_name = String::from("Red");

    // Obtain value from hashmap using get().
    // This is an Option<&V>, so we use copied() and then unwrap_or() to give a value if the key
    // does not exist.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{team_name}: {score}");

    println!("Scores:");
    
    // iterate over hashmap
    for (key, value) in &scores
    {
        println!("{key}: {value}");
    }


    let name = String::from("Bob");
    let city = String::from("New York");

    println!("city: {city}");

    let mut people = HashMap::new();

    people.insert(name, city);

    println!("{:?}", people);

    scores.insert(String::from("Green"), 23);
    scores.entry(String::from("Green")).or_insert(1);
    scores.entry(String::from("Orange")).or_insert(12);

    println!("{:?}", scores);

}







