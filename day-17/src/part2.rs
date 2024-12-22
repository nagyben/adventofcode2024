use std::collections::HashMap;

use crate::{parse_input, run_instruction, Computer};

pub fn process(input: &str) -> usize {
    let mut program = parse_input(input);
    let mut cache: HashMap<(Computer, Vec<isize>), Vec<isize>> = HashMap::new();
    let mut output = vec![];
    let mut start_a = program.a;
    let expected_output = &program.program_raw; // let output = run_program(&program)
    let mut computer = Computer {
        a: program.a,
        b: program.b,
        c: program.c,
        instruction_pointer: 0,
        program_raw: program.program_raw.clone(),
    };
    while output != *expected_output {
        start_a += 1;
        computer.a = start_a;
        output = vec![];
        while computer.instruction_pointer < program.program.len() {
            // if cache.contains_key(&(computer.clone(), program.program_raw.clone())) {
            //     println!("Cache hit!");
            //     output.extend(cache[&(computer.clone(), program.program_raw.clone())].clone());
            //     break;
            // }
            let (instruction, operand) = &program.program[computer.instruction_pointer];
            if let Some(o) = run_instruction(&mut computer, *instruction, *operand) {
                output.push(o);
            }
        }
        cache.insert(
            (computer.clone(), program.program_raw.clone()),
            output.clone(),
        );
    }
    start_a as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
        assert_eq!(process(INPUT), 117440);
    }
}
