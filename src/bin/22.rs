// (*) Create a list containing all integers within a given range..

fn range(low : i32, high : i32) -> Vec<i32> {
    let mut res= vec![];
    for i in low..high+1 {
        res.push(i)
    }
    res
}

fn main() {
    let res = range(4, 9);
    println!("{:?}", res);
}