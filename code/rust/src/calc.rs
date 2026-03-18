use std::f64::consts::PI;
pub const PHI: f64 = 1.618033988749895_f64; // (1 + sqrt(5))/2


pub const DELTA_STEP: f64 = 0.01;
pub const DELTA_TABLE_SIZE: usize = 100;
pub static DELTA_TABLE: [f64; DELTA_TABLE_SIZE] = [
    0.000000, 0.000406, 0.001624, 0.003655, 0.006501, 0.010163, 0.014644, 0.019947, 0.026074, 0.033030,
    0.040819, 0.049446, 0.058917, 0.069236, 0.080409, 0.092442, 0.105339, 0.119106, 0.133749, 0.149273,
    0.165683, 0.182984, 0.201181, 0.220279, 0.240283, 0.261198, 0.283030, 0.305785, 0.329469, 0.354089,
    0.379651, 0.406163, 0.433633, 0.462071, 0.491488, 0.521897, 0.553312, 0.585751, 0.619235, 0.653790,
    0.689445, 0.726236, 0.764207, 0.803407, 0.843896, 0.885742, 0.929026, 0.973845, 1.020311, 1.068556,
    1.118733, 1.171016, 1.225604, 1.282722, 1.342627, 1.405612, 1.472008, 1.542190, 1.616579, 1.695656,
    1.779969, 1.870138, 1.966867, 2.070958, 2.183326, 2.305012, 2.437194, 2.581202, 2.738539, 2.910902,
    3.100208, 3.308609, 3.538517, 3.792621, 4.073909, 4.385696, 4.731650, 5.115810, 5.542597, 6.016819,
    6.543691, 7.128820, 7.778177, 8.498080, 9.295115, 10.176077, 11.147809, 12.217099, 13.390496, 14.674059,
    16.073098, 17.591818, 19.232993, 20.997575, 22.884257, 24.888889, 27.004197, 29.219162, 31.519120, 33.885393,
];

/// Interpolazione lineare di delta
pub const DELTA_TABLE_K: [f64; 12] = [
    0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99,
];
pub const DELTA_TABLE_D: [f64; 12] = [
    0.000000, 0.004070, 0.016591, 0.038564, 0.071935, 0.120170,
    0.189556, 0.292521, 0.458084, 0.781805, 1.139392, 2.022778,
];

// Tabella ultra-ridotta per preview veloci (5 punti)
pub const FAST_TABLE_K: [f64; 5] = [0.0, 0.3, 0.6, 0.9, 0.99];
pub const FAST_TABLE_D: [f64; 5] = [0.000000, 0.038564, 0.189556, 0.781805, 2.022778];

// Tabella standard (12 punti)
pub const STD_TABLE_K: [f64; 12] = [
    0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99,
];
pub const STD_TABLE_D: [f64; 12] = [
    0.000000, 0.004070, 0.016591, 0.038564, 0.071935, 0.120170,
    0.189556, 0.292521, 0.458084, 0.781805, 1.139392, 2.022778,
];

// ==========================================
// Funzioni principali
// ==========================================

/// K(k) tramite AGM (usata come riferimento)
pub fn k_agm(k: f64, iterations: usize) -> f64 {
    if k == 0.0 {
        return PI / 2.0;
    }
    let mut a = 1.0;
    let mut b = (1.0 - k * k).sqrt();
    for _ in 0..iterations {
        let a_new = (a + b) / 2.0;
        let b_new = (a * b).sqrt();
        a = a_new;
        b = b_new;
    }
    PI / (2.0 * a)
}

/// Base della formula Sapri (senza correzione)
pub fn k_sapri_base(k: f64) -> f64 {
    if k == 0.0 {
        return PI / 2.0;
    }
    let kp = (1.0 - k * k).sqrt();
    (PI / 2.0) * ((1.0 + kp) / 2.0).powf(1.0 / PHI)
}

/// Delta esatto (teorema 1) – usa AGM come riferimento
pub fn delta_exact(k: f64) -> f64 {
    if k == 0.0 {
        return 0.0;
    }
    let kv = k_agm(k, 10);
    let ks = k_sapri_base(k);
    kv / ks - 1.0
}

// ==========================================
// Tabella delta (dal paper)
// ==========================================

pub fn delta_interpolate(k: f64) -> f64 {
    if k <= 0.0 {
        return 0.0;
    }
    if k >= DELTA_TABLE_K[11] {
        return DELTA_TABLE_D[11];
    }
    for i in 0..DELTA_TABLE_K.len() - 1 {
        let k0 = DELTA_TABLE_K[i];
        let k1 = DELTA_TABLE_K[i + 1];
        if k >= k0 && k <= k1 {
            let t = (k - k0) / (k1 - k0);
            return DELTA_TABLE_D[i] * (1.0 - t) + DELTA_TABLE_D[i + 1] * t;
        }
    }
    DELTA_TABLE_D[11]
}

fn interpolate_table(k: f64, table_k: &[f64], table_d: &[f64]) -> f64 {
    if k <= 0.0 { return 0.0; }
    if k >= table_k[table_k.len() - 1] { return table_d[table_d.len() - 1]; }
    for i in 0..table_k.len() - 1 {
        let k0 = table_k[i];
        let k1 = table_k[i + 1];
        if k >= k0 && k <= k1 {
            let t = (k - k0) / (k1 - k0);
            return table_d[i] * (1.0 - t) + table_d[i + 1] * t;
        }
    }
    table_d[table_d.len() - 1]
}

pub fn delta_fast(k: f64) -> f64 {
    interpolate_table(k, &FAST_TABLE_K, &FAST_TABLE_D)
}

pub fn delta_standard(k: f64) -> f64 {
    interpolate_table(k, &STD_TABLE_K, &STD_TABLE_D)
}

/// Formula Sapri completa (con tabella)
pub fn k_sapri(k: f64, use_delta: bool) -> f64 {
    let base = k_sapri_base(k);
    if !use_delta || k == 0.0 {
        base
    } else {
        let delta = delta_interpolate(k);
        base * (1.0 + delta)
    }
}

pub fn k_sapri_ultrafast(k: f64) -> f64 {
    k_sapri_base(k)  // δ = 0
}

pub fn k_sapri_fast(k: f64) -> f64 {
    let base = k_sapri_base(k);
    if k == 0.0 { base } else { base * (1.0 + delta_fast(k)) }
}

pub fn k_sapri_standard(k: f64) -> f64 {
    let base = k_sapri_base(k);
    if k == 0.0 { base } else { base * (1.0 + delta_standard(k)) }
}

pub fn k_sapri_exact(k: f64) -> f64 {
    let base = k_sapri_base(k);
    if k == 0.0 { base } else { base * (1.0 + delta_exact(k)) }
}

// ==========================================
// Serie di delta (fino a k⁶)
// ==========================================

pub fn delta_series(k: f64, order: usize) -> f64 {
    let z = k * k;
    let c1 = PHI / 4.0;
    let c2 = 1.0 / 4.0;
    let c3 = 49.0 / 384.0 + 1.0 / (12.0 * PHI);
    let mut res = c1 * z;
    if order >= 2 {
        res += c2 * z * z;
    }
    if order >= 3 {
        res += c3 * z * z * z;
    }
    res
}

// ==========================================
// Funzione ipergeometrica F(1/φ, -1/4; 1; z)
// ==========================================

pub fn f_phi(z: f64, terms: usize) -> f64 {
    if z == 0.0 {
        return 1.0;
    }
    let a = 1.0 / PHI;
    let b = -0.25;
    let mut sum = 1.0;
    let mut term = 1.0;
    for n in 1..terms {
        term *= (a + n as f64 - 1.0) * (b + n as f64 - 1.0) / (n as f64 * n as f64) * z;
        sum += term;
        if term.abs() < 1e-16 {
            break;
        }
    }
    sum
}

// ==========================================
// Trasformazione di Eulero (Teorema 3)
// ==========================================

pub fn f_euler_transformed(z: f64, terms: usize) -> f64 {
    if z == 0.0 {
        return 1.0;
    }
    let gamma = 5.0 / 4.0 - 1.0 / PHI;
    let a2 = 1.0 / (PHI * PHI);
    let b2 = 5.0 / 4.0;
    let mut sum = 1.0;
    let mut term = 1.0;
    for n in 1..terms {
        term *= (a2 + n as f64 - 1.0) * (b2 + n as f64 - 1.0) / (n as f64 * n as f64) * z;
        sum += term;
        if term.abs() < 1e-16 {
            break;
        }
    }
    (1.0 - z).powf(gamma) * sum
}

// ==========================================
//  FUNZIONI REAL TIME 
// ==========================================

/// Versione ibrida con un passo AGM (Formula 8)
/// Operazioni: ~12, precisione intermedia tra fast e standard.
pub fn k_sapri_hybrid(k: f64) -> f64 {
    if k == 0.0 {
        return PI / 2.0;
    }
    let kp = (1.0 - k * k).sqrt();
    let a1 = (1.0 + kp) / 2.0;
    let b1 = kp.sqrt();      // √(1 · √(1-k²))
    let m = (a1 + b1) / 2.0; // media dopo un passo AGM
    PI / (2.0 * m)
}

/// Approssimazione numerica dell'integrale ellittico K(k) tramite
/// campionamento aureo (Formula 10 e 11). Divide l'intervallo [0, π/2]
/// in N fette con ampiezza geometrica di ragione φ.
/// 
/// Parametri:
/// - k: modulo ellittico
/// - epsilon: precisione desiderata (determina il numero di fette)
/// 
/// Restituisce una stima di K(k) (metodo dei rettangoli al punto medio).
pub fn k_golden_sampling(k: f64, epsilon: f64) -> f64 {
    if k == 0.0 {
        return PI / 2.0;
    }
    // Numero minimo di fette secondo Formula 11
    let n = ((1.0 / epsilon).ln() / PHI.ln()).ceil() as usize;
    let n = n.max(2); // almeno 2 fette

    let total_angle = PI / 2.0;
    // Ampiezza della prima fetta (Δθ₀)
    // La somma delle ampiezze deve essere total_angle:
    // Δθ₀ * (1 + φ + φ² + ... + φ^(n-1)) = total_angle
    // La serie geometrica di ragione φ dà somma = (φ^n - 1)/(φ - 1)
    let phi_n = PHI.powi(n as i32);
    let denom = (phi_n - 1.0) / (PHI - 1.0);
    let delta0 = total_angle / denom;

    let mut sum = 0.0;
    let mut theta = 0.0;
    for i in 0..n {
        let delta = delta0 * PHI.powi(i as i32);
        let theta_mid = theta + delta / 2.0; // punto medio
        let f_val = 1.0 / (1.0 - k * k * theta_mid.sin().powi(2)).sqrt();
        sum += f_val * delta;
        theta += delta;
    }
    sum
}

/// Versione ad alta precisione: suddivide l'intervallo [0, k] in N segmenti
/// e su ciascuno applica la formula standard (con δ tabulato).
/// Utile per ottenere precisioni intermedie tra standard ed exact.
/// 
/// Parametri:
/// - k: modulo ellittico
/// - n_segments: numero di segmenti in cui dividere l'intervallo (più segmenti = più precisione)
/// 
/// Nota: questa è una quadratura composita: approssima K(k) come somma di contributi
/// su sottointervalli dove la formula standard è più accurata perché k_local è minore.
pub fn k_sapri_composite(k: f64, n_segments: usize) -> f64 {
    if k == 0.0 {
        return PI / 2.0;
    }
    let step = k / (n_segments as f64);
    let mut sum = 0.0;
    for i in 0..n_segments {
        // Punto medio del segmento (per ridurre l'errore)
        let k_local = (i as f64 + 0.5) * step;
        // Calcola il contributo come se fosse un integrale su un piccolo intervallo
        // Approssimazione: K(k) ≈ Σ K(k_local) * Δk? Non è corretto perché K non è lineare.
        // Meglio usare la definizione: K(k) è integrale in dθ, non in dk.
        // Quindi questa funzione richiede una revisione.
        // Per ora la lasciamo come placeholder, ma va ripensata.
    }
    // Implementazione corretta richiederebbe di suddividere l'angolo, non il modulo.
    // Lasciamo incompleta per ora.
    unimplemented!("k_sapri_composite richiede una ridefinizione basata sull'angolo");
}

/// Versione corretta di alta precisione: suddivide l'angolo in N fette
/// e su ciascuna applica la formula standard con k costante (quello globale).
/// In realtà non serve suddividere l'angolo perché la formula standard è già molto precisa.
/// Se vogliamo aumentare la precisione, possiamo usare il campionamento aureo (già sopra).
/// Quindi forse questa funzione non è necessaria.
/// Lasciamo commentata per ora.

// ==========================================
// Integrale ellittico di seconda specie E(k) (Formula 9, ipotesi)
// ==========================================

/// Approssimazione dell'integrale ellittico completo di seconda specie E(k)
/// basata sulla stessa struttura aurea. Versione senza correzione (δ_E = 0).
/// Da verificare numericamente.
pub fn e_sapri_approx(k: f64) -> f64 {
    if k == 0.0 {
        return PI / 2.0;
    }
    let kp = (1.0 - k * k).sqrt();
    let base = ((1.0 + kp) / 2.0).powf(1.0 / PHI);
    (PI / 2.0) * (1.0 - (k * k / 2.0) * base)
}
