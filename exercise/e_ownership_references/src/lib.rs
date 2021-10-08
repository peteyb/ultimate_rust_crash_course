pub fn inspect(s: &str) {
    if s.ends_with('s') {
        println!("plural")
    } else {
        println!("singular")
    }
}

pub fn change(s: &mut String) {
    if !s.ends_with('s') {
        s.push('s')
    }
}

pub fn eat(s: String) -> bool {
    s.starts_with('b') && s.contains('a')
}

pub fn add(x: &i32, y: &i32) -> i32 {
    *x + *y
}
