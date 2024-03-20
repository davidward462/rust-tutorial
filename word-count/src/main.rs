use std::collections::HashMap;

fn main() {

    // text to parse
    let text = "three three three one two two six six six six six six";

    // create hash map
    let mut map = HashMap::new();

    let mut total_words = 0;

    // split string on whitespace and iterate over the words
    for word in text.split_whitespace()
    {
        // insert word and give value of 0 if it does not exist
        let count = map.entry(word).or_insert(0);
        // note that or_insert() returns a type of <&mut V>
        
        // dereference mutable reference (thus making it just a normal mut) to count and update it's value if it exists 
        // increment local and global counters
        total_words += 1;
        *count += 1;
    }

    println!("total words: {total_words}");
    println!("{:?}", map);

}
