#![allow(warnings)]
#![allow(unused)]

use std::{fmt::Debug, u128};
use rand::{rngs::ThreadRng, Error, Rng};
use graph::draw_graph::draw;
use equation::solver::{parse_equation, solve};
use std::io::{stdin , stdout , Write};


mod graph;
mod equation;

type VC = Vec<Chromosome>;

trait GeneticAlg {
    fn populate(&self) -> VC;
    fn evaluation(&self, pop : &mut VC , equation : &str) -> Box<[u64]>;
    fn selection(&self, f_obj: &[u64] , pop : &mut VC, rnd : &mut ThreadRng);
    fn crossover(&self, pop : &mut VC ,rnd : &mut ThreadRng);
    fn mutation(&self, pop : &mut VC ,rnd : &mut ThreadRng);
    fn avg_score_coordinate(&self , iter : u64, score : &[u64] ) -> (f32 , f32);
}

#[derive(Clone)]
struct Chromosome {
    genes : Box<[i32]> ,
}

#[derive(Debug)]
struct Genetic {
    n_iter : u64 , 
    n_bits : u64 ,
    n_pop : u64 ,
    r_cross : f32 ,
    r_mut : f32 ,
    target : i32
}

impl std::fmt::Debug for Chromosome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.genes)
    }
}

impl Chromosome {
    fn construct(n_bits: u64 , limit : i32) -> Chromosome {
        Chromosome {
            genes : {
                let mut rng = rand::thread_rng();
                (0..n_bits).map(|_| rng.gen_range(0..=limit))
                .collect()
            }    
        }
    }

    fn countbits_from(equation : &str) -> u64 {
        let mut n_bits = 0 ;
        for char in equation.chars().into_iter() {
            if char.is_alphabetic() {
                n_bits += 1;
            }
        }
        n_bits
    }
}

impl Genetic {
    fn new(ni : u64 , nb : u64 , np : u64 , rc : f32 , rm : f32 , target : i32 ) -> Genetic {
        Genetic {
            n_iter : ni ,
            n_bits : nb ,
            n_pop : np ,
            r_cross : rc ,
            r_mut : rm ,
            target , 
        }
    }
}

impl GeneticAlg for Genetic {
    fn populate(&self) -> VC {
        let population = (0..self.n_pop)
            .map(|_| Chromosome::construct(self.n_bits , self.target))
            .collect();
        population
    }

    fn evaluation(&self, pop : &mut VC, equation: &str) -> Box<[u64]> {
        pop
            .iter()
            .map(|chromosome| (solve(&*parse_equation(equation , &*chromosome.genes))
            .unwrap() - self.target).abs() as u64 ) 
            .collect::<Vec<_>>().into_boxed_slice()
    }
    
    
    fn selection(&self, f_obj : &[u64], pop : &mut VC, rnd : &mut ThreadRng) {
        let total_fitness: f32 = f_obj
            .iter()
            .map(|&f_value| 1.0 / (f_value as f32 + 1.0))
            .sum();
        
        let mut cumulative_p = vec![];
        let mut new_pop = vec![];

        let probabilities : Vec<f32> = f_obj
            .iter()
            .map(|&f_value| (1.0 / (f_value as f32 + 1.0)) / total_fitness)
            .collect();
        
        for index in 0..pop.len() {
            let sum: f32 = probabilities
                .iter()
                .take(index + 1)
                .sum();

            cumulative_p.push(sum);
        }

        for _ in 0..pop.len(){
            let r = rnd.gen_range(0.0..1.0);
            
            for (index_c , &value) in cumulative_p.iter().enumerate() {
                if r < value {
                    new_pop.push(pop[index_c].clone());
                    break;
                }
            }
        }
        
        *pop = new_pop
    }

    fn crossover(&self, pop : &mut VC ,rnd : &mut ThreadRng) {
        let mut parents : Vec<_> = vec![]; 

        for index in 0..pop.len() {
            let r = rnd.gen_range(0.0..1.0);  
            if r < self.r_cross {
                parents.push(pop[index].clone());
            }
        }

        for index in 0..parents.len() {
            let r = rnd.gen_range(0..self.n_bits) as usize; 
            
            match (parents.get(index) , parents.get(index + 1)) {
                (Some(a) , Some(b)) => {
                    let parent1 = &a.genes[..r];
                    let parent2 = &b.genes[r..];

                    pop[index].genes = [parent1 , parent2]
                    .concat()
                    .into();
                }

                (Some(a) , None) => {
                    let parent1 = &a.genes[..r];
                    let parent2 = &parents[0].genes[r..];
                    
                    pop[index].genes = [parent1 , parent2]
                    .concat()
                    .into();
                }
                _ => {}
            }
        }
    }

    fn mutation(&self, pop : &mut VC ,rnd : &mut ThreadRng) {
        let n_bits = self.n_bits as usize;
        let total_genes = pop.len() * n_bits;    
        let mutation_number = (total_genes as f32 * self.r_mut) as u64;

        for _ in 0..mutation_number {
            let r = rnd.gen_range(0..total_genes);
            pop[r / n_bits].genes[r % n_bits] = rnd.gen_range(0..self.target);
        }   
    }

    fn avg_score_coordinate(&self, iter : u64 , score : &[u64]) -> (f32 , f32) {
        let sum : u64 = score.iter().map(|&x| x as u64).sum();
        let avg_score : u64 = sum / score.len() as u64;

        (iter as f32 , avg_score as f32) 
    }
}

fn main() -> Result<() , Error> {
    // equation string
    let mut equation = String::new();

    // target (lhs of equation)
    let mut input_target = String::new();

    // read equation from input
    print!("Equation > ");

    stdout()
    .flush()
    .expect("Failed to flush stdout");

    stdin()
    .read_line(&mut equation)
    .expect("Failed read equation !");
    
    // remove any additional characters
    equation = equation
    .trim()
    .to_string();

    // read target from input 
    print!("Equals to > ");

    stdout()
    .flush()
    .expect("Failed to flush stdout");

    stdin()
    .read_line(&mut input_target)
    .expect("Failed to read target !");

    // parsing target to integer
    let target = input_target
    .trim()
    .parse()
    .expect("Failed to parse input target !");

    // number of bits
    let nb = Chromosome::countbits_from(&equation); 
    
    // number of iterations 
    let ni = 100; 
    
    // population number 
    let np = 20; 
    
    // crossover rate 
    let rc = 0.25; 

    // mutation rate
    let rm = 0.05; 

    // random number generator
    let mut rnd = rand::thread_rng(); 

    // making initial instances
    let genetic : Genetic = Genetic::new(ni, nb, np, rc, rm , target); 

    // population
    let mut pop = genetic.populate();

    // calculate initial f_obj 
    let mut f_obj = genetic.evaluation(&mut pop , equation.as_str());

    // average score of each generation
    let mut coordinates : Vec<(f32 , f32)> = Vec::new();

    // iterating to make new generations
    for iter in 1..=genetic.n_iter {
        
        // roulette selection 
        genetic.selection(&f_obj , &mut pop , &mut rnd);
        
        // crossover process 
        genetic.crossover(&mut pop , &mut rnd);
        
        // mutation process 
        genetic.mutation(&mut pop , &mut rnd);
        
        // update f_obj for current generation
        f_obj = genetic.evaluation(&mut pop , equation.as_str());
        
        // calculate the coordination on graph
        coordinates.push(genetic.avg_score_coordinate(iter , &*f_obj));

        // also print results on console
        println!("iteration : {} f_obj : {:?} population: {:?}", iter, f_obj, pop);        

        // find the answer and break (if available)
        if f_obj.iter().any(|n| *n == 0) {
            break;
        }
    }

    // illustrate the coordinations on graph
    let _ = draw(coordinates);

    Ok(())
}