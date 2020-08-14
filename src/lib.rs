#![feature(new_uninit)]
#![feature(raw_ref_op)]

/// Initalizes a struct in place; unsound with packed structs
/// ```
/// # use core::mem::MaybeUninit;
/// # use placement_new_macros::init;
/// struct S {
///     a: u32,
/// }
/// struct T {
///     s: S,
///     t: [u32; 5],
/// }
/// # /*
/// fn foo(x: bool) -> u32 { ... }
/// # */
/// # fn foo(x: bool) -> u32 {
/// #   x as _
/// # }
/// let mut v = MaybeUninit::<T>::uninit();
/// # #[rustfmt::skip]
/// init!(v, T { s: S { a: f1 }, t: [f2; 5] }, f1 = foo(true), f2 = foo(false));
/// ```
#[macro_export]
macro_rules! init {
    ($($tt:tt)*) => {
        $crate::__init_impl!{[$crate::__write_aligned!] $($tt)*}
    }
}

/// Initalizes a struct in place
/// ```
/// # use core::mem::MaybeUninit;
/// # use placement_new_macros::init_unaligned;
/// struct S {
///     a: u8,
/// }
/// #[repr(packed)]
/// struct T {
///     s: S,
///     t: [u32; 5],
/// }
/// # /*
/// fn foo(x: bool) -> u32 { ... }
/// # */
/// # fn foo(x: bool) -> u32 {
/// #   x as _
/// # }
/// let mut v = MaybeUninit::<T>::uninit();
/// # #[rustfmt::skip]
/// init_unaligned!(v, T { s: S { a: 0 }, t: [f1; 5] }, f1 = foo(true));
/// ```
#[macro_export]
macro_rules! init_unaligned {
    ($($tt:tt)*) => {
        $crate::__init_impl!{[$crate::__write_unaligned!] $($tt)*}
    }
}

/// Makes a box in place
/// ```
/// #![feature(new_uninit)]
/// # use placement_new_macros::boxed;
/// struct S {
///     a: u32,
/// }
/// # /*
/// fn foo(x: bool) -> u32 { ... }
/// # */
/// # fn foo(x: bool) -> u32 {
/// #   x as _
/// # }
/// let _: Box<S> = boxed!(S { a: f1 }, f1 = foo(true));
/// ```
#[macro_export]
macro_rules! boxed {
    ($($tt:tt)*) => {
        {
            let mut b = Box::new_uninit();
            $crate::init!(b.as_mut(), $($tt)*);
            unsafe { b.assume_init() }
        }
    }
}

/// The actual implementation of the macro
#[macro_export]
#[doc(hidden)]
macro_rules! __init_impl {
    ([$($write:tt)*] $uninit:expr, $outer:ident {$($xs:tt)*} $(, $bind:pat = $val:expr)* $(,)?) => {{
        $(let $bind = $val;)*
        let v_mut_ptr = ::core::mem::MaybeUninit::<$outer>::as_mut_ptr(&mut $uninit);
        $crate::__init_impl!{@verify $outer {$($xs)*}};
        unsafe {
            $crate::__init_impl!{@parse [$($write)*] v_mut_ptr; $($xs)*}
        }
    }};

    (@parse [$($write:tt)*] $uninit:ident; $field:ident : $outer:ident { $($xs:tt)* } $(, $($ys:tt)*)? ) => {
        $crate::__init_impl!{@verify $outer { $($xs)*}};
        let s_mut_ptr = &mut (*$uninit).$field as *mut $outer;
        $crate::__init_impl!{@parse [$($write)*] s_mut_ptr; $($xs)*}
        $crate::__init_impl!{@parse [$($write)*] $uninit; $($($ys)*)?}
    };
    (@parse [$($write:tt)*] $uninit:ident; $field:ident : [$val:ident; $count:literal] $(, $($ys:tt)*)? ) => {
        $($write)* ($uninit, $field, [$val; $count]);
        $crate::__init_impl!{@parse [$($write)*] $uninit; $($($ys)*)?}
    };
    (@parse [$($write:tt)*] $uninit:ident; $field:ident : [$val:literal; $count:literal] $(, $($ys:tt)*)? ) => {
        $($write)* ($uninit, $field, [$val; $count]);
        $crate::__init_impl!{@parse [$($write)*] $uninit; $($($ys)*)?}
    };
    (@parse [$($write:tt)*] $uninit:ident; $field:ident : $val:ident $(, $($ys:tt)*)? ) => {
        $($write)* ($uninit, $field, $val);
        $crate::__init_impl!{@parse [$($write)*] $uninit; $($($ys)*)?}
    };
    (@parse [$($write:tt)*] $uninit:ident; $field:ident : $val:literal $(, $($ys:tt)*)? ) => {
        $($write)* ($uninit, $field, $val);
        $crate::__init_impl!{@parse [$($write)*] $uninit; $($($ys)*)?}
    };
    (@parse [$($write:tt)*] $uninit:ident; ) => {};
    (@verify $outer:ident { $($field:ident : $expr:expr),* $(,)?} ) => {
        #[allow(unreachable_code)]
        if false {
            let _ = $outer {$($field: loop {},)*};
        }
    };
}

// TODO: are references to uninitialized memory allowed?

/// Helper for aligned reads
#[macro_export]
#[doc(hidden)]
macro_rules! __write_aligned {
    ($uninit:ident, $field:ident, $val:expr) => {
        ::core::ptr::write(&mut (*$uninit).$field, $val);
    };
}

/// Helper for unaligned reads
#[macro_export]
#[doc(hidden)]
macro_rules! __write_unaligned {
    ($uninit:ident, $field:ident, $val:expr) => {
        ::core::ptr::write_unaligned(&mut (*$uninit).$field, $val);
    };
}
