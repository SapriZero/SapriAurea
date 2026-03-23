"""
denominators.py
===============
Exact computation of denominators D_n for the series expansion of K_φ(k).

The series is:
    K_φ(k) = (π/2) · Σ_{n=0}∞ (a_n + b_n·φ) / D_n · k^{2n}

This module computes a_n, b_n, D_n exactly using rational arithmetic in Q(√5).
Results are used in Appendix C of the paper.

Author: Ettore Bevilacqua (SapriZero)
Date: March 2026
"""

from fractions import Fraction
from math import gcd
from typing import List, Tuple

# ============================================================
# CONSTANTS
# ============================================================

# φ = (1 + √5)/2, represented as 0 + 1·φ in Q(√5)
# In our representation, a number is A + B·φ with A, B ∈ Q

# ============================================================
# ARITHMETIC IN Q(√5)
# ============================================================

class QuadraticField:
    """Element of Q(√5) represented as A + B·φ, where φ = (1+√5)/2.
    
    Multiplication uses the identity φ² = φ + 1.
    """
    
    def __init__(self, a: Fraction, b: Fraction):
        self.a = a  # rational part
        self.b = b  # φ part
    
    def __add__(self, other):
        return QuadraticField(self.a + other.a, self.b + other.b)
    
    def __sub__(self, other):
        return QuadraticField(self.a - other.a, self.b - other.b)
    
    def __mul__(self, other):
        # (A + Bφ)(C + Dφ) = AC + (AD + BC + BD)φ
        ac = self.a * other.a
        ad = self.a * other.b
        bc = self.b * other.a
        bd = self.b * other.b
        return QuadraticField(
            ac + bd,
            ad + bc + bd
        )
    
    def __truediv__(self, other):
        # Division by rational only (simplifies the recursion)
        if other.b != 0:
            raise ValueError("Division by non-rational not implemented")
        return QuadraticField(self.a / other.a, self.b / other.a)
    
    def __repr__(self):
        return f"({self.a} + {self.b}·φ)"


# ============================================================
# POCHHAMMER SYMBOL (a)_n = a(a+1)...(a+n-1)
# ============================================================

def pochhammer_rational(a: Fraction, n: int) -> Fraction:
    """Compute (a)_n for rational a."""
    if n == 0:
        return Fraction(1, 1)
    result = Fraction(1, 1)
    for k in range(n):
        result *= (a + k)
    return result


def pochhammer_phi(n: int) -> QuadraticField:
    """Compute (1/φ)_n in Q(√5).
    
    Uses the identity 1/φ = φ - 1, so (1/φ)_n = ∏_{k=0}^{n-1} (φ - 1 + k).
    """
    if n == 0:
        return QuadraticField(Fraction(1, 1), Fraction(0, 1))
    
    result = QuadraticField(Fraction(1, 1), Fraction(0, 1))
    for k in range(n):
        # term = (k - 1) + φ
        term = QuadraticField(Fraction(k - 1, 1), Fraction(1, 1))
        result = result * term
    return result


# ============================================================
# HYPERGEOMETRIC COEFFICIENTS
# ============================================================

def c_K(n: int) -> Fraction:
    """Coefficient c_K(n) = [(1/2)_n]² / (n!)²."""
    if n == 0:
        return Fraction(1, 1)
    half_n = pochhammer_rational(Fraction(1, 2), n)
    fact = Fraction(1, 1)
    for i in range(2, n + 1):
        fact *= Fraction(i, 1)
    return (half_n * half_n) / (fact * fact)


def c_Kphi(n: int) -> QuadraticField:
    """Coefficient c_Kφ(n) = (1/φ)_n · (-1/4)_n / (n!)²."""
    if n == 0:
        return QuadraticField(Fraction(1, 1), Fraction(0, 1))
    
    phi_part = pochhammer_phi(n)
    minus_quarter_n = pochhammer_rational(Fraction(-1, 4), n)
    fact = Fraction(1, 1)
    for i in range(2, n + 1):
        fact *= Fraction(i, 1)
    
    factor = minus_quarter_n / (fact * fact)
    return QuadraticField(phi_part.a * factor, phi_part.b * factor)


# ============================================================
# RECURSION FOR d_n = (A_n + B_n·φ) / D_n
# ============================================================

def compute_denominators(max_n: int = 35) -> List[Tuple[int, int, int, int]]:
    """
    Compute D_n, A_n, B_n for n = 0..max_n.
    
    Returns list of (n, A_n, B_n, D_n) with A_n, B_n ∈ ℤ.
    """
    results = []
    
    # Store d_n as QuadraticField objects
    d = [None] * (max_n + 1)
    
    # d_0 = 1
    d[0] = QuadraticField(Fraction(1, 1), Fraction(0, 1))
    results.append((0, 1, 0, 1))
    
    # c_Kφ(0) = 1
    c0 = c_Kphi(0)
    
    for n in range(1, max_n + 1):
        # Numerator: c_K(n) - c_Kφ(n) - Σ_{j=1}^{n-1} c_Kφ(j) * d_{n-j}
        cK = QuadraticField(c_K(n), Fraction(0, 1))
        cKphi_n = c_Kphi(n)
        
        numerator = cK - cKphi_n
        
        for j in range(1, n):
            cKphi_j = c_Kphi(j)
            term = cKphi_j * d[n - j]
            numerator = numerator - term
        
        # Divide by c_Kφ(0) = 1
        d_n = numerator
        
        # Extract common denominator
        # d_n = A + B·φ with A, B ∈ Q
        # Write A = A_num / den, B = B_num / den
        a_frac = d_n.a
        b_frac = d_n.b
        
        # Find common denominator
        den = a_frac.denominator * b_frac.denominator // gcd(a_frac.denominator, b_frac.denominator)
        
        A_num = int(a_frac.numerator * (den // a_frac.denominator))
        B_num = int(b_frac.numerator * (den // b_frac.denominator))
        
        # Reduce by gcd
        g = gcd(gcd(A_num, B_num), den)
        A_num //= g
        B_num //= g
        D_n = den // g
        
        d[n] = QuadraticField(Fraction(A_num, D_n), Fraction(B_num, D_n))
        results.append((n, A_num, B_num, D_n))
    
    return results


# ============================================================
# PRIME ANALYSIS
# ============================================================

def is_inert_prime(p: int) -> bool:
    """Check if prime p is inert in Z[φ] (p ≡ ±2 mod 5)."""
    if p == 2:
        return True
    if p == 5:
        return False
    r = p % 5
    return r == 2 or r == 3


def prime_factors(n: int) -> List[int]:
    """Return list of distinct prime factors of n."""
    if n <= 1:
        return []
    factors = []
    temp = n
    p = 2
    while p * p <= temp:
        if temp % p == 0:
            factors.append(p)
            while temp % p == 0:
                temp //= p
        p += 1 if p == 2 else 2
    if temp > 1:
        factors.append(temp)
    return factors


def verify_denominators(max_n: int = 35):
    """Verify that odd part of D_n equals product of inert odd primes ≤ n."""
    results = compute_denominators(max_n)
    
    print("=" * 70)
    print("DENOMINATORS D_n - VERIFICATION")
    print("=" * 70)
    print(f"{'n':>4} {'D_n (odd part)':>15} {'Product of inert odd primes ≤ n':>35} {'Match?':>8}")
    print("-" * 70)
    
    for n, A, B, D in results:
        if n == 0:
            continue
        
        # Get odd part of D
        D_odd = D
        while D_odd % 2 == 0:
            D_odd //= 2
        
        # Compute product of inert odd primes ≤ n
        product = 1
        for p in range(3, n + 1):   # start from 3, exclude 2
            # simple primality test
            is_prime = True
            for q in range(2, int(p ** 0.5) + 1):
                if p % q == 0:
                    is_prime = False
                    break
            if is_prime and is_inert_prime(p):
                product *= p
        
        match = "✓" if D_odd == product else "✗"
        print(f"{n:>4} {D_odd:>15} {product:>35} {match:>8}")
    
    print("=" * 70)

# ============================================================
# MAIN
# ============================================================

if __name__ == "__main__":
    verify_denominators(35)
    
    print("\n" + "=" * 70)
    print("INERT PRIMES IN Z[φ]")
    print("=" * 70)
    primes = []
    for p in range(2, 100):
        is_prime = True
        for q in range(2, int(p ** 0.5) + 1):
            if p % q == 0:
                is_prime = False
                break
        if is_prime:
            primes.append(p)
    
    inert = [p for p in primes if is_inert_prime(p)]
    print(f"p ≡ ±2 mod 5: {inert}")
