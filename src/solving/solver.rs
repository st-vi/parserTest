use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use crate::{parsing, solving};
use crate::parsing::equation_ds::{Equation, File};
use crate::solving::pb_ds::{new, Clause, Literal, PBFormula};

pub fn count(formula: &PBFormula, n: u32, start_progress: u32, end_progress: u32, cache_count: &mut HashMap<u64, u128>) -> u128 {
    let mut formula_cache_count: u32 = 0;
    let mut map_result = cache_count.get(&calculate_hash(&formula));
    match map_result {
        Some(c) => {
            return *c;
        }
        None => {
            if(formula.clauses.len() == 0){
                2_u128.pow(n)
            }else if formula.contains_false_clause() {
                0
            }else{
                //let l = formula.clauses.first().unwrap().literals.first().unwrap();
                let l = get_next_variable(formula);
                let f1 = formula.get_sub_formula(l, true);
                let f2 = formula.get_sub_formula(l, false);

                let progress_mid = start_progress + (end_progress - start_progress) / 2;
                let c1 = count(&f1, n-1, start_progress, progress_mid, cache_count);
                if(end_progress - start_progress >= 1){
                    println!("{progress_mid} %");
                }
                let c2 = count(&f2, n-1, progress_mid, end_progress, cache_count);

                let res = c1 + c2;
                cache_count.insert(calculate_hash(&formula), res);
                return res;
            }
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn count_disconnected_components(pb_formula: PBFormula, n: u32,  start_progress: u32, end_progress: u32, mut cache_count: &mut HashMap<u64, u128>) -> u128 {
    let disconnected_formula = formula_to_disconnected_formula(pb_formula, n);
    //println!("partitions: {}", disconnected_formula.partitions.len());
    let mut res: u128 = 1;
    if disconnected_formula.partitions.len() == 0 {
        return 2_u128.pow(n);
    }
    let progress = end_progress - start_progress;
    let partition_progress = progress / disconnected_formula.partitions.len() as u32;
    let mut i = 0;

    for partition in disconnected_formula.partitions {
        let tmp = count_dc(&partition.formula, partition.formula.n, start_progress + i*partition_progress, start_progress + (i+1)*partition_progress, &mut cache_count);
        i += 1;
        res *= tmp;
    }
    res
}

fn get_next_variable(pbformula: &PBFormula) -> u32 {
    match get_necessary_variable(pbformula){
        Some(t) => return t,
        None => {
            let mut counter: HashMap<u32,u64> = HashMap::new();
            for clause in &pbformula.clauses {
                for literal in &clause.literals {
                    let tmp_res = counter.get(&literal.name);
                    match tmp_res {
                        None => {
                            counter.insert(literal.name, 1);
                        },
                        Some(v) => {
                            counter.insert(literal.name, v + 1);
                        }
                    }
                }
            }
            let mut max_index: u32 = 0;
            let mut max_value: u64 = 0;
            for (k,v) in counter.iter() {
                if v > &max_value {
                    max_value = *v;
                    max_index = *k;
                }
            }
            max_index
        }
    }
}

fn get_next_variable_for_best_partition(pbformula: &PBFormula, n: u32) -> u32 {
    match get_necessary_variable(pbformula){
        Some(t) => return t,
        None => {
            let mut best_index: u32 = u32::MAX;
            let mut best_value: u64 = u64::MAX;

            let mut variables: HashSet<&Literal> = HashSet::new();
            for clause in & pbformula.clauses {
                for literal in & clause.literals {
                    variables.insert(literal);
                }
            }

            for variable in variables {
                let potential_new_formula = pbformula.get_sub_formula(variable.name, true);
                let potential_new_partition = formula_to_disconnected_formula(potential_new_formula, n);

                let mut max_number_clauses = 0;
                let mut max_number_clauses_index = 0;
                for (i, partition) in potential_new_partition.partitions.iter().enumerate() {
                    if partition.formula.clauses.len() > max_number_clauses {
                        max_number_clauses = partition.formula.clauses.len();
                        max_number_clauses_index = i;
                    }
                }
                if max_number_clauses < best_value as usize {
                    best_value = max_number_clauses as u64;
                    best_index = variable.name;
                }

            }

            best_index
        }
    }
}

fn count_dc(formula: &PBFormula, n: u32,  start_progress: u32, end_progress: u32, cache_count: &mut HashMap<u64, u128>) -> u128 {
    let mut formula_cache_count: u32 = 0;
    let mut map_result = cache_count.get(&calculate_hash(&formula));
    match map_result {
        Some(c) => {
            return *c;
        }
        None => {
            if(formula.clauses.len() == 0){
                2_u128.pow(n)
            }else if formula.contains_false_clause() {
                0
            }else{
                //let l = formula.clauses.first().unwrap().literals.first().unwrap().name;
                //let l = get_next_variable_for_best_partition(formula, n);
                let l = get_next_variable(formula);
                let f1 = formula.get_sub_formula(l, true);
                let f2 = formula.get_sub_formula(l, false);
                let n = f1.n;

                let progress_mid = start_progress + (end_progress - start_progress) / 2;
                let c1 = count_disconnected_components(f1, n, start_progress, progress_mid, cache_count);

                if(end_progress - start_progress >= 1){
                    println!("{progress_mid} %");
                }
                let c2 = count_disconnected_components(f2, n, progress_mid, end_progress, cache_count);

                let res = c1 + c2;
                cache_count.insert(calculate_hash(&formula), res);
                return res;
            }
        }
    }
}

fn get_necessary_variable(formula: &PBFormula) -> Option<u32> {
    let mut n = 0;
    let mut new_formula = formula.clone();
    for clause in &formula.clauses {
        if clause.literals.len() == 1 {
            return Some(clause.literals.get(0)?.name);
        }
    }

    None
}

fn formula_to_disconnected_formula(formula: PBFormula, n: u32) -> DisconnectedFormula {
    let mut disconnected_formula = DisconnectedFormula {
        partitions: Vec::new(),
        variables: HashSet::new()
    };
    for clause in formula.clauses {
        if clause.has_variable_overlap(&disconnected_formula.variables) {
            let mut new_partition_set = Vec::new();
            let mut new_partition = Partition{
                variables: HashSet::new(),
                formula: PBFormula{
                    n: 0,
                    clauses: Vec::new()
                }
            };
            for l in &clause.literals {
                new_partition.variables.insert(l.name);
                disconnected_formula.variables.insert(l.name);
            }
            for partition in disconnected_formula.partitions {
                if clause.has_variable_overlap(&partition.variables){
                    for v in partition.variables {
                        new_partition.variables.insert(v);
                    }
                    for e in partition.formula.clauses {
                        new_partition.formula.clauses.push(e);
                    }
                }else{
                    new_partition_set.push(partition);
                }
            }

            new_partition.formula.clauses.push(clause);
            new_partition.formula.n = new_partition.variables.len() as u32;
            new_partition_set.push(new_partition);
            disconnected_formula.partitions = new_partition_set;
        }else{
            let mut new_partition = Partition{
                variables: HashSet::new(),
                formula: PBFormula {
                    n: 0,
                    clauses: Vec::new()
                }
            };
            for l in &clause.literals {
                new_partition.variables.insert(l.name);
                disconnected_formula.variables.insert(l.name);
            }
            new_partition.formula.n = new_partition.variables.len() as u32;
            new_partition.formula.clauses.push(clause.clone());
            disconnected_formula.partitions.push(new_partition);
        }
    }

    let mut sum_n = 0;
    for f in &disconnected_formula.partitions {
        sum_n += f.formula.n;
    }
    let number_unassigned = n - sum_n;

    if number_unassigned > 0 {
        let unassigned_formula = PBFormula{
            n: number_unassigned,
            clauses: Vec::new()
        };
        disconnected_formula.partitions.push(Partition{formula: unassigned_formula, variables: HashSet::new()});
    }

/*
    println!("{} (", disconnected_formula.partitions.len());
    for p in &disconnected_formula.partitions {
        println!("\t{}", p.formula.clauses.len());
    }
    println!(")\n");
*/



    //println!("{}", disconnected_formula.partitions.len());

    disconnected_formula
}

struct Partition {
    formula: PBFormula,
    variables: HashSet<u32>
}

struct DisconnectedFormula {
    partitions: Vec<Partition>,
    variables: HashSet<u32>
}