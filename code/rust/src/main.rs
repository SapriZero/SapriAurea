use std::time::Instant;
use rand::Rng;

use sapriAurea::*;

fn main() {
    println!("==========================================");
    println!("Sapri Aurea Formula - Rust Implementation");
    println!("==========================================\n");
    println!("φ = {:.15}", PHI);

    // Esempio di creazione oggetto con le macro
    let params = obj!({
        phi: PHI,
        pi: std::f64::consts::PI,
        a: 1.0 / PHI,
        b: -0.25,
        c: 1.0
    });
    println!("\nOggetto params creato: {:?}", params);

    // --- Verifiche standard (dal companion Python) ---
    println!("\n--- Verifica tabella delta ---");
    println!(" k     | tabella   | esatta    | err%");
    for i in 0..DELTA_TABLE_K.len() {
        let k = DELTA_TABLE_K[i];
        let d_table = DELTA_TABLE_D[i];
        let d_exact = if k > 0.0 { delta_exact(k) } else { 0.0 };
        let err = if d_exact != 0.0 {
            (d_table - d_exact).abs() / d_exact * 100.0
        } else {
            0.0
        };
        println!(" {:.2}   | {:.6} | {:.6} | {:.4}%", k, d_table, d_exact, err);
    }

    // --- Nuova parte: esempio di utilizzo della struttura URCM ---
    println!("\n==========================================");
    println!("ESEMPIO: Struttura URCM a livelli");
    println!("==========================================");

    let k_test = 0.3;
    let formula = SapriAurea::new(k_test);
    formula.print_info();

    // Otteniamo l'oggetto descrittivo e stampiamo alcune chiavi
    let obj = formula.to_obj();
    println!("\n--- Oggetto descrittivo (alcune chiavi) ---");
    if let Some(val) = obj.get(&path!(desc_pa)) {
        println!("desc_pa = {:?}", val);
    }
    if let Some(val) = obj.get(&path!(au)) {
        println!("au = {:?}", val);
    }

    // Confronto tra diversi metodi di calcolo
    println!("\n--- Confronto per k = 0.5 ---");
    let k = 0.5;
    let exact = k_agm(k, 10);
    let sapri_table = SapriAurea::new(k).K;
    let sapri_exact = SapriAurea::with_exact_delta(k).K;
    println!("K esatto (AGM)        = {:.10}", exact);
    println!("Sapri + tabella       = {:.10}", sapri_table);
    println!("Sapri + delta esatto  = {:.10}", sapri_exact);
    println!("Errore tabella        = {:.6}%", (exact - sapri_table).abs() / exact * 100.0);

    // Benchmark
    
	println!("\n--- Benchmark completo (100k valutazioni) ---");
	let n = 100_000;
	let mut rng = rand::thread_rng();
	let k_vals: Vec<f64> = (0..n).map(|_| rng.gen_range(0.025..0.975)).collect();
	
	let start = Instant::now();
	for &k in &k_vals { let _ = k_sapri_ultrafast(k); }
	let t_ultra = start.elapsed();
	
	let start = Instant::now();
	for &k in &k_vals { let _ = k_sapri_fast(k); }
	let t_fast = start.elapsed();
	
	let start = Instant::now();
	for &k in &k_vals { let _ = k_sapri_standard(k); }
	let t_std = start.elapsed();
	
	let start = Instant::now();
	for &k in &k_vals { let _ = k_sapri_hybrid(k); }
	let t_hybrid = start.elapsed();
	
	let start = Instant::now();
	for &k in &k_vals { let _ = k_golden_sampling(k, 0.01); }
	let t_golden_1 = start.elapsed();
	
	let start = Instant::now();
	for &k in &k_vals { let _ = k_golden_sampling(k, 0.001); }
	let t_golden_2 = start.elapsed();
	
	let start = Instant::now();
	for &k in &k_vals { let _ = k_agm(k, 10); }
	let t_agm = start.elapsed();
	
	let start = Instant::now();
	for &k in &k_vals { let _ = k_sapri_exact(k); }
	let t_exact = start.elapsed();
	
	println!("Ultra-fast (δ=0)          : {:?}", t_ultra);
	println!("Fast (5 punti)            : {:?}", t_fast);
	println!("Standard (12 punti)       : {:?}", t_std);
	println!("Hybrid (1 passo AGM)      : {:?}", t_hybrid);
	println!("Golden sampling ε=0.01    : {:?}", t_golden_1);
	println!("Golden sampling ε=0.001   : {:?}", t_golden_2);
	println!("AGM (10 iter)             : {:?}", t_agm);
	println!("Exact (δ con AGM)         : {:?}", t_exact);
	
	println!("\n--- Confronto precisione ---");
	println!("k\tExact\t\tUltra-fast\tFast\t\tStandard");
	for &k in &[0.1, 0.3, 0.5, 0.7, 0.9] {
	    let exact = k_sapri_exact(k);
	    let ultra = k_sapri_ultrafast(k);
	    let fast = k_sapri_fast(k);
	    let std = k_sapri_standard(k);
	    println!("{:.1}\t{:.8}\t{:.8}\t{:.8}\t{:.8}", k, exact, ultra, fast, std);
	}
    
    println!("\n--- Verifica serie δ(k) (ordine 3) ---");
	let test_k = [0.05, 0.1, 0.15, 0.2, 0.25, 0.3];
	for &k in &test_k {
	    let d_exact = delta_exact(k);
	    let d_ser = delta_series(k, 3);
	    let err = (d_exact - d_ser).abs() / d_exact * 100.0;
	    println!("k={:.2}: esatta={:.8}, serie={:.8}, err={:.4}%", k, d_exact, d_ser, err);
	}
	
	println!("\n--- Verifica trasformazione di Eulero ---");
	let test_z = [0.1, 0.2, 0.3, 0.5, 0.7, 0.9];
	for &z in &test_z {
	    let lhs = f_phi(z, 150);
	    let rhs = f_euler_transformed(z, 150);
	    let err = (lhs - rhs).abs() / lhs * 100.0;
	    println!("z={:.2}: lhs={:.8}, rhs={:.8}, err={:.5}%", z, lhs, rhs, err);
	}

    println!("\nFatto.");
}
