fn main() {
    let list = ["b", "d" , "q", "a", "t" ];
    let mut elem_refs: Vec<&&str> = list.iter().collect();
    println!("{:?}", elem_refs);
    elem_refs.sort();
    println!("{:?}", elem_refs);
}
