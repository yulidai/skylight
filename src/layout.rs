use super::style::{ StyleNode, Display };
use super::css::{ Value, Unit };

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

#[derive(Debug, Default, Copy, Clone)]
pub struct Dimensions {
    content: Rect,

    padding: EdgeSizes,
    margin: EdgeSizes,
    border: EdgeSizes,
}

#[derive(Debug, Default, Copy, Clone)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug, Default, Copy, Clone)]
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

impl Rect {
    fn expanded_by(&self, edge: EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

impl Dimensions {
    fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }

    fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }

    fn margin_box(self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }
}

impl<'a> LayoutBox<'a> {
    pub fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(),
            children: Vec::new()
        }
    }

    pub fn get_style_node(&self) -> &'a StyleNode<'a> {
        match self.box_type {
            BoxType::BlockNode(node) => node,
            BoxType::InlineNode(node) => node,
            BoxType::AnonymousBlock => panic!("There is not style node within AnonymousBlock"),
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

    pub fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BoxType::BlockNode(_) => self.layout_block(containing_block),
            BoxType::InlineNode(_) | BoxType::AnonymousBlock => {},
        }
    }

    fn layout_block(&mut self, containing_block: Dimensions) {

        self.calculate_block_width(containing_block);

        self.calculate_block_position(containing_block);

        self.layout_block_children();

        self.calculate_block_height();

    }

    fn calculate_block_width(&mut self, containing_block: Dimensions) {

        let style = self.get_style_node();

        let auto = Value::Keyword("auto".to_string());
        let mut width = style.value("width").unwrap_or(auto.clone());

        let zero = Value::Length(0.0, Unit::Px);

        let mut margin_left = style.lookup("margin-left", "margin", &zero);
        let mut margin_right = style.lookup("margin-right", "margin", &zero);

        let border_left = style.lookup("border-left", "border", &zero);
        let border_right = style.lookup("border-right", "border", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        let total = sum([&margin_left, &margin_right, &border_left, &border_right, &padding_left, &padding_right]
            .iter().map(|v| v.to_px()));

        // 调整
        if width != auto && total > containing_block.content.width {
            if(margin_left == auto) {
                margin_left = Value::Length(0.0, Unit::Px);
            }

            if(margin_right == auto) {
                margin_right = Value::Length(0.0, Unit::Px);
            }
        }

        let underflow = containing_block.content.width - total;

        match (width == auto, margin_left == auto, margin_right == auto) {
            (false, false, false) => {
                margin_right = Value::Length(margin_right.to_px() + underflow, Unit::Px);
            },
            (false, true, false) => {
                margin_left = Value::Length(underflow, Unit::Px);
            },
            (false, false, true) => {
                margin_right = Value::Length(underflow, Unit::Px);
            },
            (false, true, true) => {
                margin_left = Value::Length(underflow / 2.0, Unit::Px);
                margin_right = Value::Length(underflow / 2.0, Unit::Px);
            },
            (true, _, _) => {
                if margin_left == auto { margin_left = Value::Length(underflow, Unit::Px); }
                if margin_right == auto { margin_right = Value::Length(underflow, Unit::Px); }

                if underflow >= 0.0 {
                    width = Value::Length(underflow, Unit::Px);
                } else {
                    width = Value::Length(0.0, Unit::Px);
                    margin_right = Value::Length(margin_right.to_px() + underflow, Unit::Px); // 实际是减去差值
                }
            }
        }

        let mut d = &mut self.dimensions;
        d.content.width = width.to_px();

        d.padding.left = padding_left.to_px();
        d.padding.right = padding_right.to_px();

        d.border.left = border_left.to_px();
        d.border.right = border_right.to_px();

        d.margin.left = margin_left.to_px();
        d.margin.right = margin_right.to_px();

    }

    fn calculate_block_position(&mut self, container_block: Dimensions) {
        let style = self.get_style_node();
        let mut d = self.dimensions;

        let zero = Value::Length(0.0, Unit::Px);

        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

        d.border.top = style.lookup("border-top", "border", &zero).to_px();
        d.border.bottom = style.lookup("border-bottom", "border", &zero).to_px();

        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        d.content.x = container_block.content.x + d.margin.left + d.border.left + d.padding.left;
        // container_block.content.height 是每计算一个子元素完成后自增的
        d.content.y = container_block.content.y + container_block.content.height + d.margin.top + d.border.top + d.padding.top;
    }

    fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;

        for child in &mut self.children {
            child.layout(*d);

            d.content.height += child.dimensions.margin_box().height; // 高度在这里累加
        }
    }

    fn calculate_block_height(&mut self) {
        if let Some(Value::Length(h, Unit::Px)) = self.get_style_node().value("height") {
            self.dimensions.content.height = h;
        }
    }
}

fn sum<I>(iter: I) -> f32 where I: Iterator<Item=f32> {
    iter.fold(0., |a, b| a + b)
}