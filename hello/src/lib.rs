pub fn greet() {
    println!("Hi!")
}

pub fn do_stuff_reference(s: &String) {
    println!("do_stuff_reference = {}", s);
}

pub fn do_stuff_mutable_reference(s: &mut String) {
    s.insert_str(0, "Hello ");
    (*s).insert_str(0, "Yes - "); // dereference to value (*s)
    println!("v1 do_stuff_mutable_reference = {}", s);

    *s = String::from("Replace value");
    println!("v2 do_stuff_mutable_reference = {}", s);
}

pub fn do_stuff_owner(s: String) {
    println!("do_stuff_owner = {}", s);
}
