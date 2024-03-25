fn coord() -> (i32, i32){
    (1,654)
}
fn main(){
    let (x, y) = coord();
    if y > 5{
        println!("The value of the y-coord is bigger than 5")
    }
    else if y == 5{
        println!("The value of the y-coord is equal to 5")
    }
    else{
        println!("The value o y-coord is less than 5")
    }
}