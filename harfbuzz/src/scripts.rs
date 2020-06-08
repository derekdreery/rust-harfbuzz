// This file was auto-generated using `scripts/enum_script.rs`.

use crate::sys;

/// All scripts supported by HarfBuzz
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum Script {
    /// The Common script
    Common,
    /// The Inherited script
    Inherited,
    /// The Unknown script
    Unknown,
    /// The Arabic script
    Arabic,
    /// The Armenian script
    Armenian,
    /// The Bengali script
    Bengali,
    /// The Cyrillic script
    Cyrillic,
    /// The Devanagari script
    Devanagari,
    /// The Georgian script
    Georgian,
    /// The Greek script
    Greek,
    /// The Gujarati script
    Gujarati,
    /// The Gurmukhi script
    Gurmukhi,
    /// The Hangul script
    Hangul,
    /// The Han script
    Han,
    /// The Hebrew script
    Hebrew,
    /// The Hiragana script
    Hiragana,
    /// The Kannada script
    Kannada,
    /// The Katakana script
    Katakana,
    /// The Lao script
    Lao,
    /// The Latin script
    Latin,
    /// The Malayalam script
    Malayalam,
    /// The Oriya script
    Oriya,
    /// The Tamil script
    Tamil,
    /// The Telugu script
    Telugu,
    /// The Thai script
    Thai,
    /// The Tibetan script
    Tibetan,
    /// The Bopomofo script
    Bopomofo,
    /// The Braille script
    Braille,
    /// The Canadian Syllabics script
    CanadianSyllabics,
    /// The Cherokee script
    Cherokee,
    /// The Ethiopic script
    Ethiopic,
    /// The Khmer script
    Khmer,
    /// The Mongolian script
    Mongolian,
    /// The Myanmar script
    Myanmar,
    /// The Ogham script
    Ogham,
    /// The Runic script
    Runic,
    /// The Sinhala script
    Sinhala,
    /// The Syriac script
    Syriac,
    /// The Thaana script
    Thaana,
    /// The Yi script
    Yi,
    /// The Deseret script
    Deseret,
    /// The Gothic script
    Gothic,
    /// The Old Italic script
    OldItalic,
    /// The Buhid script
    Buhid,
    /// The Hanunoo script
    Hanunoo,
    /// The Tagalog script
    Tagalog,
    /// The Tagbanwa script
    Tagbanwa,
    /// The Cypriot script
    Cypriot,
    /// The Limbu script
    Limbu,
    /// The Linear B script
    LinearB,
    /// The Osmanya script
    Osmanya,
    /// The Shavian script
    Shavian,
    /// The Tai Le script
    TaiLe,
    /// The Ugaritic script
    Ugaritic,
    /// The Buginese script
    Buginese,
    /// The Coptic script
    Coptic,
    /// The Glagolitic script
    Glagolitic,
    /// The Kharoshthi script
    Kharoshthi,
    /// The New Tai Lue script
    NewTaiLue,
    /// The Old Persian script
    OldPersian,
    /// The Syloti Nagri script
    SylotiNagri,
    /// The Tifinagh script
    Tifinagh,
    /// The Balinese script
    Balinese,
    /// The Cuneiform script
    Cuneiform,
    /// The Nko script
    Nko,
    /// The Phags Pa script
    PhagsPa,
    /// The Phoenician script
    Phoenician,
    /// The Carian script
    Carian,
    /// The Cham script
    Cham,
    /// The Kayah Li script
    KayahLi,
    /// The Lepcha script
    Lepcha,
    /// The Lycian script
    Lycian,
    /// The Lydian script
    Lydian,
    /// The Ol Chiki script
    OlChiki,
    /// The Rejang script
    Rejang,
    /// The Saurashtra script
    Saurashtra,
    /// The Sundanese script
    Sundanese,
    /// The Vai script
    Vai,
    /// The Avestan script
    Avestan,
    /// The Bamum script
    Bamum,
    /// The Egyptian Hieroglyphs script
    EgyptianHieroglyphs,
    /// The Imperial Aramaic script
    ImperialAramaic,
    /// The Inscriptional Pahlavi script
    InscriptionalPahlavi,
    /// The Inscriptional Parthian script
    InscriptionalParthian,
    /// The Javanese script
    Javanese,
    /// The Kaithi script
    Kaithi,
    /// The Lisu script
    Lisu,
    /// The Meetei Mayek script
    MeeteiMayek,
    /// The Old South Arabian script
    OldSouthArabian,
    /// The Old Turkic script
    OldTurkic,
    /// The Samaritan script
    Samaritan,
    /// The Tai Tham script
    TaiTham,
    /// The Tai Viet script
    TaiViet,
    /// The Batak script
    Batak,
    /// The Brahmi script
    Brahmi,
    /// The Mandaic script
    Mandaic,
    /// The Chakma script
    Chakma,
    /// The Meroitic Cursive script
    MeroiticCursive,
    /// The Meroitic Hieroglyphs script
    MeroiticHieroglyphs,
    /// The Miao script
    Miao,
    /// The Sharada script
    Sharada,
    /// The Sora Sompeng script
    SoraSompeng,
    /// The Takri script
    Takri,
    /// The Bassa Vah script
    BassaVah,
    /// The Caucasian Albanian script
    CaucasianAlbanian,
    /// The Duployan script
    Duployan,
    /// The Elbasan script
    Elbasan,
    /// The Grantha script
    Grantha,
    /// The Khojki script
    Khojki,
    /// The Khudawadi script
    Khudawadi,
    /// The Linear A script
    LinearA,
    /// The Mahajani script
    Mahajani,
    /// The Manichaean script
    Manichaean,
    /// The Mende Kikakui script
    MendeKikakui,
    /// The Modi script
    Modi,
    /// The Mro script
    Mro,
    /// The Nabataean script
    Nabataean,
    /// The Old North Arabian script
    OldNorthArabian,
    /// The Old Permic script
    OldPermic,
    /// The Pahawh Hmong script
    PahawhHmong,
    /// The Palmyrene script
    Palmyrene,
    /// The Pau Cin Hau script
    PauCinHau,
    /// The Psalter Pahlavi script
    PsalterPahlavi,
    /// The Siddham script
    Siddham,
    /// The Tirhuta script
    Tirhuta,
    /// The Warang Citi script
    WarangCiti,
    /// The Ahom script
    Ahom,
    /// The Anatolian Hieroglyphs script
    AnatolianHieroglyphs,
    /// The Hatran script
    Hatran,
    /// The Multani script
    Multani,
    /// The Old Hungarian script
    OldHungarian,
    /// The Signwriting script
    Signwriting,
    /// The Adlam script
    Adlam,
    /// The Bhaiksuki script
    Bhaiksuki,
    /// The Marchen script
    Marchen,
    /// The Osage script
    Osage,
    /// The Tangut script
    Tangut,
    /// The Newa script
    Newa,
    /// The Masaram Gondi script
    MasaramGondi,
    /// The Nushu script
    Nushu,
    /// The Soyombo script
    Soyombo,
    /// The Zanabazar Square script
    ZanabazarSquare,
    /// The Dogra script
    Dogra,
    /// The Gunjala Gondi script
    GunjalaGondi,
    /// The Hanifi Rohingya script
    HanifiRohingya,
    /// The Makasar script
    Makasar,
    /// The Medefaidrin script
    Medefaidrin,
    /// The Old Sogdian script
    OldSogdian,
    /// The Sogdian script
    Sogdian,
    /// The Elymaic script
    Elymaic,
    /// The Nandinagari script
    Nandinagari,
    /// The Nyiakeng Puachue Hmong script
    NyiakengPuachueHmong,
    /// The Wancho script
    Wancho,
    /// Getting the script failed or the script is invalid.
    Invalid,
}

impl Script {
    /// Get the corresponding `Script` from a `hb_script_t`.
    pub fn from_raw(raw: sys::hb_script_t) -> Self {
        match raw {
            sys::HB_SCRIPT_COMMON => Script::Common,
            sys::HB_SCRIPT_INHERITED => Script::Inherited,
            sys::HB_SCRIPT_UNKNOWN => Script::Unknown,
            sys::HB_SCRIPT_ARABIC => Script::Arabic,
            sys::HB_SCRIPT_ARMENIAN => Script::Armenian,
            sys::HB_SCRIPT_BENGALI => Script::Bengali,
            sys::HB_SCRIPT_CYRILLIC => Script::Cyrillic,
            sys::HB_SCRIPT_DEVANAGARI => Script::Devanagari,
            sys::HB_SCRIPT_GEORGIAN => Script::Georgian,
            sys::HB_SCRIPT_GREEK => Script::Greek,
            sys::HB_SCRIPT_GUJARATI => Script::Gujarati,
            sys::HB_SCRIPT_GURMUKHI => Script::Gurmukhi,
            sys::HB_SCRIPT_HANGUL => Script::Hangul,
            sys::HB_SCRIPT_HAN => Script::Han,
            sys::HB_SCRIPT_HEBREW => Script::Hebrew,
            sys::HB_SCRIPT_HIRAGANA => Script::Hiragana,
            sys::HB_SCRIPT_KANNADA => Script::Kannada,
            sys::HB_SCRIPT_KATAKANA => Script::Katakana,
            sys::HB_SCRIPT_LAO => Script::Lao,
            sys::HB_SCRIPT_LATIN => Script::Latin,
            sys::HB_SCRIPT_MALAYALAM => Script::Malayalam,
            sys::HB_SCRIPT_ORIYA => Script::Oriya,
            sys::HB_SCRIPT_TAMIL => Script::Tamil,
            sys::HB_SCRIPT_TELUGU => Script::Telugu,
            sys::HB_SCRIPT_THAI => Script::Thai,
            sys::HB_SCRIPT_TIBETAN => Script::Tibetan,
            sys::HB_SCRIPT_BOPOMOFO => Script::Bopomofo,
            sys::HB_SCRIPT_BRAILLE => Script::Braille,
            sys::HB_SCRIPT_CANADIAN_SYLLABICS => Script::CanadianSyllabics,
            sys::HB_SCRIPT_CHEROKEE => Script::Cherokee,
            sys::HB_SCRIPT_ETHIOPIC => Script::Ethiopic,
            sys::HB_SCRIPT_KHMER => Script::Khmer,
            sys::HB_SCRIPT_MONGOLIAN => Script::Mongolian,
            sys::HB_SCRIPT_MYANMAR => Script::Myanmar,
            sys::HB_SCRIPT_OGHAM => Script::Ogham,
            sys::HB_SCRIPT_RUNIC => Script::Runic,
            sys::HB_SCRIPT_SINHALA => Script::Sinhala,
            sys::HB_SCRIPT_SYRIAC => Script::Syriac,
            sys::HB_SCRIPT_THAANA => Script::Thaana,
            sys::HB_SCRIPT_YI => Script::Yi,
            sys::HB_SCRIPT_DESERET => Script::Deseret,
            sys::HB_SCRIPT_GOTHIC => Script::Gothic,
            sys::HB_SCRIPT_OLD_ITALIC => Script::OldItalic,
            sys::HB_SCRIPT_BUHID => Script::Buhid,
            sys::HB_SCRIPT_HANUNOO => Script::Hanunoo,
            sys::HB_SCRIPT_TAGALOG => Script::Tagalog,
            sys::HB_SCRIPT_TAGBANWA => Script::Tagbanwa,
            sys::HB_SCRIPT_CYPRIOT => Script::Cypriot,
            sys::HB_SCRIPT_LIMBU => Script::Limbu,
            sys::HB_SCRIPT_LINEAR_B => Script::LinearB,
            sys::HB_SCRIPT_OSMANYA => Script::Osmanya,
            sys::HB_SCRIPT_SHAVIAN => Script::Shavian,
            sys::HB_SCRIPT_TAI_LE => Script::TaiLe,
            sys::HB_SCRIPT_UGARITIC => Script::Ugaritic,
            sys::HB_SCRIPT_BUGINESE => Script::Buginese,
            sys::HB_SCRIPT_COPTIC => Script::Coptic,
            sys::HB_SCRIPT_GLAGOLITIC => Script::Glagolitic,
            sys::HB_SCRIPT_KHAROSHTHI => Script::Kharoshthi,
            sys::HB_SCRIPT_NEW_TAI_LUE => Script::NewTaiLue,
            sys::HB_SCRIPT_OLD_PERSIAN => Script::OldPersian,
            sys::HB_SCRIPT_SYLOTI_NAGRI => Script::SylotiNagri,
            sys::HB_SCRIPT_TIFINAGH => Script::Tifinagh,
            sys::HB_SCRIPT_BALINESE => Script::Balinese,
            sys::HB_SCRIPT_CUNEIFORM => Script::Cuneiform,
            sys::HB_SCRIPT_NKO => Script::Nko,
            sys::HB_SCRIPT_PHAGS_PA => Script::PhagsPa,
            sys::HB_SCRIPT_PHOENICIAN => Script::Phoenician,
            sys::HB_SCRIPT_CARIAN => Script::Carian,
            sys::HB_SCRIPT_CHAM => Script::Cham,
            sys::HB_SCRIPT_KAYAH_LI => Script::KayahLi,
            sys::HB_SCRIPT_LEPCHA => Script::Lepcha,
            sys::HB_SCRIPT_LYCIAN => Script::Lycian,
            sys::HB_SCRIPT_LYDIAN => Script::Lydian,
            sys::HB_SCRIPT_OL_CHIKI => Script::OlChiki,
            sys::HB_SCRIPT_REJANG => Script::Rejang,
            sys::HB_SCRIPT_SAURASHTRA => Script::Saurashtra,
            sys::HB_SCRIPT_SUNDANESE => Script::Sundanese,
            sys::HB_SCRIPT_VAI => Script::Vai,
            sys::HB_SCRIPT_AVESTAN => Script::Avestan,
            sys::HB_SCRIPT_BAMUM => Script::Bamum,
            sys::HB_SCRIPT_EGYPTIAN_HIEROGLYPHS => Script::EgyptianHieroglyphs,
            sys::HB_SCRIPT_IMPERIAL_ARAMAIC => Script::ImperialAramaic,
            sys::HB_SCRIPT_INSCRIPTIONAL_PAHLAVI => Script::InscriptionalPahlavi,
            sys::HB_SCRIPT_INSCRIPTIONAL_PARTHIAN => Script::InscriptionalParthian,
            sys::HB_SCRIPT_JAVANESE => Script::Javanese,
            sys::HB_SCRIPT_KAITHI => Script::Kaithi,
            sys::HB_SCRIPT_LISU => Script::Lisu,
            sys::HB_SCRIPT_MEETEI_MAYEK => Script::MeeteiMayek,
            sys::HB_SCRIPT_OLD_SOUTH_ARABIAN => Script::OldSouthArabian,
            sys::HB_SCRIPT_OLD_TURKIC => Script::OldTurkic,
            sys::HB_SCRIPT_SAMARITAN => Script::Samaritan,
            sys::HB_SCRIPT_TAI_THAM => Script::TaiTham,
            sys::HB_SCRIPT_TAI_VIET => Script::TaiViet,
            sys::HB_SCRIPT_BATAK => Script::Batak,
            sys::HB_SCRIPT_BRAHMI => Script::Brahmi,
            sys::HB_SCRIPT_MANDAIC => Script::Mandaic,
            sys::HB_SCRIPT_CHAKMA => Script::Chakma,
            sys::HB_SCRIPT_MEROITIC_CURSIVE => Script::MeroiticCursive,
            sys::HB_SCRIPT_MEROITIC_HIEROGLYPHS => Script::MeroiticHieroglyphs,
            sys::HB_SCRIPT_MIAO => Script::Miao,
            sys::HB_SCRIPT_SHARADA => Script::Sharada,
            sys::HB_SCRIPT_SORA_SOMPENG => Script::SoraSompeng,
            sys::HB_SCRIPT_TAKRI => Script::Takri,
            sys::HB_SCRIPT_BASSA_VAH => Script::BassaVah,
            sys::HB_SCRIPT_CAUCASIAN_ALBANIAN => Script::CaucasianAlbanian,
            sys::HB_SCRIPT_DUPLOYAN => Script::Duployan,
            sys::HB_SCRIPT_ELBASAN => Script::Elbasan,
            sys::HB_SCRIPT_GRANTHA => Script::Grantha,
            sys::HB_SCRIPT_KHOJKI => Script::Khojki,
            sys::HB_SCRIPT_KHUDAWADI => Script::Khudawadi,
            sys::HB_SCRIPT_LINEAR_A => Script::LinearA,
            sys::HB_SCRIPT_MAHAJANI => Script::Mahajani,
            sys::HB_SCRIPT_MANICHAEAN => Script::Manichaean,
            sys::HB_SCRIPT_MENDE_KIKAKUI => Script::MendeKikakui,
            sys::HB_SCRIPT_MODI => Script::Modi,
            sys::HB_SCRIPT_MRO => Script::Mro,
            sys::HB_SCRIPT_NABATAEAN => Script::Nabataean,
            sys::HB_SCRIPT_OLD_NORTH_ARABIAN => Script::OldNorthArabian,
            sys::HB_SCRIPT_OLD_PERMIC => Script::OldPermic,
            sys::HB_SCRIPT_PAHAWH_HMONG => Script::PahawhHmong,
            sys::HB_SCRIPT_PALMYRENE => Script::Palmyrene,
            sys::HB_SCRIPT_PAU_CIN_HAU => Script::PauCinHau,
            sys::HB_SCRIPT_PSALTER_PAHLAVI => Script::PsalterPahlavi,
            sys::HB_SCRIPT_SIDDHAM => Script::Siddham,
            sys::HB_SCRIPT_TIRHUTA => Script::Tirhuta,
            sys::HB_SCRIPT_WARANG_CITI => Script::WarangCiti,
            sys::HB_SCRIPT_AHOM => Script::Ahom,
            sys::HB_SCRIPT_ANATOLIAN_HIEROGLYPHS => Script::AnatolianHieroglyphs,
            sys::HB_SCRIPT_HATRAN => Script::Hatran,
            sys::HB_SCRIPT_MULTANI => Script::Multani,
            sys::HB_SCRIPT_OLD_HUNGARIAN => Script::OldHungarian,
            sys::HB_SCRIPT_SIGNWRITING => Script::Signwriting,
            sys::HB_SCRIPT_ADLAM => Script::Adlam,
            sys::HB_SCRIPT_BHAIKSUKI => Script::Bhaiksuki,
            sys::HB_SCRIPT_MARCHEN => Script::Marchen,
            sys::HB_SCRIPT_OSAGE => Script::Osage,
            sys::HB_SCRIPT_TANGUT => Script::Tangut,
            sys::HB_SCRIPT_NEWA => Script::Newa,
            sys::HB_SCRIPT_MASARAM_GONDI => Script::MasaramGondi,
            sys::HB_SCRIPT_NUSHU => Script::Nushu,
            sys::HB_SCRIPT_SOYOMBO => Script::Soyombo,
            sys::HB_SCRIPT_ZANABAZAR_SQUARE => Script::ZanabazarSquare,
            sys::HB_SCRIPT_DOGRA => Script::Dogra,
            sys::HB_SCRIPT_GUNJALA_GONDI => Script::GunjalaGondi,
            sys::HB_SCRIPT_HANIFI_ROHINGYA => Script::HanifiRohingya,
            sys::HB_SCRIPT_MAKASAR => Script::Makasar,
            sys::HB_SCRIPT_MEDEFAIDRIN => Script::Medefaidrin,
            sys::HB_SCRIPT_OLD_SOGDIAN => Script::OldSogdian,
            sys::HB_SCRIPT_SOGDIAN => Script::Sogdian,
            sys::HB_SCRIPT_ELYMAIC => Script::Elymaic,
            sys::HB_SCRIPT_NANDINAGARI => Script::Nandinagari,
            sys::HB_SCRIPT_NYIAKENG_PUACHUE_HMONG => Script::NyiakengPuachueHmong,
            sys::HB_SCRIPT_WANCHO => Script::Wancho,
            0 => Script::Invalid,
            _ => panic!("unrecognised script"),
        }
    }

    /// Get the corresponding `hb_script_t` from a `Script`.
    pub fn as_raw(&self) -> sys::hb_script_t {
        match self {
            Script::Common => sys::HB_SCRIPT_COMMON,
            Script::Inherited => sys::HB_SCRIPT_INHERITED,
            Script::Unknown => sys::HB_SCRIPT_UNKNOWN,
            Script::Arabic => sys::HB_SCRIPT_ARABIC,
            Script::Armenian => sys::HB_SCRIPT_ARMENIAN,
            Script::Bengali => sys::HB_SCRIPT_BENGALI,
            Script::Cyrillic => sys::HB_SCRIPT_CYRILLIC,
            Script::Devanagari => sys::HB_SCRIPT_DEVANAGARI,
            Script::Georgian => sys::HB_SCRIPT_GEORGIAN,
            Script::Greek => sys::HB_SCRIPT_GREEK,
            Script::Gujarati => sys::HB_SCRIPT_GUJARATI,
            Script::Gurmukhi => sys::HB_SCRIPT_GURMUKHI,
            Script::Hangul => sys::HB_SCRIPT_HANGUL,
            Script::Han => sys::HB_SCRIPT_HAN,
            Script::Hebrew => sys::HB_SCRIPT_HEBREW,
            Script::Hiragana => sys::HB_SCRIPT_HIRAGANA,
            Script::Kannada => sys::HB_SCRIPT_KANNADA,
            Script::Katakana => sys::HB_SCRIPT_KATAKANA,
            Script::Lao => sys::HB_SCRIPT_LAO,
            Script::Latin => sys::HB_SCRIPT_LATIN,
            Script::Malayalam => sys::HB_SCRIPT_MALAYALAM,
            Script::Oriya => sys::HB_SCRIPT_ORIYA,
            Script::Tamil => sys::HB_SCRIPT_TAMIL,
            Script::Telugu => sys::HB_SCRIPT_TELUGU,
            Script::Thai => sys::HB_SCRIPT_THAI,
            Script::Tibetan => sys::HB_SCRIPT_TIBETAN,
            Script::Bopomofo => sys::HB_SCRIPT_BOPOMOFO,
            Script::Braille => sys::HB_SCRIPT_BRAILLE,
            Script::CanadianSyllabics => sys::HB_SCRIPT_CANADIAN_SYLLABICS,
            Script::Cherokee => sys::HB_SCRIPT_CHEROKEE,
            Script::Ethiopic => sys::HB_SCRIPT_ETHIOPIC,
            Script::Khmer => sys::HB_SCRIPT_KHMER,
            Script::Mongolian => sys::HB_SCRIPT_MONGOLIAN,
            Script::Myanmar => sys::HB_SCRIPT_MYANMAR,
            Script::Ogham => sys::HB_SCRIPT_OGHAM,
            Script::Runic => sys::HB_SCRIPT_RUNIC,
            Script::Sinhala => sys::HB_SCRIPT_SINHALA,
            Script::Syriac => sys::HB_SCRIPT_SYRIAC,
            Script::Thaana => sys::HB_SCRIPT_THAANA,
            Script::Yi => sys::HB_SCRIPT_YI,
            Script::Deseret => sys::HB_SCRIPT_DESERET,
            Script::Gothic => sys::HB_SCRIPT_GOTHIC,
            Script::OldItalic => sys::HB_SCRIPT_OLD_ITALIC,
            Script::Buhid => sys::HB_SCRIPT_BUHID,
            Script::Hanunoo => sys::HB_SCRIPT_HANUNOO,
            Script::Tagalog => sys::HB_SCRIPT_TAGALOG,
            Script::Tagbanwa => sys::HB_SCRIPT_TAGBANWA,
            Script::Cypriot => sys::HB_SCRIPT_CYPRIOT,
            Script::Limbu => sys::HB_SCRIPT_LIMBU,
            Script::LinearB => sys::HB_SCRIPT_LINEAR_B,
            Script::Osmanya => sys::HB_SCRIPT_OSMANYA,
            Script::Shavian => sys::HB_SCRIPT_SHAVIAN,
            Script::TaiLe => sys::HB_SCRIPT_TAI_LE,
            Script::Ugaritic => sys::HB_SCRIPT_UGARITIC,
            Script::Buginese => sys::HB_SCRIPT_BUGINESE,
            Script::Coptic => sys::HB_SCRIPT_COPTIC,
            Script::Glagolitic => sys::HB_SCRIPT_GLAGOLITIC,
            Script::Kharoshthi => sys::HB_SCRIPT_KHAROSHTHI,
            Script::NewTaiLue => sys::HB_SCRIPT_NEW_TAI_LUE,
            Script::OldPersian => sys::HB_SCRIPT_OLD_PERSIAN,
            Script::SylotiNagri => sys::HB_SCRIPT_SYLOTI_NAGRI,
            Script::Tifinagh => sys::HB_SCRIPT_TIFINAGH,
            Script::Balinese => sys::HB_SCRIPT_BALINESE,
            Script::Cuneiform => sys::HB_SCRIPT_CUNEIFORM,
            Script::Nko => sys::HB_SCRIPT_NKO,
            Script::PhagsPa => sys::HB_SCRIPT_PHAGS_PA,
            Script::Phoenician => sys::HB_SCRIPT_PHOENICIAN,
            Script::Carian => sys::HB_SCRIPT_CARIAN,
            Script::Cham => sys::HB_SCRIPT_CHAM,
            Script::KayahLi => sys::HB_SCRIPT_KAYAH_LI,
            Script::Lepcha => sys::HB_SCRIPT_LEPCHA,
            Script::Lycian => sys::HB_SCRIPT_LYCIAN,
            Script::Lydian => sys::HB_SCRIPT_LYDIAN,
            Script::OlChiki => sys::HB_SCRIPT_OL_CHIKI,
            Script::Rejang => sys::HB_SCRIPT_REJANG,
            Script::Saurashtra => sys::HB_SCRIPT_SAURASHTRA,
            Script::Sundanese => sys::HB_SCRIPT_SUNDANESE,
            Script::Vai => sys::HB_SCRIPT_VAI,
            Script::Avestan => sys::HB_SCRIPT_AVESTAN,
            Script::Bamum => sys::HB_SCRIPT_BAMUM,
            Script::EgyptianHieroglyphs => sys::HB_SCRIPT_EGYPTIAN_HIEROGLYPHS,
            Script::ImperialAramaic => sys::HB_SCRIPT_IMPERIAL_ARAMAIC,
            Script::InscriptionalPahlavi => sys::HB_SCRIPT_INSCRIPTIONAL_PAHLAVI,
            Script::InscriptionalParthian => sys::HB_SCRIPT_INSCRIPTIONAL_PARTHIAN,
            Script::Javanese => sys::HB_SCRIPT_JAVANESE,
            Script::Kaithi => sys::HB_SCRIPT_KAITHI,
            Script::Lisu => sys::HB_SCRIPT_LISU,
            Script::MeeteiMayek => sys::HB_SCRIPT_MEETEI_MAYEK,
            Script::OldSouthArabian => sys::HB_SCRIPT_OLD_SOUTH_ARABIAN,
            Script::OldTurkic => sys::HB_SCRIPT_OLD_TURKIC,
            Script::Samaritan => sys::HB_SCRIPT_SAMARITAN,
            Script::TaiTham => sys::HB_SCRIPT_TAI_THAM,
            Script::TaiViet => sys::HB_SCRIPT_TAI_VIET,
            Script::Batak => sys::HB_SCRIPT_BATAK,
            Script::Brahmi => sys::HB_SCRIPT_BRAHMI,
            Script::Mandaic => sys::HB_SCRIPT_MANDAIC,
            Script::Chakma => sys::HB_SCRIPT_CHAKMA,
            Script::MeroiticCursive => sys::HB_SCRIPT_MEROITIC_CURSIVE,
            Script::MeroiticHieroglyphs => sys::HB_SCRIPT_MEROITIC_HIEROGLYPHS,
            Script::Miao => sys::HB_SCRIPT_MIAO,
            Script::Sharada => sys::HB_SCRIPT_SHARADA,
            Script::SoraSompeng => sys::HB_SCRIPT_SORA_SOMPENG,
            Script::Takri => sys::HB_SCRIPT_TAKRI,
            Script::BassaVah => sys::HB_SCRIPT_BASSA_VAH,
            Script::CaucasianAlbanian => sys::HB_SCRIPT_CAUCASIAN_ALBANIAN,
            Script::Duployan => sys::HB_SCRIPT_DUPLOYAN,
            Script::Elbasan => sys::HB_SCRIPT_ELBASAN,
            Script::Grantha => sys::HB_SCRIPT_GRANTHA,
            Script::Khojki => sys::HB_SCRIPT_KHOJKI,
            Script::Khudawadi => sys::HB_SCRIPT_KHUDAWADI,
            Script::LinearA => sys::HB_SCRIPT_LINEAR_A,
            Script::Mahajani => sys::HB_SCRIPT_MAHAJANI,
            Script::Manichaean => sys::HB_SCRIPT_MANICHAEAN,
            Script::MendeKikakui => sys::HB_SCRIPT_MENDE_KIKAKUI,
            Script::Modi => sys::HB_SCRIPT_MODI,
            Script::Mro => sys::HB_SCRIPT_MRO,
            Script::Nabataean => sys::HB_SCRIPT_NABATAEAN,
            Script::OldNorthArabian => sys::HB_SCRIPT_OLD_NORTH_ARABIAN,
            Script::OldPermic => sys::HB_SCRIPT_OLD_PERMIC,
            Script::PahawhHmong => sys::HB_SCRIPT_PAHAWH_HMONG,
            Script::Palmyrene => sys::HB_SCRIPT_PALMYRENE,
            Script::PauCinHau => sys::HB_SCRIPT_PAU_CIN_HAU,
            Script::PsalterPahlavi => sys::HB_SCRIPT_PSALTER_PAHLAVI,
            Script::Siddham => sys::HB_SCRIPT_SIDDHAM,
            Script::Tirhuta => sys::HB_SCRIPT_TIRHUTA,
            Script::WarangCiti => sys::HB_SCRIPT_WARANG_CITI,
            Script::Ahom => sys::HB_SCRIPT_AHOM,
            Script::AnatolianHieroglyphs => sys::HB_SCRIPT_ANATOLIAN_HIEROGLYPHS,
            Script::Hatran => sys::HB_SCRIPT_HATRAN,
            Script::Multani => sys::HB_SCRIPT_MULTANI,
            Script::OldHungarian => sys::HB_SCRIPT_OLD_HUNGARIAN,
            Script::Signwriting => sys::HB_SCRIPT_SIGNWRITING,
            Script::Adlam => sys::HB_SCRIPT_ADLAM,
            Script::Bhaiksuki => sys::HB_SCRIPT_BHAIKSUKI,
            Script::Marchen => sys::HB_SCRIPT_MARCHEN,
            Script::Osage => sys::HB_SCRIPT_OSAGE,
            Script::Tangut => sys::HB_SCRIPT_TANGUT,
            Script::Newa => sys::HB_SCRIPT_NEWA,
            Script::MasaramGondi => sys::HB_SCRIPT_MASARAM_GONDI,
            Script::Nushu => sys::HB_SCRIPT_NUSHU,
            Script::Soyombo => sys::HB_SCRIPT_SOYOMBO,
            Script::ZanabazarSquare => sys::HB_SCRIPT_ZANABAZAR_SQUARE,
            Script::Dogra => sys::HB_SCRIPT_DOGRA,
            Script::GunjalaGondi => sys::HB_SCRIPT_GUNJALA_GONDI,
            Script::HanifiRohingya => sys::HB_SCRIPT_HANIFI_ROHINGYA,
            Script::Makasar => sys::HB_SCRIPT_MAKASAR,
            Script::Medefaidrin => sys::HB_SCRIPT_MEDEFAIDRIN,
            Script::OldSogdian => sys::HB_SCRIPT_OLD_SOGDIAN,
            Script::Sogdian => sys::HB_SCRIPT_SOGDIAN,
            Script::Elymaic => sys::HB_SCRIPT_ELYMAIC,
            Script::Nandinagari => sys::HB_SCRIPT_NANDINAGARI,
            Script::NyiakengPuachueHmong => sys::HB_SCRIPT_NYIAKENG_PUACHUE_HMONG,
            Script::Wancho => sys::HB_SCRIPT_WANCHO,
            Script::Invalid => 0,
        }
    }
}
