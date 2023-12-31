use rand::Rng; 

const RANDOM_MAX: i16 = 1000;

pub struct Node{
    weights: Vec<f32>,
    activation_function: fn(f32)->f32
}

impl Node{
    pub fn new( number_of_inputs: u32, activation_function: fn(f32)->f32) -> Node{
        let mut weights = Vec::new();
        for _i in 0..number_of_inputs{
            let rand_number = (rand::thread_rng().gen_range(-RANDOM_MAX..RANDOM_MAX) as f32)/(RANDOM_MAX as f32);
            weights.push(rand_number);
        }
        Node{weights,activation_function}
    }

    pub fn display(&self){
        for i in 0..self.weights.len(){
            println!("      weight - {} = {}", i,self.weights[i]);
        }
    }

    pub fn calculate(&self,input: &Vec<f32>)->f32{
        let mut res: f32 = 0 as f32;
        if input.len() == self.weights.len(){
            for i in 0..self.weights.len(){
                res += input[i]*self.weights[i];
            }
        }
        (self.activation_function)(res)   
    }
}

