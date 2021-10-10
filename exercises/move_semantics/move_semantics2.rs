// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    /* 变量所有权: `vec0`的所有权会在`fill_vec()`返回时移交给`vec1` */

    let vec0 = Vec::new();
    // let mut vec1 = fill_vec(vec0.clone());  // 或者直接在外部拷贝
    let mut vec1 = fill_vec(&vec0);  //  修改函数`fill_vec`, 通过引用`&vec0`完成对象拷贝

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
