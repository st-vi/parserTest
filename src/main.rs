use std::collections::HashMap;
use std::fmt::{Write, Display, Formatter};
use std::fs;
use pest::Parser;
use crate::parsing::equation_ds::File;
use crate::solving::solver::count_disconnected_components;

mod parsing {
    pub mod parser;
    pub mod equation_ds;
    pub mod preprocessing;
}
mod solving {
    pub mod pb_ds;
    pub mod solver;
}

mod generating {
    pub mod generator;
}

fn main() {


    //let random_formula = generating::generator::generate_pb_formula(50, 100, 5, 1);
    //println!("{}\n", random_formula.to_pbcount_string());
    //count_with_disconnected_components_on_string(random_formula.to_pbcount_string());
    //run_solver_on_path("/home/stefan/stefan-vill-master/tmp_eval/tmp4.opb");
    //run_solver_on_file(random_formula);



    //let unparsed_file = fs::read_to_string("/home/stefan/stefan-vill-master/tmp_eval/tmp5.opb").expect("cannot read file");
    //let file = parsing::parser::parse(&unparsed_file);
    //println!("{}", file.to_dimacs_string());






    //print_file_as_pbcount_string(file);

    //run_solver_with_bcp("/home/stefan/stefan-vill-master/tmp_eval/tmp6.opb")
    //run_solver_on_path("/home/stefan/stefan-vill-master/tmp_eval/tmp6.opb");



    //run_solver_on_path("/home/stefan/stefan-vill-master/tmp_eval/tmp6.opb");
    count_with_disconnected_components("/home/stefan/stefan-vill-master/tmp_eval/tmp6.opb")

    /*
    let unparsed_file = fs::read_to_string("/home/stefan/stefan-vill-master/tmp_eval/tmp4.opb").expect("cannot read file");
    let file = parsing::parser::parse(&unparsed_file);
    let f = parsing::preprocessing::preprocess_file(file);
    println!("{}", f.to_pbcount_string());

     */

}

fn run_solver_with_bcp(path: &str){
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");
    let file = parsing::parser::parse(&unparsed_file);
    let mut cache_count = HashMap::with_capacity(100_000_000);
    let f = parsing::preprocessing::preprocess_file(file);
    let mut pb_formula = solving::pb_ds::new(&f);
    use std::time::Instant;
    let now = Instant::now();
    let res = solving::solver::count_bcp(&mut pb_formula, 0, 100, &mut cache_count);
    let elapsed = now.elapsed();
    println!("{}\nin {} s", res, elapsed.as_secs());
}

fn count_with_disconnected_components_on_string(content: String) {
    let file = parsing::parser::parse(&content);
    let mut cache_count = HashMap::with_capacity(100_000_000);
    let f = parsing::preprocessing::preprocess_file(file);
    let pb_formula = solving::pb_ds::new(&f);
    use std::time::Instant;
    let now = Instant::now();
    let n = pb_formula.n;
    let res =  count_disconnected_components(pb_formula, 0, 100, &mut cache_count);
    let elapsed = now.elapsed();
    println!("{}\nin {} s", res, elapsed.as_secs());
}

fn count_with_disconnected_components(path: &str) {
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");
    let file = parsing::parser::parse(&unparsed_file);
    let mut cache_count = HashMap::with_capacity(100_000_000);
    let f = parsing::preprocessing::preprocess_file(file);
    let pb_formula = solving::pb_ds::new(&f);
    use std::time::Instant;
    let now = Instant::now();
    let n = pb_formula.n;
    let res =  count_disconnected_components(pb_formula, 0, 100, &mut cache_count);
    let elapsed = now.elapsed();
    println!("{}\nin {} s", res, elapsed.as_secs());
}

fn run_solver_on_path(path: &str){
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");
    //let unparsed_file = fs::read_to_string("test.csv").expect("cannot read file");
    let file = parsing::parser::parse(&unparsed_file);
    run_solver_on_file(file);
}

fn run_solver_on_string(string: &str){
    let file = parsing::parser::parse(string);
    run_solver_on_file(file);
}

fn run_solver_on_file(file: File){
    let mut cache_count = HashMap::with_capacity(100);
    let f = parsing::preprocessing::preprocess_file(file);
    //println!("{}", f.to_pbcount_string());
    let pb_formula = solving::pb_ds::new(&f);
    use std::time::Instant;
    let now = Instant::now();
    let res = solving::solver::count(&pb_formula, pb_formula.n, 0, 100, &mut cache_count);
    let elapsed = now.elapsed();
    println!("{}\nin {} s", res, elapsed.as_secs());
    //println!("cache:\n{:?}", cache_count);
}

fn print_file_as_pbcount_string(file: File){
    println!("{}", file.to_pbcount_string());
}


