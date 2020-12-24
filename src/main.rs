use std::io::{stdin,stdout,Write};

fn main() {
    println!("Size? ");
    let mut _err = stdout().flush();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }


    if let Ok(size) = input.parse::<i64>(){

        let max_width = 2 * size - 1;

        let row = |stars: i64| {
            (0..(max_width - stars)/2).fold(String::new(), |acc, _|{ format!("{}{}", acc, ' ')})
        };
        
        for n in 0..size{
            print!("{}", row(2*n + 1));
            for _ in 0..(2*n + 1){ print!("*"); _err = stdout().flush(); }
            println!();
        }

        let ratio = size/4;

        for _ in 0..(ratio){

            print!("{}", row(2*ratio - 1));
            for _ in 0..2*ratio - 1{ print!("*"); _err = stdout().flush(); }
            println!();
        }; 

    }else{
        println!("Only integers are allowed")
    }

    

}
