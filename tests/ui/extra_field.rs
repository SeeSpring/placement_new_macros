use placement_new_macros::init;
use std::mem::MaybeUninit;

fn main() {
    struct T {
        t: u32,
    }
    let mut v = MaybeUninit::<T>::uninit();
    // missing field `t`
    init!(v, T { t: 0, s: 0 });
}
