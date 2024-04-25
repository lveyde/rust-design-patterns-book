use print_name::PrintName;
use print_name_derive::PrintName;

#[derive(PrintName)]
struct MyStruct;

#[derive(PrintName)]
struct MyStruct2<T> {
    _t: T,
}

#[derive(PrintName)]
struct MyStruct3<T, T2> {
    _t: T,
    _t2: T2,
}

fn main() {
    println!("{}", MyStruct::name());
    println!("{}", MyStruct2::<u32>::name());
    println!("{}", MyStruct2::<i32>::name());
    println!("{}", MyStruct3::<i32, u32>::name());
}
