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
        prog.normalize_intrs();
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

    fn normalize_rhs(&mut self, rhs: String, key: Option<String>) -> String {
        let mut ans = String::new();

        let bytes_rhs: Vec<char> = rhs
            .as_bytes()
            .iter()
            .map(|c| c.to_owned() as char)
            .collect();

        let mut i = 0;

        while i < bytes_rhs.len() {
            let c = bytes_rhs[i];
            if c == '0' || c == '1' {
                let mut tmp = String::new();

                while i < bytes_rhs.len() && (bytes_rhs[i] == '0' || bytes_rhs[i] == '1') {
                    tmp.push(bytes_rhs[i]);
                    i += 1;
                }

                let eval = self.eval(tmp.clone());
                if eval.contains("0") || eval.contains("1") {
                    let norm = self.normalize_rhs(eval, Some(tmp));
                    ans.push_str(norm.as_str());
                } else {
                    ans.push_str(eval.as_str());
                }
            } else {
                ans.push(c);
                i += 1;
            }
        }

        if let Some(key) = key {
            if let Some(pos) = self.instr.iter().position(|(a, _)| *a == key) {
                self.instr[pos] = (key, ans.clone());
            } else {
                self.instr.push((key, ans.clone()));
                self.sort();
            }
        }
        ans
    }

    fn normalize_intrs(&mut self) {
        for (pos, (key, rhs)) in self.instr.clone().iter().enumerate() {
            let s = self.normalize_rhs(rhs.to_string(), None);
            self.instr[pos] = (key.to_string(), s);
        }
    }

    pub fn eval(&self, mut to_decipher: String) -> String {
        let mut answer = String::new();

        while !to_decipher.is_empty() {
            for (key, value) in self.instr.iter() {
                if let Some(suff) = to_decipher.strip_prefix(key.as_str()) {
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
