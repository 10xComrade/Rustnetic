use std::thread::Thread;
use rand::{Rng , rngs::ThreadRng};

trait GeneticAlg {
    fn populate(&self) -> Vec<Vec<i32>>;
    fn evaluation(&self , a: i32 , b: i32 , c: i32 , d:i32) -> i32;
    fn selection(&self, p : &mut Vec<Vec<i32>>, fitness: &[i32], rnd : &mut ThreadRng);
    fn crossover(&self, p : &mut Vec<Vec<i32>>, rc : f32, rnd : &mut ThreadRng);
    fn mutation(&self , p : &mut Vec<Vec<i32>>, rm : f32 , rnd : &mut ThreadRng);
}

#[derive(Debug)]
struct Chromosome {
    genes: Vec<i32>,
}

#[derive(Debug)]
struct Genetic {
    n_iter : i32 , 
    n_bits : u16 ,
    n_pop : i32 ,
    r_cross : f32 ,
    r_mut : f32 ,
}

impl Chromosome {
    fn gen(n_bits: u16) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let genes: Vec<i32> = (0..n_bits).map(|_| rng.gen_range(0..30)).collect();
        genes 
    }
}

impl Genetic {
    fn new(ni : i32 , nb : u16 , np : i32 , rc : f32 , rm : f32 ) -> Genetic {
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
    fn populate(&self) -> Vec<Vec<i32>> {
        (0..self.n_pop).map(|_| Chromosome::gen(self.n_bits)).collect()
    }

    fn evaluation(&self , a: i32 , b: i32 , c: i32 , d:i32) -> i32{
        (a + 2 * b + 3 * c + 4 * d - 30).abs()
    }
    
    fn selection(&self, p : &mut Vec<Vec<i32>>, fitness: &[i32], rnd : &mut ThreadRng) {
        let total_fitness: f32 = fitness
            .iter()
            .map(|&item| 1.0 / (item as f32 + 1.0))
            .sum();
        
        let mut cumulative = vec![];
        let mut new_pop = vec![];

        let probabilities: Vec<f32> = fitness
            .iter()
            .map(|&f_obj| (1.0 / (1.0 + f_obj as f32)) / total_fitness)
            .collect();
        
        for index in 0..probabilities.len() {
            let sum: f32 = probabilities
                .iter()
                .take(index + 1)
                .sum();

            cumulative.push(sum);
        }

        for index_p in 0..p.len(){
            let r = rnd.gen_range(0.0..1.0);
            
            for (index_c , &value) in cumulative.iter().enumerate() {
                if r < value {
                    new_pop.push(p[index_c].clone());
                    break;
                }
            }
        }

        *p = new_pop; 
    }

    fn crossover(&self, p : &mut Vec<Vec<i32>>, rc : f32, rnd : &mut ThreadRng) {
        let mut parents : Vec<_> = vec![]; 

        for index in 0..p.len() {
            let r = rnd.gen_range(0.0..1.0);  
            
            if r < rc {
                parents.push(p[index].clone());
            }
        }

        for index in 0..parents.len() {
            let r = rnd.gen_range(0..parents[index].len()) as usize; // changed 1 to zero ( in rand )
            
            match (parents.get(index) , parents.get(index + 1)) {
                (Some(a) , Some(b)) => {
                    let crossover_point1 = &a[..r];
                    let crossover_point2 = &b[r..];

                    p[index] = [crossover_point1 , crossover_point2].concat();
                }

                (Some(a) , None) => {
                    let crossover_point1 = &a[..r];
                    let crossover_point2 = &parents[0][r..];
                    
                    p[index] = [crossover_point1 , crossover_point2].concat();
                }

                _ => {}
            }
        }
    }

    fn mutation(&self , p : &mut Vec<Vec<i32>>, rm : f32 , rnd : &mut ThreadRng) {
        let n_bits = self.n_bits as usize;
        let total_genes = p.len() * n_bits;    
        let mutation_number = (total_genes as f32 * rm) as u16;

        for _ in 0..mutation_number {
            let r = rnd.gen_range(0..total_genes);
            p[r / n_bits][r % n_bits] = rnd.gen_range(0..30);
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

    // making an instance of genetic
    let mut genetic : Genetic = Genetic::new(ni, nb, np, rc, rm); 

    // population
    let mut pop = genetic.populate();

    // iterating to make new generations
    for iter in 1..=ni {

        // evaluation 
        let fitness : Vec<_> = pop.iter().map(|chromosome| {
            if let &[a, b, c, d] = chromosome.as_slice() {
                genetic.evaluation(a, b, c, d)
            } else {
                panic!("Invalid chromosome format");
            }
        }).collect();

        // roulette selection 
        genetic.selection(&mut pop , &fitness , &mut rnd);

        // crossover process 
        genetic.crossover(&mut pop, rc, &mut rnd);

        // mutation process 
        genetic.mutation(&mut pop , rm, &mut rnd);

        // final output
        println!("iteration : {} fitness : {:?} population: {:?}" , iter , fitness , pop);

    }
}