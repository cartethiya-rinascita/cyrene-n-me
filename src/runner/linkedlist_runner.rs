use crate::data_structure::linked_list::LinkedList;

pub fn run_linkedlist() {
    let mut list: LinkedList<i32> = LinkedList::<i32>::new();

    list = list.push_left(21);
    list = list.push_left(45);
    list = list.push_left(78);
    list = list.push_left(221);
    list = list.push_left(11);

    assert_eq!(list.len(), 5);
    assert_eq!(list.get_val(), Some(11));

    println!("{:#?}\n", list)
}
