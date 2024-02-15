use std::io;
use std::fmt;

fn main() {
    
    // so we want to define the structure with nodes

    // we've defined the basic structure now I want to handel file reading and adding to the structure

    // if we define a leaf then we run into the issue of wanting to make it into a branch later
    // we only really want leaves at the very end so we dont have pointers to infinite undefined nodes

    // we could have it go A P P L E and then make that into the proper structure with E as a leaf
    // but if we add A P P L E S now our leaf needs to be a branch...


    #[derive(Debug)]
    enum Node{
        Leaf {name: char,
            terminus: bool,},

        Branch {name: char,
            terminus: bool,
            nodes:Box<Node>}
    }
    #[derive(Debug)]
    struct Trie{
        // the trie will have node children but no value and no terminus
        nodes:Box<Node>,
    }

    impl Node {
        // Method to convert a Leaf node into a Branch node
        // needs to be given a Node type object to attach to next element
        fn convert_to_branch(self, node: Node) -> Node {
            match self {
                Node::Leaf { name, terminus } => Node::Branch {
                    name,
                    terminus,
                    nodes: Box::new(node),
                },
                _ => panic!("Cannot convert non-leaf node to branch"),
            }
        }
    }

    let tree: Trie = Trie{
        nodes: Box::new(Node::Leaf{ name: 'A', terminus:true})
    };

    println!("{:?}", tree);
}
