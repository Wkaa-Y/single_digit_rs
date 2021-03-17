use std::convert::TryInto;

fn main() {
    println!("{}", get_single_digit(Vec::from([1, 2, 3, 4, 5])));
}

fn build_iterable_sum(sum: i32) -> Vec<u32> {
    sum.to_string()
        .chars()
        .map(|c| char::to_digit(c, 10).unwrap())
        .collect::<Vec<u32>>()
}

fn get_single_digit(numbers: Vec<i32>) -> i32 {
    let sum = numbers.iter().sum::<i32>();
    let iterable_sum = build_iterable_sum(sum);
    let mut final_single_digit: i32 = sum;

    while final_single_digit > 9 {
        let new_sum = iterable_sum.iter().product::<u32>();
        final_single_digit = new_sum.try_into().unwrap();
    }
    final_single_digit
}
