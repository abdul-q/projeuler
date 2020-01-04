/*
Multiples of 3 and 5

Problem 1
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.
Also, replace 1000 with a parameter. Such that, the problem will solve for a given value number.
*/
// use std::collections::HashMap;

fn main() {
    //
}

fn find_multiples(given_num: i64) -> i64 {
    //
    let mut col_3_5: Vec<f64> = Vec::new();

    for nums in 1..=given_num {
        if nums % 3 || nums % 5 {
            col_3_5.push(nums)
        }
    }

    col_3_5.iter().
} 
