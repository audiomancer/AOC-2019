use std::fs::read_to_string;
use std::io;
use std::iter::successors;

fn main() -> io::Result<()> {
    let vector_of_masses = read_to_string("masses_of_the_modules.txt")?
        .lines()
        .map(|num| {
            num.parse::<i32>()
                .expect("Unexpected string that is not a number")
        })
        .collect::<Vec<i32>>();

    println!("DAY 1 FIRST TASK ANSWER: {}", first_task(&vector_of_masses));
    println!("DAY 1 SECOND TASK ANSWER: {}", second_task(&vector_of_masses));

    Ok(())
}

fn first_task(masses: &Vec<i32>) -> i32 {
    masses.into_iter().map(fuel_equation).sum()
}

fn second_task(masses: &Vec<i32>) -> i32 {
    masses
        .into_iter()
        .map(|input| {
            successors(Some(fuel_equation(input)), |&val| Some(fuel_equation(&val)))
                .take_while(|&x| x > 0)
                .sum::<i32>()
        })
        .sum()
}

fn fuel_equation(mass: &i32) -> i32 {
    (mass / 3) - 2
}
