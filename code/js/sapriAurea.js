
/**
 * SAPRI AUREA FORMULA - Complete Implementation
 * DOI: 10.5281/zenodo.19047370
 * Paper: https://doi.org/10.5281/zenodo.19047370
 * Author: SapriZero
 * License: MIT
 *
 * 
 * This implementation includes:
 * - AGM method (Arithmetic-Geometric Mean)
 * - Sapri Aurea formula with δ correction
 * - Precision levels and adaptive sampling
 * - Comparison utilities
 */

// ============================================
// CONSTANTS
// ============================================

const PHI = (1 + Math.sqrt(5)) / 2;        // φ ≈ 1.618033988749895
const PHI_INV = 1 / PHI;                    // 1/φ ≈ 0.6180339887498949
const PI = Math.PI;                         // π ≈ 3.141592653589793
const PI_HALF = PI / 2;                      // π/2

// ============================================
// FORMULA 1: K(k) - Complete Elliptic Integral (AGM method)
// ============================================

/**
 * AGM method - complete elliptic integral of the first kind K(k)
 * @param {number} k - elliptic modulus (0 ≤ k < 1)
 * @param {number} iterations - number of AGM iterations (default: 4)
 * @returns {number} K(k)
 */
function K_AGM(k, iterations = 4) {
    if (k < 0 || k >= 1) throw new Error("k must be in [0, 1)");
    if (k === 0) return PI_HALF;
    
    let a = 1.0;
    let b = Math.sqrt(1 - k * k);
    
    for (let i = 0; i < iterations; i++) {
        const a_next = (a + b) / 2;
        const b_next = Math.sqrt(a * b);
        a = a_next;
        b = b_next;
        
        // Convergence check
        if (Math.abs(a - b) < 1e-15) break;
    }
    
    return PI_HALF / a;  // K(k) = π / (2 * AGM)
}

// ============================================
// FORMULA 3: K(k, δ) - Sapri Aurea Formula
// ============================================

/**
 * Base Sapri Aurea formula without δ correction
 * @param {number} k - elliptic modulus (0 ≤ k < 1)
 * @returns {number} base approximation
 */
function K_sapri_base(k) {
    if (k < 0 || k >= 1) throw new Error("k must be in [0, 1)");
    if (k === 0) return PI_HALF;
    
    const sqrtTerm = Math.sqrt(1 - k * k);
    const meanTerm = (1 + sqrtTerm) / 2;
    const powerTerm = Math.pow(meanTerm, PHI_INV);
    
    return PI_HALF * powerTerm;
}

/**
 * Recommended δ(k) based on empirical values
 * @param {number} k - elliptic modulus (0 ≤ k < 1)
 * @returns {number} recommended δ
 */
function delta_recommended(k) {
    if (k < 0.1) return 0;
    if (k < 0.3) return 0.23;
    if (k < 0.6) return 0.23;  // stable region
    if (k < 0.7) return 0.25;
    if (k < 0.8) return 0.4;
    if (k < 0.9) return 1.0;
    if (k < 0.95) return 1.5;
    if (k < 0.99) return 2.03;
    return 3.5;  // for k extremely close to 1
}

/**
 * Full Sapri Aurea formula with δ correction
 * @param {number} k - elliptic modulus (0 ≤ k < 1)
 * @param {number} delta - correction factor (default: auto from delta_recommended)
 * @returns {number} K(k, δ)
 */
function K_sapri(k, delta = null) {
    if (delta === null) delta = delta_recommended(k);
    return K_sapri_base(k) * (1 + delta);
}

// ============================================
// FORMULA 5: δ(k) - Continuous delta function (empirical fit)
// ============================================

/**
 * Continuous delta(k) function - empirical fit
 * @param {number} k - elliptic modulus (0 ≤ k < 1)
 * @returns {number} continuous delta
 */
function delta_continuous(k) {
    if (k >= 1) return Infinity;
    
    // Empirical fit: δ(k) ≈ a·k^b / (1 - k)^c
    const a = 1.2;
    const b = 2.3;
    const c = 0.7;
    
    return a * Math.pow(k, b) / Math.pow(1 - k, c);
}

// ============================================
// FORMULA 6: Error calculation
// ============================================

/**
 * Relative error between Sapri and AGM
 * @param {number} k - elliptic modulus
 * @param {number} delta - correction factor
 * @returns {object} error information
 */
function error_analysis(k, delta = null) {
    const K_true = K_AGM(k, 6);  // high precision AGM as reference
    const K_s = K_sapri(k, delta);
    const error = Math.abs(K_s - K_true) / K_true;
    
    return {
        k,
        K_true,
        K_sapri: K_s,
        error,
        error_percent: error * 100
    };
}

// ============================================
// FORMULA 10-11: Geometric sampling
// ============================================

/**
 * Golden sampling - distribute N points according to φ
 * @param {number} N - number of slices
 * @param {number} t_start - start parameter (default: 0)
 * @param {number} t_end - end parameter (default: π/2)
 * @returns {array} array of sample points
 */
function golden_samples(N, t_start = 0, t_end = PI_HALF) {
    const samples = [];
    const delta_t = t_end - t_start;
    
    for (let n = 0; n < N; n++) {
        // Golden ratio progression
        const ratio = Math.pow(PHI, n / N);
        const t = t_start + delta_t * (ratio - 1) / (Math.pow(PHI, (N-1)/N) - 1);
        samples.push(t);
    }
    
    return samples;
}

/**
 * Minimum slices needed for desired precision
 * @param {number} epsilon - desired precision (e.g., 0.01 for 1%)
 * @returns {number} minimum number of slices
 */
function min_slices_for_precision(epsilon) {
    return Math.ceil(Math.log(1 / epsilon) / Math.log(PHI));
}

// ============================================
// FORMULA 8: Hybrid method (Sapri + 1 AGM step)
// ============================================

/**
 * Hybrid method: Sapri base + one AGM correction step
 * @param {number} k - elliptic modulus
 * @returns {number} K_hybrid(k)
 */
function K_hybrid(k) {
    if (k === 0) return PI_HALF;
    
    const sqrtTerm = Math.sqrt(1 - k * k);
    
    // One AGM step
    const a1 = (1 + sqrtTerm) / 2;
    const b1 = Math.sqrt(1 * sqrtTerm);
    const M = (a1 + b1) / 2;
    
    return PI_HALF / M;
}

// ============================================
// FORMULA 9: Extension to second kind E(k)
// ============================================

/**
 * Elliptic integral of the second kind E(k) - base formula
 * @param {number} k - elliptic modulus
 * @param {number} delta - correction factor
 * @returns {number} E(k, δ)
 */
function E_sapri(k, delta = 0) {
    if (k === 0) return PI_HALF;
    if (k === 1) return 1.0;
    
    const sqrtTerm = Math.sqrt(1 - k * k);
    const meanTerm = (1 + sqrtTerm) / 2;
    const powerTerm = Math.pow(meanTerm, PHI_INV);
    
    return PI_HALF * (1 - (k * k / 2) * powerTerm) * (1 + delta);
}

// ============================================
// UTILITY: Compare all methods
// ============================================

/**
 * Compare all methods for a range of k values
 * @param {array} k_values - array of k values to test
 */
function compare_methods(k_values = [0, 0.1, 0.3, 0.5, 0.7, 0.8, 0.9, 0.95, 0.99]) {
    console.log("\n📊 SAPRI AUREA vs AGM COMPARISON");
    console.log("=".repeat(80));
    console.log("k\t\tAGM\t\tSapri\t\tError %\t\tδ");
    console.log("=".repeat(80));
    
    for (const k of k_values) {
        const K_true = K_AGM(k, 6);
        const delta = delta_recommended(k);
        const K_s = K_sapri(k, delta);
        const error = Math.abs(K_s - K_true) / K_true * 100;
        
        console.log(`${k.toFixed(2)}\t\t${K_true.toFixed(8)}\t${K_s.toFixed(8)}\t${error.toFixed(4)}%\t\t${delta.toFixed(2)}`);
    }
    console.log("=".repeat(80));
}

// ============================================
// PRECISION LEVELS (from the paper)
// ============================================

const PRECISION_LEVELS = {
    ULTRA_FAST: {
        method: "2-point ratio",
        operations: "1-2 per point",
        precision: "5-10%",
        application: "Rendering, previews"
    },
    FAST: {
        method: "3-point mean",
        operations: "2-3 per point",
        precision: "2-5%",
        application: "Real-time graphics"
    },
    STANDARD: {
        method: "Sapri Aurea + δ",
        operations: 7,
        precision: "0.1-1%",
        application: "Engineering, simulations"
    },
    HIGH: {
        method: "Segmentation × N",
        operations: "7×N",
        precision: "0.01-0.1%",
        application: "Research, critical calculations"
    },
    EXTREME: {
        method: "Double segmentation",
        operations: "7×N×M",
        precision: "<0.001%",
        application: "Theoretical physics"
    }
};

// ============================================
// EXPORTS
// ============================================

module.exports = {
    // Constants
    PHI,
    PHI_INV,
    PI,
    PI_HALF,
    
    // Main methods
    K_AGM,
    K_sapri_base,
    K_sapri,
    K_hybrid,
    E_sapri,
    
    // Delta functions
    delta_recommended,
    delta_continuous,
    
    // Error analysis
    error_analysis,
    compare_methods,
    
    // Sampling
    golden_samples,
    min_slices_for_precision,
    
    // Precision levels
    PRECISION_LEVELS
};

// ============================================
// EXAMPLE USAGE
// ============================================

// Run comparison if called directly
if (require.main === module) {
    console.log("\n🔬 SAPRI AUREA FORMULA - TEST RUN\n");
    
    // Test single value
    const k_test = 0.5;
    const result = error_analysis(k_test);
    console.log(`Test k = ${k_test}:`);
    console.log(`  AGM: ${result.K_true.toFixed(10)}`);
    console.log(`  Sapri: ${result.K_sapri.toFixed(10)}`);
    console.log(`  Error: ${result.error_percent.toFixed(4)}%\n`);
    
    // Compare all methods
    compare_methods();
    
    // Test golden sampling
    console.log("\n📐 Golden Sampling Example (N=5 slices):");
    const samples = golden_samples(5);
    samples.forEach((t, i) => console.log(`  slice ${i+1}: θ = ${t.toFixed(4)}`));
    
    console.log(`\n🎯 Minimum slices for 0.1% precision: ${min_slices_for_precision(0.001)}`);
}
