pub trait Increment: Clone + Sized {
    fn increment(&mut self) -> Option<Self>;
}

macro_rules! for_each_integer {
    ($m:ident) => {
        $m!(i8);
        $m!(i16);
        $m!(i32);
        $m!(i64);
        $m!(u8);
        $m!(u16);
        $m!(u32);
        $m!(u64);
    };
}

macro_rules! impl_increment {
    ($ty:ty) => {
        impl Increment for $ty {
            fn increment(&mut self) -> Option<Self> {
                self.checked_add(1)
                    .map(|next| std::mem::replace(self, next))
            }
        }
    };
}

for_each_integer!(impl_increment);
