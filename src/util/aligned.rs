
#[repr(align(1))]
pub struct Aligned1<T>(pub T);

#[repr(align(2))]
pub struct Aligned2<T>(pub T);

#[repr(align(4))]
pub struct Aligned4<T>(pub T);

#[repr(align(8))]
pub struct Aligned8<T>(pub T);

#[repr(align(16))]
pub struct Aligned16<T>(pub T);

#[repr(align(32))]
pub struct Aligned32<T>(pub T);
