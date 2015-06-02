mod list;

use list::List;

fn main() {
    let list0 = List::empty();
    let list1 = list0.prepend("hello".to_string());
    let list2 = list1.prepend("world".to_string());
    let list3 = list2.reverse();
    let head = list2.head();
    let tail = list2.tail();
 
    println!("List0: {:?}, length: {}, empty: {}", list0, list0.len(), list0.is_empty());
    println!("List1: {:?}, length: {}, empty: {}", list1, list1.len(), list1.is_empty());
    println!("List2: {:?}, length: {}, empty: {}", list2, list2.len(), list2.is_empty());
    println!("List3: {:?}, length: {}, empty: {}", list3, list3.len(), list3.is_empty());
    println!("head: {:?}, tail: {:?}", head, tail);
 
    for elem in list3.iter() {
        println!("{}", elem);
    }
}