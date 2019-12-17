// 20. Sorting
// https://www.hackerrank.com/challenges/30-sorting

use crate::functions;

pub fn main() {
    let _ = read_line().trim();
    let input: String = String::from(functions::read_line().trim());

    let mut row = input
        .split(' ')
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, std::num::ParseIntError>>()
        .unwrap();

    // println!(
    //     "{} ",
    //     row.into_iter()
    //         .map(|s| s.to_string() + " ")
    //         .collect::<String>()
    // );

    bubble_sort(&mut row);
}

fn bubble_sort(vec: &mut Vec<i32>) {

    let mut num_of_swaps: u8 = 0;

    for (i, value) in &mut vec.iter().enumerate() {
        for j in 0..vec.len() - 1 {
            if vec[j] > vec[j + 1] {
                println!("{} = {}", i, value);
                // let mut saved: i32 = vec[j];
                // vec[j] = vec[j + 1];
                // vec[j + 1] = 3;
                num_of_swaps += 1;
            }
        }
    }

    println!("{}", num_of_swaps);

}

// for (int i = 0; i < n; i++) {
//     // Track number of elements swapped during a single array traversal
//     int numberOfSwaps = 0;
    
//     for (int j = 0; j < n - 1; j++) {
//         // Swap adjacent elements if they are in decreasing order
//         if (a[j] > a[j + 1]) {
//             swap(a[j], a[j + 1]);
//             numberOfSwaps++;
//         }
//     }
    
//     // If no elements were swapped during a traversal, array is sorted
//     if (numberOfSwaps == 0) {
//         break;
//     }
// }

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read stdin!");
    return input;
}
