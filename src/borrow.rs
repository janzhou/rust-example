fn b(v: &mut Vec<i32>) -> &mut Vec<i32>{
     v.push(5);
     return v;
}

fn m(mut v: Vec<i32>) {
     b(&mut v);
}

pub fn borrow(){
    let mut v = vec![];
    println!("borrow! {:?}", b(&mut v));
    m(v);
}
