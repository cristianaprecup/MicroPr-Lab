use std::io;
fn main(){
    println!("Cristiana-Florentina Precup");
    exercise2();
    exercise3(6, 3);
    exercise4();
    println!("Exercise 5:");
    let computer = new(String::from("Asus"), String::from("i7"), 16);
    display(computer);
    let computers = [
        new(String::from("Asus"), String::from("i7"), 16),
        new(String::from("Dell"), String::from("i5"), 8),
        new(String::from("Lenovo"), String::from("i3"), 4)];
    println!("Computer menu:");
    println!("a. print all computers");
    println!("b. print the computer with the largest amount of memory");
    let input=read();
    match input.as_str() {
        "a" => {for i in computers{
                display(i);
            }
        },
        "b" => {
            let mut c : Computer = new(String::from(""), String::from(""), -1);
            for i in computers{
                if i.memory_size>c.memory_size{
                    c=i;
                }
            }
            display(c);
        },
        _ => {
            println!("Invalid option!")
        }

    }
}

fn exercise2(){
    let a = 2;
    let b = 4;
    if a>b{
        println!("{}",a);
    }
    else{
        println!("{}",b);
    }
}
fn exercise3(x: u32, n : u32){ 
    if x%n == 0{
        println!("Is divisible")
    }
    else
    {
        println!("Is not div")
    }
} 
fn exercise4(){
        let a = [1, 5, 2, 3, 14, 7];
        let mut max = a[0];
        for i in a {
            if i > max {
                max = i;
            }
        }
        println!("The maximum from array is: {max}"); 
}
//exercise 5:
struct Computer{
    brand: String,
    processor_name: String,
    memory_size: i32,
}
fn new(brand: String, processor_name: String, memory_size: i32) -> Computer{
    Computer{
        brand: brand,
        processor_name: processor_name,
        memory_size: memory_size,
    }
}
fn display(computer: Computer) {
    println!("Computer :");
    println!("Brand: {}",computer.brand);
    println!("Processor name: {}",computer.processor_name);
    println!("Memory size: {}",computer.memory_size);
}
fn read() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
