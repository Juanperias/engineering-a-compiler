use std::ops::{Range, RangeInclusive};

#[derive(PartialEq, Debug, Clone)]
pub enum State {
    S0,
    S2,
    S3
}

#[derive(Debug, Clone)]
pub enum EndState {
    Error,
    Number,
    String,
    Zero,
}

#[derive(Debug)]
pub enum Op {
    Go(State),
    Return(EndState),
}

#[derive(Debug)]
pub enum Cmp {
    Range(RangeInclusive<char>),
    Just(char),
    Any,
    Eow,
}

pub struct SymbolicAutomata {
    schema: Vec<(State, Cmp, Op)>,
}

impl SymbolicAutomata {
    pub fn new(schema: Vec<(State, Cmp, Op)>) -> Self {
        Self {
            schema,
        }
    }
    pub fn parse(&self, expr: &str) -> Vec<(EndState, String)> {
        let mut v = Vec::new();
        let mut inner = String::new();
        let mut state = State::S0;
        let len = expr.chars().count();
         

        for (index, c) in expr.chars().enumerate() {
         
            if c == ' ' && state == State::S0 {
                continue;
            }

            let mut ok = false;
            inner.push(c);

           let a = state.clone();
           let rules = self.schema.iter().filter(|&x| x.0 == a);
        

           for rule in rules {
                let cmp_res = match &rule.1 {
                    Cmp::Range(r) => r.contains(&c),
                    Cmp::Just(r) => r == &c,
                    Cmp::Any => true,
                    Cmp::Eow => c == ' ' || c == '\n' || index+1 == len,
                };

                if !cmp_res { continue; }

                ok = true;

                match &rule.2 {
                    Op::Return(s) => {
                        v.push((s.clone(), inner.clone().trim_end().trim_start().to_string()));
                        inner.clear();

                        state = State::S0;
                    
                        continue;

                    },
                    Op::Go(s) => {
                        state = s.clone();
                        continue;
                    }
                }

                
           } 
           println!("{c}: {:?}", state);

            if !ok {
                v.push((EndState::Error, inner.clone()));
            }
        }



        v
    }
}



fn main() {
    let automata = SymbolicAutomata::new(vec![
        (State::S0, Cmp::Just('0'), Op::Return(EndState::Zero)),

        (State::S0, Cmp::Range('a'..='z'), Op::Go(State::S3)),
    
        (State::S0, Cmp::Range('1'..='9'), Op::Go(State::S2)),
        (State::S2, Cmp::Range('0'..='9'), Op::Go(State::S2)),
        
        (State::S3, Cmp::Range('a'..='z'), Op::Go(State::S3)),

        (State::S2, Cmp::Eow, Op::Return(EndState::Number)),
        (State::S3, Cmp::Eow, Op::Return(EndState::String))

    ]);

    println!("{:?}", automata.parse("123 213123 0 hola"))
}
