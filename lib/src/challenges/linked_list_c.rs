use super::super::data_structures::linked_list::*;

pub fn one() {
    let mut list: LinkedList<u8> = LinkedList::new();
    list.push(5);
    list.append(3);
    list.append(2);
    println!("\t__Current_List__");
    println!("\t{:?}\n", list);

    println!("\t__Node_At_Index_1__");
    println!("\t{:?}\n", list.node_for_index(1));
    println!("\t__Node_At_Index_4__");
    println!("\t{:?}\n", list.node_for_index(4));

    println!("\tPOP: {:?}\n", list.pop());
    println!("\tPOP: {:?}\n", list.pop());
    println!("\tPOP: {:?}\n", list.pop());
    println!("\tPOP: {:?}\n", list.pop());
    
    println!("\t__Current_List__");
    println!("\t{:?}\n", list);
}