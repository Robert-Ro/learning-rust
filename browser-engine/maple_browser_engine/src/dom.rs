use std::collections::HashMap;

#[derive(Debug)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
    Comment(String),
}
#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}
pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}
pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}
pub fn comment(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Comment(data),
    }
}
pub fn elem(name: String, data: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: data,
        }),
    }
}
