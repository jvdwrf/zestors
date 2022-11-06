use super::*;
use std::{any::TypeId, marker::PhantomData};

pub struct Dyn<T: ?Sized>(PhantomData<*const T>);

unsafe impl<T: ?Sized> Send for Dyn<T> {}

unsafe impl<T: ?Sized> Sync for Dyn<T> {}

pub trait DynProtocol {
    fn msg_ids() -> Box<[TypeId]>;
}

pub trait AcceptsDyn<D> {}

macro_rules! dyn_types {
    ($($ident:ident $(<$( $ty:ident ),*>)?),*) => {
        $(
            pub trait $ident< $($($ty: Msg,)?)*>: $($( Accepts<$ty> + )?)* {}

            impl<$($($ty: Msg + 'static,)?)*> DynProtocol for dyn $ident< $($($ty,)?)*> {
                fn msg_ids() -> Box<[TypeId]> {
                    Box::new([$($(TypeId::of::<$ty>(),)?)*])
                }
            }

            impl<D, $($($ty: Msg + 'static,)?)*> AcceptsDyn<Dyn<dyn $ident<$($($ty,)?)*>>> for Dyn<D>
            where
                D: DynProtocol + ?Sized $($( + Accepts<$ty> )?)* {}

            impl<P, $($($ty: Msg + 'static,)?)*> AcceptsDyn<Dyn<dyn $ident<$($($ty,)?)*>>> for P
            where
                P: Protocol $($( + Accepts<$ty> )?)* {}
        )*
    };
}

dyn_types! {
    AcceptsNone,
    AcceptsOne<M1>,
    AcceptsTwo<M1, M2>,
    AcceptsThree<M1, M2, M3>,
    AcceptsFour<M1, M2, M3, M4>,
    AcceptsFive<M1, M2, M3, M4, M5>,
    AcceptsSix<M1, M2, M3, M4, M5, M6>,
    AcceptsSeven<M1, M2, M3, M4, M5, M6, M7>,
    AcceptsEight<M1, M2, M3, M4, M5, M6, M7, M8>,
    AcceptsNine<M1, M2, M3, M4, M5, M6, M7, M8, M9>,
    AcceptsTen<M1, M2, M3, M4, M5, M6, M7, M8, M9, M10>
}

#[macro_export]
macro_rules! AcceptsAll {
    () => {
        $crate::message::Dyn<dyn $crate::message::AcceptsNone>
    };
    ($ty1:ty) => {
        $crate::message::Dyn<dyn $crate::message::AcceptsOne<$ty1>>
    };
    ($ty1:ty, $ty2:ty) => {
        $crate::message::Dyn<dyn $crate::message::AcceptsTwo<$ty1, $ty2>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty) => {
        $crate::message::Dyn<dyn $crate::message::AcceptsThree<$ty1, $ty2, $ty3>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty) => {
        $crate::message::Dyn<dyn $crate::message::AcceptsFour<$ty1, $ty2, $ty3, $ty4>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty) => {
        $crate::message::Dyn<dyn $crate::message::AcceptsFive<$ty1, $ty2, $ty3, $ty4, $ty5>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty) => {
        $crate::message::Dyn<dyn $crate::message::AcceptsSix<$ty1, $ty2, $ty3, $ty4, $ty5, $ty6>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty, $ty7:ty) => {
        $crate::message::Dyn<dyn $crate::message::AcceptsSeven<$ty1, $ty2, $ty3, $ty4, $ty5, $ty6, $ty7>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty, $ty7:ty, $ty8:ty) => {
        $crate::message::Dyn<dyn $crate::message::AcceptsEight<$ty1, $ty2, $ty3, $ty4, $ty5, $ty6, $ty7, $ty8>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty, $ty7:ty, $ty8:ty, $ty9:ty) => {
        $crate::message::Dyn<dyn $crate::message::AcceptsNine<$ty1, $ty2, $ty3, $ty4, $ty5, $ty6, $ty7, $ty8, $ty9>>
    };
    ($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty, $ty7:ty, $ty8:ty, $ty9:ty, $ty10:ty) => {
        $crate::message::Dyn<dyn $crate::message::AcceptsTen<$ty1, $ty2, $ty3, $ty4, $ty5, $ty6, $ty7, $ty8, $ty9, $ty10>>
    };
}

mod test {
    use crate::message::{AcceptsOne, AcceptsTwo, Dyn};

    type _X = AcceptsAll![u32];
    type _XX = Dyn<dyn AcceptsOne<u32>>;
    type _Y = AcceptsAll![u32, ()];
    type _YY = Dyn<dyn AcceptsTwo<u32, ()>>;
}
