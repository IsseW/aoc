use paste::paste;
use std::ops::*;

trait Zero {
    fn zero() -> Self;
}

impl<T: Default> Zero for T {
    fn zero() -> Self {
        Self::default()
    }
}

trait One {
    fn one() -> Self;
}

macro_rules! impl_one {
    ($($types:ty), *) => {
        $(
        impl One for $types {
            fn one() -> Self { 1 as $types }
        }
        ) *
    }
}

impl_one!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, isize, usize);

macro_rules! vec_op {
    ($name:ident, $op_name:ident, $op_function:ident, ($($elem:ident), *)) => {
        impl<O: Copy, T: $op_name<O>> $op_name<O> for $name<T> {
            type Output = $name<<T as $op_name<O>>::Output>;
            fn $op_function(self, other: O) -> Self::Output { $name::new($(self.$elem.$op_function(other)), *) }
        }
        impl<O, T: $op_name<O>> $op_name<$name<O>> for $name<T> {
            type Output = $name<<T as $op_name<O>>::Output>;
            fn $op_function(self, other: $name<O>) -> Self::Output { $name::new($(self.$elem.$op_function(other.$elem)), *) }
        }

        paste! {
            impl<O: Copy, T: [< $op_name Assign >]<O>> [< $op_name Assign >]<O> for $name<T> {
                fn [<$op_function _assign >](&mut self, other: O) { $(self.$elem.[<$op_function _assign >](other);) * }
            }
            impl<O, T: [< $op_name Assign >]<O>> [< $op_name Assign >]<$name<O>> for $name<T> {
                fn [<$op_function _assign >](&mut self, other: $name<O>) { $(self.$elem.[<$op_function _assign >](other.$elem);) * }
            }
        }

    };
}

macro_rules! vec_ops {
    ($name:ident, $args:tt; $($op:ident), *) => {
        $(
            paste!{
                vec_op!($name, $op, [< $op:lower >], $args);
            }
        )*
    }
}

macro_rules! vec_solo_op {
    ($name:ident, $op_name:ident, $op_function:ident, ($($elem:ident), *)) => {
        impl<T: $op_name> $op_name for $name<T> {
            type Output = $name<<T as $op_name>::Output>;
            fn $op_function(self) -> Self::Output {
                $name::new($(self.$elem.$op_function()), *)
            }
        }
    };
}

macro_rules! vec_solo_ops {
    ($name:ident, $args:tt; $($op:ident), *) => {
        $(
            paste!{
                vec_solo_op!($name, $op, [< $op:lower >], $args);
            }
        )*
    }
}

macro_rules! vec_trait {
    ($name:ident, $op_name:ident, $op_function:ident, ($($elem:ident), *)) => {
        impl<T: $op_name> $op_name for $name<T> {
            fn $op_function() -> Self {
                Self {
                    $($elem: T::$op_function()), *
                }
            }
        }
    };
}
macro_rules! vec_traits {
    ($name:ident, $args:tt; $($op:ident), *) => {
        $(
            paste!{
                vec_trait!($name, $op, [< $op:lower >], $args);
            }
        )*
    }
}

macro_rules! vector {
    ($name:ident, $($elem:ident), *) => {
        #[derive(Default, Clone, Copy, Debug)]
        struct $name<T> {
            $(pub $elem: T), *
        }
        impl<T> $name<T> {
            fn new($($elem: T), *) -> Self {
                Self { $($elem), * }
            }

            fn dot<O>(self, other: $name<O>) -> <T as Mul<O>>::Output where T: Mul<O>, <T as Mul<O>>::Output : Add<<T as Mul<O>>::Output, Output = <T as Mul<O>>::Output>, <T as Mul<O>>::Output : Zero {
                $(self.$elem * other.$elem + ) * <T as Mul<O>>::Output::zero()
            }

            fn magnitude_sqr(self)  -> <T as Mul<T>>::Output where T: Mul<T>, <T as Mul<T>>::Output : Add<<T as Mul<T>>::Output, Output = <T as Mul<T>>::Output>, <T as Mul<T>>::Output : Zero {
                self.dot(self)
            }
        }
        vec_ops!($name, ($($elem), *); Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr);
        vec_solo_ops!($name, ($($elem), *); Neg, Not);
        vec_traits!($name, ($($elem), *); One);
    };
}

vector!(Vec2, x, y);
vector!(Vec3, x, y, z);
vector!(Vec4, x, y, z, w);
