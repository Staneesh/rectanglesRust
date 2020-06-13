#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32
{
    rectangle.width * rectangle.height
}

fn main() {
    
    let rect = Rectangle
    {
        width: 30,
        height: 30,
    };

    println!(
        "The area of a rectangle {:#?} is {} square pixels.",
        rect, area(&rect)
        );
}

