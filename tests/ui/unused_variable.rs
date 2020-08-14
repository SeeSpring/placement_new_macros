use placement_new_macros::init;
use std::mem::MaybeUninit;

#[deny(unused_variables)]
fn main() {
    struct T {
        t: u32,
    }
    let mut v = MaybeUninit::<T>::uninit();
    init!(v, T { t: 0 }, f1 = 0);
}
