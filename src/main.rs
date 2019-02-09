use std::collections::HashMap;

mod dom;
mod html;

fn main() {
    println!("Hello, world!");

    let node = dom::Node::text("First Node".to_string());
    println!("{:?}", node);

    let node = dom::Node::elem("Element Node".to_string(), HashMap::new(), Vec::new() );
    println!("{:?}", node);

    // Test Html Parser
    let source = "<div><div a=\"b\">123</div><div>456</div></div>";
    let node = html::parse(source.to_string());

    println!("html 格式化后: {:?}", node);

}
