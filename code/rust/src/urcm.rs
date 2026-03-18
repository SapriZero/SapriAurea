use crate::core::Obj;
use crate::calc::{self, PHI};
use crate::obj; // <-- import necessario
use std::f64::consts::PI;

// ... resto invariato ...

/// Struttura che rappresenta la formula Sapri Aurea a livelli (cipolla URCM)
#[derive(Debug, Clone)]
pub struct SapriAurea {
    /// Input
    pub k: f64,
    /// Livello 1: modulo complementare
    pub c: f64,
    /// Livello 2: media aritmetica
    pub m: f64,
    /// Livello 3: esponente aureo
    pub e: f64,
    /// Livello 3: potenza aurea
    pub a: f64,
    /// Costante: π/2
    pub p: f64,
    /// Livello 4: correzione δ
    pub d: f64,
    /// Livello 5: fattore di correzione
    pub u: f64,
    /// Livello 6: risultato finale
    pub K: f64,
}

impl SapriAurea {
    /// Crea una nuova istanza con δ interpolato dalla tabella (veloce)
    pub fn new(k: f64) -> Self {
        let p = PI / 2.0;
        let e = 1.0 / PHI;
        let c = (1.0 - k * k).sqrt();
        let m = (1.0 + c) / 2.0;
        let a = m.powf(e);
        let d = if k > 0.0 { calc::delta_interpolate(k) } else { 0.0 };
        let u = 1.0 + d;
        let K = p * a * u;
        Self {
            k,
            c,
            m,
            e,
            a,
            p,
            d,
            u,
            K,
        }
    }

    /// Crea una nuova istanza con δ esatto (usa AGM, più lento)
    pub fn with_exact_delta(k: f64) -> Self {
        let p = PI / 2.0;
        let e = 1.0 / PHI;
        let c = (1.0 - k * k).sqrt();
        let m = (1.0 + c) / 2.0;
        let a = m.powf(e);
        let d = if k > 0.0 { calc::delta_exact(k) } else { 0.0 };
        let u = 1.0 + d;
        let K = p * a * u;
        Self {
            k,
            c,
            m,
            e,
            a,
            p,
            d,
            u,
            K,
        }
    }

    /// Restituisce un oggetto con tutti i valori e le descrizioni geometriche
    pub fn to_obj(&self) -> Obj {
        obj!({
            // Valori numerici
            k: self.k,
            c: self.c,
            m: self.m,
            e: self.e,
            a: self.a,
            p: self.p,
            d: self.d,
            u: self.u,
            K: self.K,

            // Combinazioni a due
            pa: self.p * self.a,
            pu: self.p * self.u,
            au: self.a * self.u,

            // Combinazione a tre
            pau: self.K,

            // Reciproci
            inv_p: 2.0 / PI,
            inv_a: self.a.powf(-1.0),
            inv_u: 1.0 / self.u,

            // Altre combinazioni (con sotto-livelli)
            ca: self.c * self.a,
            ma: self.m * self.a,
            cm: self.c * self.m,
            ce: self.c * self.e,
            me: self.m * self.e,

            // Descrizioni geometriche (stringhe)
            desc_p: "Il ciclo dimezzato: chiusura del semicerchio",
            desc_a: "La spirale aurea: crescita geometrica con esponente 1/φ",
            desc_u: "Il riporto: ciò che si accumula dopo ogni ciclo, impedisce la cristallizzazione",
            desc_pa: "Spirale chiusa (approssimazione base)",
            desc_pu: "Ciclo corretto (tempo con memoria)",
            desc_au: "Anima dell'ellisse (nucleo normalizzato)",
            desc_pau: "Ellisse realizzata (integrale completo)",
            desc_c: "Modulo complementare, polarità (2) applicata allo spazio",
            desc_m: "Media aritmetica, equilibrio tra 1 e c",
            desc_e: "Inverso del numero aureo, 1/φ",
        })
    }

    /// Stampa una tabella con tutti i livelli e le combinazioni
    pub fn print_info(&self) {
        println!("\n=== Sapri Aurea Formula (URCM) ===");
        println!("k (input)               = {:.10}", self.k);
        println!("c = √(1−k²)              = {:.10}", self.c);
        println!("m = (1 + c)/2            = {:.10}", self.m);
        println!("e = 1/φ                  = {:.10}", self.e);
        println!("a = m^e                  = {:.10}", self.a);
        println!("p = π/2                  = {:.10}", self.p);
        println!("d = δ(k)                 = {:.10}", self.d);
        println!("u = 1 + d                = {:.10}", self.u);
        println!("K = p·a·u                 = {:.10}", self.K);
        println!("\n--- Combinazioni ---");
        println!("pa = p·a  (spirale chiusa)        = {:.10}", self.p * self.a);
        println!("pu = p·u  (ciclo corretto)        = {:.10}", self.p * self.u);
        println!("au = a·u  (anima ellisse)         = {:.10}", self.a * self.u);
        println!("pau = K   (ellisse realizzata)    = {:.10}", self.K);
        println!("inv_p = 2/π (apertura)            = {:.10}", 2.0 / PI);
        println!("inv_a = 1/a (spirale inversa)     = {:.10}", self.a.powf(-1.0));
        println!("inv_u = 1/(1+δ) (purezza)         = {:.10}", 1.0 / self.u);
    }
}
