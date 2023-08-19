fn main() {
    // Q1: why S has to defined as mutable?
    let mut s = String::from("hello");

    // A1: push_str has the first parameter as `&mut self`. Which requires
    // the called object to be able borrow as mutable (E0596).
    s.push_str(", world!");

    println!("{s}");

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Q2: why clone() is required here?

    println!("{}, world!", s1);
    println!("{}, rust!", s2);

    let some_string = String::from("Some String");

    takes_ownership(&some_string);

    println!("{}", some_string);

    let some_integer = 37;

    makes_copy(some_integer);

    let _a1 = gives_ownership();

    let a2 = String::from("To be given");

    // when we give a2 to #takes_and_gives_back it returns it back to us
    // so that we can use its value in a3, otherwise a2's value is lost
    // This is cumbersome. #takes_and_gives_back has to return what it took,
    // along with any results it produces. see #calc_length_ugly
    let a3 = takes_and_gives_back(a2);

    // here #calc_length_ugly should have only returned the length of the string,
    // but ownership semantics has forced it to return the String also.
    // This is why we need REFERENCES (and BORROWING)
    let (_len, a4) = calc_length_ugly(a3);

    // with REFERENCing we can just pass a reference to the string variable,
    // receiver will borrow it rather than owning. Since we don't own the value
    // we can't modify it.
    let _a4len = calc_length_good(&a4);

    let mut p1 = String::from("hello");
    // #change_string is trying to modify the borrowed value,
    // it can do this, since owner of p1 has defined it as a mutable variable and
    // #change_string is accepting a mutable reference.
    change_string(&mut p1);

    // the catch with mutable references?
    //   1. they cannot refer to immutable variables.
    //   2. only one mutable reference can exists at a time.
    // looks like a shortcoming, but very useful at preventing uncoordinated
    // mutations, which leads to data races.

    // want mutiple mutable references?
    // create multiple scopes. while this allows multiple mut refs,
    // none of them are co-existing.
    {
        let _x = &mut p1;
    }

    {
        let _y = &mut p1;
    }

    // we can't create mutable refs, while there are immutable refs are available
    // let p11 = &p1; // ok
    // let p12 = &p1; // ok
    // let p13 = &mut p1; // not ok
    //
    // however, if the compiler can guarantee that p11, p12 are not used anymore
    // when defining p13, p13 is allowed.

    let p11 = &p1;
    let p12 = &p1;
    println!("{} {}", p11, p12);

    // scopes of {p11, p12} and {p13} are not overlapping in this instance.
    let p13 = &mut p1;
    p13.push_str(", changed");
    println!("{}", p13);
}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Orginated at #gives_ownership");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("#takes_and_gives_back owns {}", some_string);
    some_string
}

fn calc_length_ugly(s: String) -> (usize, String) {
    let len = s.len();
    (len, s)
}

fn calc_length_good(s: &String) -> usize {
    // reference is auto dereferenced?
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", world!");
}
