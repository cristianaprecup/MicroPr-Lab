fn check(a: bool){
    match a{
        true => println!("it's true"),
        false => println!("it's false"),
    }
}
fn main(){
    let a = true;
    check(a);
}