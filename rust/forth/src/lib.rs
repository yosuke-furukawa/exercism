use std::collections::HashMap;
use std::collections::VecDeque;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Default, Clone)]
pub struct Forth {
    stack: Vec<Value>,
    funcs: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn definition(&mut self, def: &[&str]) -> ForthResult {
        match def {
            [":", function, args @ .., ";"] => {
                if function.chars().any(|c| c.is_numeric()) {
                    return Err(Error::InvalidWord);
                }
                let mut commands = vec![];
                for arg in args {
                    if self
                        .funcs
                        .contains_key(&arg.to_string().to_ascii_uppercase())
                    {
                        let mut command = self
                            .funcs
                            .get(&arg.to_string().to_ascii_uppercase())
                            .unwrap()
                            .clone();
                        commands.append(&mut command);
                    } else {
                        commands.push(arg.to_string());
                    }
                }
                self.funcs.insert(function.to_string().to_ascii_uppercase(), commands);
            }
            _ => {
                return Err(Error::InvalidWord);
            }
        }
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let code = input.trim().to_string();
        let mut codes: VecDeque<String> = code
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect();

        while !codes.is_empty() {
            let ch = codes.pop_front().unwrap();
            match ch.as_str() {
                c if self.funcs.contains_key(&c.to_ascii_uppercase()) => {
                    if let Some(commands) = self.funcs.get(&c.to_ascii_uppercase()) {
                        for command in commands.iter().rev() {
                            codes.push_front(command.clone());
                        }
                    }
                }
                ":" => {
                    let mut definitions = vec![":".to_string()];
                    while let Some(code) = codes.pop_front() {
                        definitions.push(code.clone());
                        if code == ";" {
                            break;
                        }
                    }
                    let result = self.definition(
                        definitions
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<&str>>()
                            .as_slice(),
                    );
                    if result.is_err() {
                        return result;
                    }
                }
                "+" => {
                    let v2 = self.stack.pop();
                    let v1 = self.stack.pop();
                    if v1 == None || v2 == None {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.push(v1.unwrap() + v2.unwrap());
                }
                "-" => {
                    let v2 = self.stack.pop();
                    let v1 = self.stack.pop();
                    if v1 == None || v2 == None {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.push(v1.unwrap() - v2.unwrap());
                }
                "*" => {
                    let v2 = self.stack.pop();
                    let v1 = self.stack.pop();
                    if v1 == None || v2 == None {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.push(v1.unwrap() * v2.unwrap());
                }
                "/" => {
                    let v2 = self.stack.pop();
                    let v1 = self.stack.pop();
                    if v1 == None || v2 == None {
                        return Err(Error::StackUnderflow);
                    }
                    if v2 == Some(0) {
                        return Err(Error::DivisionByZero);
                    }
                    self.stack.push(v1.unwrap() / v2.unwrap());
                }
                _ if ch.to_ascii_uppercase() == "DUP" => {
                    let x = self.stack.pop();
                    if x == None {
                        return Err(Error::StackUnderflow);
                    }
                    let n = x.unwrap();
                    self.stack.push(n);
                    self.stack.push(n);
                }
                _ if ch.to_ascii_uppercase() == "DROP" => {
                    if self.stack.is_empty() {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.pop();
                }
                _ if ch.to_ascii_uppercase() == "SWAP" => {
                    let v2 = self.stack.pop();
                    let v1 = self.stack.pop();
                    if v1 == None || v2 == None {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.push(v2.unwrap());
                    self.stack.push(v1.unwrap());
                }
                _ if ch.to_ascii_uppercase() == "OVER" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    let v = self.stack[self.stack.len() - 2];
                    self.stack.push(v);
                }
                num if ch.chars().collect::<Vec<char>>()[0].is_numeric() => {
                    self.stack.push(num.parse().unwrap());
                }
                _ => return Err(Error::UnknownWord),
            }
        }

        Ok(())
    }
}
