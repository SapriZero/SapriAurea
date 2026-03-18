use crate::struct_with_keys;

// Esempio di struttura generata con le macro
struct_with_keys! {
    #[derive(Debug, Clone)]
    pub SapriParams {
        phi: f64,
        pi: f64,
        a: f64,
        b: f64,
        c: f64,
    }
}

// Altre strutture possono essere definite qui
