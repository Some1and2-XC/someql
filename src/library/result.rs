/// Result type for methods which update an external source and can fail.
/// This should generally be used when your function signature looks something like:
/// `fn foo(self) -> Result<T, T>`
/// with the idea that you want the borrow checker to invalidate your input.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Result<T, E> {
    /// The ok variant, meaning your method did what you would like it to.
    Ok(T),
    /// The error variant, meaning your method failed to complete successfully and the result is a
    /// fallback value.
    Err(E),
}

impl<T, E> Result<T, E> {

    pub const fn is_ok(&self) -> bool {
        return matches!(*self, Self::Ok(_));
    }

    pub const fn is_err(&self) -> bool {
        return matches!(*self, Self::Err(_));
    }

    pub fn ok(self) -> Option<T> {
        return match self {
            Self::Ok(v) => Some(v),
            Self::Err(_) => None,
        };
    }

    pub fn err(self) -> Option<E> {
        return match self {
            Self::Ok(_) => None,
            Self::Err(v) => Some(v),
        };
    }

    pub fn from_option(ok: Option<T>, err: E) -> Self {
        return match ok {
            Some(v) => Self::Ok(v),
            None => Self::Err(err)
        };
    }

    /*
    #[must_use]
    pub fn unwrap(self) -> T {
        return match self {
            Ok(v) => v;
            Err(e) => shjkldsahjk
        }
    }
    */

}

impl<T> Result<T, T> {

    pub fn indifferent(self) -> T {
        return match self {
            Self::Ok(v) => v,
            Self::Err(v) => v,
        };
    }

}

impl<T, E> From<std::result::Result<T, E>> for Result<T, E> {
    fn from(res: std::result::Result<T, E>) -> Self {
        return match res {
            Ok(v) => Result::Ok(v),
            Err(e) => Result::Err(e),
        };
    }
}
