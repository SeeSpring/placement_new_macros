use placement_new_macros::init;
use std::mem::MaybeUninit;

fn main() {
    struct S {
        a: u32,
    }
    struct T {
        s: S,
        t: [u32; 5],
    }
    fn foo(x: bool) -> u32 {
        x as _
    }
    let mut v = MaybeUninit::<T>::uninit();
    // missing field `t`
    #[rustfmt::skip]
    init!(v, T { s: S { a: f1 } }, f1 = foo(true));
}
