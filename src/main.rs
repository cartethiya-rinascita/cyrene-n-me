use crate::data_structure::linked_list::LinkedList;
use std::collections::HashMap;

mod data_structure;
mod fibo;

fn main() {
    let n: u64 = 65;
    let mut cache = HashMap::new();
    let mut list: LinkedList<i32> = LinkedList::<i32>::new();

    for i in 0..n {
        let res = fibo::fibonacci_memo(i, &mut cache);
        print!("{res} ");
    }

    list = list.push_left(21);
    list = list.push_left(45);
    list = list.push_left(78);
    list = list.push_left(221);
    list = list.push_left(11);

    println!("\nHello, world!");
    let last_value = &list.get_val();

    println!(
        "\n{:#?}\nLength: {}\nValue: {:#?}",
        list,
        list.len(),
        last_value
    );
}
