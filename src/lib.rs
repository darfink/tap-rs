#[cfg(feature = "future")]
pub use self::future::TapFutureOps;

#[cfg(feature = "future")]
mod future;

#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate matches;

/// Tap operations for `bool`.
pub trait TapBooleanOps {
    /// Executes a closure if the value is `Result::Ok(T)`.
    fn tap_true<R, F: FnOnce(&mut bool) -> R>(self, f: F) -> Self;

    /// Executes a closure if the value is `Result::Err(E)`.
    fn tap_false<R, F: FnOnce(&mut bool) -> R>(self, f: F) -> Self;
}

impl TapBooleanOps for bool {
    fn tap_true<R, F: FnOnce(&mut bool) -> R>(mut self, f: F) -> Self {
        if self {
            let _ = f(&mut self);
        }
        self
    }

    fn tap_false<R, F: FnOnce(&mut bool) -> R>(mut self, f: F) -> Self {
        if !self {
            let _ = f(&mut self);
        }
        self
    }
}

/// Tap operations for `Result`.
pub trait TapResultOps<T, E> {
    /// Executes a closure if the value is `Result::Ok(T)`.
    fn tap_ok<R, F: FnOnce(&mut T) -> R>(self, f: F) -> Self;

    /// Executes a closure if the value is `Result::Err(E)`.
    fn tap_err<R, F: FnOnce(&mut E) -> R>(self, f: F) -> Self;
}

impl<T, E> TapResultOps<T, E> for Result<T, E> {
    fn tap_ok<R, F: FnOnce(&mut T) -> R>(mut self, f: F) -> Self {
        if let Ok(mut val) = self.as_mut() {
            let _ = f(&mut val);
        }
        self
    }

    fn tap_err<R, F: FnOnce(&mut E) -> R>(mut self, f: F) -> Self {
        if let Err(mut val) = self.as_mut() {
            let _ = f(&mut val);
        }
        self
    }
}

/// Tap operations for `Option`.
pub trait TapOptionOps<T> {
    /// Executes a closure if the value is `Option::Some(T)`.
    fn tap_some<R, F: FnOnce(&mut T) -> R>(self, f: F) -> Self;

    /// Executes a closure if the value is `Option::None`.
    fn tap_none<R, F: FnOnce() -> R>(self, f: F) -> Self;
}

impl<T> TapOptionOps<T> for Option<T> {
    fn tap_some<R, F: FnOnce(&mut T) -> R>(mut self, f: F) -> Self {
        if let Some(mut val) = self.as_mut() {
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
pub trait TapOps : Sized {
    /// Executes a closure on an object.
    fn tap<R, F: FnOnce(&mut Self) -> R>(mut self, f: F) -> Self {
        let _ = f(&mut self);
        self
    }
}

impl<T> TapOps for T where T: Sized {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean() {
        let mut foo = 0;
        let boolean = false;
        assert_eq!(boolean.tap_false(|_| foo += 5), false);
        assert_eq!(foo, 5);
    }

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

    #[test]
    fn mutable() {
        let base = [1, 2, 3];
        let mutated = base.tap(|mut arr| for elt in arr.iter_mut() {
            *elt *= 2;
        });
        assert_eq!(mutated, [2, 4, 6]);
    }
}
