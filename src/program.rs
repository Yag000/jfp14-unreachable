#[derive(Clone)]
pub enum Mode {
    Compress,
    Decompress,
}

pub struct Program {
    pub instr: Vec<(String, String)>,
    mode: Mode,
}

impl Program {
    pub fn new(instr: Vec<(String, String)>, mode: Mode) -> Program {
        let mut prog = Program {
            instr,
            mode: mode.clone(),
        };
        prog.sort(&mode);

        match mode {
            Mode::Decompress => prog.normalize_intrs(),
            _ => (),
        }
        prog
    }

    fn sort(&mut self, mode: &Mode) {
        match mode {
            Mode::Compress => self.instr.sort_by(|a, b| a.1.len().cmp(&b.1.len())),
            Mode::Decompress => self.instr.sort_by(|a, b| b.0.len().cmp(&a.0.len())),
        }
    }

    fn normalize_rhs(&self, rhs: String) -> String {
        let mut ans = "".to_string();

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

                let eval = self.eval(tmp);
                let norm = self.normalize_rhs(eval);
                ans.push_str(norm.as_str());
            } else {
                ans.push(c);
                i += 1;
            }
        }

        ans
    }

    fn normalize_intrs(&mut self) {
        for (pos, (key, rhs)) in self.instr.clone().iter().enumerate() {
            let s = self.normalize_rhs(rhs.to_string());
            self.instr[pos] = (key.to_string(), s);
        }
    }

    pub fn eval(&self, mut to_decipher: String) -> String {
        let mut answer = "".to_string();

        while !to_decipher.is_empty() {
            for (key, value) in self.instr.iter() {
                if let Some(suff) = to_decipher.strip_prefix(key.as_str()) {
                    match &self.mode {
                        Mode::Compress => answer.push_str(value),
                        Mode::Decompress => {
                            let s = self.normalize_rhs(value.to_string());

                            answer.push_str(s.as_str())
                        }
                    }
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
            ("00".to_string(), "cad".to_string()),
        ];
        let expected = vec![
            ("00".to_string(), "cad".to_string()),
            ('0'.to_string(), "abra".to_string()),
        ];

        let p1 = Program::new(v);
        let p2 = Program::new(expected);

        assert_eq!(p1.instr, p2.instr);
    }
}
