use mcl_rust::*;

// Polinomai su BigInt koeficientais
mod poly;

// go-kzg FFTSettings struktūra
pub struct FFTSettings {
    maxWidth: u64,
    rootOfUnity: poly::Coeficient,
    expandedRootsOfUnity: Vec<poly::Coeficient>,
    reverseRootsOfUnity: Vec<poly::Coeficient>,
}

// go-kzg KZGSettings struktūra
pub struct KZGSettings {
    fftSet: FFTSettings,
    secretG1: G1,
    extendedSecretG1: G1,
    secretG2: G2,
}

// go-kzg tipo commitment (neoptimizuotas)
/*
fn commit_to_poly(coefs: Fr) -> G1 {
    let ks: KZGSettings;
    let out = unsafe { G1::uninit() };
    let tmp = unsafe { G1::uninit() };
    let tmp2 = unsafe { G1::uninit() };
    for i in 0..coefs.len() {                       // Kaip gauti/sukurti polinomą pagal Fr?
        G1::mul(&mut tmp, &ks.secretG1, &coefs);
        G1::add(&mut tmp2, &out, &tmp);
        &out, &tmp2;
    }
    out
}
*/

#[allow(non_snake_case)]
fn main() {

    let b = init(CurveType::BN254);
    if !b {
        println!("init err");
    }

    let mut P1 = unsafe { G1::uninit() };
    let mut P2 = unsafe { G1::uninit() };
    let mut x = Fr::zero();
    G1::mul(&mut P2, &P1, &x);
    let mut X = unsafe { Fr::uninit() };
    // commit_to_poly(X);

    // mclBn_G1EvaluatePolynomial(mclBnG1 *out, const mclBnG1 *cVec, mclSize cSize, const mclBnFr *x);  //Funkcija neimplementuota mcl_rust
    // mcl_rust ir bls_eth_rust nėra polinomų
    // TODO Pasigilinti į finite field table
}