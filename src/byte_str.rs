pub type ByteStr = ascii::AsciiString;

pub trait ByteStrLike {
    fn from_static(inp: &str) -> Self;
    unsafe fn from_utf8_unchecked<T>(src: T) -> Self where T: AsRef<[u8]>;
}

impl ByteStrLike for ByteStr {
    fn from_static(inp: &str) -> Self {
        use std::str::FromStr;
        ByteStr::from_str(inp).unwrap()
    }

    unsafe fn from_utf8_unchecked<T>(src: T) -> Self where T: AsRef<[u8]> {
        ByteStr::from_ascii_unchecked(src.as_ref())
    }
}
