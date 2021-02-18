use num_bigint::{BigUint, Sign};

#[derive(Debug, Clone)]
pub struct Coeficient {
    data: BigUint,
    sign: Sign,
}

impl Coeficient {
    pub fn new(coef: &[u8]) -> Coeficient {
        let mut sg = Sign::Plus;
        let mut temp = coef;
        if coef[0] == b'-' {
            sg = Sign::Minus;
            temp = &coef[1..coef.len()];
        }
        let cf = BigUint::parse_bytes(temp, 10).unwrap();
        Coeficient { data: cf, sign: sg }
    }
}

impl std::fmt::Display for Coeficient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if self.sign == Sign::Plus || self.sign == Sign::NoSign {
                ""
            } else {
                "-"
            },
            self.data
        )
    }
}

impl<'a, 'b> std::ops::Add<&'b Coeficient> for &'a Coeficient {
    type Output = Coeficient;

    fn add(self, other: &'b Coeficient) -> Coeficient {
        let mut sum = Coeficient {
            data: BigUint::parse_bytes(b"0", 10).unwrap(),
            sign: Sign::NoSign,
        };
        if self.sign == Sign::Plus {
            if other.sign == Sign::Minus {
                if self.data < other.data {
                    sum.data = &other.data - &self.data;
                    sum.sign = Sign::Minus;
                } else if self.data > other.data {
                    sum.data = &self.data - &other.data;
                    sum.sign = Sign::Plus;
                }
            } else if self.data != other.data {
                sum.data = &self.data + &other.data;
                sum.sign = Sign::Plus;
            }
        } else {
            if other.sign == Sign::Plus {
                if self.data > other.data {
                    sum.data = &self.data - &other.data;
                    sum.sign = Sign::Minus;
                } else if self.data < other.data {
                    sum.data = &other.data - &self.data;
                    sum.sign = Sign::Plus;
                }
            } else if self.data != other.data {
                sum.data = &self.data + &other.data;
                sum.sign = Sign::Minus;
            }
        }
        Coeficient {
            data: sum.data,
            sign: sum.sign,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Polynomial {
    coefs: Vec<Coeficient>,
}

impl Polynomial {
    pub fn new(data: &Vec<Coeficient>) -> Polynomial {
        Polynomial {
            coefs: data.to_vec(),
        }
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output: String = "".to_owned();
        let mut iter = 0;
        for e in &self.coefs {
            if iter > 0 {
                if e.sign == Sign::Plus || e.sign == Sign::NoSign {
                    output.push_str(" + ");
                } else {
                    output.push_str(" - ");
                }
                output.push_str(&e.data.to_string());
                output.push_str("X^");
                output.push_str(&iter.to_string());
            } else {
                output.push_str(&e.data.to_string());
            }
            iter += 1;
        }
        write!(f, "{}", output)
    }
}