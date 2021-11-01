// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // straightforward
    string("red".to_string()); // conversion to String
    string(String::from("hi")); // straightforward
    string("rust is fun!".to_owned()); // once owned, it becomes String
    string("nice weather".into()); // same as above
    string(format!("Interpolation {}", "Station")); // actually, this is a String
    string_slice(&String::from("abc")[0..1]); // since we sliced it, it turns into a &str
    string_slice("  hello there ".trim()); // trim actually works with slice
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // mutable, means String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // same as above
}
