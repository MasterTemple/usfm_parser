use std::collections::HashMap;

use enum_dispatch::enum_dispatch;
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

use super::all::*;

use crate::markers::markers::{
    any::AnyMarker,
    parameters::{FromMarkerParameters, MarkerParameters},
};

pub trait Tag {
    const TAG: &'static str;
}

type MarkerFn = fn(MarkerParameters) -> AnyMarker;

pub(crate) static TAG_MAP: Lazy<HashMap<&'static str, MarkerFn>> = Lazy::new(|| {
    let mut map: HashMap<&'static str, MarkerFn> = HashMap::new();

    macro_rules! insert {
        ($($variant:ident),*) => {
            $(
                map.insert(
                    $variant::TAG,
                    |params: MarkerParameters| {
                        AnyMarker::$variant($variant::from_parameters(params))
                    },
                );
            )*
        };
    }

    insert![
        ADD, ADDPN, B, PoetryB, BD, BDIT, BK, C, CA, CD, CL, CLS, CP, D, DC,
        // DoubleForwardSlash,
        EM, F, FDC, FE, FIG, FK, FL, FM, FP, FQ, FQA, FR, FT, FV, FW, H, IB, ID, IDE, IE, IEX, ILI,
        IM, IMI, IMQ, IMT, IMTE, IO, IOR, IOT, IP, IPI, IPQ, IPR, IQ, IQT, IS, IT, K, LF, LH, LI,
        LIK, LIM, LIT, LITL, LIV, M, MI, MR, MS, MT, MTE, NB, ND, NDX, NO, ORD, P, PB, PC, PH, PI,
        PM, PMC, PMO, PMR, PN, PNG, PO, PR, PRO, Q, QA, QAC, QC, QD, QM, QR, QS, QT, R, RB, REM,
        RQ, HeadingRQ, S, SC, SD, SIG, SLS, SP, SR, STS, SUP, TC, TCR, TH, THR, TL, TOC, TOCA, TR,
        // Tilde,
        USFM, V, VA, VP, W, WA, WG, WH, WJ, X, XDC, XK, XNT, XO, XOP, XOT, XQ, XT, XTA
    ];

    map
});
