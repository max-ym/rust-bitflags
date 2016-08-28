
/// Create read-only flag.
///
/// It accepts an name argument, optional full name argument and a number,
/// which indicates the bit number with required flag.
///
/// Later, the given flag can be accessed either by short or full name
/// like with a normal function.
///
/// It will work only if it's put in a structure implementation
/// where integer number with fields can be accessed with `self.0'.
///
/// # Example
///
/// ```
/// # #![allow(unused_parens)]
/// #[macro_use]
/// extern crate alt_bitflags;
///
/// struct Something(u64);
///
/// impl Something {
///     flag_ro!(present, 0);
///     flag_ro!(tx, 1);
///     flag_ro!(e, extended, 2);
/// }
///
/// fn main() {
///     let mut something = Something(0);
///
///     if (something.present()) { /* ... */ }
///     if (something.tx()) { /* ... */ }
///
///     // These do the same thing:
///     if (something.e()) { /* ... */ }
///     if (something.extended()) { /* ... */ }
/// }
/// ```
#[macro_export]
macro_rules! flag_ro {
    ($name:ident, $pos:expr) => (
        #[inline="always"]
        pub fn $name(&self) -> bool { self.0 & (1 << $pos) != 0 }
    );

    ($name:ident, $full_name:ident, $pos:expr) => (
        flag_ro!($name, $pos);
        flag_ro!($full_name, $pos);
    );
}

/// Create read-write flag.
///
/// It accepts an name argument, optional full name argument and a number,
/// which indicates the bit number with required flag.
///
/// Later, the given flag can be accessed either by short or full name
/// like with a normal function.
///
/// It will work only if it's put in a structure implementation
/// where integer number with fields can be accessed with `self.0'.
///
/// # Example
///
/// ```
/// # #![allow(unused_parens)]
/// #[macro_use]
/// extern crate alt_bitflags;
///
/// struct Something(u64);
///
/// impl Something {
///     flag_rw!(present, set_present, 0);
///     flag_rw!(e, set_e, extended, set_extended, 1);
/// }
///
/// fn main() {
///     let mut something = Something(0);
///
///     if (something.present()) { /* ... */ }
///     if (something.e()) { /* ... */ }
///
///     something.set_e(true);
///     something.set_extended(true);
/// }
/// ```
#[macro_export]
macro_rules! flag_rw {
    ($name:ident, $set_name:ident, $pos:expr) => (
        flag_ro!($name, $pos);

        #[inline="always"]
        pub fn $set_name(&mut self, b: bool) {
            let x = b as i64;
            self.0 ^= ((-x ^ self.0 as i64) & (1 << $pos)) as u64;
        }
    );

    ($name:ident, $set_name:ident,
    $full_name:ident, $full_set_name:ident,
    $pos:expr) => (
        flag_rw!($name, $set_name, $pos);
        flag_rw!($full_name, $full_set_name, $pos);
    );
}

#[cfg(test)]
mod test;