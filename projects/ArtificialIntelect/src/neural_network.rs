use crate::layer::Layer;

pub struct NeuralNetwork{
    hidden_layers: Vec<Layer>,
    out_activation_function:fn(Vec<f32>)->Vec<f32>
}

impl NeuralNetwork{
    pub fn new(number_of_inputs:u32, hidden_layers: Vec<u32>, activation_function: fn(f32)->f32,
                out_activation_function:fn(Vec<f32>)->Vec<f32>)-> NeuralNetwork{
        let mut hidden_layers_vector: Vec<Layer> = Vec::new();
        if hidden_layers.len() as u32 > 1{
            hidden_layers_vector.push(Layer::new(number_of_inputs,hidden_layers[0],activation_function));
            for i in 1..hidden_layers.len(){
                let layer: Layer;
                if i == hidden_layers.len() - 1{
                    layer = Layer::new(hidden_layers[i-1],hidden_layers[i],|number:f32|->f32{number});
                }else{
                    layer = Layer::new(hidden_layers[i-1],hidden_layers[i],activation_function);
                }
                hidden_layers_vector.push(layer);
            }
        }
        NeuralNetwork{hidden_layers:hidden_layers_vector,out_activation_function}
    }

    pub fn display(&self){
        for i in 0..self.hidden_layers.len(){
            println!("Layer - {}",i);
            self.hidden_layers[i].display();
        }
    }

    pub fn calculate(&self,input: Vec<f32>) -> Vec<f32>{
        let mut res:Vec<f32> = input;
        for i in 0..self.hidden_layers.len(){
            println!("Layer - {}",i +1);
            res = self.hidden_layers[i].calculate(&res);
        }
        (self.out_activation_function)(res)
    }
}