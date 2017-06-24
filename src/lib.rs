#[cfg(feature = "future")]
pub use self::future::TapFutureOps;

#[cfg(feature = "future")]
mod future;

#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate matches;

/// Tap operations for `Result`.
pub trait TapResultOps<T, E> {
    /// Executes a closure if the value is `Result::Ok(T)`.
    fn tap_ok<R, F: FnOnce(&T) -> R>(self, f: F) -> Self;

    /// Executes a closure if the value is `Result::Err(E)`.
    fn tap_err<R, F: FnOnce(&E) -> R>(self, f: F) -> Self;
}

impl<T, E> TapResultOps<T, E> for Result<T, E> {
    fn tap_ok<R, F: FnOnce(&T) -> R>(self, f: F) -> Self {
        if let Ok(val) = self.as_ref() {
            let _ = f(val);
        }
        self
    }

    fn tap_err<R, F: FnOnce(&E) -> R>(self, f: F) -> Self {
        if let Err(val) = self.as_ref() {
            let _ = f(val);
        }
        self
    }
}

/// Tap operations for `Option`.
pub trait TapOptionOps<T> {
    /// Executes a closure if the value is `Option::Some(T)`.
    fn tap_some<R, F: FnOnce(&T) -> R>(self, f: F) -> Self;

    /// Executes a closure if the value is `Option::None`.
    fn tap_none<R, F: FnOnce() -> R>(self, f: F) -> Self;
}

impl<T> TapOptionOps<T> for Option<T> {
    fn tap_some<R, F: FnOnce(&T) -> R>(self, f: F) -> Self {
        if let Some(val) = self.as_ref() {
            let _ = f(val);
        }
        self
    }

    fn tap_none<R, F: FnOnce() -> R>(self, f: F) -> Self {
        if self.is_none() {
            let _ = f();
        }
        self
    }
}

/// Tap operations for all types.
pub trait TapOps {
    /// Executes a closure on an object.
    fn tap<R, F: FnOnce(&Self) -> R>(self, f: F) -> Self where Self: Sized {
        let _ = f(&self);
        self
    }
}

impl<T> TapOps for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option() {
        let mut foo = 0;
        let option = Some(5);
        assert_eq!(option.tap_some(|x| foo += *x), Some(5));
        assert_eq!(foo, 5);
    }

    #[test]
    fn result() {
        let mut foo = 0;
        let option: Result<i32, i32> = Ok(5);
        assert_eq!(option.tap_ok(|x| foo += *x), Ok(5));
        assert_eq!(foo, 5);
    }

    #[test]
    fn tap() {
        let mut foo = 0;
        assert_eq!(5.tap(|x| foo += *x), 5);
        assert_eq!(foo, 5);
    }
}
