use rand::Rng; 

const RANDOM_MAX: i16 = 1000;

pub struct Node{
    number_of_inputs: u32,
    weights: Vec<f32>,
}

impl Node{
    pub fn display(&self){
        for i in self.weights.iter(){
            println!("{}", i);
        }
    }
}

pub fn create_node(number_of_inputs: u32) -> Node{
    let weights = Vec::new();
    let mut res = Node{number_of_inputs,weights};
    for _i in 0..res.number_of_inputs{
        let rand_number = (rand::thread_rng().gen_range(-RANDOM_MAX..RANDOM_MAX) as f32)/(RANDOM_MAX as f32);
        res.weights.push(rand_number);
    }
    res
}

