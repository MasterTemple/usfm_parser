use serde::{Deserialize, Serialize, de::DeserializeOwned};
use strum::{AsRefStr, EnumIter, EnumString};

#[derive(crate::Cmp!)]
#[derive(Clone, Copy, Debug, AsRefStr, EnumIter, EnumString, strum::Display, Serialize, Deserialize)]
pub enum BookCode {
    /// 01
    /// GEN
    /// Genesis
    /// ‘1 Moses’ in some Bibles
    #[strum(serialize = "01", serialize = "GEN")]
    #[serde(rename = "GEN")]
    Genesis,

    /// 02
    /// EXO
    /// Exodus
    /// ‘2 Moses’ in some Bibles
    #[strum(serialize = "02", serialize = "EXO")]
    #[serde(rename = "EXO")]
    Exodus,

    /// 03
    /// LEV
    /// Leviticus
    /// ‘3 Moses’ in some Bibles
    #[strum(serialize = "03", serialize = "LEV")]
    #[serde(rename = "LEV")]
    Leviticus,

    /// 04
    /// NUM
    /// Numbers
    /// ‘4 Moses’ in some Bibles
    #[strum(serialize = "04", serialize = "NUM")]
    #[serde(rename = "NUM")]
    Numbers,

    /// 05
    /// DEU
    /// Deuteronomy
    /// ‘5 Moses’ in some Bibles
    #[strum(serialize = "05", serialize = "DEU")]
    #[serde(rename = "DEU")]
    Deuteronomy,

    /// 06
    /// JOS
    /// Joshua
    #[strum(serialize = "06", serialize = "JOS")]
    #[serde(rename = "JOS")]
    Joshua,

    /// 07
    /// JDG
    /// Judges
    #[strum(serialize = "07", serialize = "JDG")]
    #[serde(rename = "JDG")]
    Judges,

    /// 08
    /// RUT
    /// Ruth
    #[strum(serialize = "08", serialize = "RUT")]
    #[serde(rename = "RUT")]
    Ruth,

    /// 09
    /// 1SA
    /// 1 Samuel
    /// 1 Kings or Kingdoms in Orthodox Bibles;
    /// do not confuse this abbreviation with
    /// ISA for Isaiah
    #[strum(serialize = "09", serialize = "1SA")]
    #[serde(rename = "1SA")]
    Samuel1,

    /// 10
    /// 2SA
    /// 2 Samuel
    /// 2 Kings or Kingdoms in Orthodox Bibles
    #[strum(serialize = "10", serialize = "2SA")]
    #[serde(rename = "2SA")]
    Samuel2,

    /// 11
    /// 1KI
    /// 1 Kings
    /// 3 Kings or Kingdoms in Orthodox Bibles
    #[strum(serialize = "11", serialize = "1KI")]
    #[serde(rename = "1KI")]
    Kings1,

    /// 12
    /// 2KI
    /// 2 Kings
    /// 4 Kings or Kingdoms in Orthodox Bibles
    #[strum(serialize = "12", serialize = "2KI")]
    #[serde(rename = "2KI")]
    Kings2,

    /// 13
    /// 1CH
    /// 1 Chronicles
    /// 1 Paralipomenon in Orthodox Bibles
    #[strum(serialize = "13", serialize = "1CH")]
    #[serde(rename = "1CH")]
    Chronicles1,

    /// 14
    /// 2CH
    /// 2 Chronicles
    /// 2 Paralipomenon in Orthodox Bibles
    #[strum(serialize = "14", serialize = "2CH")]
    #[serde(rename = "2CH")]
    Chronicles2,

    /// 15
    /// EZR
    /// Ezra
    /// This is for Hebrew Ezra, sometimes called
    /// 1 Ezra or 1 Esdras; also for Ezra-Nehemiah
    /// when one book
    #[strum(serialize = "15", serialize = "EZR")]
    #[serde(rename = "EZR")]
    Ezra,

    /// 16
    /// NEH
    /// Nehemiah
    /// Sometimes appended to Ezra; called
    /// 2 Esdras in the Vulgate
    #[strum(serialize = "16", serialize = "NEH")]
    #[serde(rename = "NEH")]
    Nehemiah,

    /// 17
    /// EST
    /// Esther (Hebrew)
    /// This is for Hebrew Esther; for the longer
    /// Greek LXX Esther use ESG
    #[strum(serialize = "17", serialize = "EST")]
    #[serde(rename = "EST")]
    Esther,

    /// 18
    /// JOB
    /// Job
    #[strum(serialize = "18", serialize = "JOB")]
    #[serde(rename = "JOB")]
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
    #[serde(rename = "PSA")]
    Psalms,

    /// 20
    /// PRO
    /// Proverbs
    /// 31 Proverbs, but 24 Proverbs in the
    /// Ethiopian Bible
    #[strum(serialize = "20", serialize = "PRO")]
    #[serde(rename = "PRO")]
    Proverbs,

    /// 21
    /// ECC
    /// Ecclesiastes
    /// Qoholeth in Catholic Bibles; for
    /// Ecclesiasticus use SIR
    #[strum(serialize = "21", serialize = "ECC")]
    #[serde(rename = "ECC")]
    Ecclesiastes,

    /// 22
    /// SNG
    /// Song of Songs
    /// Song of Solomon, or Canticles of Canticles in
    /// Catholic Bibles
    #[strum(serialize = "22", serialize = "SNG")]
    #[serde(rename = "SNG")]
    SongofSolomon,

    /// 23
    /// ISA
    /// Isaiah
    /// Do not confuse this abbreviation with 1SA
    /// for 1 Samuel
    #[strum(serialize = "23", serialize = "ISA")]
    #[serde(rename = "ISA")]
    Isaiah,

    /// 24
    /// JER
    /// Jeremiah
    /// The Book of Jeremiah; for the Letter of
    /// Jeremiah use LJE
    #[strum(serialize = "24", serialize = "JER")]
    #[serde(rename = "JER")]
    Jeremiah,

    /// 25
    /// LAM
    /// Lamentations
    /// The Lamentations of Jeremiah
    #[strum(serialize = "25", serialize = "LAM")]
    #[serde(rename = "LAM")]
    Lamentations,

    /// 26
    /// EZK
    /// Ezekiel
    #[strum(serialize = "26", serialize = "EZK")]
    #[serde(rename = "EZK")]
    Ezekiel,

    /// 27
    /// DAN
    /// Daniel (Hebrew)
    /// This is for Hebrew Daniel; for the longer
    /// Greek LXX Daniel use DAG
    #[strum(serialize = "27", serialize = "DAN")]
    #[serde(rename = "DAN")]
    Daniel,

    /// 28
    /// HOS
    /// Hosea
    #[strum(serialize = "28", serialize = "HOS")]
    #[serde(rename = "HOS")]
    Hosea,

    /// 29
    /// JOL
    /// Joel
    #[strum(serialize = "29", serialize = "JOL")]
    #[serde(rename = "JOL")]
    Joel,

    /// 30
    /// AMO
    /// Amos
    #[strum(serialize = "30", serialize = "AMO")]
    #[serde(rename = "AMO")]
    Amos,

    /// 31
    /// OBA
    /// Obadiah
    #[strum(serialize = "31", serialize = "OBA")]
    #[serde(rename = "OBA")]
    Obadiah,

    /// 32
    /// JON
    /// Jonah
    /// Do not confuse this abbreviation with JHN
    /// for John
    #[strum(serialize = "32", serialize = "JON")]
    #[serde(rename = "JON")]
    Jonah,

    /// 33
    /// MIC
    /// Micah
    #[strum(serialize = "33", serialize = "MIC")]
    #[serde(rename = "MIC")]
    Micah,

    /// 34
    /// NAM
    /// Nahum
    #[strum(serialize = "34", serialize = "NAM")]
    #[serde(rename = "NAM")]
    Nahum,

    /// 35
    /// HAB
    /// Habakkuk
    #[strum(serialize = "35", serialize = "HAB")]
    #[serde(rename = "HAB")]
    Habakkuk,

    /// 36
    /// ZEP
    /// Zephaniah
    #[strum(serialize = "36", serialize = "ZEP")]
    #[serde(rename = "ZEP")]
    Zephaniah,

    /// 37
    /// HAG
    /// Haggai
    #[strum(serialize = "37", serialize = "HAG")]
    #[serde(rename = "HAG")]
    Haggai,

    /// 38
    /// ZEC
    /// Zechariah
    #[strum(serialize = "38", serialize = "ZEC")]
    #[serde(rename = "ZEC")]
    Zechariah,

    /// 39
    /// MAL
    /// Malachi
    #[strum(serialize = "39", serialize = "MAL")]
    #[serde(rename = "MAL")]
    Malachi,

    /// 41
    /// MAT
    /// Matthew
    /// The Gospel according to Matthew
    #[strum(serialize = "41", serialize = "MAT")]
    #[serde(rename = "MAT")]
    Matthew,

    /// 42
    /// MRK
    /// Mark
    /// The Gospel according to Mark
    #[strum(serialize = "42", serialize = "MRK")]
    #[serde(rename = "MRK")]
    Mark,

    /// 43
    /// LUK
    /// Luke
    /// The Gospel according to Luke
    #[strum(serialize = "43", serialize = "LUK")]
    #[serde(rename = "LUK")]
    Luke,

    /// 44
    /// JHN
    /// John
    /// The Gospel according to John
    #[strum(serialize = "44", serialize = "JHN")]
    #[serde(rename = "JHN")]
    John,

    /// 45
    /// ACT
    /// Acts
    /// The Acts of the Apostles
    #[strum(serialize = "45", serialize = "ACT")]
    #[serde(rename = "ACT")]
    Acts,

    /// 46
    /// ROM
    /// Romans
    /// The Letter of Paul to the Romans
    #[strum(serialize = "46", serialize = "ROM")]
    #[serde(rename = "ROM")]
    Romans,

    /// 47
    /// 1CO
    /// 1 Corinthians
    /// The First Letter of Paul to the Corinthians
    #[strum(serialize = "47", serialize = "1CO")]
    #[serde(rename = "1CO")]
    Corinthians1,

    /// 48
    /// 2CO
    /// 2 Corinthians
    /// The Second Letter of Paul to the Corinthians
    #[strum(serialize = "48", serialize = "2CO")]
    #[serde(rename = "2CO")]
    Corinthians2,

    /// 49
    /// GAL
    /// Galatians
    /// The Letter of Paul to the Galatians
    #[strum(serialize = "49", serialize = "GAL")]
    #[serde(rename = "GAL")]
    Galatians,

    /// 50
    /// EPH
    /// Ephesians
    /// The Letter of Paul to the Ephesians
    #[strum(serialize = "50", serialize = "EPH")]
    #[serde(rename = "EPH")]
    Ephesians,

    /// 51
    /// PHP
    /// Philippians
    /// The Letter of Paul to the Philippians
    #[strum(serialize = "51", serialize = "PHP")]
    #[serde(rename = "PHP")]
    Philippians,

    /// 52
    /// COL
    /// Colossians
    /// The Letter of Paul to the Colossians
    #[strum(serialize = "52", serialize = "COL")]
    #[serde(rename = "COL")]
    Colossians,

    /// 53
    /// 1TH
    /// 1 Thessalonians
    /// The First Letter of Paul to the Thessalonians
    #[strum(serialize = "53", serialize = "1TH")]
    #[serde(rename = "1TH")]
    Thessalonians1,

    /// 54
    /// 2TH
    /// 2 Thessalonians
    /// The Second Letter of Paul to the
    /// Thessalonians
    #[strum(serialize = "54", serialize = "2TH")]
    #[serde(rename = "2TH")]
    Thessalonians2,

    /// 55
    /// 1TI
    /// 1 Timothy
    /// The First Letter of Paul to Timothy
    #[strum(serialize = "55", serialize = "1TI")]
    #[serde(rename = "1TI")]
    Timothy1,

    /// 56
    /// 2TI
    /// 2 Timothy
    /// The Second Letter of Paul to Timothy
    #[strum(serialize = "56", serialize = "2TI")]
    #[serde(rename = "2TI")]
    Timothy2,

    /// 57
    /// TIT
    /// Titus
    /// The Letter of Paul to Titus
    #[strum(serialize = "57", serialize = "TIT")]
    #[serde(rename = "TIT")]
    Titus,

    /// 58
    /// PHM
    /// Philemon
    /// The Letter of Paul to Philemon
    #[strum(serialize = "58", serialize = "PHM")]
    #[serde(rename = "PHM")]
    Philemon,

    /// 59
    /// HEB
    /// Hebrews
    /// The Letter to the Hebrews
    #[strum(serialize = "59", serialize = "HEB")]
    #[serde(rename = "HEB")]
    Hebrews,

    /// 60
    /// JAS
    /// James
    /// The Letter of James
    #[strum(serialize = "60", serialize = "JAS")]
    #[serde(rename = "JAS")]
    James,

    /// 61
    /// 1PE
    /// 1 Peter
    /// The First Letter of Peter
    #[strum(serialize = "61", serialize = "1PE")]
    #[serde(rename = "1PE")]
    Peter1,

    /// 62
    /// 2PE
    /// 2 Peter
    /// The Second Letter of Peter
    #[strum(serialize = "62", serialize = "2PE")]
    #[serde(rename = "2PE")]
    Peter2,

    /// 63
    /// 1JN
    /// 1 John
    /// The First Letter of John
    #[strum(serialize = "63", serialize = "1JN")]
    #[serde(rename = "1JN")]
    John1,

    /// 64
    /// 2JN
    /// 2 John
    /// The Second Letter of John
    #[strum(serialize = "64", serialize = "2JN")]
    #[serde(rename = "2JN")]
    John2,

    /// 65
    /// 3JN
    /// 3 John
    /// The Third Letter of John
    #[strum(serialize = "65", serialize = "3JN")]
    #[serde(rename = "3JN")]
    John3,

    /// 66
    /// JUD
    /// Jude
    /// The Letter of Jude; do not confuse this
    /// abbreviation with JDG for Judges, or
    /// JDT for Judith
    #[strum(serialize = "66", serialize = "JUD")]
    #[serde(rename = "JUD")]
    Jude,

    /// 67
    /// REV
    /// Revelation
    /// The Revelation to John; called Apocalypse in
    /// Catholic Bibles
    #[strum(serialize = "67", serialize = "REV")]
    #[serde(rename = "REV")]
    Revelation,

    /// 68
    /// TOB
    /// Tobit
    #[strum(serialize = "68", serialize = "TOB")]
    #[serde(rename = "TOB")]
    Tobit,

    /// 69
    /// JDT
    /// Judith
    #[strum(serialize = "69", serialize = "JDT")]
    #[serde(rename = "JDT")]
    Judith,

    /// 70
    /// ESG
    /// Esther Greek
    #[strum(serialize = "70", serialize = "ESG")]
    #[serde(rename = "ESG")]
    EstherGreek,

    /// 71
    /// WIS
    /// Wisdom of Solomon
    #[strum(serialize = "71", serialize = "WIS")]
    #[serde(rename = "WIS")]
    Wisdom,

    /// 72
    /// SIR
    /// Sirach
    /// Ecclesiasticus or Jesus son of Sirach
    #[strum(serialize = "72", serialize = "SIR")]
    #[serde(rename = "SIR")]
    Sirach,

    /// 73
    /// BAR
    /// Baruch
    /// 5 chapters in Orthodox Bibles (LJE is
    /// separate); 6 chapters in Catholic Bibles
    /// (includes LJE); called 1 Baruch in Syriac
    /// Bibles
    #[strum(serialize = "73", serialize = "BAR")]
    #[serde(rename = "BAR")]
    Baruch,

    /// 74
    /// LJE
    /// Letter of Jeremiah
    /// Sometimes included in Baruch; called ‘Rest
    /// of Jeremiah’ in Ethiopia
    #[strum(serialize = "74", serialize = "LJE")]
    #[serde(rename = "LJE")]
    Letter,

    /// 75
    /// S3Y
    /// Song of the 3 Young Men
    /// Includes the Prayer of Azariah; sometimes
    /// included in Greek Daniel
    #[strum(serialize = "75", serialize = "S3Y")]
    #[serde(rename = "S3Y")]
    Song,

    /// 76
    /// SUS
    /// Susanna
    /// Sometimes included in Greek Daniel
    #[strum(serialize = "76", serialize = "SUS")]
    #[serde(rename = "SUS")]
    Susanna,

    /// 77
    /// BEL
    /// Bel and the Dragon
    /// Sometimes included in Greek Daniel; called
    /// ‘Rest of Daniel’ in Ethiopia
    #[strum(serialize = "77", serialize = "BEL")]
    #[serde(rename = "BEL")]
    Bel,

    /// 78
    /// 1MA
    /// 1 Maccabees
    /// Called ‘3 Maccabees’ in some traditions,
    /// printed in Catholic and Orthodox Bibles
    #[strum(serialize = "78", serialize = "1MA")]
    #[serde(rename = "1MA")]
    Maccabees1,

    /// 79
    /// 2MA
    /// 2 Maccabees
    /// Called ‘1 Maccabees’ in some traditions,
    /// printed in Catholic and Orthodox Bibles
    #[strum(serialize = "79", serialize = "2MA")]
    #[serde(rename = "2MA")]
    Maccabees2,

    /// 80
    /// 3MA
    /// 3 Maccabees
    /// Called ‘2 Maccabees’ in some traditions,
    /// printed in Orthodox Bibles
    #[strum(serialize = "80", serialize = "3MA")]
    #[serde(rename = "3MA")]
    Maccabees3,

    /// 81
    /// 4MA
    /// 4 Maccabees
    /// In an appendix to the Greek Bible and in the
    /// Georgian Bible
    #[strum(serialize = "81", serialize = "4MA")]
    #[serde(rename = "4MA")]
    Maccabees4,

    /// 82
    /// 1ES
    /// 1 Esdras (Greek)
    /// The 9 chapter book of Greek Ezra in the LXX,
    /// called ‘2 Esdras’ in Russian Bibles,
    /// and called ‘3 Esdras’ in the Vulgate;
    /// when Ezra-Nehemiah is one book use EZR
    #[strum(serialize = "82", serialize = "1ES")]
    #[serde(rename = "1ES")]
    Esdras1,

    /// 83
    /// 2ES
    /// 2 Esdras (Latin)
    /// The 16 chapter book of Latin Esdras called
    /// ‘3 Esdras’ in Russian Bibles and called
    /// ‘4 Esdras’ in the Vulgate; for the 12 chapter
    /// Apocalypse of Ezra use EZA
    #[strum(serialize = "83", serialize = "2ES")]
    #[serde(rename = "2ES")]
    Esdras2,

    /// 84
    /// MAN
    /// Prayer of Manasseh
    /// Sometimes appended to 2 Chronicles,
    /// included in Orthodox Bibles
    #[strum(serialize = "84", serialize = "MAN")]
    #[serde(rename = "MAN")]
    PrayerofManasseh,

    /// 85
    /// PS2
    /// Psalm 151
    /// An additional Psalm in the Septuagint,
    /// appended to Psalms in Orthodox Bibles
    #[strum(serialize = "85", serialize = "PS2")]
    #[serde(rename = "PS2")]
    Psalm151,

    /// 86
    /// ODA
    /// Odae/Odes
    /// A book in some editions of the Septuagint;
    /// Odes has different contents in Greek,
    /// Russian, and Syriac traditions
    #[strum(serialize = "86", serialize = "ODA")]
    #[serde(rename = "ODA")]
    OdaeOdes,

    /// 87
    /// PSS
    /// Psalms of Solomon
    /// A book in some editions of the Septuagint,
    /// but not printed in modern Bibles
    #[strum(serialize = "87", serialize = "PSS")]
    #[serde(rename = "PSS")]
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
    #[serde(rename = "EZA")]
    EzraApocalypse,

    /// A5
    /// 5EZ
    /// 5 Ezra
    /// 2 chapter Latin preface to Ezra Apocalypse;
    /// formerly called 5ES
    #[strum(serialize = "A5", serialize = "5EZ")]
    #[serde(rename = "5EZ")]
    Ezra5,

    /// A6
    /// 6EZ
    /// 6 Ezra
    /// 2 chapter Latin conclusion to Ezra
    /// Apocalypse; formerly called 6ES
    #[strum(serialize = "A6", serialize = "6EZ")]
    #[serde(rename = "6EZ")]
    Ezra6,

    /// B2
    /// DAG
    /// Daniel Greek
    /// The 14 chapter version of Daniel from the
    /// Septuagint including Greek additions
    #[strum(serialize = "B2", serialize = "DAG")]
    #[serde(rename = "DAG")]
    DanielGreek,

    /// B3
    /// PS3
    /// Psalms 152-155
    /// Additional Psalms 152-155 found in West
    /// Syriac manuscripts
    #[strum(serialize = "B3", serialize = "PS3")]
    #[serde(rename = "PS3")]
    Psalms152To155,

    /// B4
    /// 2BA
    /// 2 Baruch (Apocalypse)
    /// The Apocalypse of Baruch in Syriac Bibles
    #[strum(serialize = "B4", serialize = "2BA")]
    #[serde(rename = "2BA")]
    Baruch2,

    /// B5
    /// LBA
    /// Letter of Baruch
    /// Sometimes appended to 2 Baruch;
    /// sometimes separate in Syriac Bibles
    #[strum(serialize = "B5", serialize = "LBA")]
    #[serde(rename = "LBA")]
    LetterOfBaruch,

    /// B6
    /// JUB
    /// Jubilees
    /// Ancient Hebrew book used in the Ethiopian
    /// Bible
    #[strum(serialize = "B6", serialize = "JUB")]
    #[serde(rename = "JUB")]
    Jubilees,

    /// B7
    /// ENO
    /// Enoch
    /// Sometimes called ‘1 Enoch’; ancient Hebrew
    /// book in the Ethiopian Bible
    #[strum(serialize = "B7", serialize = "ENO")]
    #[serde(rename = "ENO")]
    Enoch,

    /// B8
    /// 1MQ
    /// 1 Meqabyan/Mekabis
    /// Book of Mekabis of Benjamin in the
    /// Ethiopian Bible
    #[strum(serialize = "B8", serialize = "1MQ")]
    #[serde(rename = "1MQ")]
    Meqabyan1,

    /// B9
    /// 2MQ
    /// 2 Meqabyan/Mekabis
    /// Book of Mekabis of Moab in the Ethiopian
    /// Bible
    #[strum(serialize = "B9", serialize = "2MQ")]
    #[serde(rename = "2MQ")]
    Meqabyan2,

    /// C0
    /// 3MQ
    /// 3 Meqabyan/Mekabis
    /// Book of Meqabyan in the Ethiopian Bible
    #[strum(serialize = "C0", serialize = "3MQ")]
    #[serde(rename = "3MQ")]
    Meqabyan3,

    /// C1
    /// REP
    /// Reproof
    /// Proverbs part 2: Used in the Ethiopian Bible
    #[strum(serialize = "C1", serialize = "REP")]
    #[serde(rename = "REP")]
    Reproof,

    /// C2
    /// 4BA
    /// 4 Baruch
    /// Paralipomenon of Jeremiah, called ‘Rest of
    /// the Words of Baruch’ in Ethiopia; may
    /// include or exclude the Letter of Jeremiah
    /// as chapter 1, used in the Ethiopian Bible
    #[strum(serialize = "C2", serialize = "4BA")]
    #[serde(rename = "4BA")]
    Baruch4,

    /// C3
    /// LAO
    /// Letter to the Laodiceans
    /// A Latin Vulgate book, found in the Vulgate
    /// and some medieval Catholic translations
    #[strum(serialize = "C3", serialize = "LAO")]
    #[serde(rename = "LAO")]
    LetterToLaodiceans,

    /// A0
    /// FRT
    /// Front Matter
    #[strum(serialize = "A0", serialize = "FRT")]
    #[serde(rename = "FRT")]
    FrontMatter,

    /// A1
    /// BAK
    /// Back Matter
    #[strum(serialize = "A1", serialize = "BAK")]
    #[serde(rename = "BAK")]
    BackMatter,

    /// A2
    /// OTH
    /// Other Matter
    #[strum(serialize = "A2", serialize = "OTH")]
    #[serde(rename = "OTH")]
    OtherMatter,

    /// A7
    /// INT
    /// Introduction Matter
    #[strum(serialize = "A7", serialize = "INT")]
    #[serde(rename = "INT")]
    IntroductionMatter,

    /// A8
    /// CNC
    /// Concordance
    #[strum(serialize = "A8", serialize = "CNC")]
    #[serde(rename = "CNC")]
    Concordance,

    /// A9
    /// GLO
    /// Glossary / Wordlist
    #[strum(serialize = "A9", serialize = "GLO")]
    #[serde(rename = "GLO")]
    Glossary,

    /// B0
    /// TDX
    /// Topical Index
    #[strum(serialize = "B0", serialize = "TDX")]
    #[serde(rename = "TDX")]
    TopicalIndex,

    /// B1
    /// NDX
    /// Names Index
    #[strum(serialize = "B1", serialize = "NDX")]
    #[serde(rename = "NDX")]
    NamesIndex,

    /// 94
    /// XXA
    /// Extra material
    #[strum(serialize = "94", serialize = "XXA")]
    #[serde(rename = "XXA")]
    ExtraMaterialA,

    /// 95
    /// XXB
    /// Extra material
    #[strum(serialize = "95", serialize = "XXB")]
    #[serde(rename = "XXB")]
    ExtraMaterialB,

    /// 96
    /// XXC
    /// Extra material
    #[strum(serialize = "96", serialize = "XXC")]
    #[serde(rename = "XXC")]
    ExtraMaterialC,

    /// 97
    /// XXD
    /// Extra material
    #[strum(serialize = "97", serialize = "XXD")]
    #[serde(rename = "XXD")]
    ExtraMaterialD,

    /// 98
    /// XXE
    /// Extra material
    #[strum(serialize = "98", serialize = "XXE")]
    #[serde(rename = "XXE")]
    ExtraMaterialE,

    /// 99
    /// XXF
    /// Extra material
    #[strum(serialize = "99", serialize = "XXF")]
    #[serde(rename = "XXF")]
    ExtraMaterialF,

    /// 100
    /// XXG
    /// Extra material
    #[strum(serialize = "100", serialize = "XXG")]
    #[serde(rename = "XXG")]
    ExtraMaterialG,
}

// impl BookCode {
//     /// This returns a `&str` because there are 'numbers' like `A1`
//     pub fn number(&self) -> &'static str {}
//     /// These are 3-character identifiers, such as `"GEN"` for `"Genesis"`
//     pub fn identifier(&self) -> &'static str {}
// }
