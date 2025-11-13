use rust_server::data_structure::LinkedList;

#[test]
fn linked_list_push_left() {
    let mut list = LinkedList::<i32>::new();
    list = list.push_left(21);
    list = list.push_left(45);
    list = list.push_left(78);
    list = list.push_left(221);
    list = list.push_left(11);

    println!("{:#?}", list);
}
