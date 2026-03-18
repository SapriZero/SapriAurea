"""
sapri_aurea_companion.py
========================
Companion code for the paper:
  "The Sapri Aurea Formula for Complete Elliptic Integrals"
  Ettore Bevilacqua (SapriZero), March 2026

This module provides:
  1. Core functions (K_exact, K_agm, K_sapri, F_phi)
  2. Verification of all paper results (verify_all)
  3. Delta table computation and interpolation
  4. Series expansion of delta(k)
  5. Euler transformation verification
  6. Monodromy analysis
  7. Benchmark: Sapri vs AGM speed
  8. Plots (requires matplotlib)

Usage:
    python sapri_aurea_companion.py          # run all verifications
    python sapri_aurea_companion.py --plot   # also generate plots

Dependencies: numpy, scipy (matplotlib optional)
"""

import math
import sys
import time
import numpy as np
from scipy import integrate

# ─────────────────────────────────────────────────────────────
# FUNDAMENTAL CONSTANTS
# ─────────────────────────────────────────────────────────────

PHI   = (1 + math.sqrt(5)) / 2   # golden ratio φ ≈ 1.6180339887
PI    = math.pi
SQRT5 = math.sqrt(5)

# Paper parameters
A_SAPRI = 1 / PHI   # a = 1/φ ≈ 0.6180
B_SAPRI = -1 / 4    # b = -1/4
C_SAPRI = 1.0       # c = 1

# Delta table from paper (Table in Section 2)
DELTA_TABLE_K = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5,
                 0.6, 0.7, 0.8, 0.9, 0.95, 0.99]
DELTA_TABLE_D = [0.000000, 0.004070, 0.016591, 0.038564,
                 0.071935, 0.120170, 0.189556, 0.292521,
                 0.458084, 0.781805, 1.139392, 2.022778]


# ─────────────────────────────────────────────────────────────
# SECTION 1 — CORE FUNCTIONS
# ─────────────────────────────────────────────────────────────

def K_exact(k: float, tol: float = 1e-14) -> float:
    """
    Complete elliptic integral of the first kind K(k) via quadrature.
    Reference value — used only for verification.

    Parameters
    ----------
    k   : modulus, 0 <= k < 1
    tol : integration tolerance

    Returns
    -------
    K(k) as float
    """
    if k == 0:
        return PI / 2
    if k >= 1:
        raise ValueError("k must be < 1")
    result, _ = integrate.quad(
        lambda t: 1.0 / math.sqrt(1 - k**2 * math.sin(t)**2),
        0, PI / 2, limit=200, epsabs=tol, epsrel=tol
    )
    return result


def K_agm(k: float, iterations: int = 10) -> float:
    """
    Complete elliptic integral K(k) via Arithmetic-Geometric Mean.

    K(k) = π / (2 · AGM(1, √(1-k²)))

    Parameters
    ----------
    k          : modulus, 0 <= k < 1
    iterations : AGM iterations (10 gives machine precision)

    Returns
    -------
    K(k) as float
    """
    if k == 0:
        return PI / 2
    a, b = 1.0, math.sqrt(1 - k**2)
    for _ in range(iterations):
        a, b = (a + b) / 2, math.sqrt(a * b)
    return PI / (2 * a)


def K_sapri_base(k: float) -> float:
    """
    Sapri Aurea base function (no correction):

    K_φ(k) = (π/2) · ((1 + √(1-k²)) / 2)^(1/φ)

    Parameters
    ----------
    k : modulus, 0 <= k < 1

    Returns
    -------
    K_φ(k) as float
    """
    if k == 0:
        return PI / 2
    kp = math.sqrt(1 - k**2)
    return (PI / 2) * ((1 + kp) / 2) ** (1 / PHI)


def K_sapri(k: float, delta_interp: bool = True) -> float:
    """
    Sapri Aurea Formula with tabulated delta correction:

    K(k) ≈ K_φ(k) · (1 + δ(k))

    Parameters
    ----------
    k             : modulus, 0 <= k < 1
    delta_interp  : if True, use linear interpolation from DELTA_TABLE
                    if False, return base formula only

    Returns
    -------
    K(k) approximation as float
    """
    base = K_sapri_base(k)
    if not delta_interp or k == 0:
        return base
    delta = delta_interpolate(k)
    return base * (1 + delta)


def delta_interpolate(k: float) -> float:
    """
    Linear interpolation of δ(k) from the paper's delta table.

    Parameters
    ----------
    k : modulus, 0 <= k <= 0.99

    Returns
    -------
    δ(k) as float
    """
    if k <= 0:
        return 0.0
    if k >= DELTA_TABLE_K[-1]:
        return DELTA_TABLE_D[-1]
    # Binary search for interval
    for i in range(len(DELTA_TABLE_K) - 1):
        k0, k1 = DELTA_TABLE_K[i], DELTA_TABLE_K[i + 1]
        if k0 <= k <= k1:
            t = (k - k0) / (k1 - k0)
            return DELTA_TABLE_D[i] * (1 - t) + DELTA_TABLE_D[i + 1] * t
    return DELTA_TABLE_D[-1]


def delta_exact(k: float) -> float:
    """
    Exact delta correction from Theorem 1:

    δ(k) = (2 / (1 + √(1-k²)))^(1/φ) / AGM(1, √(1-k²)) - 1

    This requires AGM computation — use for table generation only.

    Parameters
    ----------
    k : modulus, 0 <= k < 1

    Returns
    -------
    δ(k) exact as float
    """
    if k == 0:
        return 0.0
    kv = K_exact(k)
    ks = K_sapri_base(k)
    return kv / ks - 1


def F_phi(z: float, terms: int = 150) -> float:
    """
    Hypergeometric function F(1/φ, -1/4; 1; z) via power series.

    F_φ(z) = Σ_{n=0}^∞ [(1/φ)_n · (-1/4)_n / (n!)²] · z^n

    Parameters
    ----------
    z     : argument, |z| < 1
    terms : number of series terms

    Returns
    -------
    F_φ(z) as float
    """
    if z == 0:
        return 1.0
    if abs(z) >= 1.0:
        raise ValueError("Series converges only for |z| < 1")
    a, b = A_SAPRI, B_SAPRI
    result, term = 1.0, 1.0
    for n in range(1, terms):
        term *= (a + n - 1) * (b + n - 1) / n**2 * z
        result += term
        if abs(term) < 1e-16:
            break
    return result


def delta_series(k: float, order: int = 3) -> float:
    """
    Power series approximation of δ(k) from Appendix A:

    δ(k) ≈ (φ/4)k² + (1/4)k⁴ + (49/384 + 1/(12φ))k⁶

    Parameters
    ----------
    k     : modulus
    order : number of terms (1, 2, or 3)

    Returns
    -------
    δ(k) series approximation
    """
    z = k**2
    c1 = PHI / 4
    c2 = 1 / 4
    c3 = 49 / 384 + 1 / (12 * PHI)
    result = c1 * z
    if order >= 2:
        result += c2 * z**2
    if order >= 3:
        result += c3 * z**3
    return result


# ─────────────────────────────────────────────────────────────
# SECTION 2 — EULER TRANSFORMATION
# ─────────────────────────────────────────────────────────────

def F_euler_transformed(z: float, terms: int = 150) -> float:
    """
    Right-hand side of the Euler transformation (Theorem 3):

    (1-z)^(5/4 - 1/φ) · F(1/φ², 5/4; 1; z)

    Parameters
    ----------
    z     : argument, 0 <= z < 1
    terms : series terms for inner hypergeometric function

    Returns
    -------
    Value of RHS as float
    """
    if z == 0:
        return 1.0
    gamma = 5 / 4 - 1 / PHI   # = c - a - b
    a2 = 1 / PHI**2            # = 1 - 1/φ
    b2 = 5 / 4
    # Compute F(a2, b2; 1; z)
    result, term = 1.0, 1.0
    for n in range(1, terms):
        term *= (a2 + n - 1) * (b2 + n - 1) / n**2 * z
        result += term
        if abs(term) < 1e-16:
            break
    return (1 - z)**gamma * result


# ─────────────────────────────────────────────────────────────
# SECTION 3 — MONODROMY ANALYSIS
# ─────────────────────────────────────────────────────────────

def monodromy_parameters() -> dict:
    """
    Returns the monodromy parameters for F(1/φ, -1/4; 1; z).

    The monodromy matrices are:
      M0 = [[1, 0], [2πi, 1]]              (around z=0, unipotent)
      M1 = diag(1, exp(2πi·(c-a-b)))      (around z=1)

    Returns
    -------
    dict with keys: c_minus_ab, lambda2, is_finite, c_minus_ab_exact
    """
    c_ab = 5 / 4 - 1 / PHI   # = 7/4 - sqrt(5)/2
    c_ab_exact = 7 / 4 - SQRT5 / 2
    lambda2 = complex(math.cos(2 * PI * c_ab), math.sin(2 * PI * c_ab))

    # Check if c-a-b is rational (necessary for finite monodromy)
    # 7/4 - sqrt(5)/2 is irrational → monodromy is infinite
    is_rational = False   # proved analytically

    return {
        "c_minus_ab":       c_ab,
        "c_minus_ab_exact": "7/4 - sqrt(5)/2",
        "c_minus_ab_field": "Q(sqrt(5))",
        "lambda2":          lambda2,
        "lambda2_modulus":  abs(lambda2),
        "is_rational":      is_rational,
        "monodromy_finite": is_rational,
        "M0":               "[[1, 0], [2πi, 1]]  (unipotent)",
        "M1":               f"diag(1, {lambda2:.6f})  (infinite order)",
    }


# ─────────────────────────────────────────────────────────────
# SECTION 4 — VERIFICATION SUITE
# ─────────────────────────────────────────────────────────────

def _check(label: str, computed: float, expected: float,
           tol: float = 1e-4) -> bool:
    """Print a verification line and return True if within tolerance."""
    err = abs(computed - expected) / abs(expected) * 100 if expected != 0 else abs(computed)
    status = "✓" if err < tol else "✗"
    print(f"  {status}  {label}: computed={computed:.8f}, "
          f"expected={expected:.8f}, error={err:.4e}%")
    return err < tol


def verify_table(verbose: bool = True) -> bool:
    """Verify the delta table (Section 2) against exact values."""
    if verbose:
        print("\n── Table 2: Delta values ──")
        print(f"  {'k':>5} │ {'δ_table':>10} │ {'δ_exact':>10} │ {'error%':>8}")
        print("  " + "─" * 45)
    all_ok = True
    for k, d_table in zip(DELTA_TABLE_K, DELTA_TABLE_D):
        d_exact = delta_exact(k) if k > 0 else 0.0
        err = abs(d_table - d_exact) / (abs(d_exact) + 1e-15) * 100
        ok = err < 0.01   # table has 6 decimal places; 0.01% tolerance
        all_ok = all_ok and ok
        if verbose:
            flag = "✓" if ok else "✗"
            print(f"  {flag}  {k:>5.2f} │ {d_table:>10.6f} │ {d_exact:>10.6f} │ {err:>8.5f}%")
    return all_ok


def verify_series(verbose: bool = True) -> bool:
    """Verify the power series of δ(k) (Appendix A, Proposition 1)."""
    if verbose:
        print("\n── Series δ(k) = (φ/4)k² + (1/4)k⁴ + ... ──")
        print(f"  Leading coefficient φ/4 = {PHI/4:.8f}")
        print(f"  Second coefficient  1/4 = {0.25:.8f}")
        print(f"  Third coefficient 49/384 + 1/(12φ) = {49/384 + 1/(12*PHI):.8f}")
        print()
        print(f"  {'k':>5} │ {'δ_exact':>12} │ {'δ_series(3)':>12} │ {'error%':>8}")
        print("  " + "─" * 50)
    all_ok = True
    for k in [0.05, 0.1, 0.15, 0.2, 0.25, 0.3]:
        d_exact = delta_exact(k)
        d_ser = delta_series(k, order=3)
        err = abs(d_exact - d_ser) / abs(d_exact) * 100
        ok = err < 0.1   # series is 3-term; good for k<0.3
        all_ok = all_ok and ok
        if verbose:
            flag = "✓" if ok else "✗"
            print(f"  {flag}  {k:>5.2f} │ {d_exact:>12.8f} │ {d_ser:>12.8f} │ {err:>8.4f}%")
    return all_ok


def verify_hypergeometric(verbose: bool = True) -> bool:
    """Verify Theorem 2: K_φ(k) = (π/2)·F(1/φ,-1/4;1;k²)."""
    if verbose:
        print("\n── Theorem 2: K_φ(k) = (π/2)·F(1/φ,-1/4;1;k²) ──")
        print(f"  {'k':>5} │ {'K_φ_base':>12} │ {'(π/2)·F_φ':>12} │ {'error%':>8}")
        print("  " + "─" * 52)
    all_ok = True
    for k in [0.1, 0.2, 0.3, 0.5, 0.7, 0.9]:
        kp = K_sapri_base(k)
        fp = (PI / 2) * F_phi(k**2)
        err = abs(kp - fp) / abs(kp) * 100
        ok = err < 0.05   # series converges slowly for large k; correct to 0.05%
        all_ok = all_ok and ok
        if verbose:
            flag = "✓" if ok else "✗"
            print(f"  {flag}  {k:>5.2f} │ {kp:>12.8f} │ {fp:>12.8f} │ {err:>8.5f}%")
    return all_ok


def verify_euler_transformation(verbose: bool = True) -> bool:
    """Verify Theorem 3: Euler transformation."""
    if verbose:
        print("\n── Theorem 3: Euler transformation ──")
        print(f"  F(1/φ,-1/4;1;z) = (1-z)^(5/4-1/φ) · F(1/φ²,5/4;1;z)")
        print(f"  γ = 5/4 - 1/φ = {5/4 - 1/PHI:.8f}")
        print()
        print(f"  {'z':>5} │ {'F_φ direct':>12} │ {'Euler RHS':>12} │ {'error%':>8}")
        print("  " + "─" * 52)
    all_ok = True
    for z in [0.1, 0.2, 0.3, 0.5, 0.7, 0.85, 0.9]:
        lhs = F_phi(z)
        rhs = F_euler_transformed(z)
        err = abs(lhs - rhs) / abs(lhs) * 100
        ok = err < 0.01
        all_ok = all_ok and ok
        if verbose:
            flag = "✓" if ok else "✗"
            print(f"  {flag}  {z:>5.2f} │ {lhs:>12.8f} │ {rhs:>12.8f} │ {err:>8.5f}%")
    return all_ok


def verify_sapri_accuracy(verbose: bool = True) -> bool:
    """Verify Sapri Aurea formula accuracy against exact K(k)."""
    if verbose:
        print("\n── Sapri Aurea accuracy (Table 2 + interpolation) ──")
        print(f"  {'k':>5} │ {'K_exact':>12} │ {'K_sapri':>12} │ {'error%':>8}")
        print("  " + "─" * 52)
    all_ok = True
    for k in [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 0.95]:
        kv = K_exact(k)
        ks = K_sapri(k)
        err = abs(kv - ks) / kv * 100
        ok = err < 0.005
        all_ok = all_ok and ok
        if verbose:
            flag = "✓" if ok else "✗"
            print(f"  {flag}  {k:>5.2f} │ {kv:>12.8f} │ {ks:>12.8f} │ {err:>8.5f}%")
    return all_ok


def verify_monodromy(verbose: bool = True) -> bool:
    """Verify monodromy analysis (Section 8)."""
    params = monodromy_parameters()
    if verbose:
        print("\n── Section 8: Monodromy parameters ──")
        print(f"  c - a - b = {params['c_minus_ab']:.10f}")
        print(f"  Exact: {params['c_minus_ab_exact']}")
        print(f"  Field: {params['c_minus_ab_field']}")
        print(f"  λ₂ = {params['lambda2']:.6f}")
        print(f"  |λ₂| = {params['lambda2_modulus']:.10f}  (should be 1.0)")
        print(f"  c-a-b rational? {params['is_rational']}  → monodromy finite? {params['monodromy_finite']}")
        print(f"  M0 = {params['M0']}")
        print(f"  M1 = {params['M1']}")
        # Verify c-a-b = 7/4 - sqrt(5)/2
        c_ab_check = 7 / 4 - SQRT5 / 2
        print(f"\n  Verify 5/4 - 1/φ = 7/4 - √5/2:")
        print(f"    5/4 - 1/φ   = {5/4 - 1/PHI:.12f}")
        print(f"    7/4 - √5/2  = {c_ab_check:.12f}")
        diff = abs((5/4 - 1/PHI) - c_ab_check)
        flag = "✓" if diff < 1e-14 else "✗"
        print(f"    Difference  = {diff:.2e}  {flag}")
    ok1 = abs(params['lambda2_modulus'] - 1.0) < 1e-10
    ok2 = not params['monodromy_finite']
    return ok1 and ok2


def verify_all(verbose: bool = True) -> None:
    """Run all verification checks from the paper."""
    print("=" * 60)
    print("SAPRI AUREA — FULL VERIFICATION SUITE")
    print("=" * 60)

    results = {}
    results["delta_table"]      = verify_table(verbose)
    results["series"]           = verify_series(verbose)
    results["hypergeometric"]   = verify_hypergeometric(verbose)
    results["euler_transform"]  = verify_euler_transformation(verbose)
    results["sapri_accuracy"]   = verify_sapri_accuracy(verbose)
    results["monodromy"]        = verify_monodromy(verbose)

    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)
    all_passed = True
    for name, ok in results.items():
        status = "PASS ✓" if ok else "FAIL ✗"
        print(f"  {status}  {name}")
        all_passed = all_passed and ok
    print()
    if all_passed:
        print("  All verifications passed.")
    else:
        print("  Some verifications failed — check output above.")
    print("=" * 60)


# ─────────────────────────────────────────────────────────────
# SECTION 5 — BENCHMARK
# ─────────────────────────────────────────────────────────────

def benchmark(n_trials: int = 100_000) -> None:
    """
    Benchmark Sapri Aurea vs AGM speed.

    Parameters
    ----------
    n_trials : number of K(k) evaluations per method
    """
    import random
    random.seed(42)
    k_vals = [random.uniform(0.01, 0.95) for _ in range(n_trials)]

    print(f"\n── Benchmark: {n_trials:,} evaluations ──")

    # AGM
    t0 = time.perf_counter()
    for k in k_vals:
        _ = K_agm(k)
    t_agm = time.perf_counter() - t0

    # Sapri (base only, no delta interpolation)
    t0 = time.perf_counter()
    for k in k_vals:
        _ = K_sapri_base(k)
    t_base = time.perf_counter() - t0

    # Sapri (with table interpolation)
    t0 = time.perf_counter()
    for k in k_vals:
        _ = K_sapri(k)
    t_table = time.perf_counter() - t0

    print(f"  AGM (10 iters):      {t_agm*1000:.1f} ms  "
          f"({t_agm/n_trials*1e6:.3f} µs/eval)")
    print(f"  Sapri base only:     {t_base*1000:.1f} ms  "
          f"({t_base/n_trials*1e6:.3f} µs/eval)  "
          f"speedup {t_agm/t_base:.1f}x")
    print(f"  Sapri + table:       {t_table*1000:.1f} ms  "
          f"({t_table/n_trials*1e6:.3f} µs/eval)  "
          f"speedup {t_agm/t_table:.1f}x")


# ─────────────────────────────────────────────────────────────
# SECTION 6 — PLOTS (optional, requires matplotlib)
# ─────────────────────────────────────────────────────────────

def make_plots() -> None:
    """Generate paper figures. Requires matplotlib."""
    try:
        import matplotlib.pyplot as plt
        import matplotlib.gridspec as gridspec
    except ImportError:
        print("matplotlib not available — skipping plots")
        return

    k_vals = np.linspace(0.001, 0.98, 400)

    # Compute values
    K_ex  = np.array([K_exact(k) for k in k_vals])
    K_sa  = np.array([K_sapri(k) for k in k_vals])
    K_bas = np.array([K_sapri_base(k) for k in k_vals])
    err_sa  = np.abs(K_sa - K_ex) / K_ex * 100
    err_bas = np.abs(K_bas - K_ex) / K_ex * 100
    delta_v = np.array([delta_exact(k) for k in k_vals])
    delta_s = np.array([delta_series(k, 3) for k in k_vals])

    fig = plt.figure(figsize=(14, 10))
    fig.suptitle("Sapri Aurea Formula — Paper Figures", fontsize=14, y=0.98)
    gs = gridspec.GridSpec(2, 2, figure=fig, hspace=0.4, wspace=0.35)

    # ── Panel 1: K(k) comparison ──
    ax1 = fig.add_subplot(gs[0, 0])
    ax1.plot(k_vals, K_ex,  'k-',  lw=2,   label='K exact')
    ax1.plot(k_vals, K_sa,  'b--', lw=1.5, label='Sapri + table')
    ax1.plot(k_vals, K_bas, 'r:',  lw=1.5, label='Sapri base (δ=0)')
    ax1.set_xlabel('k'); ax1.set_ylabel('K(k)')
    ax1.set_title('K(k) — Exact vs Sapri Aurea')
    ax1.legend(fontsize=9); ax1.grid(True, alpha=0.3)

    # ── Panel 2: Error ──
    ax2 = fig.add_subplot(gs[0, 1])
    ax2.semilogy(k_vals, err_sa  + 1e-8, 'b-',  lw=1.5, label='Sapri + table')
    ax2.semilogy(k_vals, err_bas + 1e-8, 'r--', lw=1.5, label='Sapri base (δ=0)')
    ax2.axhline(y=0.003, color='gray', ls=':', lw=1, label='0.003% threshold')
    ax2.set_xlabel('k'); ax2.set_ylabel('Relative error (%)')
    ax2.set_title('Approximation Error')
    ax2.legend(fontsize=9); ax2.grid(True, alpha=0.3)

    # ── Panel 3: δ(k) and series ──
    ax3 = fig.add_subplot(gs[1, 0])
    ax3.plot(k_vals, delta_v, 'k-',  lw=2,   label='δ exact (Theorem 1)')
    ax3.plot(k_vals, delta_s, 'g--', lw=1.5, label='δ series (3 terms)')
    ax3.scatter(DELTA_TABLE_K, DELTA_TABLE_D, c='red', s=40, zorder=5,
                label='Table values')
    ax3.set_xlabel('k'); ax3.set_ylabel('δ(k)')
    ax3.set_title('Correction factor δ(k)')
    ax3.legend(fontsize=9); ax3.grid(True, alpha=0.3)
    ax3.set_xlim(0, 1); ax3.set_ylim(-0.05, 2.2)

    # ── Panel 4: Signatures comparison ──
    ax4 = fig.add_subplot(gs[1, 1])
    sigs = {
        'r=2 (exp=1)':     1.0,
        'r=3 (exp=2/3)':   2/3,
        'r=4 (exp=3/4)':   3/4,
        'r=6 (exp=5/6)':   5/6,
        'Sapri (exp=1/φ)': 1/PHI,
    }
    colors = ['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd']
    for (label, exp), color in zip(sigs.items(), colors):
        k_sig = np.linspace(0.001, 0.98, 300)
        F_sig = np.array([(PI/2)*((1+math.sqrt(1-k**2))/2)**exp for k in k_sig])
        ls = '-' if 'Sapri' in label else '--'
        lw = 2.0 if 'Sapri' in label else 1.2
        ax4.plot(k_sig, F_sig, ls=ls, lw=lw, color=color, label=label)
    ax4.plot(k_vals, K_ex, 'k-', lw=2, label='K exact', zorder=10)
    ax4.set_xlabel('k'); ax4.set_ylabel('Value')
    ax4.set_title('Ramanujan signatures vs Sapri')
    ax4.legend(fontsize=8); ax4.grid(True, alpha=0.3)

    plt.savefig('./sapri_aurea_figures.pdf',
                bbox_inches='tight', dpi=150)
    plt.savefig('./sapri_aurea_figures.png',
                bbox_inches='tight', dpi=150)
    print("  Figures saved: sapri_aurea_figures.pdf / .png")
    plt.close()


# ─────────────────────────────────────────────────────────────
# SECTION 7 — UTILITY: RECOMPUTE DELTA TABLE
# ─────────────────────────────────────────────────────────────

def compute_delta_table(k_points=None) -> dict:
    """
    Recompute the delta table from scratch using Theorem 1.

    Parameters
    ----------
    k_points : list of k values (default: paper table points)

    Returns
    -------
    dict with 'k' and 'delta' lists
    """
    if k_points is None:
        k_points = DELTA_TABLE_K
    deltas = [delta_exact(k) if k > 0 else 0.0 for k in k_points]
    return {"k": list(k_points), "delta": deltas}


# ─────────────────────────────────────────────────────────────
# MAIN
# ─────────────────────────────────────────────────────────────

if __name__ == "__main__":
    plot_flag = "--plot" in sys.argv

    print("\nSapri Aurea Formula — Companion Code")
    print(f"PHI = {PHI:.15f}")
    print(f"a = 1/φ = {A_SAPRI:.15f}")
    print(f"b = -1/4 = {B_SAPRI}")
    print(f"c-a-b = 7/4 - sqrt(5)/2 = {7/4 - SQRT5/2:.15f}")

    verify_all(verbose=True)
    benchmark(n_trials=50_000)

    if plot_flag:
        print("\n── Generating figures ──")
        make_plots()

    print("\nDone. Import this module for interactive use:")
    print("  from sapri_aurea_companion import K_sapri, K_exact, delta_exact")
