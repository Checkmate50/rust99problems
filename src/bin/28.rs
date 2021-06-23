// (**) Group the elements of a set into disjoint subsets.

trait HasLength {
    fn leng(&self) -> usize;
}

macro_rules! impl_trait(($($T:ty)+) => {
    $(impl HasLength for $T {
      fn leng(&self) -> usize { self.len() }
    })+
  });
  
impl_trait!(String str);
impl_trait!(&[u8]);

fn lfsort<T:HasLength>(v : &mut Vec<T>) {
    v.sort_by(|x, y| x.leng().cmp(&y.leng()));
}

fn main() {
    let s = vec!["abc", "de", "fgh", "de", "ijkl", "mn", "o"];
    let mut res = s.iter().
        map(|x| x.
            as_bytes()).
        collect::<Vec<_>>();
    lfsort(&mut res);
    println!("{:?}", res.into_iter().
        map(|x| std::str::from_utf8(x)).collect::<Vec<_>>());
}