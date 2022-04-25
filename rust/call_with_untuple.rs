/// What does this do?
///
/// Well say you have a macro that returns a tuple
/// ```rust
/// macro_rules! tuple {
///     ($($item:expr),*) => { ( $($item:expr),* ) }
/// }
/// ```
///
/// And a function that uses the items of that tuple
/// but not the tuple itself
/// ```rust
/// fn add(a: u32, b: u32) -> u32 {
///     a + b
/// }
/// ```
///
/// Instead of changing the macro or the function
/// or use `let` destructuring, "simply" use _this_!
/// ```rust
/// let five = call_with_untuple!(add, tuple!(2, 3));
/// assert_eq!(five, 5);
/// ```
macro_rules! call_with_untuple {
    ($func:tt, ($($item:expr),*)) => { $func($($item),*) }
}
