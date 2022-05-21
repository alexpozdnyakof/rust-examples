use std::io;



fn main() {
    loop {
        println!("Enter n for fibonnaci between 1 and 186");
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("🤬🤬🤬 not a number 🤬🤬🤬");
                println!("⏭ try again");
                continue
            }
        };
        let result: u128 = fibonacci(n);
        println!("fibonacci {} member is {}", n , result);
    }

}

fn fibonacci(n: i32) -> u128 {
    if n < 0 {
        panic!("negative is unacceptable!");
    } else if n == 0 {
        panic!("zero is unacceptable");
    } else if n==1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}

