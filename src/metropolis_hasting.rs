/* 
1. Provide a function to sample from.
2. Define a proposal or transient Kernel.
3. Apply the Metropolis-Hastings algorithm, by finding 'acceptance probability' for each sample and moving the Markov chain. 
*/

/* TODO */
// 1. Use Array instead of Vec for faster processing
// 2. Prevent the use of RefCell for x_curr, to explicitely mention outside the struct
// 3. 'lag' yet to be implemented!! and understood as well!!


use rand_distr::{Normal, Uniform, Distribution};
use std::cell::RefCell;


pub struct MetropolisHastingSampling<'a>{
    pub x_init: Vec<f32>,
    pub x_curr: RefCell<Vec<f32>>,  // RefCell allows interior mutability, so that I dont have to make my struct mutable
    kernel: Option<String>,
    f: &'a dyn Fn(f32)->f32,
}

fn get_x_next(x_proposed_val: f32, x_t_val: f32, prob: f32)->(f32, bool){
    if Uniform::from(0.0..1.0).sample(& mut rand::thread_rng()) < prob{
        return (x_proposed_val, true);
    }else{
        return (x_t_val, false);
    }
}

impl<'a> MetropolisHastingSampling<'a> {

    pub fn new(x_init: &Vec<f32>, kernel: Option<String>, f: &'a dyn Fn(f32)->f32)->MetropolisHastingSampling<'a>{
        return MetropolisHastingSampling{
            x_init: x_init.clone(),
            x_curr: RefCell::new(x_init.clone()),
            kernel,
            f,
        };
    }

    fn sample_gaussian(&self, sigma: Option<f32>)-> Vec<f32>{

        let sigma = match sigma{
            Some(s) => s,
            None => 1.0,
        };

        // Initialized a Standard Normal Distribution
        let normal: Normal<f32> = Normal::new(0.0, sigma).unwrap();
        // Sampled from the distribution
        let samples: Vec<f32> = (0..self.x_curr.borrow().len())// borrow as non-mutable
                                .map(|_| normal.sample(&mut rand::thread_rng()))
                                .collect();
        // Added the samples to x_t
        let mut x_proposed: Vec<f32> = Vec::new();
        for i in 0..self.x_curr.borrow().len(){
            x_proposed.push(self.x_curr.borrow()[i] + samples[i]);
        }
        let x_proposed = x_proposed; // Made the x_proposed non_mutable

        return x_proposed;
    }

    fn get_proposal(&self, sigma: Option<f32>)->Vec<f32>{ // x_t is a Non-Mutable Vector Reference for borrowing

        /* Sample proposal from given kernel and x_t */

        let  kernel_type: String;

        match &self.kernel{ // Borrowing here otherwise value gets moved to 'k'
            Some(k) => kernel_type = k.to_string(), // Used to_string because k is a reference
            None => kernel_type = String::from("gaussian"),
        }   
        if kernel_type == String::from("gaussian"){
            return self.sample_gaussian(sigma);
        }
        else {
            unimplemented!("Kernel: {} Not Implemented", kernel_type);
        }
    }

    fn get_acceptance_probability(&self, x_proposed: &Vec<f32>)->Vec<f32> {

        let mut p: Vec<f32> = vec![1.0; self.x_curr.borrow().len()];
        for i in 0..self.x_curr.borrow().len(){
            // Since f32 can represent NaN, we use f32::min & f32::EPSILON to prevent division by zero
            p[i] = f32::min(p[i], (self.f)(x_proposed[i])/((self.f)(self.x_curr.borrow()[i]) + f32::EPSILON)); 
            
        }//  In Rust, when accessing a function pointer or a reference to a function, you need to use parentheses to call it, as it's treated like a value.

        return p;
    }

    // println!("Kernel: {}", distribution);
    // println!("Precision: {}", f32::EPSILON);

    pub fn step(&self, sigma: Option<f32>)->(Vec<f32>, Vec<bool>){
        
        let x_proposed = self.get_proposal(sigma);
        let prob = self.get_acceptance_probability(&x_proposed);
        let (x_step, acceptance): (Vec<f32>, Vec<bool>) = (0..self.x_curr.borrow().len())
                                        .map(|i| get_x_next(x_proposed[i], self.x_curr.borrow()[i], prob[i]))
                                        .unzip();

        *self.x_curr.borrow_mut() = x_step;
        return (x_proposed, acceptance);
    }

    pub fn sample(&self, num_iter: i32, sigma: Option<f32>, burnin: Option<i32>, lag: Option<i32>){

        let burnin = match burnin{
            Some(b) => b,
            None => (0.1 * num_iter as f32) as i32,
        };
        let _lag = match lag{
            Some(l) => l,
            None => 1,
        };

        // Warmup
        for _ in 0..burnin{
            (_, _) = self.step(sigma); // borrow as mutable
        }
        
        // Iteration
        for _ in 0..num_iter{
            // let (x_proposed, new_vec, acceptance) = metropolis_hasting::step(&vec, None, &target);
            (_, _) = self.step(sigma); // borrow as mutable
        }
    }

}

