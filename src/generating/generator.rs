use bimap::{BiHashMap, BiMap};
use rand::Rng;
use crate::parsing::equation_ds::{Equation, EquationPart, EquationSide, EquationSymbol, File};
use crate::parsing::parser::parse;

pub fn generate_pb_formula(number_equations: u32, number_variables: u32, size_equation: u32, max_factor: i32) -> File {
    let mut file = File{
        equations: Vec::new(),
        name_map: BiHashMap::new(),
        name_index: 0
    };

    for i in (1..=number_equations) {
        file.equations.push(generate_random_equation(number_variables, size_equation, max_factor, &mut file.name_map));
    }

    parse(&file.to_string())
}

fn generate_random_equation(number_variables: u32, size_equation: u32, max_factor: i32, name_map: &mut BiMap<String, u32>) -> Equation {
    let mut random_equation_side_1 = EquationSide{
        literals: Vec::new()
    };
    let mut random_equation_side_2 = EquationSide{
        literals: Vec::new()
    };
    random_equation_side_2.literals.push(EquationPart::Factor(1));
    //random_equation_side_2.literals.push(generate_random_rhs(size_equation as i32, max_factor));
    for i in (1..=size_equation){
        random_equation_side_1.literals.push(generate_random_literal(number_variables, max_factor, name_map));
    }

    Equation {
        symbol: EquationSymbol::ge,
        e1: random_equation_side_1,
        e2: random_equation_side_2
    }
}

fn generate_random_rhs(size_equation: i32, max_factor: i32) -> EquationPart {
    let random_number = rand::thread_rng().gen_range(0..=size_equation/2*max_factor);
    EquationPart::Factor(random_number as i32)
}

fn generate_random_equation_symbol() -> EquationSymbol {
    let random_number = rand::thread_rng().gen_range(0..=1);
    if random_number == 0 {
        EquationSymbol::eq
    }else{
        EquationSymbol::ge
    }
}

fn generate_random_literal(number_variables: u32, max_factor: i32, name_map: &mut BiMap<String, u32>) -> EquationPart {
    let random_variable_index = rand::thread_rng().gen_range(1..=number_variables);
    let random_factor = 1; // rand::thread_rng().gen_range(1..=max_factor);
    let name = format!("x_{}", random_variable_index);
    name_map.insert(name, random_variable_index);
    EquationPart::Literal {
        factor: random_factor,
        name: random_variable_index
    }
}