use std::collections::HashMap;

pub struct Tree {
    element : String,
    attributes : HashMap<String, String>,
    children : Vec<Child>
}

pub enum Child {
    TextType(String),
    TreeType(Tree)
}

trait Spacer {
    fn print_space(&mut self, level:usize);
}

impl Spacer for string_builder::Builder {
    fn print_space(&mut self, level:usize) {
        for _ in 1..level {
            self.append("\t");
        }
    }
}

impl Tree {
    pub fn new(name : String) -> Tree {
        Tree{ element:name, attributes:HashMap::new(), children:Vec::new()}
    }

    pub fn insert_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    pub fn insert_child(&mut self, child : Child) {
        self.children.push(child);
    }

    pub fn print_with_builder(&self, builder : &mut string_builder::Builder) {
        self.print_with_builder_and_depth(builder, 0);
    }

    fn print_with_builder_and_depth(&self, builder : &mut string_builder::Builder, level : usize) {
        builder.print_space(level);
        builder.append(format!("<{}",self.element));

        for (k,v) in self.attributes.iter() {
            builder.append(format!(r#" {}="{}""#, k, v));
        }

        builder.append(">\n");

        for i in &self.children {
            match i {
                Child::TextType(t) => {
                    builder.print_space(level + 1);
                    builder.append(t.clone());
                },
                Child::TreeType(t) => {
                    t.print_with_builder_and_depth(builder, level + 1);
                }
            }
        }

        builder.append(format!("</{}>",self.element));
    }
}