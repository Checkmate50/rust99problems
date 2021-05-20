// (**) Flatten a nested list structure.

enum NestedList<T> {
    Elem(T),
    List(Vec<NestedList<T>>),
}

fn my_flatten<T:Clone>(v : NestedList<T>) -> Vec<T> {
    match v {
        NestedList::Elem(e) => vec![e],
        NestedList::List(mut l) => {
            let x = l.pop();
            match x {
                None => vec![],
                Some(s) => {
                    let mut vn = my_flatten(s);
                    let mut res = my_flatten(NestedList::List(l));
                    res.append(&mut vn);
                    res
                }
            }
        }
    }
}

fn main() {
    let v = NestedList::Elem(5);
    println!("{:?}", my_flatten(v));
    let v2 = NestedList::List(vec![NestedList::Elem(5),
                                     NestedList::List(vec![NestedList::Elem(6),
                                                            NestedList::Elem(7)]),
                                     NestedList::Elem(8)]);
    println!("{:?}", my_flatten(v2));
    let v3 : NestedList<i32> = NestedList::List(vec![]);
    println!("{:?}", my_flatten(v3));
}