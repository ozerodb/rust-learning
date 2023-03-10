/*
Problem 2 - Even Fibonacci numbers

Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:

1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
 */

fn main() {
    let mut term1 = 1;
    let mut term2 = 2;
    let mut even_sum = 2;
    let mut current_value: i32;

    loop {
        current_value = term1 + term2;
        if current_value > 4000000 {
            break;
        }
        if current_value % 2 == 0 {
            even_sum += current_value;
        }
        term1 = term2;
        term2 = current_value;
    }

    println!("{}", even_sum);
}
