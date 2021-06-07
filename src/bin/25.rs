// (*) Insert an element at a given position into a list.

extern crate rand;
use rand::Rng;

fn rnd_select<T:Clone>(v : &[T], count : usize) -> Vec<T> {
    let mut res= vec![];
    let mut vals = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..v.len() {
        vals.push(v[i].clone());
    }
    for _ in 0..count {
        if vals.len() == 0 {
            break;
        }
        res.push(vals.remove(rng.gen_range(0..vals.len())))
    }
    res
}

fn rnd_permu<T:Clone>(v: &[T]) -> Vec<T> {
    rnd_select(v, v.len())
}

fn main() {
    let s = "abcdefg";
    let res = rnd_permu(s.as_bytes());
    println!("{:?}", std::str::from_utf8(&res));
}