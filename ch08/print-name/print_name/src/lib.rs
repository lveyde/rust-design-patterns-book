pub trait PrintName {
    fn name() -> String;
    fn print_name() {
        println!("{}", Self::name());
    }
}
