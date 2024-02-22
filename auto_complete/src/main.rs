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
    struct Node{
            name: char,
            terminus: bool,
            nodes: Option<Box<Node>>
        }
    
    #[derive(Debug)]
    struct Trie{
        // the trie will have node children but no value and no terminus
        nodes:Box<Node>,
    }


    let mut tree: Trie = Trie{
        nodes: Box::new(Node{ name: '.', terminus:true, nodes: None })
    };


    *tree.nodes = Node { name: li1[0], terminus: false, nodes: None };

    for i in 1..5{
        let current = li1[i];
        // if root
        let newNode: Node = Node { name: li1[i], terminus: false, nodes: None};
        
        tree.nodes.nodes = Some(Box::new(newNode));

    }

    println!("{:?}", tree);
}
