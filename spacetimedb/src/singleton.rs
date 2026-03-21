use std::fmt::Display;

#[derive(
    spacetimedb::SpacetimeType, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Singleton {
    #[default]
    Singleton,
}

impl Display for Singleton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Singleton")
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! _singleton {
    (table_macro{ $($full:tt)* }{ #[$(spacetimedb::)?table$($args:tt)?] $_:item }) => {
        $crate::_singleton!(accessor{ $($full)* }{ $($args)? });
    };
    (table_macro{ $($full:tt)* }{ #[$_:meta] $($rest:tt)* }) => { $crate::_singleton!(table_macro{ $($full)* }{ $($rest)* }); };
    (table_macro$_:tt$__:tt) => { compile_error!("Missing `spacetimedb::table` macro"); };

    (accessor{ $($full:tt)* }{ ( accessor = $accessor:ident $(, $($_:meta),*)? ) }) => {
        $crate::_singleton!(table{ $($full)* });
    };
    (accessor{ $($full:tt)* }{ ( $_:meta $(, $($args:tt)*)? ) }) => { $crate::_singleton!(accessor{ $($full)* }{ ( $($($args)*)? ) }); };
    (accessor$_:tt$__:tt) => { compile_error!("Missing `accessor` table parameter"); };

    (table{ $(#[$meta:meta])* $vis:vis struct $name:ident { $($body:tt)* } }) => {
        ::respan::call_site! {
            $(#[$meta])*
            $vis struct $name {
                #[primary_key]
                #[create_wrapper]
                singleton: crate::singleton::Singleton,

                $($body)*
            }
        }

        impl From<()> for ::preinterpret::preinterpret!([!ident_camel! $name Singleton]) {
            fn from(_: ()) -> Self {
                Self::default()
            }
        }
    };
}

#[macro_export]
macro_rules! singleton {
    ($($table:tt)*) => {
        $crate::_singleton!(table_macro{ $($table)* }{ $($table)* });
    };
}
