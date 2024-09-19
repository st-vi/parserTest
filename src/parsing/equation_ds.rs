use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use bimap::BiMap;

pub struct File {
    pub name_map: BiMap<String, u32>,
    pub equations: Vec<Equation>,
    pub name_index: u32
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl File {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for e in &self.equations {
            result.push_str(&*e.to_string(&self));
            result.push_str("\n");
        }
        result
    }

    pub fn to_pbcount_string(&self) -> String {
        let mut result = String::new();
        result.push_str("* #variable= ");
        result.push_str(&*self.name_map.len().to_string());
        result.push_str(" #constraint= ");
        result.push_str(&*self.equations.len().to_string());
        result.push_str("\n");
        for e in &self.equations {
            result.push_str(&*e.to_pbcount_string(&self));
            result.push_str(";\n");
        }
        result
    }

    pub fn to_dimacs_string(&self) -> String {
        let mut result = String::new();
        result.push_str("p cnf ");
        result.push_str(&*self.name_map.len().to_string());
        result.push_str(" ");
        result.push_str(&*self.equations.len().to_string());
        result.push_str("\n");
        for e in &self.equations {
            result.push_str(&*e.to_dimacs_string());
            result.push_str("\n");
        }
        result
    }
}

pub struct Equation {
    pub e1: EquationSide,
    pub e2: EquationSide,
    pub symbol: EquationSymbol
}

impl Display for Equation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e1, self.symbol, self.e2)
    }
}
impl Equation {
    fn to_string(&self, file: &File) -> String {
        format!("{} {} {}", self.e1.to_string(file), self.symbol, self.e2.to_string(file))
    }
    fn to_pbcount_string(&self, file: &File) -> String {
        format!("{} {} {}", self.e1.to_pbcount_string(file), self.symbol, self.e2.to_pbcount_string(file))
    }
    fn to_dimacs_string(&self) -> String {
        format!("{} 0", self.e1.to_dimacs_string())
    }
}

#[derive(PartialEq)]
pub enum EquationSymbol {
    eq,
    ge,
    le
}

impl Display for EquationSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EquationSymbol::eq => write!(f, "="),
            EquationSymbol::ge => write!(f, ">="),
            EquationSymbol::le => write!(f, "<=")
        }

    }
}

#[derive(Clone)]
pub struct EquationSide {
    pub literals: Vec<EquationPart>,
}

impl Display for EquationSide {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(for (i,l) in self.literals.iter().enumerate() {
            if i == 0{
                write!(f, "{}", l);
            }else{
                write!(f, " {}", l);
            }

        })
    }
}
impl EquationSide {
    fn to_string(&self, file: &File) -> String {
        let mut result = String::new();
        for (i,l) in self.literals.iter().enumerate() {
            if i == 0{
                write!(&mut result, "{}", l.to_string(file)).unwrap();
            }else{
                write!(&mut result, " {}", l.to_string(file)).unwrap();
            }

        }

        result
    }

    fn to_pbcount_string(&self, file: &File) -> String {
        let mut result = String::new();
        for (i,l) in self.literals.iter().enumerate() {
            if i == 0{
                write!(&mut result, "{}", l.to_pbcount_string(file)).unwrap();
            }else{
                write!(&mut result, " {}", l.to_pbcount_string(file)).unwrap();
            }

        }

        result
    }

    fn to_dimacs_string(&self) -> String {
        let mut result = String::new();
        for (i,l) in self.literals.iter().enumerate() {
            if i == 0{
                write!(&mut result, "{}", l.to_dimacs_string()).unwrap();
            }else{
                write!(&mut result, " {}", l.to_dimacs_string()).unwrap();
            }

        }

        result
    }
}

#[derive(Clone)]
pub enum EquationPart {
    Literal {
        factor: i32,
        name: u32,
        pos: bool
    },
    Factor(i32)
}

impl Display for EquationPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EquationPart::Factor(i) => {
                if i <= &0 {
                    write!(f, "{}", i)
                } else {
                    write!(f, "+{}", i)}
            },
            EquationPart::Literal {factor, name, pos} => write!(f, "{}*{}", factor, name)
        }
    }
}
impl EquationPart {
    fn to_string(&self, file: &File) -> String {
        let mut result = String::new();
        match self {
            EquationPart::Factor(i) => {
                if i <= &0 {
                    write!(result, "{}", i).unwrap();
                } else {
                    write!(result, "+{}", i).unwrap();}
            },
            EquationPart::Literal {factor, name, pos} => {
                let tmp = file.name_map.get_by_right(name);
                if factor < &0 {
                    write!(result, "{}*{}", factor, file.name_map.get_by_right(name).unwrap()).unwrap();
                } else {
                    write!(result, "+{}*{}", factor, file.name_map.get_by_right(name).unwrap()).unwrap();
                }
            }
        }
        result
    }

    fn to_pbcount_string(&self, file: &File) -> String {
        let mut result = String::new();
        match self {
            EquationPart::Factor(i) => {
                if i <= &0 {
                    write!(result, "{}", i).unwrap();
                } else {
                    write!(result, "+{}", i).unwrap();}
            },
            EquationPart::Literal {factor, name, pos} => {
                if factor < &0 {
                    write!(result, "{} x{}", factor, name +1).unwrap();
                } else {
                    write!(result, "+{} x{}", factor, name +1).unwrap();
                }
            }
        }
        result
    }

    fn to_dimacs_string(&self) -> String {
        let mut result = String::new();
        match self {
            EquationPart::Factor(i) => {

            },
            EquationPart::Literal {factor, name,pos} => {
                if factor < &0 {
                    write!(result, "-{}", name +1).unwrap();
                } else {
                    write!(result, "{}", name +1).unwrap();
                }
            }
        }
        result
    }
}