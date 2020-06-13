fn area(width: u32, height: u32) -> u32
{
    width * height
}

fn main() {
    let width = 30;
    let height = 30;

    println!(
        "The area of a rectangle is {} square pixels.",
        area(width, height)
        );
}

