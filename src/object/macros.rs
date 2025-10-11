
#[macro_export]
macro_rules! object_with {
    ($name:expr, $type:ty, $obj:ident, $blk:expr) => {{
        let storage = $crate::object::STORAGE.lock();
        let $obj = storage.get::<$type>($name).unwrap();
        $blk
    }};
}

#[macro_export]
macro_rules! object_with_mut {
    ($name:expr, $type:ty, $obj:ident, $blk:expr) => {{
        let storage = $crate::object::STORAGE.lock();
        let mut $obj = storage.get_mut::<$type>($name).unwrap();
        $blk
    }};
}

#[macro_export]
macro_rules! object_insert {
    ($name:expr, $value:expr) => {{
        let mut storage = $crate::object::STORAGE.lock_mut();
        storage.insert($name, $value);
    }};
}

#[macro_export]
macro_rules! object_remove {
    ($name:expr) => {{
        let mut storage = $crate::object::STORAGE.lock_mut();
        storage.remove($name);
    }}
}