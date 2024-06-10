use std::cmp::Ordering;

pub struct Program {
    instr: Vec<(String, String)>,
}

fn bin_to_u32(s: &str) -> u32 {
    u32::from_str_radix(s, 2).unwrap()
}

impl Program {
    pub fn new(instr: Vec<(String, String)>) -> Program {
        let mut prog = Program { instr };
        prog.sort();
        prog
    }

    fn sort(&mut self) {
        self.instr.sort_by(|a, b| {
            let a = bin_to_u32(&a.0);
            let b = bin_to_u32(&b.0);
            match a.cmp(&b) {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Less,
            }
        });
    }

    pub fn eval(to_decipher: String) -> String {
        todo!()
    }
}
