// (**) Truth tables for logical expressions (3)

fn myand(l : bool, r : bool) -> bool {
    l && r
}

fn myor(l : bool, r : bool) -> bool {
    l || r
}

fn mynand(l : bool, r : bool) -> bool {
    !(l && r)
}

fn mynor(l : bool, r : bool) -> bool {
    !(l || r)
}

fn myxor(l : bool, r : bool) -> bool {
    l ^ r
}

fn myimpl(l : bool, r : bool) -> bool {
    !l || r
}

fn myequ(l : bool, r : bool) -> bool {
    l == r
}

fn truth_table<const N: usize>(f : &dyn Fn(&[bool; N]) -> bool) -> String {
    let mut s : String = "".to_string();
    for i in 0..(2 as i32).pow(N as u32) {
        s = format!("{}\n", s);
        let mut bools: [bool; N] = [false;N];
        let mut n = i;
        for j in 0..N {
            bools[j] = n % 2 == 0;
            s = format!("{} {}", s, n % 2 == 0);
            n >>= 1;
        }
        s = format!("{} {}", s, f(&bools));
    }
    s
}

fn main() {
    println!("{}", truth_table::<3>(&|[x, y, z]| 
        myequ(myand(*x, myor(*x, *y)), myand(*x, myor(*y, myand(*x, *z))))));
}