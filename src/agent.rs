use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use std::cmp::Ordering;
// use std::time::Instant;

#[derive(Debug, Eq, Clone)]
pub struct Agent {
    pub alleles: Vec<char>,
    password: String,
}

impl Ord for Agent {
    // negative sort
    fn cmp(&self, other: &Self) -> Ordering {
        other.fitness().cmp(&self.fitness())
    }
}

impl PartialOrd for Agent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Agent {
    fn eq(&self, other: &Self) -> bool {
        self.fitness() == other.fitness()
    }
}

impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.alleles.iter().collect::<String>())
    }
}

impl Agent {
    pub fn new(password: String) -> Self {
        let alleles: Vec<char> = (0..password.len()).map(|_| {
                                                        let val: RandomChar =
                            StdRng::from_entropy().sample(Standard);
                                                        val.letter
                                                    })
                                                    .collect();
        Agent { alleles, password }
    }

    pub fn breed(&self, other: &Self) -> Self {
        let mut rng = rand::thread_rng();
        let mut alleles: Vec<char> = Vec::new();
        for i in 0..self.alleles.len() {
            if 0.5 < rng.gen() {
                alleles.push(self.alleles[i]);
            } else {
                alleles.push(other.alleles[i]);
            }
        }
        let mut child = Agent { alleles,
                                password: self.password.clone() };
        child.mutate(&mut rng);
        child
    }

    fn mutate(&mut self, rng: &mut ThreadRng) {
        if rng.gen::<f64>() * 100.0 < std::f64::consts::E {
            let idx = rng.gen_range(0, self.alleles.len());
            let val: RandomChar = StdRng::from_entropy().sample(Standard);
            self.alleles[idx] = val.letter;
        }
    }

    pub fn fitness(&self) -> usize {
        if self.password.len() != self.alleles.len() {
            eprintln!("passwords of incorrect length!!");
            return 0;
        }
        /*
         * Real fitness
         */
        let mut score = 0;
        self.password
            .chars()
            .zip(self.alleles.iter())
            .for_each(|(a, b)| {
                if a == *b {
                    score += 1;
                }
            });
        score * 10000 / self.password.len()

        /*
         * timing fitness
        let now = Instant::now();
        for i in 0..self.alleles.len() {
            if self.alleles[i] != self.password.chars().nth(i).unwrap_or('S') {
                break;
            }
        }
        now.elapsed().as_nanos()
        */
    }
}

#[derive(Debug)]
struct RandomChar {
    letter: char,
}

impl Distribution<RandomChar> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RandomChar {
        let letters: Vec<char> =
            " abcdefghijklmnopqrstuvwxyz".chars().collect();
        RandomChar { letter: letters[rng.gen_range(0, letters.len())] }
    }
}
