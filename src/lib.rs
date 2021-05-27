use std::fmt;

pub fn print_option(o: Option<impl fmt::Display>) {
    match o {
        None    =>  println!("None"),
        Some(s) =>  println!("{}", s)
    }
}

pub fn pack<T:PartialEq+Clone>(v : &[T]) -> Vec<Vec<T>> {
    if v.len() == 0 {
        return vec![];
    }
    let mut cur = v[0].clone();
    let mut res = vec![vec![]];
    for i in 0..v.len() {
        if cur != v[i] {
            cur = v[i].clone();
            res.push(vec![]);
        }
        let l = res.len() - 1;
        res[l].push(cur.clone());
    }
    res
}

pub enum ListEncoding<T> {
    Single(T),
    Multiple(T, usize)
}

impl<T:std::fmt::Debug> std::fmt::Debug for ListEncoding<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListEncoding::Single(v) =>
                fmt.debug_tuple("Single").field(v).finish(),
            ListEncoding::Multiple(v, s) =>
                fmt.debug_tuple("Multiple").field(v).field(s).finish()
        }
    }
}