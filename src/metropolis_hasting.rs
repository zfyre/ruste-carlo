/* 
1. Provide a function to sample from.
2. Define a proposal or transient Kernel.
3. Apply the Metropolis-Hastings algorithm, by finding 'acceptance probability' for each sample and moving the Markov chain. 
*/


use rand_distr::{Normal, Uniform, Distribution};


fn sample_gaussian(x_t: &Vec<f32>)-> Vec<f32>{

    // Initialized a Standard Normal Distribution
    let normal: Normal<f32> = Normal::new(0.0, 1.0).unwrap();
    // Sampled from the distribution
    let samples: Vec<f32> = (0..x_t.len())
                            .map(|_| normal.sample(&mut rand::thread_rng()))
                            .collect();
    // Added the samples to x_t
    let mut x_proposed: Vec<f32> = Vec::new();
    for i in 0..x_t.len(){
        x_proposed.push(x_t[i] + samples[i]);
    }
    let x_proposed = x_proposed; // Made the x_proposed non_mutable

    return x_proposed;
}

fn get_proposal(x_t: &Vec<f32>,  kernel: Option<String>)->Vec<f32>{ // x_t is a Non-Mutable Vector Reference for borrowing

    /* Sample proposal from given kernel and x_t */

    let  kernel_type: String;

    match kernel{
        Some(k) => kernel_type = k,
        None => kernel_type = String::from("gaussian"),
    }   
    if kernel_type == String::from("gaussian"){
        return sample_gaussian(&x_t);
    }
    else {
        unimplemented!("Kernel: {} Not Implemented", kernel_type);
    }
}

fn get_acceptance_probability(x_proposed: &Vec<f32>, x_t: &Vec<f32>, f: &dyn Fn(f32)->f32)->Vec<f32> {

    let mut p: Vec<f32> = vec![1.0; x_t.len()];
    for i in 0..x_t.len(){
        // Since f32 can represent NaN, we use f32::min & f32::EPSILON to prevent division by zero
        p[i] = f32::min(p[i], f(x_proposed[i])/(f(x_t[i]) + f32::EPSILON)); 
        
    }

    return p;
}

// println!("Kernel: {}", distribution);
// println!("Precision: {}", f32::EPSILON);

fn get_x_next(x_proposed_val: f32, x_t_val: f32, prob: f32)->(f32, bool){
    if Uniform::from(0.0..1.0).sample(& mut rand::thread_rng()) < prob{
        return (x_proposed_val, true);
    }else{
        return (x_t_val, false);
    }
}

pub fn step(x_t: &Vec<f32>, kernel: Option<String>, f: &dyn Fn(f32)->f32)->(Vec<f32>, Vec<f32>, Vec<bool>){
    
    let x_proposed = get_proposal(&x_t, kernel);
    let prob = get_acceptance_probability(&x_proposed, &x_t, f);
    let (x_step, acceptance): (Vec<f32>, Vec<bool>) = (0..x_t.len())
                                    .map(|i| get_x_next(x_proposed[i], x_t[i], prob[i]))
                                    .unzip();

    return (x_proposed, x_step, acceptance);
}