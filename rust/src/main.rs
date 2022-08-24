struct Person{
    name: String ,
    age: u8, 
}

enum Op<T>{
    Add, 
    Sub, 
    Div, 
    Mul,
    Id(T)
}

struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    op: Op<T>
}

type ChildNode<T> = Option<Box<Node<T>>>;



struct BinaryTree<T> {
    head: Option<Node<T>>
}


impl Node<i32> {
    pub fn new (op: Op<i32>, l: Node<i32>,  r: Node<i32>) -> Self {
        Node::<i32>{
            op: op, 
            left: Some(Box::new(l)), 
            right: Some(Box::new(r))
        }
    }
}

fn main() {
    println!("Hello, world!");

    
    let name = String::from("Peter"); 
    let age = 27; 
    let p = Person{name, age};
}
