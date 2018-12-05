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
            MyError::Io(ref err) => write!(f, "IO error: {}", err),
            MyError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for MyError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
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