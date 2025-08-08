use std::convert::TryFrom;
use strum::{AsRefStr, EnumIter, EnumString};

#[derive(Clone, Copy, Debug, EnumString, AsRefStr)]
pub enum BibleIdentifier {
    /// 01
    /// GEN
    /// Genesis
    /// ‘1 Moses’ in some Bibles
    #[strum(serialize = "01", serialize = "GEN")]
    GEN,

    /// 02
    /// EXO
    /// Exodus
    /// ‘2 Moses’ in some Bibles
    #[strum(serialize = "02", serialize = "EXO")]
    Exodus,

    /// 03
    /// LEV
    /// Leviticus
    /// ‘3 Moses’ in some Bibles
    #[strum(serialize = "03", serialize = "LEV")]
    Leviticus,

    /// 04
    /// NUM
    /// Numbers
    /// ‘4 Moses’ in some Bibles
    #[strum(serialize = "04", serialize = "NUM")]
    Numbers,

    /// 05
    /// DEU
    /// Deuteronomy
    /// ‘5 Moses’ in some Bibles
    #[strum(serialize = "05", serialize = "DEU")]
    Deuteronomy,

    /// 06
    /// JOS
    /// Joshua
    #[strum(serialize = "06", serialize = "JOS")]
    Joshua,

    /// 07
    /// JDG
    /// Judges
    #[strum(serialize = "07", serialize = "JDG")]
    Judges,

    /// 08
    /// RUT
    /// Ruth
    #[strum(serialize = "08", serialize = "RUT")]
    Ruth,

    /// 09
    /// 1SA
    /// 1 Samuel
    /// 1 Kings or Kingdoms in Orthodox Bibles;
    /// do not confuse this abbreviation with
    /// ISA for Isaiah
    #[strum(serialize = "09", serialize = "1SA")]
    Samuel1,

    /// 10
    /// 2SA
    /// 2 Samuel
    /// 2 Kings or Kingdoms in Orthodox Bibles
    #[strum(serialize = "10", serialize = "2SA")]
    Samuel2,

    /// 11
    /// 1KI
    /// 1 Kings
    /// 3 Kings or Kingdoms in Orthodox Bibles
    #[strum(serialize = "11", serialize = "1KI")]
    Kings1,

    /// 12
    /// 2KI
    /// 2 Kings
    /// 4 Kings or Kingdoms in Orthodox Bibles
    #[strum(serialize = "12", serialize = "2KI")]
    Kings2,

    /// 13
    /// 1CH
    /// 1 Chronicles
    /// 1 Paralipomenon in Orthodox Bibles
    #[strum(serialize = "13", serialize = "1CH")]
    Chronicles1,

    /// 14
    /// 2CH
    /// 2 Chronicles
    /// 2 Paralipomenon in Orthodox Bibles
    #[strum(serialize = "14", serialize = "2CH")]
    Chronicles2,

    /// 15
    /// EZR
    /// Ezra
    /// This is for Hebrew Ezra, sometimes called
    /// 1 Ezra or 1 Esdras; also for Ezra-Nehemiah
    /// when one book
    #[strum(serialize = "15", serialize = "EZR")]
    Ezra,

    /// 16
    /// NEH
    /// Nehemiah
    /// Sometimes appended to Ezra; called
    /// 2 Esdras in the Vulgate
    #[strum(serialize = "16", serialize = "NEH")]
    Nehemiah,

    /// 17
    /// EST
    /// Esther (Hebrew)
    /// This is for Hebrew Esther; for the longer
    /// Greek LXX Esther use ESG
    #[strum(serialize = "17", serialize = "EST")]
    Esther,

    /// 18
    /// JOB
    /// Job
    #[strum(serialize = "18", serialize = "JOB")]
    Job,

    /// 19
    /// PSA
    /// Psalms
    /// 150 Psalms in Hebrew, 151 Psalms in
    /// Orthodox Bibles, 155 Psalms in West
    /// Syriac Bibles, if you put Psalm 151
    /// separately in an Apocrypha use PS2,
    /// for Psalms 152-155 use PS3
    #[strum(serialize = "19", serialize = "PSA")]
    Psalms,

    /// 20
    /// PRO
    /// Proverbs
    /// 31 Proverbs, but 24 Proverbs in the
    /// Ethiopian Bible
    #[strum(serialize = "20", serialize = "PRO")]
    Proverbs,

    /// 21
    /// ECC
    /// Ecclesiastes
    /// Qoholeth in Catholic Bibles; for
    /// Ecclesiasticus use SIR
    #[strum(serialize = "21", serialize = "ECC")]
    Ecclesiastes,

    /// 22
    /// SNG
    /// Song of Songs
    /// Song of Solomon, or Canticles of Canticles in
    /// Catholic Bibles
    #[strum(serialize = "22", serialize = "SNG")]
    SongofSolomon,

    /// 23
    /// ISA
    /// Isaiah
    /// Do not confuse this abbreviation with 1SA
    /// for 1 Samuel
    #[strum(serialize = "23", serialize = "ISA")]
    Isaiah,

    /// 24
    /// JER
    /// Jeremiah
    /// The Book of Jeremiah; for the Letter of
    /// Jeremiah use LJE
    #[strum(serialize = "24", serialize = "JER")]
    Jeremiah,

    /// 25
    /// LAM
    /// Lamentations
    /// The Lamentations of Jeremiah
    #[strum(serialize = "25", serialize = "LAM")]
    Lamentations,

    /// 26
    /// EZK
    /// Ezekiel
    #[strum(serialize = "26", serialize = "EZK")]
    Ezekiel,

    /// 27
    /// DAN
    /// Daniel (Hebrew)
    /// This is for Hebrew Daniel; for the longer
    /// Greek LXX Daniel use DAG
    #[strum(serialize = "27", serialize = "DAN")]
    Daniel,

    /// 28
    /// HOS
    /// Hosea
    #[strum(serialize = "28", serialize = "HOS")]
    Hosea,

    /// 29
    /// JOL
    /// Joel
    #[strum(serialize = "29", serialize = "JOL")]
    Joel,

    /// 30
    /// AMO
    /// Amos
    #[strum(serialize = "30", serialize = "AMO")]
    Amos,

    /// 31
    /// OBA
    /// Obadiah
    #[strum(serialize = "31", serialize = "OBA")]
    Obadiah,

    /// 32
    /// JON
    /// Jonah
    /// Do not confuse this abbreviation with JHN
    /// for John
    #[strum(serialize = "32", serialize = "JON")]
    Jonah,

    /// 33
    /// MIC
    /// Micah
    #[strum(serialize = "33", serialize = "MIC")]
    Micah,

    /// 34
    /// NAM
    /// Nahum
    #[strum(serialize = "34", serialize = "NAM")]
    Nahum,

    /// 35
    /// HAB
    /// Habakkuk
    #[strum(serialize = "35", serialize = "HAB")]
    Habakkuk,

    /// 36
    /// ZEP
    /// Zephaniah
    #[strum(serialize = "36", serialize = "ZEP")]
    Zephaniah,

    /// 37
    /// HAG
    /// Haggai
    #[strum(serialize = "37", serialize = "HAG")]
    Haggai,

    /// 38
    /// ZEC
    /// Zechariah
    #[strum(serialize = "38", serialize = "ZEC")]
    Zechariah,

    /// 39
    /// MAL
    /// Malachi
    #[strum(serialize = "39", serialize = "MAL")]
    Malachi,

    /// 41
    /// MAT
    /// Matthew
    /// The Gospel according to Matthew
    #[strum(serialize = "41", serialize = "MAT")]
    Matthew,

    /// 42
    /// MRK
    /// Mark
    /// The Gospel according to Mark
    #[strum(serialize = "42", serialize = "MRK")]
    Mark,

    /// 43
    /// LUK
    /// Luke
    /// The Gospel according to Luke
    #[strum(serialize = "43", serialize = "LUK")]
    Luke,

    /// 44
    /// JHN
    /// John
    /// The Gospel according to John
    #[strum(serialize = "44", serialize = "JHN")]
    John,

    /// 45
    /// ACT
    /// Acts
    /// The Acts of the Apostles
    #[strum(serialize = "45", serialize = "ACT")]
    Acts,

    /// 46
    /// ROM
    /// Romans
    /// The Letter of Paul to the Romans
    #[strum(serialize = "46", serialize = "ROM")]
    Romans,

    /// 47
    /// 1CO
    /// 1 Corinthians
    /// The First Letter of Paul to the Corinthians
    #[strum(serialize = "47", serialize = "1CO")]
    Corinthians1,

    /// 48
    /// 2CO
    /// 2 Corinthians
    /// The Second Letter of Paul to the Corinthians
    #[strum(serialize = "48", serialize = "2CO")]
    Corinthians2,

    /// 49
    /// GAL
    /// Galatians
    /// The Letter of Paul to the Galatians
    #[strum(serialize = "49", serialize = "GAL")]
    Galatians,

    /// 50
    /// EPH
    /// Ephesians
    /// The Letter of Paul to the Ephesians
    #[strum(serialize = "50", serialize = "EPH")]
    Ephesians,

    /// 51
    /// PHP
    /// Philippians
    /// The Letter of Paul to the Philippians
    #[strum(serialize = "51", serialize = "PHP")]
    Philippians,

    /// 52
    /// COL
    /// Colossians
    /// The Letter of Paul to the Colossians
    #[strum(serialize = "52", serialize = "COL")]
    Colossians,

    /// 53
    /// 1TH
    /// 1 Thessalonians
    /// The First Letter of Paul to the Thessalonians
    #[strum(serialize = "53", serialize = "1TH")]
    Thessalonians1,

    /// 54
    /// 2TH
    /// 2 Thessalonians
    /// The Second Letter of Paul to the
    /// Thessalonians
    #[strum(serialize = "54", serialize = "2TH")]
    Thessalonians2,

    /// 55
    /// 1TI
    /// 1 Timothy
    /// The First Letter of Paul to Timothy
    #[strum(serialize = "55", serialize = "1TI")]
    Timothy1,

    /// 56
    /// 2TI
    /// 2 Timothy
    /// The Second Letter of Paul to Timothy
    #[strum(serialize = "56", serialize = "2TI")]
    Timothy2,

    /// 57
    /// TIT
    /// Titus
    /// The Letter of Paul to Titus
    #[strum(serialize = "57", serialize = "TIT")]
    Titus,

    /// 58
    /// PHM
    /// Philemon
    /// The Letter of Paul to Philemon
    #[strum(serialize = "58", serialize = "PHM")]
    Philemon,

    /// 59
    /// HEB
    /// Hebrews
    /// The Letter to the Hebrews
    #[strum(serialize = "59", serialize = "HEB")]
    Hebrews,

    /// 60
    /// JAS
    /// James
    /// The Letter of James
    #[strum(serialize = "60", serialize = "JAS")]
    James,

    /// 61
    /// 1PE
    /// 1 Peter
    /// The First Letter of Peter
    #[strum(serialize = "61", serialize = "1PE")]
    Peter1,

    /// 62
    /// 2PE
    /// 2 Peter
    /// The Second Letter of Peter
    #[strum(serialize = "62", serialize = "2PE")]
    Peter2,

    /// 63
    /// 1JN
    /// 1 John
    /// The First Letter of John
    #[strum(serialize = "63", serialize = "1JN")]
    John1,

    /// 64
    /// 2JN
    /// 2 John
    /// The Second Letter of John
    #[strum(serialize = "64", serialize = "2JN")]
    John2,

    /// 65
    /// 3JN
    /// 3 John
    /// The Third Letter of John
    #[strum(serialize = "65", serialize = "3JN")]
    John3,

    /// 66
    /// JUD
    /// Jude
    /// The Letter of Jude; do not confuse this
    /// abbreviation with JDG for Judges, or
    /// JDT for Judith
    #[strum(serialize = "66", serialize = "JUD")]
    Jude,

    /// 67
    /// REV
    /// Revelation
    /// The Revelation to John; called Apocalypse in
    /// Catholic Bibles
    #[strum(serialize = "67", serialize = "REV")]
    Revelation,

    /// 68
    /// TOB
    /// Tobit
    #[strum(serialize = "68", serialize = "TOB")]
    Tobit,

    /// 69
    /// JDT
    /// Judith
    #[strum(serialize = "69", serialize = "JDT")]
    Judith,

    /// 70
    /// ESG
    /// Esther Greek
    #[strum(serialize = "70", serialize = "ESG")]
    EstherGreek,

    /// 71
    /// WIS
    /// Wisdom of Solomon
    #[strum(serialize = "71", serialize = "WIS")]
    Wisdom,

    /// 72
    /// SIR
    /// Sirach
    /// Ecclesiasticus or Jesus son of Sirach
    #[strum(serialize = "72", serialize = "SIR")]
    Sirach,

    /// 73
    /// BAR
    /// Baruch
    /// 5 chapters in Orthodox Bibles (LJE is
    /// separate); 6 chapters in Catholic Bibles
    /// (includes LJE); called 1 Baruch in Syriac
    /// Bibles
    #[strum(serialize = "73", serialize = "BAR")]
    Baruch,

    /// 74
    /// LJE
    /// Letter of Jeremiah
    /// Sometimes included in Baruch; called ‘Rest
    /// of Jeremiah’ in Ethiopia
    #[strum(serialize = "74", serialize = "LJE")]
    Letter,

    /// 75
    /// S3Y
    /// Song of the 3 Young Men
    /// Includes the Prayer of Azariah; sometimes
    /// included in Greek Daniel
    #[strum(serialize = "75", serialize = "S3Y")]
    Song,

    /// 76
    /// SUS
    /// Susanna
    /// Sometimes included in Greek Daniel
    #[strum(serialize = "76", serialize = "SUS")]
    Susanna,

    /// 77
    /// BEL
    /// Bel and the Dragon
    /// Sometimes included in Greek Daniel; called
    /// ‘Rest of Daniel’ in Ethiopia
    #[strum(serialize = "77", serialize = "BEL")]
    Bel,

    /// 78
    /// 1MA
    /// 1 Maccabees
    /// Called ‘3 Maccabees’ in some traditions,
    /// printed in Catholic and Orthodox Bibles
    #[strum(serialize = "78", serialize = "1MA")]
    Maccabees1,

    /// 79
    /// 2MA
    /// 2 Maccabees
    /// Called ‘1 Maccabees’ in some traditions,
    /// printed in Catholic and Orthodox Bibles
    #[strum(serialize = "79", serialize = "2MA")]
    Maccabees2,

    /// 80
    /// 3MA
    /// 3 Maccabees
    /// Called ‘2 Maccabees’ in some traditions,
    /// printed in Orthodox Bibles
    #[strum(serialize = "80", serialize = "3MA")]
    Maccabees3,

    /// 81
    /// 4MA
    /// 4 Maccabees
    /// In an appendix to the Greek Bible and in the
    /// Georgian Bible
    #[strum(serialize = "81", serialize = "4MA")]
    Maccabees4,

    /// 82
    /// 1ES
    /// 1 Esdras (Greek)
    /// The 9 chapter book of Greek Ezra in the LXX,
    /// called ‘2 Esdras’ in Russian Bibles,
    /// and called ‘3 Esdras’ in the Vulgate;
    /// when Ezra-Nehemiah is one book use EZR
    #[strum(serialize = "82", serialize = "1ES")]
    Esdras1,

    /// 83
    /// 2ES
    /// 2 Esdras (Latin)
    /// The 16 chapter book of Latin Esdras called
    /// ‘3 Esdras’ in Russian Bibles and called
    /// ‘4 Esdras’ in the Vulgate; for the 12 chapter
    /// Apocalypse of Ezra use EZA
    #[strum(serialize = "83", serialize = "2ES")]
    Esdras2,

    /// 84
    /// MAN
    /// Prayer of Manasseh
    /// Sometimes appended to 2 Chronicles,
    /// included in Orthodox Bibles
    #[strum(serialize = "84", serialize = "MAN")]
    PrayerofManasseh,

    /// 85
    /// PS2
    /// Psalm 151
    /// An additional Psalm in the Septuagint,
    /// appended to Psalms in Orthodox Bibles
    #[strum(serialize = "85", serialize = "PS2")]
    Psalm151,

    /// 86
    /// ODA
    /// Odae/Odes
    /// A book in some editions of the Septuagint;
    /// Odes has different contents in Greek,
    /// Russian, and Syriac traditions
    #[strum(serialize = "86", serialize = "ODA")]
    OdaeOdes,

    /// 87
    /// PSS
    /// Psalms of Solomon
    /// A book in some editions of the Septuagint,
    /// but not printed in modern Bibles
    #[strum(serialize = "87", serialize = "PSS")]
    PsalmsOfSolomon,

    /// A4
    /// EZA
    /// Ezra Apocalypse
    /// 12 chapter book of Ezra Apocalypse; called
    /// ‘3 Ezra’ in the Armenian Bible, called
    /// ‘Ezra Shealtiel’ in the Ethiopian Bible;
    /// formerly called 4ES; called ‘2 Esdras’ when
    /// it includes 5 Ezra and 6 Ezra
    #[strum(serialize = "A4", serialize = "EZA")]
    EzraApocalypse,

    /// A5
    /// 5EZ
    /// 5 Ezra
    /// 2 chapter Latin preface to Ezra Apocalypse;
    /// formerly called 5ES
    #[strum(serialize = "A5", serialize = "5EZ")]
    Ezra5,

    /// A6
    /// 6EZ
    /// 6 Ezra
    /// 2 chapter Latin conclusion to Ezra
    /// Apocalypse; formerly called 6ES
    #[strum(serialize = "A6", serialize = "6EZ")]
    Ezra6,

    /// B2
    /// DAG
    /// Daniel Greek
    /// The 14 chapter version of Daniel from the
    /// Septuagint including Greek additions
    #[strum(serialize = "B2", serialize = "DAG")]
    DanielGreek,

    /// B3
    /// PS3
    /// Psalms 152-155
    /// Additional Psalms 152-155 found in West
    /// Syriac manuscripts
    #[strum(serialize = "B3", serialize = "PS3")]
    Psalms152To155,

    /// B4
    /// 2BA
    /// 2 Baruch (Apocalypse)
    /// The Apocalypse of Baruch in Syriac Bibles
    #[strum(serialize = "B4", serialize = "2BA")]
    Baruch2,

    /// B5
    /// LBA
    /// Letter of Baruch
    /// Sometimes appended to 2 Baruch;
    /// sometimes separate in Syriac Bibles
    #[strum(serialize = "B5", serialize = "LBA")]
    LetterOfBaruch,

    /// B6
    /// JUB
    /// Jubilees
    /// Ancient Hebrew book used in the Ethiopian
    /// Bible
    #[strum(serialize = "B6", serialize = "JUB")]
    Jubilees,

    /// B7
    /// ENO
    /// Enoch
    /// Sometimes called ‘1 Enoch’; ancient Hebrew
    /// book in the Ethiopian Bible
    #[strum(serialize = "B7", serialize = "ENO")]
    Enoch,

    /// B8
    /// 1MQ
    /// 1 Meqabyan/Mekabis
    /// Book of Mekabis of Benjamin in the
    /// Ethiopian Bible
    #[strum(serialize = "B8", serialize = "1MQ")]
    Meqabyan1,

    /// B9
    /// 2MQ
    /// 2 Meqabyan/Mekabis
    /// Book of Mekabis of Moab in the Ethiopian
    /// Bible
    #[strum(serialize = "B9", serialize = "2MQ")]
    Meqabyan2,

    /// C0
    /// 3MQ
    /// 3 Meqabyan/Mekabis
    /// Book of Meqabyan in the Ethiopian Bible
    #[strum(serialize = "C0", serialize = "3MQ")]
    Meqabyan3,

    /// C1
    /// REP
    /// Reproof
    /// Proverbs part 2: Used in the Ethiopian Bible
    #[strum(serialize = "C1", serialize = "REP")]
    Reproof,

    /// C2
    /// 4BA
    /// 4 Baruch
    /// Paralipomenon of Jeremiah, called ‘Rest of
    /// the Words of Baruch’ in Ethiopia; may
    /// include or exclude the Letter of Jeremiah
    /// as chapter 1, used in the Ethiopian Bible
    #[strum(serialize = "C2", serialize = "4BA")]
    Baruch4,

    /// C3
    /// LAO
    /// Letter to the Laodiceans
    /// A Latin Vulgate book, found in the Vulgate
    /// and some medieval Catholic translations
    #[strum(serialize = "C3", serialize = "LAO")]
    LetterToLaodiceans,

    /// A0
    /// FRT
    /// Front Matter
    #[strum(serialize = "A0", serialize = "FRT")]
    FrontMatter,

    /// A1
    /// BAK
    /// Back Matter
    #[strum(serialize = "A1", serialize = "BAK")]
    BackMatter,

    /// A2
    /// OTH
    /// Other Matter
    #[strum(serialize = "A2", serialize = "OTH")]
    OtherMatter,

    /// A7
    /// INT
    /// Introduction Matter
    #[strum(serialize = "A7", serialize = "INT")]
    IntroductionMatter,

    /// A8
    /// CNC
    /// Concordance
    #[strum(serialize = "A8", serialize = "CNC")]
    Concordance,

    /// A9
    /// GLO
    /// Glossary / Wordlist
    #[strum(serialize = "A9", serialize = "GLO")]
    Glossary,

    /// B0
    /// TDX
    /// Topical Index
    #[strum(serialize = "B0", serialize = "TDX")]
    TopicalIndex,

    /// B1
    /// NDX
    /// Names Index
    #[strum(serialize = "B1", serialize = "NDX")]
    NamesIndex,

    /// 94
    /// XXA
    /// Extra material
    #[strum(serialize = "94", serialize = "XXA")]
    ExtraMaterialA,

    /// 95
    /// XXB
    /// Extra material
    #[strum(serialize = "95", serialize = "XXB")]
    ExtraMaterialB,

    /// 96
    /// XXC
    /// Extra material
    #[strum(serialize = "96", serialize = "XXC")]
    ExtraMaterialC,

    /// 97
    /// XXD
    /// Extra material
    #[strum(serialize = "97", serialize = "XXD")]
    ExtraMaterialD,

    /// 98
    /// XXE
    /// Extra material
    #[strum(serialize = "98", serialize = "XXE")]
    ExtraMaterialE,

    /// 99
    /// XXF
    /// Extra material
    #[strum(serialize = "99", serialize = "XXF")]
    ExtraMaterialF,

    /// 100
    /// XXG
    /// Extra material
    #[strum(serialize = "100", serialize = "XXG")]
    ExtraMaterialG,
}
