use crate::node::Node;

pub struct Layer{
    number_of_inputs: u32,
    nodes: Vec<Node>
}

impl Layer{
    pub fn new( number_of_inputs: u32, number_of_nodes: u32, activation_function: fn(f32)->f32) -> Layer{
        let mut nodes: Vec<Node> = Vec::new();
        for _i in 0..number_of_nodes{
            nodes.push(Node::new(number_of_inputs, activation_function));
        }
        Layer{number_of_inputs,nodes}
    }

    pub fn display(&self){
        for i in 0..self.nodes.len(){
            println!("  Node - {}",i);
            self.nodes[i].display();
        }
    }

    pub fn calculate(&self,input: Vec<f32>)->Vec<f32>{
        let mut res: Vec<f32> = Vec::new();
        for i in self.nodes.iter(){
            res.push(i.calculate(&input));
        }
        res 
    }
}