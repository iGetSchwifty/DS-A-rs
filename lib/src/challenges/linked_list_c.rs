use super::super::data_structures::linked_list::*;

pub fn one() {
    let mut list: LinkedList<u8> = LinkedList::new();
    list.push(5);
    list.append(3);
    list.append(2);
    println!("{:?}", list);
}