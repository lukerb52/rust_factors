use std::io;

fn find_factors(input: u32) -> Vec<u32> {
    //let returner: Vec<u32>;
    let mut returner = vec![];
    returner.push(input);

    let dinput = input as f64;
    let max: u32 = input/2;
    for bi in 0..max{
        let i = max-bi;
        //println!("{}",i);
        let di = i as f64;
        let itest = (input/i) as f64;
        if dinput/di == itest {
            returner.push(i);
        }
    }
    return returner;
}

fn print_vector(input: Vec<u32>){
    let len = input.len();
    println!("{:?}\n{} in total", input, len);
}

fn print_table(input: Vec<u32>){
    //println!("\nMultiplication Table:");
    let len = input.len();

    let range =
        if len%2 !=0{
            len/2+1
        }
        else{
            len/2
        };
    for i in 0..range{
        println!("{} * {} = {}", input[i], input[len-i-1], input[0] );
    }
}

fn number_input() -> u32 {
    println!("Type a positive number to find the factors of: ");

    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let input: u32 = line
        .trim()
        .parse()
        .expect("Wanted a positive number");
    return input;
}

fn main() {
    let ni = number_input();
    //let factors = find_factors(ni);
    println!("\nFactors: ");
    print_vector(find_factors(ni));

    println!("\nMultiplication Table: ");
    print_table(find_factors(ni));
}
