use mcl_rust::*;
// use bls_eth_rust as bls;
// #[link(name = "mcl_rust", kind = "static")]

// Polinomai su BigInt koeficientais
// mod poly;

//** go-kzg FFTSettings struktūra **//
/* pub struct FFTSettings {
    maxWidth: u64,
    rootOfUnity: poly::Coeficient,
    expandedRootsOfUnity: Vec<poly::Coeficient>,
    reverseRootsOfUnity: Vec<poly::Coeficient>,
} */

//** go-kzg KZGSettings struktūra **//
/* pub struct KZGSettings {
    fftSet: FFTSettings,
    secretG1: G1,
    extendedSecretG1: G1,
    secretG2: G2,
} */


// extern "C" {
//     fn CopyFr (to: &Fr, from: &Fr);
// }

// pub fn CopyFr(to: &mut Fr, from: &Fr){
//     to.d = from.d;
// } 

#[allow(non_snake_case)]
fn GenerateTestingSetup(secret: &mut str, n: u64) -> Vec<G1>{
    let m: i32 = 1;
	let mut s = unsafe {Fr::uninit()};
    let pointG1 = unsafe {G1::uninit()};
    Fr::set_str(&mut s, secret, m);
	// let sPow: bls.Fr;
	// bls.CopyFr(&sPow, &bls.ONE)
    let mut sPow: Fr = Fr::from_int(1);
    // let sOut = Vec::new();
    let mut sOut = vec![pointG1; n as usize];
    for i in 0..n {
        // sOut.push()
        let pointG11 = unsafe {G1::uninit()};
        G1::mul(&mut sOut[i as usize], &pointG11, &sPow);
        let mut tmp = unsafe {Fr::uninit()};
        CopyFr(&mut tmp, &sPow);
        Fr::mul(&mut sPow, &tmp, &s);
    }
/*     for i in 0..n {
        sOut.push(&mut unsafe{G1::uninit()});
        G1::mul(m,m,&s);
    } */
    sOut
}

//** */ go-kzg tipo commitment (neoptimizuotas) **//
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
    

/*     let pol = poly::Polynomial::new(vec![
        poly::Coeficient::new(b"0"),
        poly::Coeficient::new(b"5"),
        poly::Coeficient::new(b"3"),
    ]); */

    // println!("{:?}", b);

    // let out = ;

    let mut P1 = unsafe { G1::uninit() };
    // let mut P1ref = &mut P1;
    let P2 = unsafe { G1::uninit() };
    // let mut X = Fr::from_int(15);
    let mut s: Fr;

    // println!("{:?}", P1);
    let mut a = String::from("1");
    // P1 = GenerateTestingSetup("19274098162409612094609126491241927409816240961209460912649124", 32+1);
    let setup: Vec<G1> = GenerateTestingSetup(&mut a, 32+1);
    println!("{:?}", setup);
    // G1::mul(&mut P1, &P2, &X);
    // println!("{:?}",P1);
    // mclBnFp_setInt32();
    // let mut X: Fr;

    // commit_to_poly(X);


}
    // mclBn_G1EvaluatePolynomial(mclBnG1 *out, const mclBnG1 *cVec, mclSize cSize, const mclBnFr *x);  //Funkcija neimplementuota mcl_rust
    // TODO Pasigilinti į finite field table