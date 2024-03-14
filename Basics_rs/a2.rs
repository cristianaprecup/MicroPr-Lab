fn sum(a: i32, b: i32)-> i32{
    a+b
}
fn print(result : i32){
    println!("{:?}",result)
}
fn main(){
    let result=sum(4,5);
    print(result);
}