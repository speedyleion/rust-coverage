//          Copyright Nick G 2022.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

#[derive(Debug, PartialEq)]
pub enum Error {
    Bar,
    Baz
}

fn bar(flag: bool) -> Result<(), Error> {
    if flag {
        Err(Error::Bar)
    } else {
        Ok(())
    }
}

fn baz(flag: bool) -> Result<(), Error> {
    if flag {
        Err(Error::Bar)
    } else {
        Ok(())
    }
}

pub fn foo(flag: bool) -> Result<(), Error> {
    bar(flag)?;
    baz(flag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_flag_is_ok() {
        assert_eq!(foo(false), Ok(()));
    }
    #[test]
    fn flag_is_error() {
        assert_eq!(foo(true), Err(Error::Bar));
    }
}
