use std::fs;

fn main() {
    let res = match fs::read_to_string("input.txt") {
        Ok(res) => res,
        Err(e) => {
            panic!("Error reading input.txt: {}", e);
        }
    };
    let input: Vec<i32> = res
        .trim_end()
        .split(',')
        .map(|x| match x.parse::<i32>() {
            Ok(val) => val,
            Err(e) => panic!("Error parsing {} from &str to int32: {}", x, e),
        })
        .collect();
    println!("answer is: {:?}", run_intcode_computer(input));
}

fn run_intcode_computer(input: Vec<i32>) -> i32 {
    let mut res: Vec<i32> = input.clone();
    res[1] = 12;
    res[2] = 2;
    for (index, val) in input.iter().enumerate().step_by(4) {
        let mut iter_with_values = input.iter().skip(index + 1).take(3).map(|&x| x as usize);
        let pos_a = iter_with_values.next().unwrap();
        let pos_b = iter_with_values.next().unwrap();
        let pos_res = iter_with_values.next().unwrap();
        match *val as i32 {
            1 => res[pos_res] = res[pos_a] + res[pos_b],
            2 => res[pos_res] = res[pos_a] * res[pos_b],
            _ => break,
        }
    }
    res[0]
}
