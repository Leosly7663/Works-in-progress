use std::io;
use std::fmt;
use std::ptr::null;


fn main() {

    let li1 = vec!['A','P','P','L','E'];
    let li2 = vec!['A','P','P','L','E','S'];
    let li3 = vec!['A','P','E'];
  
    // so we want to define the structure with nodes

    // we've defined the basic structure now I want to handel file reading and adding to the structure

    // if we define a leaf then we run into the issue of wanting to make it into a branch later
    // we only really want leaves at the very end so we dont have pointers to infinite undefined nodes

    // we could have it go A P P L E and then make that into the proper structure with E as a leaf
    // but if we add A P P L E S now our leaf needs to be a branch...


    #[derive(Debug)]
    enum Node{
        Leaf {
            name: char,
            terminus: bool,},

        Branch {
            name: char,
            terminus: bool,
            nodes: Option<Box<Node>>}
    }
    #[derive(Debug)]
    struct Trie{
        // the trie will have node children but no value and no terminus
        nodes:Box<Node>,
    }

    impl Node {
        // Method to convert a Leaf node into a Branch node
        // needs to be given a Node type object to attach to next element
        fn convert_to_branch(self, node:Option<Box<Node>>) -> Node {
            match self {
                Node::Leaf { name, terminus } => Node::Branch {
                    name,
                    terminus,
                    nodes: node,
                },
                _ => panic!("Cannot convert non-leaf node to branch"),
            }
        }
    }

    let mut tree: Trie = Trie{
        nodes: Box::new(Node::Leaf{ name: '.', terminus:true, })
    };


    *tree.nodes = Node::Branch { name: li1[0], terminus: false, Some(Box::new(Node::Null)) };

    for i in 1..5{
        let current = li1[i];
        // if root
        let newNode = Node::Leaf { name: li1[i], terminus: false};

    }

    println!("{:?}", tree);
}
