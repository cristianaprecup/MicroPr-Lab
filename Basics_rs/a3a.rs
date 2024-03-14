fn check(a: i32){
    if a > 5{
        println!("is > 5");
    }
    else if a < 5{
        println!("is < 5")
    }
    else{
        println!("is == 5")
    }
}
fn main(){
    let a = 2;
    check(a);
}