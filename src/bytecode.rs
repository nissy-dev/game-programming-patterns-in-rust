#![allow(dead_code)]

enum Instructions {
    InstSetHealth,
    InstGetHealth(i32),
    InstPlaySound,
    InstLiteral(i32),
    InstAdd,
}

struct VM {
    instructions: Vec<Instructions>,
    stack: Vec<i32>,
    current_inst_index: usize,
}

impl VM {
    fn new(instructions: Vec<Instructions>) -> Self {
        VM {
            instructions,
            stack: Vec::new(),
            current_inst_index: 0,
        }
    }

    fn interpreter(&mut self) {
        match self.instructions[self.current_inst_index] {
            Instructions::InstSetHealth => {
                let value = self.stack.pop().unwrap();
                println!("Set health to {}", value);
            }
            Instructions::InstGetHealth(value) => {
                self.stack.push(value);
                println!("Get health to {}", value);
            }
            Instructions::InstPlaySound => {
                let value = self.stack.pop().unwrap();
                println!("Play sound {}", value);
            }
            Instructions::InstLiteral(value) => {
                self.stack.push(value);
                println!("Literal {}", value);
            }
            Instructions::InstAdd => {
                let value1 = self.stack.pop().unwrap();
                let value2 = self.stack.pop().unwrap();
                self.stack.push(value1 + value2);
                println!("Add {}", value1 + value2);
            }
        }
        self.current_inst_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytecode() {
        let instructions = vec![
            Instructions::InstLiteral(3),
            Instructions::InstGetHealth(45),
            Instructions::InstLiteral(0),
            Instructions::InstPlaySound,
            Instructions::InstAdd,
            Instructions::InstSetHealth,
        ];
        let mut vm = VM::new(instructions);
        vm.interpreter();
        assert_eq!(vm.stack, vec![3]);
        vm.interpreter();
        assert_eq!(vm.stack, vec![3, 45]);
        vm.interpreter();
        assert_eq!(vm.stack, vec![3, 45, 0]);
        vm.interpreter();
        assert_eq!(vm.stack, vec![3, 45]);
        vm.interpreter();
        assert_eq!(vm.stack, vec![48]);
        vm.interpreter();
        assert_eq!(vm.stack, vec![]);
    }
}
