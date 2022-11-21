use std::env::{args, Args};
// THe above import wants to import from the standard library env-module .. and we ask for the args function itself, and the Args Struct type (think of like a Type)

fn main() {
    let mut args: Args = args();
    println!("{:?}", args);
    let first: String = args.nth(1).unwrap();
    let operator: String = args.nth(0).unwrap();
    let second: String = args.nth(0).unwrap();
    // Below we're instructing the println macro to use a 'default' type of formatting to
    // 'display' the args object .. otherwise this would error b/c println doesnt assume how to present an object
    println!("output: {:?} {} {}", first, operator, second);
}

// fn nth(&mut self, n: usize) -> Option<String> {
//     // assume n = 0
//     // inner = ["1", "2"]
//     self.inner.next()
//     // calling next again results in 2nd element being passed
// }
