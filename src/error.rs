#[derive(Debug)]
pub enum DecodeError {
    /// Weird FFI errors that should never happen
    /// (i.e. if you get this with a published version it's a bug.)
    FfiError(&'static str),

    /// Reading the header failed for some reason.
    ReadHeader,

}
