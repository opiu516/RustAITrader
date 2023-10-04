use crate::node::Node;

pub struct Layer{
    nodes: Vec<Node>
}

impl Layer{
    pub fn new( number_of_inputs: u32, activation_function: fn(f32)->f32) -> Layer{
        let mut nodes: Vec<Node> = Vec::new();
        for _i in 0..number_of_inputs{
            nodes.push(Node::new(number_of_inputs, activation_function));
        }
        Layer{nodes}
    }

    pub fn display(&self){
        for i in 0..self.nodes.len(){
            println!("Node - {}",i);
            self.nodes[i].display();
        }
    }
}