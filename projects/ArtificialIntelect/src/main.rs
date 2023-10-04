mod layer;
mod node;

fn simple_activation_function(number:f32) -> f32{
    let mut res:f32 = 0.0;
    if number > 0.0 {
        res = 1.0;
    }
    res
}

fn main() {
    let layer = layer::Layer::new(4,simple_activation_function);
    layer.display();
    //println!("res = {}",layer.calculate(vec![2.0,1.0,3.0,1.5]));
}
