//! Provides the [gamma](https://en.wikipedia.org/wiki/Gamma_function) and
//! related functions

use crate::factorial::consts as consts;

/// Auxiliary variable when evaluating the `gamma_ln` function
const GAMMA_R: f64 = 10.900511;

/// Polynomial coefficients for approximating the `gamma_ln` function
const GAMMA_DK: &'static [f64] = &[
    2.48574089138753565546e-5,
    1.05142378581721974210,
    -3.45687097222016235469,
    4.51227709466894823700,
    -2.98285225323576655721,
    1.05639711577126713077,
    -1.95428773191645869583e-1,
    1.70970543404441224307e-2,
    -5.71926117404305781283e-4,
    4.63399473359905636708e-6,
    -2.71994908488607703910e-9,
];

/// Computes the logarithm of the gamma function
/// with an accuracy of 16 floating point digits.
/// The implementation is derived from
/// "An Analysis of the Lanczos Gamma Approximation",
/// Glendon Ralph Pugh, 2004 p. 116
pub fn ln_gamma(x: f64) -> f64 {
    use std::f64::consts::{PI, E};

    if x < 0.5 {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (t.0 as f64 - x));

        consts::LN_PI
            - (PI * x).sin().ln()
            - s.ln()
            - consts::LN_2_SQRT_E_OVER_PI
            - (0.5 - x) * ((0.5 - x + GAMMA_R) / E).ln()
    } else {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (x + t.0 as f64 - 1.0));

        s.ln()
            + consts::LN_2_SQRT_E_OVER_PI
            + (x - 0.5) * ((x - 0.5 + GAMMA_R) / E).ln()
    }
}

/// Computes the gamma function with an accuracy
/// of 16 floating point digits. The implementation
/// is derived from "An Analysis of the Lanczos Gamma Approximation",
/// Glendon Ralph Pugh, 2004 p. 116
pub fn gamma(x: f64) -> f64 {
    use std::f64::consts::{PI, E};
    if x < 0.5 {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (t.0 as f64 - x));

        PI
            / ((PI * x).sin()
                * s
                * consts::TWO_SQRT_E_OVER_PI
                * ((0.5 - x + GAMMA_R) / E).powf(0.5 - x))
    } else {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (x + t.0 as f64 - 1.0));

        s * consts::TWO_SQRT_E_OVER_PI * ((x - 0.5 + GAMMA_R) / E).powf(x - 0.5)
    }
}


#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(test)]
mod test {
    use std::f64::{self, consts};
    use approx::assert_abs_diff_eq;
    #[test]
    fn test_gamma() {

        assert!(super::gamma(f64::NAN).is_nan());
        assert_abs_diff_eq!(super::gamma(1.000001e-35), 9.9999900000099999900000099999899999522784235098567139293e+34, epsilon=1e20);
        assert_abs_diff_eq!(super::gamma(1.000001e-10), 9.99998999943278432519738283781280989934496494539074049002e+9, epsilon=1e-5);
        assert_abs_diff_eq!(super::gamma(1.000001e-5), 99999.32279432557746387178953902739303931424932435387031653234, epsilon=1e-10);
        assert_abs_diff_eq!(super::gamma(1.000001e-2), 99.43248512896257405886134437203369035261893114349805309870831, epsilon=1e-13);
        assert_abs_diff_eq!(super::gamma(-4.8), -0.06242336135475955314181664931547009890495158793105543559676, epsilon=1e-13);
        assert_abs_diff_eq!(super::gamma(-1.5), 2.363271801207354703064223311121526910396732608163182837618410, epsilon=1e-13);
        assert_abs_diff_eq!(super::gamma(-0.5), -3.54490770181103205459633496668229036559509891224477425642761, epsilon=1e-13);
        assert_abs_diff_eq!(super::gamma(1.0e-5 + 1.0e-16), 99999.42279322556767360213300482199406241771308740302819426480, epsilon=1e-9);
        assert_abs_diff_eq!(super::gamma(0.1), 9.513507698668731836292487177265402192550578626088377343050000, epsilon=1e-14);
        assert_eq!(super::gamma(1.0 - 1.0e-14), 1.000000000000005772156649015427511664653698987042926067639529);
        assert_abs_diff_eq!(super::gamma(1.0), 1.0, epsilon=1e-15);
        assert_abs_diff_eq!(super::gamma(1.0 + 1.0e-14), 0.99999999999999422784335098477029953441189552403615306268023, epsilon=1e-15);
        assert_abs_diff_eq!(super::gamma(1.5), 0.886226925452758013649083741670572591398774728061193564106903, epsilon=1e-14);
        assert_abs_diff_eq!(super::gamma(consts::PI/2.0), 0.890560890381539328010659635359121005933541962884758999762766, epsilon=1e-15);
        assert_eq!(super::gamma(2.0), 1.0);
        assert_abs_diff_eq!(super::gamma(2.5), 1.329340388179137020473625612505858887098162092091790346160355, epsilon=1e-13);
        assert_abs_diff_eq!(super::gamma(3.0), 2.0, epsilon=1e-14);
        assert_abs_diff_eq!(super::gamma(consts::PI), 2.288037795340032417959588909060233922889688153356222441199380, epsilon=1e-13);
        assert_abs_diff_eq!(super::gamma(3.5), 3.323350970447842551184064031264647217745405230229475865400889, epsilon=1e-14);
        assert_abs_diff_eq!(super::gamma(4.0), 6.0, epsilon=1e-13);
        assert_abs_diff_eq!(super::gamma(4.5), 11.63172839656744892914422410942626526210891830580316552890311, epsilon=1e-12);
        assert_abs_diff_eq!(super::gamma(5.0 - 1.0e-14), 23.99999999999963853175957637087420162718107213574617032780374, epsilon=1e-13);
        assert_abs_diff_eq!(super::gamma(5.0), 24.0, epsilon=1e-12);
        assert_abs_diff_eq!(super::gamma(5.0 + 1.0e-14), 24.00000000000036146824042363510111050137786752408660789873592, epsilon=1e-12);
        assert_abs_diff_eq!(super::gamma(5.5), 52.34277778455352018114900849241819367949013237611424488006401, epsilon=1e-12);
        assert_abs_diff_eq!(super::gamma(10.1), 454760.7514415859508673358368319076190405047458218916492282448, epsilon=1e-7);
        assert_abs_diff_eq!(super::gamma(150.0 + 1.0e-12), 3.8089226376496421386707466577615064443807882167327097140e+260, epsilon=1e248);
    }

    #[test]
    fn test_ln_gamma() {
        assert!(super::ln_gamma(f64::NAN).is_nan());
        assert_eq!(super::ln_gamma(1.000001e-35), 80.59047725479209894029636783061921392709972287131139201585211);
        assert_abs_diff_eq!(super::ln_gamma(1.000001e-10), 23.02584992988323521564308637407936081168344192865285883337793, epsilon=1e-14);
        assert_abs_diff_eq!(super::ln_gamma(1.000001e-5), 11.51291869289055371493077240324332039045238086972508869965363, epsilon=1e-14);
        assert_eq!(super::ln_gamma(1.000001e-2), 4.599478872433667224554543378460164306444416156144779542513592);
        assert_abs_diff_eq!(super::ln_gamma(0.1), 2.252712651734205959869701646368495118615627222294953765041739, epsilon=1e-14);
        assert_abs_diff_eq!(super::ln_gamma(1.0 - 1.0e-14), 5.772156649015410852768463312546533565566459794933360600e-15, epsilon=1e-15);
        assert_abs_diff_eq!(super::ln_gamma(1.0), 0.0, epsilon=1e-15);
        assert_abs_diff_eq!(super::ln_gamma(1.0 + 1.0e-14), -5.77215664901524635936177848990288632404978978079827014e-15, epsilon=1e-15);
        assert_abs_diff_eq!(super::ln_gamma(1.5), -0.12078223763524522234551844578164721225185272790259946836386, epsilon=1e-14);
        assert_abs_diff_eq!(super::ln_gamma(consts::PI/2.0), -0.11590380084550241329912089415904874214542604767006895, epsilon=1e-14);
        assert_eq!(super::ln_gamma(2.0), 0.0);
        assert_abs_diff_eq!(super::ln_gamma(2.5), 0.284682870472919159632494669682701924320137695559894729250145, epsilon=1e-13);
        assert_abs_diff_eq!(super::ln_gamma(3.0), 0.693147180559945309417232121458176568075500134360255254120680, epsilon=1e-14);
        assert_abs_diff_eq!(super::ln_gamma(consts::PI), 0.82769459232343710152957855845235995115350173412073715, epsilon=1e-13);
        assert_abs_diff_eq!(super::ln_gamma(3.5), 1.200973602347074224816021881450712995770238915468157197042113, epsilon=1e-14);
        assert_abs_diff_eq!(super::ln_gamma(4.0), 1.791759469228055000812477358380702272722990692183004705855374, epsilon=1e-14);
        assert_abs_diff_eq!(super::ln_gamma(4.5), 2.453736570842442220504142503435716157331823510689763131380823, epsilon=1e-13);
        assert_abs_diff_eq!(super::ln_gamma(5.0 - 1.0e-14), 3.178053830347930558470257283303394288448414225994179545985931, epsilon=1e-14);
        assert_abs_diff_eq!(super::ln_gamma(5.0), 3.178053830347945619646941601297055408873990960903515214096734, epsilon=1e-14);
        assert_abs_diff_eq!(super::ln_gamma(5.0 + 1.0e-14), 3.178053830347960680823625919312848824873279228348981287761046, epsilon=1e-13);
        assert_abs_diff_eq!(super::ln_gamma(5.5), 3.957813967618716293877400855822590998551304491975006780729532, epsilon=1e-14);
        assert_abs_diff_eq!(super::ln_gamma(10.1), 13.02752673863323795851370097886835481188051062306253294740504, epsilon=1e-14);
        assert_abs_diff_eq!(super::ln_gamma(150.0 + 1.0e-12), 600.0094705553324354062157737572509902987070089159051628001813, epsilon=1e-12);
        assert_abs_diff_eq!(super::ln_gamma(1.001e+7), 1.51342135323817913130119829455205139905331697084416059779e+8, epsilon=1e-13);
    }
}
