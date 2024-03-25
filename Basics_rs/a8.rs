enum Flavors{
    Apple,
    Lemon,
    Orange,
}
struct Drinks{
    flavor: Flavors,
    fluid_oz: f64,
}
fn display(drink: Drinks){
    println!("oz: {:?}", drink.fluid_oz);
    match drink.flavor {
        Flavors::Apple => println!{"apple"},
        Flavors::Lemon => println!{"lemon"},
        Flavors::Orange => println!{"orange"},
    }
}
fn main(){
    let sweet = Drinks{
        flavor: Flavors:: Orange,
        fluid_oz: 6.0
    };
    display(sweet);
}