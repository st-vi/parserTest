use std::collections::HashMap;
use crate::parsing::equation_ds::{Equation, EquationPart, EquationSymbol, File};
use crate::parsing::equation_ds::EquationPart::{Factor, Literal};

pub fn preprocess_file(mut file: File) -> File{
    let mut new_equation_list = Vec::new();

    for mut equation in file.equations {
        preprocessing_literals_left_numbers_right(&mut equation);
        preprocessing_add_up_same(&mut equation);
        if equation.symbol == EquationSymbol::eq {
            let e1 = Equation{
                symbol: EquationSymbol::le,
                e1: equation.e1.clone(),
                e2: equation.e2.clone()
            };
            let e2 = Equation{
                symbol: EquationSymbol::ge,
                e1: equation.e1.clone(),
                e2: equation.e2.clone()
            };
            new_equation_list.push(e1);
            new_equation_list.push(e2);
        }else{
            new_equation_list.push(equation);
        }
    }
    file.equations = new_equation_list;
    preprocessing_all_ge(&mut file);
    file
}

fn preprocessing_all_ge(file: &mut File){
    for mut eq in &mut file.equations {
        if eq.symbol == EquationSymbol::le {
            for l in &mut eq.e1.literals {
                match l {
                    EquationPart::Literal { factor,..} => {
                        *factor *= -1;
                    }
                    EquationPart::Factor(f) => {
                        *f *= -1;
                    }
                }
            }
            for l in &mut eq.e2.literals {
                match l {
                    EquationPart::Literal { factor,..} => {
                        *factor *= -1;
                    }
                    EquationPart::Factor(f) => {
                        *f *= -1;
                    }
                }
            }
            eq.symbol = EquationSymbol::ge;
        }
    }
}

fn preprocessing_literals_left_numbers_right(equation: &mut Equation){
    let mut lit_vec = Vec::new();
    let mut num_vec = Vec::new();

    for item in &equation.e1.literals {
        match item {
            EquationPart::Literal{
                factor,
                name
            } => {
                lit_vec.push(Literal {
                    factor: *factor,
                    name: *name})
            },
            EquationPart::Factor(i) => {
                num_vec.push(Factor(*i * -1))
            }
        }
    }
    for item in &equation.e2.literals {
        match item {
            EquationPart::Literal{
                factor,
                name
            } => {
                lit_vec.push(Literal {
                    factor: *factor * -1,
                    name: *name})
            },
            EquationPart::Factor(i) => {
                num_vec.push(Factor(*i))
            }
        }
    }

    equation.e1.literals = lit_vec;
    equation.e2.literals = num_vec;
}

fn preprocessing_add_up_same(equation: &mut Equation){
    let mut hash_map: HashMap<u32, i32> = HashMap::new();
    let mut factors = 0;
    for l in &equation.e1.literals {
        match l {
            EquationPart::Factor(f) => {
                factors += f;
            }
            EquationPart::Literal {factor, name} => {
                let res = hash_map.get(&name);
                match res {
                    None => {
                        hash_map.insert(*name, *factor);
                    }
                    Some(v) => {
                        hash_map.insert(*name, v + factor);
                    }
                }
            }
        }
    }
    for l in &equation.e2.literals {
        match l {
            EquationPart::Factor(f) => {
                factors += f;
            }
            EquationPart::Literal {factor, name} => {
                let res = hash_map.get(&name);
                match res {
                    None => {
                        hash_map.insert(*name, *factor);
                    }
                    Some(v) => {
                        hash_map.insert(*name, v + factor);
                    }
                }
            }
        }
    }
    equation.e2.literals = Vec::from([Factor(factors)]);
    let mut output: Vec<EquationPart> = Vec::new();
    for (i, f) in hash_map {
        if f == 0 {
            continue;
        }
        output.push(EquationPart::Literal {
            name: i,
            factor: f
        });
    }
    equation.e1.literals = output;
}