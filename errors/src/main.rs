use std::fs::File;

fn main() {
    let open_file_result = File::open("hello.txt").expect("There is a missing file.");

}
