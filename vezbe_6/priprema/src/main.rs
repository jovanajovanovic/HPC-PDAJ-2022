mod linked_list;
mod linked_list_2;
mod binary_tree;
use linked_list::Link;
use linked_list_2::Node;
use binary_tree::BinaryTreeNode;

use crate::linked_list_2::address;

fn main() {
    let mut list = Link::None;
    
    list.push(1);
    list.push(2);
    list.push(3);


    println!("{}", list.pop().unwrap());
    println!("{}", list.pop().unwrap());
    println!("{}", list.pop().unwrap());

    println!("++++++++++++++++++++++++++++++++++++++++++");
    let mut head = Node::new(8,address::Nil);

    head.append(10);
    head.append(9);
    head.append(14);

    head.print_list();


    println!("+++++++++++++++++++++++++++++++++++++++++");
    println!("Binary tree");
    let mut root = BinaryTreeNode::new(8);
    root.insert(10);
    root.insert(2);
    root.insert(11);
    root.insert(4);
    root.print_tree();



}
