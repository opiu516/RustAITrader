mod layer;
mod node;
mod neural_network;

use rand::Rng; 

fn simple_activation_function(number:f32) -> f32{
    let mut res:f32 = 0.0;
    if number > 0.0 {
        res = 1.0;
    }
    res
}

fn main() {
    let input = rand::thread_rng().gen_range(1..10);
    let hidden_layers = rand::thread_rng().gen_range(1..50);
    let mut hidden_layers_vector : Vec<u32> = Vec::new();
    let mut inputs : Vec<f32> = Vec::new();
    for _i in 0..hidden_layers{
        hidden_layers_vector.push(rand::thread_rng().gen_range(5..10));
    }
    for _i in 0..input{
        inputs.push((rand::thread_rng().gen_range(-100..100) as f32)/(10 as f32));
    }
    let network = neural_network::NeuralNetwork::new(input,hidden_layers_vector,simple_activation_function,|vec: Vec<f32>| -> Vec<f32> { vec });
    println!("res = {:?}",network.calculate(inputs));
}
