// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)


fn main() {
    my_macro!();
}

/* 表示该宏可被外部crate使用: */
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
