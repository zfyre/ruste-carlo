// use std::io;
mod metropolis_hasting;
use std::fs::File;
use std::io::{Write, BufWriter};

fn target(x: f32) -> f32 {

    // f(x) = exp(-x^2) * (2 + sin(5x) + sin(2x))

    return -(x*x).exp() * (2.0 + (5.0*x).sin() + (2.0*x).sin());
}


fn main() {
    let vec:Vec<f32> = vec![0.0; 10000]; // five elements of type f32

    // let x_proposed = metropolis_hasting::get_proposal(&vec, None);
    
    
    let num_iter = 10000;
    // let burnin = (0.1 * num_iter as f32) as i32;


    // for _ in 0..burnin{
    //     (_, vec, _) = metropolis_hasting::step(&vec, None, &target);
    // }

    // for _ in 0..num_iter{
        // let (x_proposed, new_vec, acceptance) = metropolis_hasting::step(&vec, None, &target);
    //     (_, vec, _) = metropolis_hasting::step(&vec, None, &target);
    // }
    let sampler = metropolis_hasting::MetropolisHastingSampling::new(&vec, None, &target);
    sampler.sample(num_iter, None,None, None);

    // println!("x_init: {:?}", sampler.x_init);
    println!("x_curr: {:?}", sampler.x_curr.borrow());

    let data = sampler.x_curr.borrow().clone();


    let file = File::create("data.bin").expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    for &value in &data {
        writer.write_all(&value.to_le_bytes()).expect("Failed to write data");
    }
    
}
