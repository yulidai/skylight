use std::collections::HashMap;

mod dom;
mod html;
mod css;
mod style;

fn main() {
    println!("Hello, world!");

    let node = dom::Node::text("First Node".to_string());
    println!("{:?}", node);

    let node = dom::Node::elem("Element Node".to_string(), HashMap::new(), Vec::new() );
    println!("{:?}", node);

    // Test Html Parser
    //let source = "<div><div a=\"b\">123</div><div class=\"myclass\">456</div></div>";
    let source = "<div class=\"myclass\">456</div>";
    let node = html::parse(source.to_string());
    println!("html 格式化后: {:?}", node);

    // Test CSS Parser
    let source = "div.myclass { color: #eeeeee; } my_tag#my_id.my_class { background-color: #777777; } .my_class_2 { background-color: black; width: 70px; }";
    let stylesheet = css::parse(source.to_string());
    println!("CSS 格式化后: {:?}", stylesheet);

    // Test Style Module
    let style_tree_result = style::style_tree(&node, &stylesheet);
    println!("Style Tree 格式化后: {:?}", style_tree_result);
}
