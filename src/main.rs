use std::fmt::Debug;
use rand::{Rng , rngs::ThreadRng};


trait GeneticAlg {
    fn populate(&self) -> Vec<Chromosome>;
    fn evaluation(&self , pop : &Vec<Chromosome>) -> Box<[u16]>;
    fn selection(&self, f_obj: &[u16] , pop : &mut Vec<Chromosome>, rnd : &mut ThreadRng);
    fn crossover(&self, pop : &mut Vec<Chromosome> ,rnd : &mut ThreadRng);
    fn mutation(&self, pop : &mut Vec<Chromosome> ,rnd : &mut ThreadRng);
}

#[derive(Clone)]
struct Chromosome {
    genes : Box<[i32]> ,
}

#[derive(Debug)]
struct Genetic {
    n_iter : u16 , 
    n_bits : u16 ,
    n_pop : u16 ,
    r_cross : f32 ,
    r_mut : f32 ,
}

impl std::fmt::Debug for Chromosome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.genes)
    }
}

impl Chromosome {
    fn construct(n_bits: u16) -> Chromosome {
        Chromosome {
            genes : {
                let mut rng = rand::thread_rng();
                (0..n_bits).map(|_| rng.gen_range(0..30)).collect() 
            }    
        }
    }
}

impl Genetic {
    fn new(ni : u16 , nb : u16 , np : u16 , rc : f32 , rm : f32 ) -> Genetic {
        Genetic {
            n_iter : ni ,
            n_bits : nb ,
            n_pop : np ,
            r_cross : rc ,
            r_mut : rm ,
        }
    }
}

impl GeneticAlg for Genetic {
    fn populate(&self) -> Vec<Chromosome>{
        let population = (0..self.n_pop)
            .map(|_| Chromosome::construct(self.n_bits))
            .collect();
        population
    }

    fn evaluation(&self , pop : &Vec<Chromosome>) -> Box<[u16]> {
        pop.iter().map(|chromosome| {
            if let &[a, b, c, d] = &*chromosome.genes {
                (a + 2 * b + 3 * c + 4 * d - 30).abs() as u16
            } else {
                panic!("Invalid chromosome format");
            }
        }).collect::<Vec<_>>().into_boxed_slice()
    }
    
    fn selection(&self, f_obj : &[u16], pop : &mut Vec<Chromosome>, rnd : &mut ThreadRng) {
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

    fn crossover(&self, pop : &mut Vec<Chromosome> ,rnd : &mut ThreadRng) {
        let mut parents : Vec<_> = vec![]; 

        for index in 0..pop.len() {
            let r = rnd.gen_range(0.0..1.0);  
            
            if r < self.r_cross {
                parents.push(pop[index].clone());
            }
        }

        for index in 0..parents.len() {
            let r = rnd.gen_range(0..self.n_bits) as usize; // changed 1 to zero ( in rand )
            
            match (parents.get(index) , parents.get(index + 1)) {
                (Some(a) , Some(b)) => {
                    let crossover_point1 = &a.genes[..r];
                    let crossover_point2 = &b.genes[r..];

                    pop[index].genes = [crossover_point1 , crossover_point2]
                        .concat()
                        .into();
                }

                (Some(a) , None) => {
                    let crossover_point1 = &a.genes[..r];
                    let crossover_point2 = &parents[0].genes[r..];
                    
                    pop[index].genes = [crossover_point1 , crossover_point2]
                        .concat()
                        .into();
                }
                _ => {}
            }
        }
    }

    fn mutation(&self, pop : &mut Vec<Chromosome> ,rnd : &mut ThreadRng) {
        let n_bits = self.n_bits as usize;
        let total_genes = pop.len() * n_bits;    
        let mutation_number = (total_genes as f32 * self.r_mut) as u16;

        for _ in 0..mutation_number {
            let r = rnd.gen_range(0..total_genes);
            pop[r / n_bits].genes[r % n_bits] = rnd.gen_range(0..30);
        }   
    }
}

fn main() {
    // number of iterations 
    let ni = 100; 

    // number of bits
    let nb = 4; 
    
    // population number 
    let np = 6; 
    
    // crossover rate 
    let rc = 0.5; 

    // mutation rate
    let rm = 0.05; 

    // random number generator
    let mut rnd = rand::thread_rng(); 

    // making initial instances
    let genetic : Genetic = Genetic::new(ni, nb, np, rc, rm); 

    // population
    let mut pop = genetic.populate();

    // calculate initial f_obj 
    let mut f_obj = genetic.evaluation(&pop);

    // // iterating to make new generations
    for iter in 1..=genetic.n_iter {
                
        // roulette selection 
        genetic.selection(&f_obj , &mut pop , &mut rnd);
        
        // crossover process 
        genetic.crossover(&mut pop , &mut rnd);
        
        // mutation process 
        genetic.mutation(&mut pop , &mut rnd);
        
        // update f_obj for current generation
        f_obj = genetic.evaluation(&pop);
        
        // show results 
        println!("iteration : {} f_obj : {:?} population: {:?}", iter, f_obj, pop);
        
    }
}