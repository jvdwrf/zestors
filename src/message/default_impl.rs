use super::*;
use std::{borrow::Cow, rc::Rc, sync::Arc};

impl<M> MsgKind<M> for () {
    type Sent = M;
    type Returned = ();
    fn create(msg: M) -> (M, ()) {
        (msg, ())
    }
    fn cancel(sends: M, _returns: ()) -> M {
        sends
    }
}

macro_rules! default_base_types {
    ($(
        $ty:ty
    ),*) => {
        $(
            impl Msg for $ty {
                type Kind = ();
            }
        )*
    };
}

default_base_types! {
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
    (),
    String, &'static str
}

macro_rules! default_tuples {
    ($(
        ($($id:ident: $na:ident + $na2:ident),*),
    )*) => {
        $(
            impl<$($id),*> MsgKind<($($id,)*)> for ($($id::Kind,)*)
            where
                $($id: Msg,)*
            {
                type Sent = ($(Sent<$id>,)*);

                type Returned = ($(Returned<$id>,)*);

                fn create(($($na,)*): ($($id,)*)) -> (Self::Sent, Self::Returned) {
                    $( let $na2 = <$id::Kind as MsgKind<$id>>::create($na); )*
                    (($( $na2.0, )*), ($( $na2.1, )*))
                }

                fn cancel(($($na,)*): Self::Sent, ($($na2,)*): Self::Returned) -> ($($id,)*) {
                    ($( <$id::Kind as MsgKind<$id>>::cancel($na, $na2), )*)
                }
            }

            impl<$($id),*> Msg for ($($id,)*)
            where
                $($id: Msg,)*
            {
                type Kind = ();
            }
        )*
    };
}

default_tuples!(
    (M1: m1 + m_1),
    (M1: m1 + m_1, M2: m2 + m_2),
    (M1: m1 + m_1, M2: m2 + m_2, M3: m3 + m_3),
    (M1: m1 + m_1, M2: m2 + m_2, M3: m3 + m_3, M4: m4 + m_4),
    (
        M1: m1 + m_1,
        M2: m2 + m_2,
        M3: m3 + m_3,
        M4: m4 + m_4,
        M5: m5 + m_5
    ),
    (
        M1: m1 + m_1,
        M2: m2 + m_2,
        M3: m3 + m_3,
        M4: m4 + m_4,
        M5: m5 + m_5,
        M6: m6 + m_6
    ),
    (
        M1: m1 + m_1,
        M2: m2 + m_2,
        M3: m3 + m_3,
        M4: m4 + m_4,
        M5: m5 + m_5,
        M6: m6 + m_6,
        M7: m7 + m_7
    ),
    (
        M1: m1 + m_1,
        M2: m2 + m_2,
        M3: m3 + m_3,
        M4: m4 + m_4,
        M5: m5 + m_5,
        M6: m6 + m_6,
        M7: m7 + m_7,
        M8: m8 + m_8
    ),
    (
        M1: m1 + m_1,
        M2: m2 + m_2,
        M3: m3 + m_3,
        M4: m4 + m_4,
        M5: m5 + m_5,
        M6: m6 + m_6,
        M7: m7 + m_7,
        M8: m8 + m_8,
        M9: m9 + m_9
    ),
    (
        M1: m1 + m_1,
        M2: m2 + m_2,
        M3: m3 + m_3,
        M4: m4 + m_4,
        M5: m5 + m_5,
        M6: m6 + m_6,
        M7: m7 + m_7,
        M8: m8 + m_8,
        M9: m9 + m_9,
        M10: m10 + m_10
    ),
);

macro_rules! default_wrappers {
    ($(
        $(:$lf:lifetime)?
        $wrapper:ty
        $(where $_:ty: $where:ident)*
    ,)*) => {
        $(
            impl<$($lf,)? M> Msg for $wrapper
                where M: Msg<Kind = ()> + $($where +)*
            {
                type Kind = ();
            }
        )*
    };
}

default_wrappers!(
    Box<M>,
    Arc<M>,
    Rc<M>,
    Vec<M>,
    Box<[M]>,
    :'a Cow<'a, M> where M: Clone,
);
