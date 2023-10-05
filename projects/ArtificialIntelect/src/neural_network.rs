use crate::layer::Layer;

pub struct NeuralNetwork{
    hidden_layers: Vec<Layer>,
    number_of_inputs: u32,
    out_activation_function:fn(Vec<f32>)->Vec<f32>
}

impl NeuralNetwork{
    pub fn new(number_of_inputs:u32, hidden_layers: Vec<u32>, activation_function: fn(f32)->f32,
                out_activation_function:fn(Vec<f32>)->Vec<f32>)-> NeuralNetwork{
        let mut hidden_layers_vector: Vec<Layer> = Vec::new();
        if hidden_layers.len() as u32 > 1{
            hidden_layers_vector.push(Layer::new(number_of_inputs,hidden_layers[0],activation_function));
            for i in 1..hidden_layers.len(){
                let layer = Layer::new(hidden_layers[i-1],hidden_layers[i],activation_function);
                hidden_layers_vector.push(layer);
            }
        }
        NeuralNetwork{hidden_layers:hidden_layers_vector,number_of_inputs,out_activation_function}
    }

    pub fn display(&self){
        for i in 0..self.hidden_layers.len(){
            println!("Layer - {}",i);
            self.hidden_layers[i].display();
        }
    }
}