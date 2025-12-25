use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ILoc {
    LoadI(String, u64), // load an imm value to "Mem Segment" in this moment is just an String
    Print(String), // prints segment content
}

pub struct Engine {
    mem: HashMap<String, u64>
}


#[derive(Debug)]
pub struct LivenessVarInfo {
    pub def: Option<Def>,
    pub die: Die,
}

impl LivenessVarInfo {
    pub fn set_def(&mut self, def: Def) {
        if self.def.is_some() {
            return;
        }

        self.def = Some(def);
    }
    pub fn set_die(&mut self, die: Die) {
        self.die = die;
    }
}

#[derive(Debug)]
pub struct Def(usize);

#[derive(Debug)]
pub struct Die(usize);



impl Engine {
    pub fn new() -> Self {
        Self {
            mem: HashMap::new()
        }
    }

    // -> Used Var
    fn r(&mut self, iloc: ILoc) -> String {
        let mut used = String::new();

        match iloc {
            ILoc::LoadI(pair, val) => {
                used = pair.clone();
                self.mem.insert(pair, val);
            },
            ILoc::Print(pair) => {
                let val = self.mem.get(&pair);
                used = pair;
    
                println!("ENGINE PRINT: {}", val.unwrap());
            }
        }

        used
    }
    pub fn liveness_detector(&mut self, ins: &[ILoc]) -> HashMap<String, LivenessVarInfo>{
        let mut result: HashMap<String, LivenessVarInfo> = HashMap::new();

        for (index, iloc) in ins.iter().enumerate() { 
            match iloc {
                ILoc::Print(v) => {
                    if !result.contains_key(v) {
                        result.insert(v.to_owned(), LivenessVarInfo { def: None, die: Die(index) });
                    }

                    let a = result.get_mut(v).unwrap();
         
                    a.set_die(Die(index));

                },
                ILoc::LoadI(v, _) => {
                    if !result.contains_key(v) {
                        result.insert(v.to_owned(), LivenessVarInfo { def: Some(Def(index)), die: Die(index) });
                    }


                    let a = result.get_mut(v).unwrap();
                    
                    a.set_def(Def(index));
                }
            }
        }

        result    
    }
    pub fn run(&mut self, ins: Vec<ILoc>) {
       let liveness = self.liveness_detector(&ins); 


       for (index, iloc) in ins.iter().enumerate() {
            println!("{index}: ");
            let used = self.r(iloc.clone());

            let live = liveness.get(&used).unwrap();

            if live.def.as_ref().unwrap().0 == index {
                println!("liveness engine: {used} defined");
            }

            if live.die.0 == index {
                println!("liveness: engine: {used} died");
            }

       }
    } 
}

fn main() {
    let mut engine = Engine::new();
    let ins = vec![
        ILoc::LoadI("A".to_string(), 2),
        ILoc::Print("A".to_string()),
        ILoc::Print("A".to_string()),
        ILoc::LoadI("B".to_string(), 90),
        ILoc::Print("A".to_string()),
        ILoc::Print("B".to_string()),
        ILoc::LoadI("C".to_string(), 90),



    ];

    engine.run(ins); 
}
