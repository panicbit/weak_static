//! This crate provides a macro to create a static value that gets created when needed
//! and dropped as soon as its not needed anymore.
//! It requires the `lazy_static` macro to be imported.
//!
//! # Example
//!
//! ```rust
//! #[macro_use] extern crate lazy_static;
//! #[macro_use] extern crate weak_static;
//! 
//! struct Foo;
//!
//! impl Foo {
//!     fn new() -> Self {
//!         println!("new");
//!         Foo
//!     }
//! }
//!
//! impl Drop for Foo {
//!     fn drop(&mut self) {
//!         println!("drop");
//!     }
//! }
//! 
//! weak_static! {
//!     static FOO: Foo = Foo::new();
//! }
//!
//! fn main() {
//!     {
//!         let _foo1 = FOO();
//!         let _foo2 = FOO();
//!         let _foo3 = FOO();
//!     }
//!     
//!     {
//!         let _foo4 = FOO();
//!         let _foo5 = FOO();
//!         let _foo6 = FOO();
//!     }
//! }
//! ```
//!
//! Outputs:
//!
//! ```text
//! new
//! drop
//! new
//! drop
//! ```
//!
//! The `new` prints corresponds to the `FOO()` calls of `_foo1` and `_foo4`.
//! The `drop` prints correspond to the last FOO reference being dropped.
//!

#[macro_export]
macro_rules! weak_static {
    (static $ident:ident : $typ:ty = $init:expr; ) => (
        #[allow(non_snake_case)]
        fn $ident() -> ::std::sync::Arc<$typ> {
            #[warn(non_snake_case)]
            {
                lazy_static! {
                    static ref VALUE: ::std::sync::Mutex<::std::sync::Weak<$typ>> =
                        ::std::default::Default::default();
                }
                
                let mut value = VALUE.lock().unwrap();
                
                value.upgrade().unwrap_or_else(|| {
                    let new_value = ::std::sync::Arc::new($init);

                    *value = ::std::sync::Arc::downgrade(&new_value);
                    
                    new_value
                })
            }
        }
    )
}
