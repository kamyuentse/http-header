#[derive(Debug)]
pub enum Error {
    Header,
    Method,

    Utf8Error(::std::str::Utf8Error),
}

impl From<::std::str::Utf8Error> for Error {
    fn from(e: ::std::str::Utf8Error) -> Error {
        Error::Utf8Error(e)
    }
}