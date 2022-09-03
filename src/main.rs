use std::io;
fn main() {
    println!("Please write an unsigned-integer to decompose ?!");
    
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Err: Reading number");
    let mut number:u32 = number.trim().parse().expect("Err: Parsing number");
    
    let mut prime:u32 = 2;

    while number > 1 {
        
        if is_even(number, prime)
        {
            number = divide(number, prime);
            println!("Found: {}",prime);
            continue;
        }

        if ! is_even(number, prime)
        {
            //increase prime by one
            prime = prime +1;
            continue;
        }

        
    }
    

}

fn is_even(n1:u32,n2:u32)->bool
{
    if n1%n2 == 0
    {
        return true
    }
    
    false
}

fn divide(n1:u32,n2:u32)->u32
 {
    n1/n2
 }
