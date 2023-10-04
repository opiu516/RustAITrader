mod node;

fn main() {
    let node = node::create_node(4,node::simple_activation_function);
    node.display();
    println!("res = {}",node.calculate(vec![2.0,1.0,3.0,1.5]));
}
