// ==========================================
// Funzioni funzionali (eval, compose, option helpers)
// ==========================================

#[inline(always)]
pub fn eval<T>(condition: bool, then_val: T, else_val: T) -> T {
    if condition { then_val } else { else_val }
}

#[inline(always)]
pub fn eval_lazy<T, F1, F2>(condition: bool, then_fn: F1, else_fn: F2) -> T
where
    F1: FnOnce() -> T,
    F2: FnOnce() -> T,
{
    if condition { then_fn() } else { else_fn() }
}

#[macro_export]
macro_rules! eval {
    ($cond:expr, $then:expr, $else:expr) => {
        if $cond { $then } else { $else }
    };
    ($cond:expr, { $then:expr }, { $else:expr }) => {
        if $cond { $then } else { $else }
    };
}

#[macro_export]
macro_rules! compose {
    ($f:expr) => { $f };
    ($f:expr, $($rest:expr),+) => {
        move |x| $f($crate::compose!($($rest),+)(x))
    };
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }

    #[inline(always)]
    pub fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }

    #[inline(always)]
    pub fn map_right<F, S>(self, f: F) -> Either<L, S>
    where
        F: FnOnce(R) -> S,
    {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(f(r)),
        }
    }

    #[inline(always)]
    pub fn unwrap_right(self) -> R {
        match self {
            Either::Right(r) => r,
            Either::Left(_) => panic!("Called unwrap_right on a Left value"),
        }
    }
}

#[inline(always)]
pub fn bind<T, R, F>(value: Option<T>, f: F) -> Option<R>
where
    F: FnOnce(T) -> Option<R>,
{
    value.and_then(f)
}

#[inline(always)]
pub fn fmap<T, R, F>(value: Option<T>, f: F) -> Option<R>
where
    F: FnOnce(T) -> R,
{
    value.map(f)
}

#[inline(always)]
pub fn tap<T, F>(value: T, f: F) -> T
where
    F: FnOnce(&T),
{
    f(&value);
    value
}

#[inline(always)]
pub fn mask(condition: bool) -> usize {
    condition as usize
}

#[inline(always)]
pub fn identity<T>(value: T) -> T {
    value
}

// ==========================================
// Gestione stato con default
// ==========================================

#[inline(always)]
pub fn get_or_default<T: Clone>(opt: &Option<T>, default: T) -> T {
    opt.as_ref().cloned().unwrap_or(default)
}

#[inline(always)]
pub fn get_or_default_with<T: Clone, F>(opt: &Option<T>, default_fn: F) -> T
where
    F: FnOnce() -> T,
{
    opt.clone().unwrap_or_else(default_fn)
}

#[inline(always)]
pub fn set_or_default<T: Clone>(target: &mut Option<T>, value: Option<T>, default: T) {
    *target = Some(value.unwrap_or(default));
}

#[inline(always)]
pub fn set_or_default_with<T, F>(target: &mut Option<T>, value: Option<T>, default_fn: F)
where
    F: FnOnce() -> T,
{
    *target = Some(value.unwrap_or_else(default_fn));
}

#[inline(always)]
pub fn get_curried<T: Clone>(default: T) -> impl Fn(&Option<T>) -> T {
    move |opt| opt.as_ref().cloned().unwrap_or(default.clone())
}

#[inline(always)]
pub fn set_curried<T: Clone>(default: T) -> impl Fn(&mut Option<T>, Option<T>) {
    move |target, value| {
        *target = Some(value.unwrap_or(default.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(eval(true, 10, 20), 10);
        assert_eq!(eval(false, 10, 20), 20);
    }

    #[test]
    fn test_eval_lazy() {
        let mut counter = 0;
        let r = eval_lazy(true, || { counter += 1; 42 }, || { counter += 2; 0 });
        assert_eq!(r, 42);
        assert_eq!(counter, 1);
    }

    #[test]
    fn test_eval_macro() {
        assert_eq!(eval!(true, 10, 20), 10);
        assert_eq!(eval!(false, 10, 20), 20);
    }

    #[test]
    fn test_get_or_default() {
        let some = Some(42);
        let none: Option<i32> = None;
        assert_eq!(get_or_default(&some, 0), 42);
        assert_eq!(get_or_default(&none, 0), 0);
    }
}
