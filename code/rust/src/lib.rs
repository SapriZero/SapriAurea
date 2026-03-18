pub mod core;
pub mod fp;
pub mod calc;
pub mod def;
pub mod urcm;  // <-- nuovo modulo

pub use core::{Obj, Value};
pub use fp::*;
pub use calc::*;
pub use def::*;
pub use urcm::SapriAurea;


/*
// ==========================================
// Macro per la creazione di percorsi (path!)
// ==========================================

#[macro_export]
macro_rules! path {
    ($seg:ident) => { vec![stringify!($seg).to_string()] };
    ($first:ident.$($rest:ident).*) => {
        {
            let mut __path = vec![stringify!($first).to_string()];
            $(
                __path.push(stringify!($rest).to_string());
            )*
            __path
        }
    };
    ($s:expr) => { vec![$s.to_string()] };
}

// ==========================================
// Macro per la creazione di oggetti (obj!)
// ==========================================

#[macro_export]
macro_rules! obj {
    // Versione base: obj!({ chiave: valore, ... })
    ({ $($key:tt : $val:expr),* $(,)? }) => {
        {
            let mut __obj = $crate::core::Obj::new();
            $(
                __obj = __obj.set(&$crate::path!($key), $crate::core::Value::from($val));
            )*
            __obj
        }
    };
    // Con default: obj!(default_obj => { ... })
    ($default:expr => { $($key:tt : $val:expr),* $(,)? }) => {
        {
            let mut __obj = $default.clone();
            $(
                __obj = __obj.set(&$crate::path!($key), $crate::core::Value::from($val));
            )*
            __obj
        }
    };
    // Con contesto: obj!(ctx, { ... }) (simile alla base)
    ($ctx:expr, { $($key:tt : $val:expr),* $(,)? }) => {
        {
            let mut __obj = $crate::core::Obj::new();
            $(
                __obj = __obj.set(&$crate::path!($key), $crate::core::Value::from($val));
            )*
            __obj
        }
    };
    // Currying: obj!([key1, key2] => valore) restituisce una funzione
    ([$($key:expr),*] => $val:expr) => {
        move |obj: $crate::core::Obj| obj.set(&[$($key),*], $crate::core::Value::from($val))
    };
}

// ==========================================
// Macro cascade! (ereditarietà di oggetti)
// ==========================================

#[macro_export]
macro_rules! cascade {
    ($first:expr $(, $rest:expr)*) => {
        {
            let mut __result = $first.clone();
            $(
                __result = __result.merge($rest.clone());
            )*
            __result
        }
    };
}

// ==========================================
// Macro struct_with_keys! (genera struct con estrazione da Obj)
// ==========================================

#[macro_export]
macro_rules! struct_with_keys {
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident {
            $($field:ident : $type:ty),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $( pub $field: $type ),*
        }

        impl $name {
            pub fn from_obj(obj: &$crate::core::Obj) -> Result<Self, String> {
                Ok(Self {
                    $(
                        $field: obj
                            .get(&$crate::path!($name.$field))
                            .ok_or_else(|| format!("Missing key: {}", stringify!($name.$field)))?
                            .clone()
                            .try_into()?,
                    )*
                })
            }

            pub fn try_from_obj(obj: &$crate::core::Obj) -> Option<Self> {
                Some(Self {
                    $(
                        $field: obj.get(&$crate::path!($name.$field))?.clone().try_into().ok()?,
                    )*
                })
            }
        }
    };
}
*/
