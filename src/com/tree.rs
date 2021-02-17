use std::collections::HashMap;

pub struct Tree {
    element : String,
    attributes : HashMap<String, String>,
    keys : Vec<String>,
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
    pub fn new<T: std::string::ToString>(name : T) -> Tree
    {
        Tree{
            element: name.to_string(),
            attributes:HashMap::new(),
            keys:Vec::new(),
            children:Vec::new()}
    }

    pub fn insert_attribute<
        T: std::string::ToString,
        U: std::string::ToString>(mut self, key: T, value: U) -> Tree {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }

    pub fn insert_key<T: std::string::ToString> (mut self, key: T) -> Tree {
        self.keys.push(key.to_string());
        self
    }

    pub fn insert_child(mut self, child : Child) -> Tree {
        self.children.push(child);
        self
    }

    pub fn insert_tree(self, tree : Tree ) -> Tree {
        self.insert_child(Child::TreeType(tree))
    }

    pub fn insert_text<T: std::string::ToString>(self, text : T) -> Tree {
        self.insert_child(Child::TextType(text.to_string()))
    }

    pub fn print(&self) -> String {
        let mut builder = string_builder::Builder::default();

        self.print_with_builder(&mut builder);

        builder.string().unwrap()
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

        for k in self.keys.iter() {
            builder.append(format!(" {}", k));
        }

        builder.append(">\n");

        for i in &self.children {
            match i {
                Child::TextType(t) => {
                    builder.print_space(level + 1);
                    builder.append(t.clone());
                    builder.append("\n");
                },
                Child::TreeType(t) => {
                    t.print_with_builder_and_depth(builder, level + 1);
                }
            }
        }
        builder.print_space(level);
        builder.append(format!("</{}>\n",self.element));
    }
}