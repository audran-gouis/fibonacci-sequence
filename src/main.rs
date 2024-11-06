use std::io;

fn get_fib_number(index : usize) -> usize {
    
    let f_0 : usize = 0;
    let f_1 : usize = 1;

    let mut fib_sequence : Vec<usize>  = vec! [f_0, f_1];

    let mut fib_number : usize;

    for n in 1..index {
        fib_number = fib_sequence[n] + fib_sequence[n-1];
        fib_sequence.push(fib_number);
    }

    let return_value : usize = fib_sequence[index];

    println!("{return_value}");
    return return_value;
}

fn main() {
    // ask for a number

    loop {
        println!("Please input your index number : ");
    
        let mut user_input = String::new();
    
        io::stdin() 
            .read_line(&mut user_input)  
            .expect("Failed to read line");
       
        let user_input_int : usize = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                continue;
            }
        };

        get_fib_number(user_input_int);

    };
}