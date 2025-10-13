pub fn zeros() -> [u32; 100] {
    let arr: [u32; 100] = [0; 100];
    println!("{:?}", arr);
    arr
}

pub fn first_3(s: &[u32]) -> &[u32] {
    &s[0..3]
}

pub fn last_3(s: &[u32]) -> &[u32] {
     &s[s.len() - 3..]
}

/*                Done              @mirmohmmadluqman */