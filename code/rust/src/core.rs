use std::collections::HashMap;

// ==========================================
// Tipi di valore supportati
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Obj(Obj),
    Array(Vec<Value>),
}

// Conversioni dai tipi base
impl From<f64> for Value { fn from(f: f64) -> Self { Value::Float(f) } }
impl From<i64> for Value { fn from(i: i64) -> Self { Value::Int(i) } }
impl From<bool> for Value { fn from(b: bool) -> Self { Value::Bool(b) } }
impl From<String> for Value { fn from(s: String) -> Self { Value::String(s) } }
impl From<&str> for Value { fn from(s: &str) -> Self { Value::String(s.to_string()) } }

// TryInto per estrarre valori tipati
impl TryInto<f64> for Value {
    type Error = String;
    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Value::Float(f) => Ok(f),
            Value::Int(i) => Ok(i as f64),
            _ => Err(format!("Cannot convert {:?} to f64", self)),
        }
    }
}
impl TryInto<i64> for Value {
    type Error = String;
    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Value::Int(i) => Ok(i),
            Value::Float(f) if f.fract() == 0.0 => Ok(f as i64),
            _ => Err(format!("Cannot convert {:?} to i64", self)),
        }
    }
}
impl TryInto<bool> for Value {
    type Error = String;
    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Value::Bool(b) => Ok(b),
            _ => Err(format!("Cannot convert {:?} to bool", self)),
        }
    }
}
impl TryInto<String> for Value {
    type Error = String;
    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(format!("Cannot convert {:?} to String", self)),
        }
    }
}

// ==========================================
// Oggetto principale (mappa di valori)
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub struct Obj {
    data: HashMap<String, Value>,
}

impl Obj {
    pub fn new() -> Self {
        Obj { data: HashMap::new() }
    }

    /// Imposta un valore lungo un percorso (versione funzionale ricorsiva)
    pub fn set(self, path: &[impl AsRef<str>], value: Value) -> Self {
        let path: Vec<String> = path.iter().map(|s| s.as_ref().to_string()).collect();
        Obj {
            data: Self::set_rec(self.data, path, value),
        }
    }

    fn set_rec(mut data: HashMap<String, Value>, mut path: Vec<String>, value: Value) -> HashMap<String, Value> {
        match path.len() {
            1 => {
                data.insert(path[0].clone(), value);
                data
            }
            _ => {
                let seg = path.remove(0);
                let entry = data.entry(seg.clone()).or_insert_with(|| Value::Obj(Obj::new()));
                match entry {
                    Value::Obj(obj) => {
                        // Prendi i dati dell'oggetto interno (senza clonare)
                        let inner_data = std::mem::take(&mut obj.data);
                        let new_inner_data = Self::set_rec(inner_data, path, value);
                        obj.data = new_inner_data;
                        data
                    }
                    _ => {
                        // Sostituisci con un nuovo oggetto
                        let new_data = Self::set_rec(HashMap::new(), path, value);
                        *entry = Value::Obj(Obj { data: new_data });
                        data
                    }
                }
            }
        }
    }

    /// Ottiene un valore lungo un percorso (versione funzionale)
    pub fn get(&self, path: &[impl AsRef<str>]) -> Option<&Value> {
        let mut current = &self.data;
        let mut path_iter = path.iter().peekable();

        while let Some(seg) = path_iter.next() {
            let seg = seg.as_ref();
            match current.get(seg) {
                Some(Value::Obj(obj)) => {
                    current = &obj.data;
                }
                Some(v) => {
                    if path_iter.peek().is_none() {
                        return Some(v);
                    } else {
                        return None;
                    }
                }
                None => return None,
            }
        }
        None
    }

    /// Unisce due oggetti (merge ricorsivo)
    pub fn merge(mut self, other: Obj) -> Self {
        fn merge_rec(this: &mut HashMap<String, Value>, other: HashMap<String, Value>) {
            for (k, v) in other {
                match (this.get_mut(&k), v) {
                    (Some(Value::Obj(this_obj)), Value::Obj(other_obj)) => {
                        merge_rec(&mut this_obj.data, other_obj.data);
                    }
                    (_, v) => {
                        this.insert(k, v);
                    }
                }
            }
        }
        merge_rec(&mut self.data, other.data);
        self
    }
}

impl Default for Obj {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// Macro (invariate)
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

#[macro_export]
macro_rules! obj {
    ({ $($key:tt : $val:expr),* $(,)? }) => {
        {
            let mut __obj = $crate::core::Obj::new();
            $(
                __obj = __obj.set(&$crate::path!($key), $crate::core::Value::from($val));
            )*
            __obj
        }
    };
    ($default:expr => { $($key:tt : $val:expr),* $(,)? }) => {
        {
            let mut __obj = $default.clone();
            $(
                __obj = __obj.set(&$crate::path!($key), $crate::core::Value::from($val));
            )*
            __obj
        }
    };
    ($ctx:expr, { $($key:tt : $val:expr),* $(,)? }) => {
        {
            let mut __obj = $crate::core::Obj::new();
            $(
                __obj = __obj.set(&$crate::path!($key), $crate::core::Value::from($val));
            )*
            __obj
        }
    };
    ([$($key:expr),*] => $val:expr) => {
        move |obj: $crate::core::Obj| obj.set(&[$($key),*], $crate::core::Value::from($val))
    };
}

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
