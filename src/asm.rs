use std::collections::HashSet;

#[derive(Clone)]
pub struct Instruction {
    pub kind: Kind,
    pub val: i32,
}

#[derive(Clone)]
pub enum Kind {
    Nop,
    Acc,
    Jmp,
}

#[derive(Clone)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub visited: HashSet<usize>,
    pub acc: i32,
    pub isp: usize,
}

pub enum Termination {
    Normal(i32),
    Looping(i32),
}

impl Program {
    fn parse_instructions(s: &str) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        for line in s.lines() {
            let kind = match &line[..3] {
                "nop" => Kind::Nop,
                "acc" => Kind::Acc,
                "jmp" => Kind::Jmp,
                x => panic!("Invalid instruction: {:?}", x),
            };
            let val = line[4..].parse().unwrap();

            instructions.push(Instruction { kind, val });
        }

        instructions
    }

    pub fn from_str(s: &str) -> Program {
        let instructions = Self::parse_instructions(s);

        Program {
            instructions,
            acc: 0,
            isp: 0,
            visited: HashSet::new(),
        }
    }

    pub fn execute_next(&mut self) -> Option<Termination> {
        if self.isp < self.instructions.len() {
            if !self.visited.insert(self.isp) {
                return Some(Termination::Looping(self.acc));
            }

            let instruction = &self.instructions[self.isp];
            match &instruction.kind {
                Kind::Nop => self.isp += 1,
                Kind::Acc => {
                    self.acc += instruction.val;
                    self.isp += 1;
                }
                Kind::Jmp => self.isp = (instruction.val + self.isp as i32) as usize,
            }

            return None;
        }

        Some(Termination::Normal(self.acc))
    }

    pub fn run(&mut self) -> Termination {
        loop {
            if let Some(termination) = self.execute_next() {
                return termination;
            }
        }
    }
}
