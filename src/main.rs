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

fn print_factors(input: u32){
        println!("{:?}", find_factors(input));
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
    print_factors(number_input());
}
