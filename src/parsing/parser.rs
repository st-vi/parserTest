use bimap::BiHashMap;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use crate::parsing::equation_ds::*;

pub fn parse(content: &str) -> File {
    let file = TestParser::parse(Rule::file, content)
        .expect("unsuccessful parse")
        .next().unwrap();
    parse_file(file)
}

#[derive(Parser)]
#[grammar = "opb.pest"] // points to the grammar file we created
struct TestParser;


fn parse_file(rule: Pair<Rule>) -> File {
    let mut file = File{
        equations: Vec::new(),
        name_map: BiHashMap::new(),
        name_index: 0
    };
    for inner_rule in rule.into_inner(){
        match inner_rule.as_rule() {
            Rule::equation=> {
                let tmp = parse_equation(inner_rule, &mut file);
                file.equations.push(tmp);
            }
            Rule::EOI => (),
            _ => unreachable!()
        }
    }
    file
}

fn parse_equation(rule: Pair<Rule>, file: &mut File) -> Equation {
    let mut equation_sides = Vec::new();
    let mut equation_symbol = EquationSymbol::eq;
    for inner_rule in rule.into_inner(){
        let tmp = inner_rule.as_rule();
        match inner_rule.as_rule() {
            Rule::equation_side=> {
                equation_sides.push(parse_equation_side(inner_rule, file));
            }
            Rule::equation_symbol => {
                match inner_rule.as_str() {
                    "=" => {
                        equation_symbol = EquationSymbol::eq;
                    }
                    "<=" => {
                        equation_symbol = EquationSymbol::le;
                    }
                    ">=" => {
                        equation_symbol = EquationSymbol::ge;
                    }
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        }
    }
    if equation_sides.len() != 2 {
        panic!()
    }else{
        Equation {
            e2: equation_sides.pop().unwrap(),
            e1: equation_sides.pop().unwrap(),
            symbol: equation_symbol
        }
    }
}

fn parse_equation_side(rule: Pair<Rule>, file: &mut File) -> EquationSide {
    let mut equation_side = EquationSide{
        literals: Vec::new()
    };
    for inner_rule in rule.into_inner(){
        match inner_rule.as_rule() {
            Rule::literal=> {
                equation_side.literals.push(parse_literal(inner_rule, file));
            }
            Rule::first_literal => {
                equation_side.literals.push(parse_literal(inner_rule, file));
            }
            Rule::implicit_one_literal => {
                equation_side.literals.push(parse_literal(inner_rule, file));
            }
            Rule::factor_value => {
                equation_side.literals.push(EquationPart::Factor(inner_rule.as_str().parse().unwrap()));
            }
            Rule::factor => {
                equation_side.literals.push(EquationPart::Factor(parse_factor(inner_rule)));
            }
            _ => unreachable!()
        }
    }
    equation_side
}

fn parse_literal(rule: Pair<Rule>, file: &mut File) -> EquationPart {
    let mut factor: i32 = 1;
    let mut name = "";

    for inner_rule in rule.into_inner(){
        match inner_rule.as_rule() {
            Rule::factor_sign => {
                if inner_rule.as_str().eq("-") {
                    factor = factor * -1;
                }
            }
            Rule::factor_value => {
                let tmp = inner_rule.as_str().trim();
                let tmp_value: i32 = inner_rule.as_str().trim().parse().unwrap();

                factor = factor * tmp_value;
            }
            Rule::var_name => {
                name = inner_rule.as_str();
            }
            Rule::factor => {
                factor = parse_factor(inner_rule);
            }
            _ => unreachable!()
        }
    }

    let map_entry = file.name_map.get_by_left(&name.to_string());
    let mut index: u32;
    match map_entry {
        Some(i) => {
            index = *i;
        },
        None => {
            index = file.name_index;
            file.name_map.insert(name.to_string(), index);
            file.name_index = file.name_index + 1;
        }
    }
    EquationPart::Literal {
        name: index,
        factor,
        pos: true
    }
}

fn parse_factor(rule: Pair<Rule>) -> i32 {
    let mut factor: i32 = 1;

    for inner_rule in rule.into_inner() {
        match inner_rule.as_rule() {
            Rule::factor_sign => {
                if inner_rule.as_str().eq("-") {
                    factor = factor * -1;
                }
            }
            Rule::factor_value => {
                let tmp_value: i32 = inner_rule.as_str().trim().parse().unwrap();
                factor = factor * tmp_value;
            }
            _ => unreachable!()
        }
    }
    factor
}
