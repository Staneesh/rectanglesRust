#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

impl Rectangle 
{
    fn area(&self) -> u32
    {
        self.width * self.height
    }
}

fn main() {
    
    let rect = Rectangle
    {
        width: 30,
        height: 30,
    };

    println!(
        "The area of a rectangle {:#?} is {} square pixels.",
        rect, rect.area() 
        );
}

