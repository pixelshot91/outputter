pub trait StringExt {
    fn surround_with_space(self) -> String;
}
impl StringExt for String {
    fn surround_with_space(self) -> String {
        format!(" {self} ")
    }
}
