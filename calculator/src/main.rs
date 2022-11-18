use std::env::{args, Args};

fn main() {
    let mut given_args: Args = args();

    let left_hand: String = given_args.nth(1).unwrap();
    let operator: char = given_args.nth(0).unwrap().chars().next().unwrap();
    let right_hand: String = given_args.nth(0).unwrap();

    let lh_number: f32 = left_hand.parse().unwrap();
    let rh_number: f32 = right_hand.parse().unwrap();
    let result: f32 = operate(operator, lh_number, rh_number);

    println!(
        "{:?}",
        output(
            lh_number,
            operator,
            rh_number,
            result
        )
    );
}

fn operate(operator: char, left_hand: f32, right_hand: f32) -> f32 {
    // A function to convert chars to operations.
    if operator == '+' {
        left_hand + right_hand
    } else if operator == '-' {
        left_hand - right_hand
    } else if operator == '/' {
        left_hand / right_hand
    } else if operator == '*' {
        left_hand * right_hand
    } else {
        0.0
    }
}

fn output(lf: f32, op: char, rh: f32, res: f32) -> String {
    // A function to output the result of the calculation.
    format!("{} {} {} = {}", lf, op, rh, res)
}
