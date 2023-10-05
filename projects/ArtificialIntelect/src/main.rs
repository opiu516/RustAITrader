mod layer;
mod node;
mod neural_network;

fn simple_activation_function(number:f32) -> f32{
    let mut res:f32 = 0.0;
    if number > 0.0 {
        res = 1.0;
    }
    res
}

fn main() {
    let network = neural_network::NeuralNetwork::new(10,vec![5,5,4,3],simple_activation_function,|vec: Vec<f32>| -> Vec<f32> { vec });
    network.display();
    //println!("res = {:?}",layer.calculate(vec![2.0,1.0]));
}
