// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    my_macro!();
}
#[macro_export] //标注说明，只要将定义了宏的 crate 引入作用域，宏就应当是可用的。如果没有该标注，这个宏就不能被引入作用域。
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
