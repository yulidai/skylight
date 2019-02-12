use super::style::{ StyleNode, Display };

// data struct
#[derive(Debug)]
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>
}

#[derive(Debug)]
pub enum BoxType<'a> {
    BlockNode(&'a StyleNode<'a>),
    InlineNode(&'a StyleNode<'a>),
    AnonymousBlock,
}

#[derive(Debug, Default)]
pub struct Dimensions {
    content: Rect,

    padding: EdgeSizes,
    margin: EdgeSizes,
    border: EdgeSizes,
}

#[derive(Debug, Default)]
struct Rect {
    x: f32,
    y: f32,
    weight: f32,
    height: f32,
}

#[derive(Debug, Default)]
struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

// create layout tree
pub fn layout_tree<'a>(style_node: &'a StyleNode) -> LayoutBox<'a> {
    let mut root = LayoutBox::new( match style_node.display() {
        Display::Block => BoxType::BlockNode(style_node),
        Display::Inline => BoxType::InlineNode(style_node),
        _ => panic!("The root node's display is invalud or none!"),
    });

    for child in &style_node.children {
        match child.display() {
            Display::Block => root.children.push(layout_tree(child)),
            Display::Inline => root.get_inline_container().children.push(layout_tree(child)),
            _ => {}
        }
    }

    return root;
}

impl<'a> LayoutBox<'a> {
    pub fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(),
            children: Vec::new()
        }
    }

    pub fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
            BoxType::BlockNode(_) => {
                match self.children.last() {
                    Some(&LayoutBox { box_type: BoxType::AnonymousBlock, .. }) => {},
                    _ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock))
                }

                self.children.last_mut().unwrap()
            }
        }
    }
}