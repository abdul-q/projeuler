/*
Multiples of 3 and 5

Problem 1
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.
Also, replace 1000 with a parameter. Such that, the problem will solve for a given value(number).
*/
// use std::collections::HashMap;

fn main() {
    //
    // let mut sum_of_col = 0;
    // for elmt in find_multiples(8456){
    //     sum_of_col += elmt;
    // }
    // println!("{}", sum_of_col);
    // let iterated = find_multiples(10);
    // let summation = sum_output(iterated);
    // println!("{}", summation);
    print_out_sum(1000);
}

fn find_multiples(given_num: i64) -> Vec<i64> {
    //
    let mut col_3_5: Vec<i64> = Vec::new();

    for nums in 1..given_num {
        if nums % 3 == 0 || nums % 5 == 0 {
            col_3_5.push(nums as i64)
        }
    }
    col_3_5
} 

fn sum_output(iterated: Vec<i64>) -> i64 {
    let mut sum_ite = 0;
    for elmt in iterated{
        sum_ite += elmt;
    }
    sum_ite
}

fn print_out_sum(nums: i64) {
    let iterated = find_multiples(nums);
    let summation = sum_output(iterated);
    println!("{}", summation);
}
