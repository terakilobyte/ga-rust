use clap::{App, Arg};
use rand::prelude::*;

mod agent;
use agent::Agent;

fn main() {
    let matches = App::new("ga-rust")
    .version("0.1")
    .author("Nathan Leniz")
    .about("Genetic algorithm to find an input word")
    .arg(Arg::with_name("target")
    .short("t")
    .long("target")
    .help("The target word to evolve towards")
    .takes_value(true)
    )
    .get_matches();
    let target = matches.value_of("target")
                        .unwrap_or("super secret password");
    let mut population = (0..100).map(|_| Agent::new(target.to_string()))
                                 .collect::<Vec<Agent>>();

    let mut best = population[0].clone();
    let mut gens = 0;
    while best.alleles.iter().collect::<String>() != target {
        population.sort();
        best = population[0].clone();
        let worst = population[population.len() - 1].clone();
        println!("BEST: {}, Score: {}, Worst: {}, Score: {}, GEN: {}",
                 best,
                 best.fitness(),
                 worst,
                 worst.fitness(),
                 gens);
        population = next_gen(population);
        gens += 1;
    }
    println!("Best: {}, Score: {}, Gens: {}",
             best,
             best.fitness(),
             gens - 1);
}

fn next_gen(mut population: Vec<Agent>) -> Vec<Agent> {
    population.sort();
    let mut next_pop: Vec<Agent> = Vec::with_capacity(100);
    let mut rng = rand::thread_rng();
    for i in 0..10 {
        let p1 = &population[i * 2];
        let p2 = &population[i + 1];
        for _ in 0..5 {
            next_pop.push(p1.breed(&p2));
        }
    }
    for _ in 0..10 {
        let p1 = &population[rng.gen_range(0, population.len())];
        let p2 = &population[rng.gen_range(0, population.len())];
        for _ in 0..5 {
            next_pop.push(p1.breed(&p2));
        }
    }
    next_pop
}
