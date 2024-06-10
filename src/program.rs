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

    pub fn eval(&self, mut to_decipher: String) -> String {
        let mut answer = String::new();

        while !to_decipher.is_empty() {
            for (key, value) in self.instr.iter() {
                if let Some((_, suff)) = to_decipher.split_once(key.as_str()) {
                    answer.push_str(value);
                    to_decipher = suff.to_string();
                    break;
                }
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::Program;

    #[test]
    fn basics() {
        let v = vec![
            ('0'.to_string(), "abra".to_string()),
            ("1".to_string(), "cad".to_string()),
        ];
        let expected = vec![
            ("1".to_string(), "cad".to_string()),
            ('0'.to_string(), "abra".to_string()),
        ];

        let p1 = Program::new(v);
        let p2 = Program::new(expected);

        assert_eq!(p1.instr, p2.instr);
    }
}
