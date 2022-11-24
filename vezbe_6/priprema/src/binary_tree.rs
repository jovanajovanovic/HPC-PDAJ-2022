// detaljniju implementaciju mozete pogledati na sajt https://hackernoon.com/how-to-insert-binary-tree-in-rust
// ovde su napravljene neke manje izmene  
pub struct BinaryTreeNode {
    value: i32, 
    left: Option<Box<BinaryTreeNode>>,
    right: Option<Box<BinaryTreeNode>>,
}

impl BinaryTreeNode{
    pub fn new(value: i32) -> Self{
        Self { value, left: None, right: None }
    }
    pub fn insert(&mut self, new_val: i32){
        //ne mozemo da imamo duplikate
        if self.value == new_val {
            return;
        }
 
        //detekcija da li element treba da stavimo kao levo ili desno dete
        let target_node = if new_val < self.value {
            &mut self.left
        }else{
            &mut self.right
        };

        match target_node{
            Some(ref mut sub_node) => sub_node.insert(new_val),
            None => {
                let new_node = BinaryTreeNode {value: new_val, left: None, right: None};
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node
            }

        }
    }

    pub fn print_tree(&self){
        println!("value = {}", self.value);

        match &self.left {
            Some(sub_node) => sub_node.print_tree(),
            None => {}
        }

        match  &self.right {
            Some(sub_node)=> sub_node.print_tree(),
            None => {}
        }
    }
}