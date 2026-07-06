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

macro_rules! mask_trailing_ones {
  ($T:ty, $count:expr) => {{
    const BITS: u32 = core::mem::size_of::<$T>() as u32 * 8;
    if $count == 0 {
      0 as $T
    } else if $count >= BITS {
      <$T>::MAX
    } else {
      <$T>::MAX >> (BITS - $count)
    }
  }};
}

macro_rules! impl_dragon_int {
  ($t:ty) => {
    impl crate::support::string::conversion::ftoa::DragonInt for $t {
      const ZERO: Self = n!(0);
      const ONE: Self = n!(1);
      const TEN: Self = n!(10);
    }
  };
}
