use std::io;

fn main() {
    loop {
        print_choices();
        
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 {
            degrees_func();
        } else if choice == 2 {
            fibonnaci_func();
        } else if choice == 3 {
            break;
        } else {
            println!("Unknown option!");
        }

        println!("\n\n");
    }
}

fn print_choices() {
    println!("Choose;\n\
              1. F to C conversion\n\
              2. Fibonnaci print\n\
              3. Exit")
}

fn degrees_func() {
    let mut input = String::new();

    println!("\nInput °F value: ");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Wrong input");
            return;
        }
    };

    let celsius = to_celsius(input);

    println!("{input}°F = {celsius}°C!");
}

fn fibonnaci_func() {
    let mut input = String::new();

    println!("\nInput n: ");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Wrong input");
            return;
        }
    };

    let fibonnaci_ret = fibonnaci(input);

    println!("f({input}) = {fibonnaci_ret}");
}

fn to_celsius(f_degree: f64) -> f64 {
    (f_degree - 32.0) * 5.0/9.0
}

fn fibonnaci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fibonnaci(n-1) + fibonnaci(n-2);
}

// F0 = 0, F1 = 1, and Fn = Fn-1 + Fn-2 for n > 1