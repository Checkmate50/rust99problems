// (**) Group the elements of a set into disjoint subsets.

fn combinations<T:Clone>(v : &[T], count : usize) -> Vec<Vec<T>> {
    let mut res = vec![];
    if count == 0 || v.len() == 0 {
        return res;
    }
    if count == 1 {
        for i in v {
            res.push(vec![i.clone()]);
        }
        return res;
    }
    for i in 0..v.len() {
        for vc in combinations(&v[i+1..v.len()], count-1) {
            let mut next = vec![];
            next.push(v[i].clone());
            for vci in vc {
                next.push(vci);
            }
            res.push(next);
        }
    }
    res
}

fn group<T:Clone+PartialEq>(v : &[T], counts : &[usize]) -> Vec<Vec<Vec<T>>> {
    let mut res = vec![];
    if counts.len() == 0 {
        return res;
    }
    let hd = counts[0];
    let cres = combinations(v, hd);
    
    for combo in &cres {
        let mut next = vec![];
        for i in v {
            if !combo.contains(i) {
                next.push(i.clone());
            }
        }
        let subresults = group(&next, &counts[1..]);
        let mut temp = vec![];
        let mut inner_temp = vec![];
        for item in combo {
            inner_temp.push(item.clone());
        }
        temp.push(inner_temp);
        if subresults.len() == 0 {
            res.push(temp);
            continue;
        }
        for sr in subresults {
            for item in &sr {
                inner_temp = vec![];
                for i in item {
                    inner_temp.push(i.clone());
                }
                temp.push(inner_temp);
            }
            res.push(temp);
            temp = vec![];
            inner_temp = vec![];
            for item in combo {
                inner_temp.push(item.clone());
            }
            temp.push(inner_temp);
        }
    }
    res
}

fn main() {
    let s = vec!["aldo","beat","carla","david","evi","flip","gary","hugo","ida"];
    let res1 = group(&(s.iter().
        map(|x| x.
            as_bytes()).
        collect::<Vec<_>>()), &[2, 3, 4]);
    let res2 = group(&(s.iter().
        map(|x| x.
            as_bytes()).
        collect::<Vec<_>>()), &[2, 2, 5]);
    println!("{} {}", res1.len(), res2.len());
    println!("{:?}\n{:?}", res1.into_iter()
        .map(|x| x.into_iter()
            .map(|y| y.into_iter()
                .map(|z| String::from_utf8(z.to_vec())
                .unwrap())
                .collect::<Vec<_>>())
            .collect::<Vec<_>>()).
        collect::<Vec<_>>(),
        res2.into_iter()
        .map(|x| x.into_iter()
            .map(|y| y.into_iter()
                .map(|z| String::from_utf8(z.to_vec())
                .unwrap())
                .collect::<Vec<_>>())
            .collect::<Vec<_>>()).
        collect::<Vec<_>>());
}