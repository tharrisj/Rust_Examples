
#[derive(Debug)]
enum List {
    Node(Node),
    Nil,
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Box<List>,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, val: i32) -> List {
        List::Node(
            Node{
                val: val,
                next: Box::new(self)
            }
        )
    }

    fn stringify(&self) -> String {
        match self {
            List::Node(Node{val: head, next: ref tail}) => {
                format!("{}, {}", head, tail.stringify())
            }
            List::Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut test = List::new();

    test = test.prepend(42_i32);
    test = test.prepend(1_i32);

    println!("test: {:?}", test);
    println!("Using stringify: {}", test.stringify());
}
