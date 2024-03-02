struct Circle
{
    radius: f32,
}

// functions associated with the Circle struct
impl Circle
{
    // pi * r^2
    fn area(&self) -> f32
    {
        3.1415 * (self.radius * self.radius)
    }
}

struct Rectangle
{
    length: f32,
    width: f32,
}

// functions associated with Rectangle struct
impl Rectangle
{
    fn area(&self) -> f32
    {
        self.width * self.length
    }
}

fn main() {
    println!("Begin...");

    let c1 = Circle
    {
        radius: 4.0,
    };

    let r1 = Rectangle
    {
        width: 2.0,
        length: 3.0,
    };
    
    let areaC1 = c1.area();
    let areaR1 = r1.area();
    
    println!("area of circle = {}\narea of rectangle = {}", areaC1, areaR1);

}









