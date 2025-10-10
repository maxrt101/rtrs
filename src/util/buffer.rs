
// pub struct Buffer<const N: usize>([u8; N]);
// 
// impl<const N: usize> Buffer<N> {
//     pub const fn new() -> Self {
//         Self([0; N])
//     }
// 
//     fn check<T>(&self) {
//         assert!(
//             size_of::<T>() <= size_of_val(&self.0),
//             "Value won't fit! ({} <= {})", size_of::<T>(), size_of_val(&self.0)
//         );
// 
//         // TODO: not self not self.0 correctly represent requested alignment
//         // assert!(align_of::<T>() <= align_of_val(&self.0), "Value's alignment is too strict!");
//     }
// 
//     pub fn get_ptr<T>(&self) -> *const T {
//         self.check::<T>();
// 
//         self.0.as_ptr().cast::<T>()
//     }
// 
//     pub fn get_mut_ptr<T>(&self) -> *mut T {
//         self.check::<T>();
// 
//         self.get_ptr::<T>().cast_mut()
//     }
// 
//     pub fn get_ref<T>(&self) -> &T {
//         self.check::<T>();
// 
//         unsafe { &*self.get_ptr::<T>() }
//     }
// 
//     pub fn get_mut<T>(&self) -> &mut T {
//         self.check::<T>();
// 
//         unsafe { &mut *self.get_mut_ptr::<T>() }
//     }
// 
//     pub fn write<T>(&self, src: T) -> &mut T {
//         self.check::<T>();
// 
//         unsafe {
//             core::ptr::write(self.get_mut_ptr(), src);
//             &mut *self.get_mut_ptr::<T>()
//         }
//     }
// }

pub struct Buffer<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Buffer<N> {
    pub const fn new() -> Self {
        Self {
            data: [0; N],
        }
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        N
    }

    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> *const u8 {
        &self.data as *const u8
    }

    #[inline(always)]
    pub unsafe fn as_mut_ptr(&self) -> *mut u8 {
        &self.data as *const _ as *mut u8
    }

    #[inline(always)]
    pub unsafe fn ptr_bounds(&self) -> (*const u8, *const u8) {
        unsafe { (self.as_ptr(), (self.as_ptr() as usize + self.size()) as *const u8) }
    }

    #[inline(always)]
    pub unsafe fn mut_ptr_bounds(&self) -> (*mut u8, *mut u8) {
        unsafe { (self.as_mut_ptr(), (self.as_mut_ptr() as usize + self.size()) as *mut u8) }
    }
}

