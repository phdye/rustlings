fn find_nth<'a> (elems: &'a [&str], n: usize) -> &'a str {
    println!("elems = {:?}", elems);
    let mut elem_refs: Vec<&&str> = elems.iter().collect();
    println!("refs0 = {:?}", elem_refs);
    elem_refs.sort();
    println!("refs0 = {:?}", elem_refs);
    let t = elem_refs[n];
    *t
}

fn main() {
    let list = ["b", "d" , "q", "a", "t" ];
    let n : usize = 2;
    let t = find_nth(&list, n);
    println!("{}", t);

    let mut list_iter = list.iter();
    while let Some(x) = list_iter.next() {
        println!("{}", x);
    }
}
