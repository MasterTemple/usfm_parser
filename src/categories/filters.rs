/*!
usfm-grammar/node-usfm-parser/src/filters.js
*/

use crate::markers::markers::all::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub enum BookHeaders {
    IDE,
    USFM,
    H,
    TOC,
    TOCA, // IDENTIFICATION
    IMT,
    IS,
    IP,
    IPI,
    IM,
    IMI,
    IPQ,
    IMQ,
    IPR,
    IQ,
    IB,
    ILI,
    IOT,
    IO,
    IEX,
    IMTE,
    IE, // INTRO
}

pub enum Titles {
    MT,
    MTE,
    CL,
    CD,
    MS,
    MR,
    S,
    SR,
    R,
    D,
    SP,
    SD, // HEADINGS
}

pub enum Comments {
    STS,
    REM,
    LIT,
    RESTORE, // COMMENT MARKERS
}

pub enum Paragraphs {
    P,
    M,
    PO,
    PR,
    CLS,
    PMO,
    PM,
    PMC, // PARAGRAPHS-QUOTES-LISTS-TABLES
    PMR,
    PI,
    MI,
    NB,
    PC,
    PH,
    Q,
    QR,
    QC,
    QA,
    QM,
    QD,
    LH,
    LI,
    LF,
    LIM,
    LITL,
    TR,
    TC,
    TH,
    TCR,
    THR,
    TABLE,
    B,
}

pub enum Characters {
    ADD,
    BK,
    DC,
    IOR,
    IQT,
    K,
    LITL,
    ND,
    ORD,
    PN,
    PNG,
    QAC,
    QS,
    QT,
    RQ,
    SIG,
    SLS,
    TL,
    WJ, // SPECIAL-TEXT
    EM,
    BD,
    BDIT,
    IT,
    NO,
    SC,
    SUP, // CHARACTER STYLING
    RB,
    PRO,
    W,
    WH,
    WA,
    WG, // SPECIAL-FEATURES
    LIK,
    LIV, // STRUCTURED LIST ENTRIES
    JMP,
}

pub enum Notes {
    F,
    FE,
    EF,
    EFE,
    X,
    EX, // FOOTNOTES-AND-CROSSREFS
    FR,
    FT,
    FK,
    FQ,
    FQA,
    FL,
    FW,
    FP,
    FV,
    FDC,
    XO,
    XOP,
    XT,
    XTA,
    XK,
    XQ,
    XOT,
    XNT,
    XDC,
}

pub enum StudyBible {
    ESB,
    CAT, // SIDEBARS-EXTENDED-CONTENTS
}

pub enum BCV {
    ID,
    C,
    V,
}

// pub enum TEXT { TEXT-IN-EXCLUDED-PARENT, TEXT }