use std::io;
use std::num;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            MyError::Io(ref err) => write!(f, "IO error: {}", err),
            MyError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for MyError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            MyError::Io(ref err) => Some(err),
            MyError::Parse(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::Io(err)
    }
}

impl From<num::ParseIntError> for MyError {
    fn from(err: num::ParseIntError) -> MyError {
        MyError::Parse(err)
    }
}


pub type MyResult<T> = Result<T, MyError>;