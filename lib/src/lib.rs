pub mod challenges;
pub mod data_structures;

pub fn run() {
    println!("\n\n____START____\n");
    println!("____Start_Stack____");
    challenges::stack_c::one();
    challenges::stack_c::two();
    println!("____END_Stack____");

    println!("____Start_Linked_List____");
    challenges::linked_list_c::one();

    println!("____END_Linked_List____");
}