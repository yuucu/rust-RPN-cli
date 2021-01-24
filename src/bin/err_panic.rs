use std::fmt;

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path).map_err(|e| MyError::Io(e))?;

    let ret = num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| MyError::Num(e));
    ret
}

fn main() {
    let result = get_int_from_file();
    match result {
        Ok(x) => println!("{}", x),
        Err(e) => match e {
            MyError::Io(cause) => println!("{}", cause),
            MyError::Num(cause) => println!("{}", cause),
        },
    }
}

enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "{}", cause),
            MyError::Num(cause) => write!(f, "{}", cause),
        }
    }
}
