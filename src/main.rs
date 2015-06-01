mod list;

use list::List;

fn main() {
    let list1 = List::empty().prepend("hello".to_string());
    let list2 = list1.prepend("world".to_string());
    let head = list2.head();
    let tail = list2.tail();

    println!("List1: {:?}, length: {}", list1, list1.length());
    println!("List2: {:?}, length: {}", list2, list2.length());
    println!("head: {:?}, tail: {:?}", head, tail);

    for elem in list2.iter() {
        println!("{}", elem);
    }
}
