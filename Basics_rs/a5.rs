fn main(){
    let mut a = 1;
    loop {
        println!("{:?}", a);
        a = a + 1;
        if a == 5{
            break;
        }
    }
    println!("done");
}