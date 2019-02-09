use std::collections::HashMap;

mod dom;

fn main() {
    println!("Hello, world!");

    let node = dom::Node::text("First Node".to_string());
    println!("{:?}", node);

    let node = dom::Node::elem("Element Node".to_string(), HashMap::new(), Vec::new() );
    println!("{:?}", node);
}
