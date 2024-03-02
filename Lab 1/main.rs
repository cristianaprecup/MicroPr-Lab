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
    println!("Computers menu:");
    println!("1. Display all computers");
    println!("2. Print the computer with the largest memory\n");

    let input = read_i32();

    match input {
        1 => {
            for computer in &computers {
                display(computer);
            }
        },
        _ => {
            let mut computer_with_max : &Computer = &computers[0];
            for computer in &computers {
                if computer.memory_size > computer_with_max.memory_size {
                    computer_with_max = computer;
                }
            }
            display(computer_with_max);
        },
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
fn read_i32() -> i32 {
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.parse().unwrap()
}
