use std::collections::HashMap;
use std::rc::Rc;

type Path = Vec<String>;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Obj(Obj),
    // Array?
}

#[derive(Clone, Debug, PartialEq)]
pub struct Obj {
    data: Rc<HashMap<String, Value>>,
}

impl Obj {
    pub fn new() -> Self {
        Obj { data: Rc::new(HashMap::new()) }
    }

    pub fn set(self, path: &[impl AsRef<str>], value: Value) -> Self {
        // Dobbiamo navigare il path e creare oggetti intermedi se necessario.
        // Poiché è immutabile, dobbiamo clonare i rami.
        // Implementazione complessa: per semplicità, supponiamo che path sia di un solo livello? No, la macro supporta path annidati.
        // Dovremmo creare una nuova struttura con il percorso aggiornato.
        // Questo è un po' laborioso. Forse è meglio usare un approccio mutabile con Rc e RefCell, ma per semplicità possiamo limitarci a path di un solo livello per ora.
        // In realtà, le macro path! producono un Vec<String>, quindi supportano annidamento. Dobbiamo implementare un set ricorsivo.
        // Per ora, possiamo semplificare: supponiamo che il path sia sempre un singolo segmento. Ma non è vero.
        // Forse è meglio usare una libreria esistente come serde_json::Value, ma vogliamo mantenere le macro.
        // Possiamo definire Obj come un wrapper di serde_json::Map<String, Value> e implementare set ricorsivo.
        // useremo serde_json per la flessibilità.
    }
}
