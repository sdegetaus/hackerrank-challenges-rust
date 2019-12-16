// 2. Operators
// https://www.hackerrank.com/challenges/30-operators

use crate::functions;

pub fn main() {
    let meal_cost: f32 = functions::read_line().trim().parse().unwrap();
    let tip_percent: f32 = functions::read_line().trim().parse().unwrap();
    let tax_percent: f32 = functions::read_line().trim().parse().unwrap();

    let total_cost: f32 =
        meal_cost + (meal_cost * (tip_percent / 100.0)) + (meal_cost * (tax_percent / 100.0));
    println!("{}", total_cost.round());
}
