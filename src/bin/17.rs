use std::collections::HashMap;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32>
{
    let mut program = Program::from_str(input);
    let result = program.run();

    let result = result.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");
    println!("Result: {}", result);
    Some(0)
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut program = Program::from_str(input);
    let expected_result = program.instructions.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");

    let mut todo = vec![(1, 0)];
    let prog = program.instructions.clone().iter().map(|&i|i as i64).collect::<Vec<i64>>();

    let mut lowest = i128::MAX;
    while let Some((i, a)) = todo.pop() {
        for a in a..a+8 {
            program.clear();
            program.registers.insert('A', a);
            let start = prog.len() - i;
            if start >16 {
                continue;
            }

            if program.run() == &prog[start..] {
                todo.push((i + 1, a * 8));
                if i == prog.len() {
                    println!("{}", a);
                    lowest = lowest.min(a);
                }
            }
        }
    }
    Some(lowest)
}

#[derive(Debug)]
struct Program{
    registers: HashMap<char, i128>,
    pointer: usize,
    instructions: Vec<usize>,
    output: Vec<i64>,
}


impl Program {
    fn clear(&mut self){
        self.registers.insert('A', 0);
        self.registers.insert('B', 0);
        self.registers.insert('C', 0);
        self.pointer = 0;
        self.output.clear();
    }
    fn run(&mut self) -> Vec<i64> {
        let mut i =0;
        while self.pointer < self.instructions.len() {
            self.process();
            i+=1;
            // if i > 10{
            //     break;
            // }
        }
        self.output.clone()
    }
    fn combo_operand(&self, id:usize)->usize{
        if id >= 0 && id <= 3 {
            return id;
        }
        if id == 4{
            return self.registers[&'A'] as usize;
        }
        if id == 5{
            return self.registers[&'B'] as usize;
        }
        if id == 6{
            return self.registers[&'C'] as usize;
        }
        unreachable!("Invalid operand id: {}", id);
    }
    fn process(&mut self){
        let opcode= self.instructions[self.pointer];
        let mut increment_pc = true;
        // println!("Processing opcode: {}", opcode);
        // println!("Registers: {:?}", self.registers);
        match opcode {
            0=> {
                self.registers.insert('A', self.registers[&'A'] / 2i128.pow(self.combo_operand(self.instructions[self.pointer + 1]) as u32));
            }
            1=> {
                self.registers.insert('B', self.registers[&'B'] ^ self.instructions[self.pointer + 1] as i128);
            }
            2=> {
                self.registers.insert('B', (self.combo_operand(self.instructions[self.pointer + 1]) % 8) as i64 as i128);
            }
            3=>{
                if self.registers[&'A'] == 0{
                    // do nothing
                }else{
                    self.pointer =self.instructions[self.pointer + 1] as usize;
                    increment_pc = false;
                }

            }
            4=>{
                self.registers.insert('B', self.registers[&'B'] ^ self.registers[&'C']);
            }
            5=>{
                self.output.push((self.combo_operand(self.instructions[self.pointer + 1]) % 8) as i64);
            }
            6=>{
                self.registers.insert('B', self.registers[&'A'] / 2i128.pow(self.combo_operand(self.instructions[self.pointer + 1]) as u32));
            }
            7=>{
                self.registers.insert('C', self.registers[&'A'] / 2i128.pow(self.combo_operand(self.instructions[self.pointer + 1]) as u32));
            }
            _ => {
                unreachable!("Invalid opcode: {}", opcode);
            }
        }


        if increment_pc{
            self.pointer += 2;
        }
    }
    fn from_str(input: &str) -> Self {
        let mut registers = HashMap::new();
        let mut instructions = Vec::new();

        for line in input.lines() {
            if line.starts_with("Register") {
                let parts: Vec<&str> = line.split(':').collect();
                let register = parts[0].chars().last().unwrap();
                let value = parts[1].trim().parse().unwrap();
                registers.insert(register, value);
            } else if line.starts_with("Program:") {
                let parts: Vec<&str> = line.split(':').collect();
                instructions = parts[1].trim().split(',').map(|s| s.parse().unwrap()).collect();
            }
        }

        Program {
            registers,
            pointer: 0,
            instructions,
            output: Vec::new(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#);
        assert_eq!(result, Some(117440));
    }
}
