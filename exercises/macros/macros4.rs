// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.


#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
    // 定义了两个不同的匹配分支，匹配时要用分号隔开
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
