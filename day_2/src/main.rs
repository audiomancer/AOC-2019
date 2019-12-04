use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
    let tuple_for_first_task = (12, 2);
    let value_for_second_task = 19690720;
    let initial_vector = read_to_string("puzzle_input.txt")?
        .trim()
        .split(",")
        .map(|num| {
            num.parse::<usize>()
                .expect("Unexpected string that is not a number")
        })
        .collect::<Vec<usize>>();

    println!(
        "DAY 2 FIRST TASK ANSWER IS: {:?}",
        opcode_formatter(&initial_vector, tuple_for_first_task)
    );

    println!(
        "DAY 2 SECOND TASK ANSWER IS: {:?}",
        noun_verb_for_given_value(&initial_vector, value_for_second_task)
    );

    Ok(())
}

fn noun_verb_for_given_value(input: &Vec<usize>, value: usize) -> Result<usize, String> {
    for noun in 0..100 {
        for verb in 0..100 {
            if opcode_formatter(input, (noun, verb)) == value {
                return Ok(noun * 100 + verb); // formatting part
            }
        }
    }

    Err("Value not found!".to_string())
}

fn opcode_formatter(input: &Vec<usize>, noun_verb_tuple: (usize, usize)) -> usize {
    let mut mem = input.clone();
    mem[1] = noun_verb_tuple.0;
    mem[2] = noun_verb_tuple.1;

    for pointer in 0..mem.len() {
        if pointer % 4 == 0 {
            let opcode = mem[pointer];

            if opcode == 99 {
                break;
            } else if opcode != 1 && opcode != 2 {
                panic!("Unknown op code")
            }

            let par1 = mem[mem[pointer + 1]];
            let par2 = mem[mem[pointer + 2]];
            let dest = mem[pointer + 3];

            let eq_result = if opcode == 1 {
                par1 + par2
            } else {
                par1 * par2
            };

            mem[dest] = eq_result; // overwriting mem in each iteration
        }
    }

    return mem[0]; // returns first memory address
}
