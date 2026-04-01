enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn list_sum(list: &List) -> i32 {
    match list {
        List::Cons(val, next) => val + list_sum(next), // Rust automatically dereferences the Box here!
        List::Nil => 0,
    }
}
