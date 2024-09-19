use std::collections::{HashMap, HashSet};
use std::iter::Map;
use crate::parsing::equation_ds::File;
use crate::parsing::equation_ds::EquationPart;
use crate::parsing::equation_ds::EquationPart::Literal;

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
    pub literals: Vec<PbLiteral>,
    pub sub_unas: i32,
    pub max_l: Option<PbLiteral>
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
pub struct PbLiteral {
    pub name: u32,
    pub factor: i32,
    pub pos: bool
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
            literals: Vec::new(),
            sub_unas: 0,
            max_l: None
        };
        for equation_part in &equation.e1.literals {
            match equation_part {
                EquationPart::Literal { factor, name , pos} => {
                    let l =
                    if factor < &0 {
                        clause.rhs -= factor;
                        PbLiteral {
                            name: *name,
                            factor: -1 * *factor,
                            pos: false
                        }
                    }else{
                        PbLiteral {
                            name: *name,
                            factor: *factor,
                            pos: true
                        }
                    };
                    clause.sub_unas += l.factor;
                    match clause.max_l.as_mut() {
                        None => {
                            clause.max_l = Some(PbLiteral{name: l.name, factor: l.factor, pos: l.pos});
                        },
                        Some(t) => {
                            if l.factor >= t.factor {
                                t.factor = l.factor;
                                t.name = l.name;
                                t.pos = l.pos;
                            }
                        }
                    }

                    clause.literals.push(l);

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

fn find_duplicates<I, T>(iter: I) -> Vec<T>
where
    I: IntoIterator<Item = T>,
    T: std::hash::Hash + Eq + Clone,
{
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();

    for item in iter {
        if !seen.insert(item.clone()) {
            duplicates.push(item);
        }
    }

    duplicates
}

impl PBFormula {

    pub fn bcp(&mut self) -> bool{
        loop {
            //get implications
            let mut implications: HashMap<u32, PbLiteral> = HashMap::new();
            &self.clauses.retain(|x| x.rhs > 0);
            for c in &self.clauses {
                if c.sub_unas == c.rhs {
                    for l in &c.literals{
                        implications.insert(l.name, PbLiteral{name: l.name, factor: l.factor, pos: l.pos});
                    }
                }else {
                    match &c.max_l {
                        Some(t) => {
                            if c.sub_unas < c.rhs + t.factor {
                                implications.insert(t.name, PbLiteral{name: t.name, factor: t.factor, pos: t.pos});
                            }
                        }
                        _ => {}
                    }
                }
            }

            if implications.is_empty() {
                return true;
            } else {
                if self.n < implications.len() as u32 {
                    println!("test");
                }
                //set implications
                self.n = self.n - implications.len() as u32;
                for (n, i) in implications {
                    for c in &mut self.clauses {
                        if c.literals.iter().filter(|x| x.name == i.name).count() > 0 {
                            for l in &c.literals{
                                if l.name == i.name {
                                    c.sub_unas -= l.factor;
                                    if l.pos == i.pos {
                                        c.rhs -= l.factor;
                                    }
                                }
                            }
                        }
                        c.literals.retain(|x| x.name != i.name);

                        c.max_l = None;
                        for l in &c.literals{
                            match c.max_l.as_mut() {
                                None => c.max_l = Some(PbLiteral {factor: l.factor, pos: l.pos, name: l.name}),
                                Some(t) => {
                                    if t.factor < l.factor {
                                        t.factor = l.factor;
                                        t.name = l.name;
                                        t.pos = l.pos;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

    }
    pub fn contains_false_clause(&self) -> bool {
        for c in &self.clauses {
            if c.literals.is_empty() && c.rhs > 0 {
                return true;
            }
        }
        false
    }

    pub fn get_sub_formula(&self, literal_index: u32, take: bool) -> PBFormula {
        /*
        if self.n == 0{
            println!("test");
        }

         */
        let mut new_formula = PBFormula{
            n: self.n - 1,
            clauses: Vec::new()
        };

        for c in &self.clauses {
            if c.contains(literal_index) {
                let mut new_clause = Clause{
                    rhs: c.rhs,
                    literals: Vec::new(),
                    sub_unas: 0,
                    max_l: None

                };
                for l in &c.literals {
                    if l.name == literal_index {
                        if take == l.pos {
                            new_clause.rhs -= l.factor;
                        }
                    }else{
                        new_clause.literals.push(l.clone());
                        new_clause.sub_unas += l.factor;
                        match new_clause.max_l.as_mut() {
                            None => {
                                new_clause.max_l = Some(PbLiteral{name: l.name, factor: l.factor, pos: l.pos});
                            },
                            Some(t) => {
                                if l.factor >= t.factor {
                                    t.factor = l.factor;
                                    t.name = l.name;
                                    t.pos = l.pos;
                                }
                            }
                        }
                    }
                }


                if new_clause.sub_unas < new_clause.rhs {
                    new_clause.literals = Vec::new();
                    new_clause.rhs = 1;
                }

                new_formula.clauses.push(new_clause);
            }else{
                new_formula.clauses.push(c.clone());
            }

        }

        new_formula
    }
}