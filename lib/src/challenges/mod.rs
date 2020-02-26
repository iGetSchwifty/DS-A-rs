use super::data_structures::stack::*;
pub fn stack_one() {
    //
    let challenge_text = "Create a function that prints the contents of an array in reversed order";
    println!("{}", challenge_text);
    println!("");

    let content_words = ["1", "2", "3"].to_vec();
    println!("{:?}", content_words);
    print_in_reverse(content_words);
    println!("");

    let content_numbers = [7, 6, 5, 4].to_vec();
    println!("{:?}", content_numbers);
    print_in_reverse(content_numbers);
    println!("");
}

pub fn stack_two() {
    let challenge_text = "Check for balanced parentheses.
    Given a string, check if there are ( and ) chracters,
    and return true if the pararentheses in the string are balanced";
    println!("{}", challenge_text);
    println!("");

    let balanced = "h((e))llo(world)()";
    let not_balanced = "(hello world";

    println!("{} is balanced: {}", balanced, check_for_balance(balanced));
    println!("{} is balanced: {}", not_balanced, check_for_balance(not_balanced));

    println!("");
}

// O(n)
fn print_in_reverse<T: std::fmt::Debug>(content_to_reverse: Vec<T>) {
    let mut stack = Stack::new(content_to_reverse);

    while !stack.empty() {
        println!("{:?}", stack.pop());
    }
}

// O(n)
fn check_for_balance(data: &str) -> bool {
    let stack_data: Vec<char> = [].to_vec();
    let mut stack = Stack::new(stack_data);
    for c in data.chars() {
        if c == '(' {
            stack.push(c);
        } else if c == ')' {
            stack.pop();
        }
    };
    stack.empty()
}