type StrVec = Vec[&str];

fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    fn pr(refs : &StrVec) {
        print!("refs  =  [ ");
        for r in refs {
            print!("{}, ", r);
        }
        println!("]");
    }
    println!("elems = {:?}", elems);
    let mut elem_refs: StrVec = elems.iter().collect();
    # println!("refs0 = {:?}", elem_refs);
    pr(&elem_refs);
    elem_refs.sort();
    # println!("refs0 = {:?}", elem_refs);
    pr(&
        
        
        
        
        
        elem_refs);
    let t = elem_refs[n];
    return t.clone();
}

fn main() {
    let list = ["b", "d" , "q", "a", "t" ];
    let n : usize = 2;
    let t = find_nth(&list, n);
    println!("{}", t);
}
  