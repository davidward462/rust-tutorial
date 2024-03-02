#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

// associated functions for struct Rectangle
impl Rectangle
{
    fn area(&self) -> u32
    {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }
}

fn main() {

    let rect1 = Rectangle
    {
        width:3,
        height:4,
    };
    
    let rect2 = Rectangle
    {
        width:1,
        height:2,
    };

    let rect3 = Rectangle
    {
        width:6,
        height:10,
    };

    println!("rect1 = {:#?}", rect1);

    println!("The area of the rectangle is {}", rect1.area() ); 
    
    println!("{}", rect1.can_hold(&rect2));

}
