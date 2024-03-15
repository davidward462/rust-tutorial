use std::collections::HashMap;

fn main() {

    // text to parse
    let text = "An apple pie, a bright blue sky";

    // create hash map
    let mut map = HashMap::new();

    // split string on whitespace and iterate over the words
    for word in text.split_whitespace()
    {
        // insert word and give value of 0 if it does not exist
        let count = map.entry(word).or_insert(0);
        // note that or_insert() returns a type of <&mut V>
        
        // dereference mutable reference (thus making it just a normal mut) to count and update it's value if it exists 
        *count += 1;
    }

    println!("{:?}", map);

}
