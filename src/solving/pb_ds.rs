use std::collections::HashSet;
use crate::parsing::equation_ds::File;
use crate::parsing::equation_ds::EquationPart;

#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Debug)]
#[derive(Clone)]
pub struct PBFormula{
    pub n: u32,
    pub clauses: Vec<Clause>
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
pub struct Clause{
    pub rhs: i32,
    pub literals: Vec<Literal>
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
pub struct Literal{
    pub name: u32,
    pub factor: i32,
}

pub fn new(file: &File) -> PBFormula {
    let mut pb_formula = PBFormula{
        n: file.name_map.len() as u32,
        clauses: Vec::new()
    };

    for equation in &file.equations {
        let mut rhs = 0;
        match equation.e2.literals.get(0).unwrap() {
            EquationPart::Literal { .. } => {
                ();
            } EquationPart::Factor(f) => {
                rhs = *f;
            }
        }
        let mut clause = Clause{
            rhs: rhs,
            literals: Vec::new()
        };
        for equation_part in &equation.e1.literals {
            match equation_part {
                EquationPart::Literal { factor, name } => {
                    clause.literals.push(Literal{
                        name: *name,
                        factor: *factor
                    })
                } EquationPart::Factor(_) => {
                    ();
                }
            }
        }
        pb_formula.clauses.push(clause);
    }

    pb_formula
}

impl Clause {
    fn contains(&self, literal_index: u32) -> bool {
        for l in &self.literals{
            if l.name == literal_index { return true }
        }
        false
    }

    fn get_max_pos_imp(&self) -> i32{
        self.literals.iter().map(|x| x.factor).filter(|x| x > &0).sum()
    }

    fn get_max_neg_imp(&self) -> i32{
        self.literals.iter().map(|x| x.factor).filter(|x| x < &0).sum()
    }
    pub(crate) fn has_variable_overlap(&self, variables: &HashSet<u32>) -> bool {
        for l in &self.literals {
            if variables.contains(&l.name) {
                return true;
            }
        }
        return false;
    }
}

impl PBFormula {
    pub fn contains_false_clause(&self) -> bool {
        for c in &self.clauses {
            if c.literals.is_empty() && c.rhs > 0 {
                return true;
            }
        }
        false
    }

    pub fn get_sub_formula(&self, literal_index: u32, take: bool) -> PBFormula {
        if self.n == 0{
            println!("test");
        }
        let mut new_formula = PBFormula{
            n: self.n - 1,
            clauses: Vec::new()
        };

        for c in &self.clauses {
            if c.contains(literal_index) {
                let mut new_clause = Clause{
                    rhs: c.rhs,
                    literals: Vec::new()
                };
                for l in &c.literals {
                    if l.name == literal_index {
                        if take {
                            new_clause.rhs -= l.factor;
                        }
                    }else{
                        new_clause.literals.push(l.clone());
                    }
                }

                if new_clause.get_max_pos_imp() < new_clause.rhs {
                    new_clause.literals = Vec::new();
                    new_clause.rhs = 1;
                }
                if !((new_clause.literals.len() == 0 && new_clause.rhs <= 0) || new_clause.get_max_neg_imp() >= new_clause.rhs){
                    new_formula.clauses.push(new_clause);
                }
            }else{
                new_formula.clauses.push(c.clone());
            }

        }

        new_formula
    }
}