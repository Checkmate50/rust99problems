// (**) Drop every N'th element from a list.

fn drop_every<T> (v : &mut Vec<T>, n : i32) {
    let mut i = 0;
    let mut count = 0;
    while i < v.len() {
        count += 1;
        if count % n == 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    }
}

fn main() {
    let s = "abcdefghik";
    let mut v = s.as_bytes().to_vec();
    drop_every(&mut v, 3);
    println!("{:?}", std::str::from_utf8(&v));
}