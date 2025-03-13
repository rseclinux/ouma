macro_rules! cfg_if {
    ($(
        if #[cfg($($meta:meta),*)] { $($it:item)* }
    ) else * else {
        $($it2:item)*
    }) => {
        cfg_if! {
            @__items
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            ( () ($($it2)*) ),
        }
    };
    (
        if #[cfg($($i_met:meta),*)] { $($i_it:item)* }
        $(
            else if #[cfg($($e_met:meta),*)] { $($e_it:item)* }
        )*
    ) => {
        cfg_if! {
            @__items
            () ;
            ( ($($i_met),*) ($($i_it)*) ),
            $( ( ($($e_met),*) ($($e_it)*) ), )*
            ( () () ),
        }
    };

    (@__items ($($not:meta,)*) ; ) => {};
    (@__items ($($not:meta,)*) ; ( ($($m:meta),*) ($($it:item)*) ),
     $($rest:tt)*) => {
        cfg_if! { @__apply cfg(all($($m,)* not(any($($not),*)))), $($it)* }
        cfg_if! { @__items ($($not,)* $($m,)*) ; $($rest)* }
    };

    (@__apply $m:meta, $($it:item)*) => {
        $(#[$m] $it)*
    };
}

macro_rules! s {
    ($(
        $(#[$attr:meta])*
        pub $t:ident $i:ident { $($field:tt)* }
    )*) => ($(
        s!(it: $(#[$attr])* pub $t $i { $($field)* });
    )*);

    (it: $(#[$attr:meta])* pub union $i:ident { $($field:tt)* }) => (
compile_error!("unions cannot derive extra traits");
    );

    (it: $(#[$attr:meta])* pub struct $i:ident { $($field:tt)* }) => (
        __item! {
            #[repr(C)]
            #[::core::prelude::v1::derive(Debug, Eq, Hash, PartialEq)]
#[::core::prelude::v1::derive(::core::clone::Clone, ::core::marker::Copy)]
#[allow(deprecated)]
$(#[$attr])*
pub struct $i { $($field)* }
        }
    );
}

macro_rules! s_paren {
    ($(
        $(#[$attr:meta])*
        pub struct $i:ident ( $($field:tt)* );
    )*) => ($(
        __item! {
            #[::core::prelude::v1::derive(Debug, Eq, Hash, PartialEq)]
#[::core::prelude::v1::derive(::core::clone::Clone, ::core::marker::Copy)]
 $(#[$attr])*
pub struct $i ( $($field)* );
        }
    )*);
}

macro_rules! s_no_extra_traits {
    ($(
        $(#[$attr:meta])*
        pub $t:ident $i:ident { $($field:tt)* }
    )*) => ($(
        s_no_extra_traits!(it: $(#[$attr])* pub $t $i { $($field)* });
    )*);

    (it: $(#[$attr:meta])* pub union $i:ident { $($field:tt)* }) => (
        __item! {
 #[repr(C)]
 #[::core::prelude::v1::derive(::core::clone::Clone, ::core::marker::Copy)]
$(#[$attr])*
 pub union $i { $($field)* }
        }

        impl ::core::fmt::Debug for $i {
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
f.debug_struct(::core::stringify!($i)).finish_non_exhaustive()
}
        }
    );

    (it: $(#[$attr:meta])* pub struct $i:ident { $($field:tt)* }) => (
        __item! {
#[repr(C)]
 #[::core::prelude::v1::derive(::core::clone::Clone, ::core::marker::Copy)]
$(#[$attr])*
            pub struct $i { $($field)* }
        }
    );
}
