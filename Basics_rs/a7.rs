enum Color{
    Red,
    Green,
    Blue,
}
fn main(){
    let color =Color::Red;
    match color{
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
    }
}