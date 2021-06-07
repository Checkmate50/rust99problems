// (*) Insert an element at a given position into a list.

extern crate rand;
use rand::Rng;

fn range(low : i32, high : i32) -> Vec<i32> {
    let mut res= vec![];
    for i in low..high+1 {
        res.push(i)
    }
    res
}

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

fn diff_select(count : usize, high : i32) -> Vec<i32> {
    rnd_select(&range(0, high), count)
}

fn main() {
    let res = diff_select(6, 49);
    println!("{:?}", res);
}