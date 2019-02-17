extern crate image;

use std::collections::HashMap;

mod dom;
mod html;
mod css;
mod style;
mod layout;
mod painting;

use std::fs::File;
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let node = dom::Node::text("First Node".to_string());
    println!("{:?}", node);

    let node = dom::Node::elem("Element Node".to_string(), HashMap::new(), Vec::new() );
    println!("{:?}", node);

    // Test Html Parser
    //let source = "<div><div a=\"b\">123</div><div class=\"myclass\">456</div></div>";
    //let source = "<div class=\"a\"><div class=\"b\"><div class=\"c\"><div class=\"d\"><div class=\"e\"><div class=\"f\"><div class=\"g\"></div></div></div></div></div></div></div>";
    let source = "<div class=\"a\"><div class=\"b\"><div class=\"c\"></div></div></div>";
    let node = html::parse(source.to_string());
    println!("html 格式化后: {:?}", node);

    // Test CSS Parser
    let source = "* { display: block; padding: 50px; } .a { background: #4cb4e7; } .b { background: #ffc09f; } .c { background: #ffee93; } .d { background: #008000; } .e { background: #0000ff; } .f { background: #4b0082; } .g { background: #800080; }";
    let stylesheet = css::parse(source.to_string());
    println!("CSS 格式化后: {:?}", stylesheet);

    // Test Style Module
    let style_tree_result = style::style_tree(&node, &stylesheet);
    println!("Style Tree 格式化后: {:?}", style_tree_result);

    // Test Layout Module
    let initial_containing_block = layout::Dimensions {
        content: layout::Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 },
        padding: Default::default(),
        border: Default::default(),
        margin: Default::default(),
    };

    let layout_tree = layout::layout_tree(&style_tree_result, initial_containing_block);
    println!("Layout Tree 格式化后: {:?}", layout_tree);

    // Test Paiting Module
    let canvas = painting::paint(&layout_tree, initial_containing_block.content);
    let path = Path::new("output.png");
    let file = File::create(&path).unwrap();

    // Save an image:
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let buffer: Vec<image::Rgba<u8>> = unsafe { std::mem::transmute(canvas.pixels) };
    let img = image::ImageBuffer::from_fn(w, h, |x: u32, y: u32| buffer[(y * w + x) as usize]);

    let result = image::ImageRgba8(img).save(&path);
    match result {
        Ok(_) => println!("Saved output success"),
        Err(_) => println!("Error saving output failed")
    }
}
