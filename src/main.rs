// use std::io;
mod metropolis_hasting;


fn target(x: f32) -> f32 {

    // f(x) = exp(-x^2) * (2 + sin(5x) + sin(2x))

    return x.powf(-x*x) * (2.0 + (5.0*x)).sin() + (2.0*x).sin();
}

fn main() {
    let mut vec:Vec<f32> = vec![0.0; 10000]; // five elements of type f32

    // let x_proposed = metropolis_hasting::get_proposal(&vec, None);
    
    
    let num_iter = 1000;
    let burnin = (0.1 * num_iter as f32) as i32;


    for _ in 0..burnin{
        (_, vec, _) = metropolis_hasting::step(&vec, None, &target);
    }

    for _ in 0..num_iter{
        // let (x_proposed, new_vec, acceptance) = metropolis_hasting::step(&vec, None, &target);
        (_, vec, _) = metropolis_hasting::step(&vec, None, &target);
    }
    
    print!("x_step: {:?}", vec);
}
