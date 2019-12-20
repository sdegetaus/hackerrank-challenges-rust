// 25. Running Time and Complexity
// https://www.hackerrank.com/challenges/30-running-time-and-complexity

// use crate::functions;

pub fn main() {
    let mut test_cases: u32 = match String::from(read_line().trim()).parse::<u32>() {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            0
        }
    };

    while test_cases > 0 {
        let n: u32 = String::from(read_line().trim()).parse::<u32>().unwrap();

        let mut flag = false;

        for i in 2..n / 2 {
            if n % i == 0 {
                flag = true;
                break;
            }
        }

        // a better way?
        if n == 4 {
            println!("Not prime");
            test_cases -= 1;
            continue;
        }

        if n != 1 {
            if flag == false {
                println!("Prime");
            } else {
                println!("Not prime");
            }
        } else {
            println!("Not prime");
        }

        test_cases -= 1;
    }
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read stdin!");
    return input;
}
