fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        // переменная i привязывается к значению,
        // содержащемуся внутри Some
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);

    dbg!(plus_one(five));

    dbg!(plus_one(None));
}
