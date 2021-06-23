// (**) Define predicates and/2, or/2, nand/2, nor/2, xor/2, impl/2 and equ/2 
// (for logical equivalence) which succeed or fail according to the result of 
// their respective operations; e.g. and(A,B) will succeed, if and only if both A and B succeed.

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

fn truth_table(f : &dyn Fn(bool, bool) -> bool) -> String {
    let mut s : String = "".to_string();
    for b1 in [true, false].iter() {
        for b2 in [true, false].iter() {
            s = format!("{}\n{} {} {}", s, b1, b2, f(*b1, *b2));
        }
    }
    s
}

fn main() {
    println!("{}", truth_table(&|x, y| myand(x, myor(x, y))));
}