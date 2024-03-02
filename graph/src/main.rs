struct Point
{
    x: i32,
    y: i32,
}

impl Point
{
    fn show_data(self)
    {
        println!("({}, {})", self.x, self.y);
    }
}

fn main() 
{
    let p0 = Point { x: 0, y: 0 };

    p0.show_data();

}
