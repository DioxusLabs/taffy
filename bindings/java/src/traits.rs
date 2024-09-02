pub trait FromJavaEnum<RustType> {
    const JAVA_CLASS: &'static str;
    fn from_ordinal(ordinal: i32) -> Option<RustType>;
}
