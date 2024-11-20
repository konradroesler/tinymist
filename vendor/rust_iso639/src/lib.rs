use phf::{phf_map, Map};

#[cfg(test)]
mod tests {

    #[test]
    fn test_from_code_1() {
        let l = crate::from_code_1("zh");
        print!("test_from_code result {:?}", l)
    }

    #[test]
    fn test_from_code_2t() {
        let l = crate::from_code_2t("zho");
        print!("test_from_code_2t result {:?}", l)
    }

    #[test]
    fn test_from_code_2b() {
        let l = crate::from_code_2b("chi");
        print!("test_from_code_2b result {:?}", l)
    }

    #[test]
    fn test_from_code_3() {
        let l = crate::from_code_3("zho");
        print!("test_from_code_3 result {:?}", l)
    }

    #[test]
    fn test_all() {
        println!("{:?}", crate::ALL);
        println!("{:?}", crate::ALL_1);
        println!("{:?}", crate::ALL_2B);
        println!("{:?}", crate::ALL_2T);
        println!("{:?}", crate::ALL_3);


        println!("{:?}", crate::ALL_MAP);
        println!("{:?}", crate::ALL_1_MAP);
        println!("{:?}", crate::ALL_2B_MAP);
        println!("{:?}", crate::ALL_2T_MAP);
        println!("{:?}", crate::ALL_3_MAP);

    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]

pub struct LanguageCode<'a> {
    ///ISO Language Name
    pub name: &'static str,
    ///639-1
    pub code: &'static str,
    ///639-2/T
    pub code_2t: &'static str,
    ///639-2/B
    pub code_2b: &'static str,
    //639-3 Macrolanguage
    pub code_3: &'static str,

    pub individual_languages: &'a [IndividualLanguages],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IndividualLanguages {
    ///Name
    pub name: &'static str,
    ///Code
    pub code: &'static str,
}


pub fn from_code_1(code: &str) -> Option<LanguageCode> {
    let up = code.to_lowercase();
    ALL_1_MAP.get(up.as_str()).cloned()
}

pub fn from_code_2t(code: &str) -> Option<LanguageCode> {
    let up = code.to_lowercase();
    ALL_2T_MAP.get(up.as_str()).cloned()
}
pub fn from_code_2b(code: &str) -> Option<LanguageCode> {
    let up = code.to_lowercase();
    ALL_2B_MAP.get(up.as_str()).cloned()
}
pub fn from_code_3(code: &str) -> Option<LanguageCode> {
    let up = code.to_lowercase();
    ALL_3_MAP.get(up.as_str()).cloned()
}


pub const AB: LanguageCode = LanguageCode {
    name: "Abkhazian",
    code: "ab",
    code_2t: "abk",
    code_2b: "abk",
    code_3: "abk",
    individual_languages: &[
    ],
};


pub const AA: LanguageCode = LanguageCode {
    name: "Afar",
    code: "aa",
    code_2t: "aar",
    code_2b: "aar",
    code_3: "aar",
    individual_languages: &[
    ],
};


pub const AF: LanguageCode = LanguageCode {
    name: "Afrikaans",
    code: "af",
    code_2t: "afr",
    code_2b: "afr",
    code_3: "afr",
    individual_languages: &[
    ],
};


pub const AK: LanguageCode = LanguageCode {
    name: "Akan",
    code: "ak",
    code_2t: "aka",
    code_2b: "aka",
    code_3: "aka",
    individual_languages: &[
        IndividualLanguages {
            name: "Fanti",
            code: "fat",
        },
        IndividualLanguages {
            name: "Twi",
            code: "twi",
        },
    ],
};


pub const SQ: LanguageCode = LanguageCode {
    name: "Albanian",
    code: "sq",
    code_2t: "sqi",
    code_2b: "alb",
    code_3: "sqi",
    individual_languages: &[
        IndividualLanguages {
            name: "Arbëreshë Albanian",
            code: "aae",
        },
        IndividualLanguages {
            name: "Arvanitika Albanian",
            code: "aat",
        },
        IndividualLanguages {
            name: "Gheg Albanian",
            code: "aln",
        },
        IndividualLanguages {
            name: "Tosk Albanian",
            code: "als",
        },
    ],
};


pub const AM: LanguageCode = LanguageCode {
    name: "Amharic",
    code: "am",
    code_2t: "amh",
    code_2b: "amh",
    code_3: "amh",
    individual_languages: &[
    ],
};


pub const AR: LanguageCode = LanguageCode {
    name: "Arabic",
    code: "ar",
    code_2t: "ara",
    code_2b: "ara",
    code_3: "ara",
    individual_languages: &[
        IndividualLanguages {
            name: "Algerian Saharan Arabic",
            code: "aao",
        },
        IndividualLanguages {
            name: "Tajiki Arabic",
            code: "abh",
        },
        IndividualLanguages {
            name: "Baharna Arabic",
            code: "abv",
        },
        IndividualLanguages {
            name: "Mesopotamian Arabic",
            code: "acm",
        },
        IndividualLanguages {
            name: "Ta'izzi-Adeni Arabic",
            code: "acq",
        },
        IndividualLanguages {
            name: "Hijazi Arabic",
            code: "acw",
        },
        IndividualLanguages {
            name: "Omani Arabic",
            code: "acx",
        },
        IndividualLanguages {
            name: "Cypriot Arabic",
            code: "acy",
        },
        IndividualLanguages {
            name: "Dhofari Arabic",
            code: "adf",
        },
        IndividualLanguages {
            name: "Tunisian Arabic",
            code: "aeb",
        },
        IndividualLanguages {
            name: "Saidi Arabic",
            code: "aec",
        },
        IndividualLanguages {
            name: "Gulf Arabic",
            code: "afb",
        },
        IndividualLanguages {
            name: "South Levantine Arabic",
            code: "ajp",
        },
        IndividualLanguages {
            name: "North Levantine Arabic",
            code: "apc",
        },
        IndividualLanguages {
            name: "Sudanese Arabic",
            code: "apd",
        },
        IndividualLanguages {
            name: "Standard Arabic",
            code: "arb",
        },
        IndividualLanguages {
            name: "Algerian Arabic",
            code: "arq",
        },
        IndividualLanguages {
            name: "Najdi Arabic",
            code: "ars",
        },
        IndividualLanguages {
            name: "Moroccan Arabic",
            code: "ary",
        },
        IndividualLanguages {
            name: "Egyptian Arabic",
            code: "arz",
        },
        IndividualLanguages {
            name: "Uzbeki Arabic",
            code: "auz",
        },
        IndividualLanguages {
            name: "Eastern Egyptian Bedawi Arabic",
            code: "avl",
        },
        IndividualLanguages {
            name: "Hadrami Arabic",
            code: "ayh",
        },
        IndividualLanguages {
            name: "Libyan Arabic",
            code: "ayl",
        },
        IndividualLanguages {
            name: "Sanaani Arabic",
            code: "ayn",
        },
        IndividualLanguages {
            name: "North Mesopotamian Arabic",
            code: "ayp",
        },
        IndividualLanguages {
            name: "",
            code: "bbz",
        },
        IndividualLanguages {
            name: "Sudanese Creole Arabic",
            code: "pga",
        },
        IndividualLanguages {
            name: "Chadian Arabic",
            code: "shu",
        },
        IndividualLanguages {
            name: "Shihhi Arabic",
            code: "ssh",
        },
    ],
};


pub const AN: LanguageCode = LanguageCode {
    name: "Aragonese",
    code: "an",
    code_2t: "arg",
    code_2b: "arg",
    code_3: "arg",
    individual_languages: &[
    ],
};


pub const HY: LanguageCode = LanguageCode {
    name: "Armenian",
    code: "hy",
    code_2t: "hye",
    code_2b: "arm",
    code_3: "hye",
    individual_languages: &[
    ],
};


pub const AS: LanguageCode = LanguageCode {
    name: "Assamese",
    code: "as",
    code_2t: "asm",
    code_2b: "asm",
    code_3: "asm",
    individual_languages: &[
    ],
};


pub const AV: LanguageCode = LanguageCode {
    name: "Avaric",
    code: "av",
    code_2t: "ava",
    code_2b: "ava",
    code_3: "ava",
    individual_languages: &[
    ],
};


pub const AE: LanguageCode = LanguageCode {
    name: "Avestan",
    code: "ae",
    code_2t: "ave",
    code_2b: "ave",
    code_3: "ave",
    individual_languages: &[
    ],
};


pub const AY: LanguageCode = LanguageCode {
    name: "Aymara",
    code: "ay",
    code_2t: "aym",
    code_2b: "aym",
    code_3: "aym",
    individual_languages: &[
        IndividualLanguages {
            name: "Southern Aymara",
            code: "ayc",
        },
        IndividualLanguages {
            name: "Central Aymara",
            code: "ayr",
        },
    ],
};


pub const AZ: LanguageCode = LanguageCode {
    name: "Azerbaijani",
    code: "az",
    code_2t: "aze",
    code_2b: "aze",
    code_3: "aze",
    individual_languages: &[
        IndividualLanguages {
            name: "South Azerbaijani",
            code: "azb",
        },
        IndividualLanguages {
            name: "North Azerbaijani",
            code: "azj",
        },
    ],
};


pub const BM: LanguageCode = LanguageCode {
    name: "Bambara",
    code: "bm",
    code_2t: "bam",
    code_2b: "bam",
    code_3: "bam",
    individual_languages: &[
    ],
};


pub const BA: LanguageCode = LanguageCode {
    name: "Bashkir",
    code: "ba",
    code_2t: "bak",
    code_2b: "bak",
    code_3: "bak",
    individual_languages: &[
    ],
};


pub const EU: LanguageCode = LanguageCode {
    name: "Basque",
    code: "eu",
    code_2t: "eus",
    code_2b: "baq",
    code_3: "eus",
    individual_languages: &[
    ],
};


pub const BE: LanguageCode = LanguageCode {
    name: "Belarusian",
    code: "be",
    code_2t: "bel",
    code_2b: "bel",
    code_3: "bel",
    individual_languages: &[
    ],
};


pub const BN: LanguageCode = LanguageCode {
    name: "Bengali",
    code: "bn",
    code_2t: "ben",
    code_2b: "ben",
    code_3: "ben",
    individual_languages: &[
    ],
};


pub const BI: LanguageCode = LanguageCode {
    name: "Bislama",
    code: "bi",
    code_2t: "bis",
    code_2b: "bis",
    code_3: "bis",
    individual_languages: &[
    ],
};


pub const BS: LanguageCode = LanguageCode {
    name: "Bosnian",
    code: "bs",
    code_2t: "bos",
    code_2b: "bos",
    code_3: "bos",
    individual_languages: &[
    ],
};


pub const BR: LanguageCode = LanguageCode {
    name: "Breton",
    code: "br",
    code_2t: "bre",
    code_2b: "bre",
    code_3: "bre",
    individual_languages: &[
    ],
};


pub const BG: LanguageCode = LanguageCode {
    name: "Bulgarian",
    code: "bg",
    code_2t: "bul",
    code_2b: "bul",
    code_3: "bul",
    individual_languages: &[
    ],
};


pub const MY: LanguageCode = LanguageCode {
    name: "Burmese",
    code: "my",
    code_2t: "mya",
    code_2b: "bur",
    code_3: "mya",
    individual_languages: &[
    ],
};


pub const CA: LanguageCode = LanguageCode {
    name: "Catalan, Valencian",
    code: "ca",
    code_2t: "cat",
    code_2b: "cat",
    code_3: "cat",
    individual_languages: &[
    ],
};


pub const CH: LanguageCode = LanguageCode {
    name: "Chamorro",
    code: "ch",
    code_2t: "cha",
    code_2b: "cha",
    code_3: "cha",
    individual_languages: &[
    ],
};


pub const CE: LanguageCode = LanguageCode {
    name: "Chechen",
    code: "ce",
    code_2t: "che",
    code_2b: "che",
    code_3: "che",
    individual_languages: &[
    ],
};


pub const NY: LanguageCode = LanguageCode {
    name: "Chichewa, Chewa, Nyanja",
    code: "ny",
    code_2t: "nya",
    code_2b: "nya",
    code_3: "nya",
    individual_languages: &[
    ],
};


pub const ZH: LanguageCode = LanguageCode {
    name: "Chinese",
    code: "zh",
    code_2t: "zho",
    code_2b: "chi",
    code_3: "zho",
    individual_languages: &[
        IndividualLanguages {
            name: "Min Dong Chinese",
            code: "cdo",
        },
        IndividualLanguages {
            name: "Jinyu Chinese",
            code: "cjy",
        },
        IndividualLanguages {
            name: "Mandarin Chinese",
            code: "cmn",
        },
        IndividualLanguages {
            name: "Northern Pinghua",
            code: "cnp",
        },
        IndividualLanguages {
            name: "Pu-Xian Chinese",
            code: "cpx",
        },
        IndividualLanguages {
            name: "Southern Pinghua",
            code: "csp",
        },
        IndividualLanguages {
            name: "Huizhou Chinese",
            code: "czh",
        },
        IndividualLanguages {
            name: "Min Zhong Chinese",
            code: "czo",
        },
        IndividualLanguages {
            name: "Gan Chinese",
            code: "gan",
        },
        IndividualLanguages {
            name: "Hakka Chinese",
            code: "hak",
        },
        IndividualLanguages {
            name: "Xiang Chinese",
            code: "hsn",
        },
        IndividualLanguages {
            name: "Literary Chinese",
            code: "lzh",
        },
        IndividualLanguages {
            name: "Min Bei Chinese",
            code: "mnp",
        },
        IndividualLanguages {
            name: "Min Nan Chinese",
            code: "nan",
        },
        IndividualLanguages {
            name: "Wu Chinese",
            code: "wuu",
        },
        IndividualLanguages {
            name: "Yue Chinese",
            code: "yue",
        },
    ],
};


pub const CU: LanguageCode = LanguageCode {
    name: "Old&nbsp;Church&nbsp;Slavonic",
    code: "cu",
    code_2t: "chu",
    code_2b: "chu",
    code_3: "chu",
    individual_languages: &[
    ],
};


pub const CV: LanguageCode = LanguageCode {
    name: "Chuvash",
    code: "cv",
    code_2t: "chv",
    code_2b: "chv",
    code_3: "chv",
    individual_languages: &[
    ],
};


pub const KW: LanguageCode = LanguageCode {
    name: "Cornish",
    code: "kw",
    code_2t: "cor",
    code_2b: "cor",
    code_3: "cor",
    individual_languages: &[
    ],
};


pub const CO: LanguageCode = LanguageCode {
    name: "Corsican",
    code: "co",
    code_2t: "cos",
    code_2b: "cos",
    code_3: "cos",
    individual_languages: &[
    ],
};


pub const CR: LanguageCode = LanguageCode {
    name: "Cree",
    code: "cr",
    code_2t: "cre",
    code_2b: "cre",
    code_3: "cre",
    individual_languages: &[
        IndividualLanguages {
            name: "Southern East Cree",
            code: "crj",
        },
        IndividualLanguages {
            name: "Plains Cree",
            code: "crk",
        },
        IndividualLanguages {
            name: "Northern East Cree",
            code: "crl",
        },
        IndividualLanguages {
            name: "Moose Cree",
            code: "crm",
        },
        IndividualLanguages {
            name: "Swampy Cree",
            code: "csw",
        },
        IndividualLanguages {
            name: "Woods Cree",
            code: "cwd",
        },
    ],
};


pub const HR: LanguageCode = LanguageCode {
    name: "Croatian",
    code: "hr",
    code_2t: "hrv",
    code_2b: "hrv",
    code_3: "hrv",
    individual_languages: &[
    ],
};


pub const CS: LanguageCode = LanguageCode {
    name: "Czech",
    code: "cs",
    code_2t: "ces",
    code_2b: "cze",
    code_3: "ces",
    individual_languages: &[
    ],
};


pub const DA: LanguageCode = LanguageCode {
    name: "Danish",
    code: "da",
    code_2t: "dan",
    code_2b: "dan",
    code_3: "dan",
    individual_languages: &[
    ],
};


pub const DV: LanguageCode = LanguageCode {
    name: "Divehi, Dhivehi, Maldivian",
    code: "dv",
    code_2t: "div",
    code_2b: "div",
    code_3: "div",
    individual_languages: &[
    ],
};


pub const NL: LanguageCode = LanguageCode {
    name: "Dutch, Flemish",
    code: "nl",
    code_2t: "nld",
    code_2b: "dut",
    code_3: "nld",
    individual_languages: &[
    ],
};


pub const DZ: LanguageCode = LanguageCode {
    name: "Dzongkha",
    code: "dz",
    code_2t: "dzo",
    code_2b: "dzo",
    code_3: "dzo",
    individual_languages: &[
    ],
};


pub const EN: LanguageCode = LanguageCode {
    name: "English",
    code: "en",
    code_2t: "eng",
    code_2b: "eng",
    code_3: "eng",
    individual_languages: &[
    ],
};


pub const EO: LanguageCode = LanguageCode {
    name: "Esperanto",
    code: "eo",
    code_2t: "epo",
    code_2b: "epo",
    code_3: "epo",
    individual_languages: &[
    ],
};


pub const ET: LanguageCode = LanguageCode {
    name: "Estonian",
    code: "et",
    code_2t: "est",
    code_2b: "est",
    code_3: "est",
    individual_languages: &[
        IndividualLanguages {
            name: "Standard Estonian",
            code: "ekk",
        },
        IndividualLanguages {
            name: "Võro",
            code: "vro",
        },
    ],
};


pub const EE: LanguageCode = LanguageCode {
    name: "Ewe",
    code: "ee",
    code_2t: "ewe",
    code_2b: "ewe",
    code_3: "ewe",
    individual_languages: &[
    ],
};


pub const FO: LanguageCode = LanguageCode {
    name: "Faroese",
    code: "fo",
    code_2t: "fao",
    code_2b: "fao",
    code_3: "fao",
    individual_languages: &[
    ],
};


pub const FJ: LanguageCode = LanguageCode {
    name: "Fijian",
    code: "fj",
    code_2t: "fij",
    code_2b: "fij",
    code_3: "fij",
    individual_languages: &[
    ],
};


pub const FI: LanguageCode = LanguageCode {
    name: "Finnish",
    code: "fi",
    code_2t: "fin",
    code_2b: "fin",
    code_3: "fin",
    individual_languages: &[
    ],
};


pub const FR: LanguageCode = LanguageCode {
    name: "French",
    code: "fr",
    code_2t: "fra",
    code_2b: "fre",
    code_3: "fra",
    individual_languages: &[
    ],
};


pub const FY: LanguageCode = LanguageCode {
    name: "Western Frisian",
    code: "fy",
    code_2t: "fry",
    code_2b: "fry",
    code_3: "fry",
    individual_languages: &[
    ],
};


pub const FF: LanguageCode = LanguageCode {
    name: "Fulah",
    code: "ff",
    code_2t: "ful",
    code_2b: "ful",
    code_3: "ful",
    individual_languages: &[
        IndividualLanguages {
            name: "Maasina Fulfulde",
            code: "ffm",
        },
        IndividualLanguages {
            name: "Adamawa Fulfulde",
            code: "fub",
        },
        IndividualLanguages {
            name: "Pulaar",
            code: "fuc",
        },
        IndividualLanguages {
            name: "Borgu Fulfulde",
            code: "fue",
        },
        IndividualLanguages {
            name: "Pular",
            code: "fuf",
        },
        IndividualLanguages {
            name: "Western Niger Fulfulde",
            code: "fuh",
        },
        IndividualLanguages {
            name: "Bagirmi Fulfulde",
            code: "fui",
        },
        IndividualLanguages {
            name: "Central-Eastern Niger Fulfulde",
            code: "fuq",
        },
        IndividualLanguages {
            name: "Nigerian Fulfulde",
            code: "fuv",
        },
    ],
};


pub const GD: LanguageCode = LanguageCode {
    name: "Gaelic, Scottish Gaelic",
    code: "gd",
    code_2t: "gla",
    code_2b: "gla",
    code_3: "gla",
    individual_languages: &[
    ],
};


pub const GL: LanguageCode = LanguageCode {
    name: "Galician",
    code: "gl",
    code_2t: "glg",
    code_2b: "glg",
    code_3: "glg",
    individual_languages: &[
    ],
};


pub const LG: LanguageCode = LanguageCode {
    name: "Ganda",
    code: "lg",
    code_2t: "lug",
    code_2b: "lug",
    code_3: "lug",
    individual_languages: &[
    ],
};


pub const KA: LanguageCode = LanguageCode {
    name: "Georgian",
    code: "ka",
    code_2t: "kat",
    code_2b: "geo",
    code_3: "kat",
    individual_languages: &[
    ],
};


pub const DE: LanguageCode = LanguageCode {
    name: "German",
    code: "de",
    code_2t: "deu",
    code_2b: "ger",
    code_3: "deu",
    individual_languages: &[
    ],
};


pub const EL: LanguageCode = LanguageCode {
    name: "Greek, Modern (1453–)",
    code: "el",
    code_2t: "ell",
    code_2b: "gre",
    code_3: "ell",
    individual_languages: &[
    ],
};


pub const KL: LanguageCode = LanguageCode {
    name: "Kalaallisut, Greenlandic",
    code: "kl",
    code_2t: "kal",
    code_2b: "kal",
    code_3: "kal",
    individual_languages: &[
    ],
};


pub const GN: LanguageCode = LanguageCode {
    name: "Guarani",
    code: "gn",
    code_2t: "grn",
    code_2b: "grn",
    code_3: "grn",
    individual_languages: &[
        IndividualLanguages {
            name: "Western Bolivian Guaraní",
            code: "gnw",
        },
        IndividualLanguages {
            name: "Paraguayan Guaraní",
            code: "gug",
        },
        IndividualLanguages {
            name: "Eastern Bolivian Guaraní",
            code: "gui",
        },
        IndividualLanguages {
            name: "Mbyá Guaraní",
            code: "gun",
        },
        IndividualLanguages {
            name: "Chiripá",
            code: "nhd",
        },
    ],
};


pub const GU: LanguageCode = LanguageCode {
    name: "Gujarati",
    code: "gu",
    code_2t: "guj",
    code_2b: "guj",
    code_3: "guj",
    individual_languages: &[
    ],
};


pub const HT: LanguageCode = LanguageCode {
    name: "Haitian, Haitian Creole",
    code: "ht",
    code_2t: "hat",
    code_2b: "hat",
    code_3: "hat",
    individual_languages: &[
    ],
};


pub const HA: LanguageCode = LanguageCode {
    name: "Hausa",
    code: "ha",
    code_2t: "hau",
    code_2b: "hau",
    code_3: "hau",
    individual_languages: &[
    ],
};


pub const HE: LanguageCode = LanguageCode {
    name: "Hebrew",
    code: "he",
    code_2t: "heb",
    code_2b: "heb",
    code_3: "heb",
    individual_languages: &[
    ],
};


pub const HZ: LanguageCode = LanguageCode {
    name: "Herero",
    code: "hz",
    code_2t: "her",
    code_2b: "her",
    code_3: "her",
    individual_languages: &[
    ],
};


pub const HI: LanguageCode = LanguageCode {
    name: "Hindi",
    code: "hi",
    code_2t: "hin",
    code_2b: "hin",
    code_3: "hin",
    individual_languages: &[
    ],
};


pub const HO: LanguageCode = LanguageCode {
    name: "Hiri Motu",
    code: "ho",
    code_2t: "hmo",
    code_2b: "hmo",
    code_3: "hmo",
    individual_languages: &[
    ],
};


pub const HU: LanguageCode = LanguageCode {
    name: "Hungarian",
    code: "hu",
    code_2t: "hun",
    code_2b: "hun",
    code_3: "hun",
    individual_languages: &[
    ],
};


pub const IS: LanguageCode = LanguageCode {
    name: "Icelandic",
    code: "is",
    code_2t: "isl",
    code_2b: "ice",
    code_3: "isl",
    individual_languages: &[
    ],
};


pub const IO: LanguageCode = LanguageCode {
    name: "Ido",
    code: "io",
    code_2t: "ido",
    code_2b: "ido",
    code_3: "ido",
    individual_languages: &[
    ],
};


pub const IG: LanguageCode = LanguageCode {
    name: "Igbo",
    code: "ig",
    code_2t: "ibo",
    code_2b: "ibo",
    code_3: "ibo",
    individual_languages: &[
    ],
};


pub const ID: LanguageCode = LanguageCode {
    name: "Indonesian",
    code: "id",
    code_2t: "ind",
    code_2b: "ind",
    code_3: "ind",
    individual_languages: &[
    ],
};


pub const IA: LanguageCode = LanguageCode {
    name: "Interlingua (International Auxiliary Language Association)",
    code: "ia",
    code_2t: "ina",
    code_2b: "ina",
    code_3: "ina",
    individual_languages: &[
    ],
};


pub const IE: LanguageCode = LanguageCode {
    name: "Interlingue, Occidental",
    code: "ie",
    code_2t: "ile",
    code_2b: "ile",
    code_3: "ile",
    individual_languages: &[
    ],
};


pub const IU: LanguageCode = LanguageCode {
    name: "Inuktitut",
    code: "iu",
    code_2t: "iku",
    code_2b: "iku",
    code_3: "iku",
    individual_languages: &[
        IndividualLanguages {
            name: "Eastern Canadian Inuktitut",
            code: "ike",
        },
        IndividualLanguages {
            name: "Western Canadian Inuktitut",
            code: "ikt",
        },
    ],
};


pub const IK: LanguageCode = LanguageCode {
    name: "Inupiaq",
    code: "ik",
    code_2t: "ipk",
    code_2b: "ipk",
    code_3: "ipk",
    individual_languages: &[
        IndividualLanguages {
            name: "North Alaskan Inupiatun",
            code: "esi",
        },
        IndividualLanguages {
            name: "Northwest Alaska Inupiatun",
            code: "esk",
        },
    ],
};


pub const GA: LanguageCode = LanguageCode {
    name: "Irish",
    code: "ga",
    code_2t: "gle",
    code_2b: "gle",
    code_3: "gle",
    individual_languages: &[
    ],
};


pub const IT: LanguageCode = LanguageCode {
    name: "Italian",
    code: "it",
    code_2t: "ita",
    code_2b: "ita",
    code_3: "ita",
    individual_languages: &[
    ],
};


pub const JA: LanguageCode = LanguageCode {
    name: "Japanese",
    code: "ja",
    code_2t: "jpn",
    code_2b: "jpn",
    code_3: "jpn",
    individual_languages: &[
    ],
};


pub const JV: LanguageCode = LanguageCode {
    name: "Javanese",
    code: "jv",
    code_2t: "jav",
    code_2b: "jav",
    code_3: "jav",
    individual_languages: &[
    ],
};


pub const KN: LanguageCode = LanguageCode {
    name: "Kannada",
    code: "kn",
    code_2t: "kan",
    code_2b: "kan",
    code_3: "kan",
    individual_languages: &[
    ],
};


pub const KR: LanguageCode = LanguageCode {
    name: "Kanuri",
    code: "kr",
    code_2t: "kau",
    code_2b: "kau",
    code_3: "kau",
    individual_languages: &[
        IndividualLanguages {
            name: "Manga Kanuri",
            code: "kby",
        },
        IndividualLanguages {
            name: "Central Kanuri",
            code: "knc",
        },
        IndividualLanguages {
            name: "Tumari Kanuri",
            code: "krt",
        },
    ],
};


pub const KS: LanguageCode = LanguageCode {
    name: "Kashmiri",
    code: "ks",
    code_2t: "kas",
    code_2b: "kas",
    code_3: "kas",
    individual_languages: &[
    ],
};


pub const KK: LanguageCode = LanguageCode {
    name: "Kazakh",
    code: "kk",
    code_2t: "kaz",
    code_2b: "kaz",
    code_3: "kaz",
    individual_languages: &[
    ],
};


pub const KM: LanguageCode = LanguageCode {
    name: "Central Khmer",
    code: "km",
    code_2t: "khm",
    code_2b: "khm",
    code_3: "khm",
    individual_languages: &[
    ],
};


pub const KI: LanguageCode = LanguageCode {
    name: "Kikuyu, Gikuyu",
    code: "ki",
    code_2t: "kik",
    code_2b: "kik",
    code_3: "kik",
    individual_languages: &[
    ],
};


pub const RW: LanguageCode = LanguageCode {
    name: "Kinyarwanda",
    code: "rw",
    code_2t: "kin",
    code_2b: "kin",
    code_3: "kin",
    individual_languages: &[
    ],
};


pub const KY: LanguageCode = LanguageCode {
    name: "Kirghiz, Kyrgyz",
    code: "ky",
    code_2t: "kir",
    code_2b: "kir",
    code_3: "kir",
    individual_languages: &[
    ],
};


pub const KV: LanguageCode = LanguageCode {
    name: "Komi",
    code: "kv",
    code_2t: "kom",
    code_2b: "kom",
    code_3: "kom",
    individual_languages: &[
        IndividualLanguages {
            name: "Komi-Permyak",
            code: "koi",
        },
        IndividualLanguages {
            name: "Komi-Zyrian",
            code: "kpv",
        },
    ],
};


pub const KG: LanguageCode = LanguageCode {
    name: "Kongo",
    code: "kg",
    code_2t: "kon",
    code_2b: "kon",
    code_3: "kon",
    individual_languages: &[
        IndividualLanguages {
            name: "Koongo",
            code: "kng",
        },
        IndividualLanguages {
            name: "San Salvador Kongo",
            code: "kwy",
        },
        IndividualLanguages {
            name: "Laari",
            code: "ldi",
        },
    ],
};


pub const KO: LanguageCode = LanguageCode {
    name: "Korean",
    code: "ko",
    code_2t: "kor",
    code_2b: "kor",
    code_3: "kor",
    individual_languages: &[
    ],
};


pub const KJ: LanguageCode = LanguageCode {
    name: "Kuanyama, Kwanyama",
    code: "kj",
    code_2t: "kua",
    code_2b: "kua",
    code_3: "kua",
    individual_languages: &[
    ],
};


pub const KU: LanguageCode = LanguageCode {
    name: "Kurdish",
    code: "ku",
    code_2t: "kur",
    code_2b: "kur",
    code_3: "kur",
    individual_languages: &[
        IndividualLanguages {
            name: "Central Kurdish",
            code: "ckb",
        },
        IndividualLanguages {
            name: "Northern Kurdish",
            code: "kmr",
        },
        IndividualLanguages {
            name: "Southern Kurdish",
            code: "sdh",
        },
    ],
};


pub const LO: LanguageCode = LanguageCode {
    name: "Lao",
    code: "lo",
    code_2t: "lao",
    code_2b: "lao",
    code_3: "lao",
    individual_languages: &[
    ],
};


pub const LA: LanguageCode = LanguageCode {
    name: "Latin",
    code: "la",
    code_2t: "lat",
    code_2b: "lat",
    code_3: "lat",
    individual_languages: &[
    ],
};


pub const LV: LanguageCode = LanguageCode {
    name: "Latvian",
    code: "lv",
    code_2t: "lav",
    code_2b: "lav",
    code_3: "lav",
    individual_languages: &[
        IndividualLanguages {
            name: "Latgalian",
            code: "ltg",
        },
        IndividualLanguages {
            name: "Standard Latvian",
            code: "lvs",
        },
    ],
};


pub const LI: LanguageCode = LanguageCode {
    name: "Limburgan, Limburger, Limburgish",
    code: "li",
    code_2t: "lim",
    code_2b: "lim",
    code_3: "lim",
    individual_languages: &[
    ],
};


pub const LN: LanguageCode = LanguageCode {
    name: "Lingala",
    code: "ln",
    code_2t: "lin",
    code_2b: "lin",
    code_3: "lin",
    individual_languages: &[
    ],
};


pub const LT: LanguageCode = LanguageCode {
    name: "Lithuanian",
    code: "lt",
    code_2t: "lit",
    code_2b: "lit",
    code_3: "lit",
    individual_languages: &[
    ],
};


pub const LU: LanguageCode = LanguageCode {
    name: "Luba-Katanga",
    code: "lu",
    code_2t: "lub",
    code_2b: "lub",
    code_3: "lub",
    individual_languages: &[
    ],
};


pub const LB: LanguageCode = LanguageCode {
    name: "Luxembourgish, Letzeburgesch",
    code: "lb",
    code_2t: "ltz",
    code_2b: "ltz",
    code_3: "ltz",
    individual_languages: &[
    ],
};


pub const MK: LanguageCode = LanguageCode {
    name: "Macedonian",
    code: "mk",
    code_2t: "mkd",
    code_2b: "mac",
    code_3: "mkd",
    individual_languages: &[
    ],
};


pub const MG: LanguageCode = LanguageCode {
    name: "Malagasy",
    code: "mg",
    code_2t: "mlg",
    code_2b: "mlg",
    code_3: "mlg",
    individual_languages: &[
        IndividualLanguages {
            name: "Bara Malagasy",
            code: "bhr",
        },
        IndividualLanguages {
            name: "",
            code: "bjq",
        },
        IndividualLanguages {
            name: "Northern Betsimisaraka Malagasy",
            code: "bmm",
        },
        IndividualLanguages {
            name: "Southern Betsimisaraka Malagasy",
            code: "bzc",
        },
        IndividualLanguages {
            name: "Masikoro Malagasy",
            code: "msh",
        },
        IndividualLanguages {
            name: "Plateau Malagasy",
            code: "plt",
        },
        IndividualLanguages {
            name: "Sakalava Malagasy",
            code: "skg",
        },
        IndividualLanguages {
            name: "Tandroy-Mahafaly Malagasy",
            code: "tdx",
        },
        IndividualLanguages {
            name: "Tesaka Malagasy",
            code: "tkg",
        },
        IndividualLanguages {
            name: "Tanosy Malagasy",
            code: "txy",
        },
        IndividualLanguages {
            name: "Tankarana Malagasy",
            code: "xmv",
        },
        IndividualLanguages {
            name: "Tsimihety Malagasy",
            code: "xmw",
        },
    ],
};


pub const MS: LanguageCode = LanguageCode {
    name: "Malay",
    code: "ms",
    code_2t: "msa",
    code_2b: "may",
    code_3: "msa",
    individual_languages: &[
        IndividualLanguages {
            name: "Banjar",
            code: "bjn",
        },
        IndividualLanguages {
            name: "Bacanese Malay",
            code: "btj",
        },
        IndividualLanguages {
            name: "Berau Malay",
            code: "bve",
        },
        IndividualLanguages {
            name: "Bukit Malay",
            code: "bvu",
        },
        IndividualLanguages {
            name: "Cocos Islands Malay",
            code: "coa",
        },
        IndividualLanguages {
            name: "Duano",
            code: "dup",
        },
        IndividualLanguages {
            name: "Haji",
            code: "hji",
        },
        IndividualLanguages {
            name: "Indonesian",
            code: "ind",
        },
        IndividualLanguages {
            name: "Jakun",
            code: "jak",
        },
        IndividualLanguages {
            name: "Jambi Malay",
            code: "jax",
        },
        IndividualLanguages {
            name: "Kubu",
            code: "kvb",
        },
        IndividualLanguages {
            name: "Kerinci",
            code: "kvr",
        },
        IndividualLanguages {
            name: "Brunei",
            code: "kxd",
        },
        IndividualLanguages {
            name: "Sekak",
            code: "lce",
        },
        IndividualLanguages {
            name: "Lubu",
            code: "lcf",
        },
        IndividualLanguages {
            name: "Col",
            code: "liw",
        },
        IndividualLanguages {
            name: "North Moluccan Malay",
            code: "max",
        },
        IndividualLanguages {
            name: "Kedah Malay",
            code: "meo",
        },
        IndividualLanguages {
            name: "Pattani Malay",
            code: "mfa",
        },
        IndividualLanguages {
            name: "Bangka",
            code: "mfb",
        },
        IndividualLanguages {
            name: "Minangkabau",
            code: "min",
        },
        IndividualLanguages {
            name: "",
            code: "mly",
        },
        IndividualLanguages {
            name: "Kota Bangun Kutai Malay",
            code: "mqg",
        },
        IndividualLanguages {
            name: "Sabah Malay",
            code: "msi",
        },
        IndividualLanguages {
            name: "Musi",
            code: "mui",
        },
        IndividualLanguages {
            name: "Orang Kanaq",
            code: "orn",
        },
        IndividualLanguages {
            name: "Orang Seletar",
            code: "ors",
        },
        IndividualLanguages {
            name: "Pekal",
            code: "pel",
        },
        IndividualLanguages {
            name: "Central Malay",
            code: "pse",
        },
        IndividualLanguages {
            name: "Temuan",
            code: "tmw",
        },
        IndividualLanguages {
            name: "Urak Lawoi'",
            code: "urk",
        },
        IndividualLanguages {
            name: "Kaur",
            code: "vkk",
        },
        IndividualLanguages {
            name: "Tenggarong Kutai Malay",
            code: "vkt",
        },
        IndividualLanguages {
            name: "Manado Malay",
            code: "xmm",
        },
        IndividualLanguages {
            name: "Malay (individual language)",
            code: "zlm",
        },
        IndividualLanguages {
            name: "Negeri Sembilan Malay",
            code: "zmi",
        },
        IndividualLanguages {
            name: "Standard Malay",
            code: "zsm",
        },
    ],
};


pub const ML: LanguageCode = LanguageCode {
    name: "Malayalam",
    code: "ml",
    code_2t: "mal",
    code_2b: "mal",
    code_3: "mal",
    individual_languages: &[
    ],
};


pub const MT: LanguageCode = LanguageCode {
    name: "Maltese",
    code: "mt",
    code_2t: "mlt",
    code_2b: "mlt",
    code_3: "mlt",
    individual_languages: &[
    ],
};


pub const GV: LanguageCode = LanguageCode {
    name: "Manx",
    code: "gv",
    code_2t: "glv",
    code_2b: "glv",
    code_3: "glv",
    individual_languages: &[
    ],
};


pub const MI: LanguageCode = LanguageCode {
    name: "Maori",
    code: "mi",
    code_2t: "mri",
    code_2b: "mao",
    code_3: "mri",
    individual_languages: &[
    ],
};


pub const MR: LanguageCode = LanguageCode {
    name: "Marathi",
    code: "mr",
    code_2t: "mar",
    code_2b: "mar",
    code_3: "mar",
    individual_languages: &[
    ],
};


pub const MH: LanguageCode = LanguageCode {
    name: "Marshallese",
    code: "mh",
    code_2t: "mah",
    code_2b: "mah",
    code_3: "mah",
    individual_languages: &[
    ],
};


pub const MN: LanguageCode = LanguageCode {
    name: "Mongolian",
    code: "mn",
    code_2t: "mon",
    code_2b: "mon",
    code_3: "mon",
    individual_languages: &[
        IndividualLanguages {
            name: "Halh Mongolian",
            code: "khk",
        },
        IndividualLanguages {
            name: "Peripheral Mongolian",
            code: "mvf",
        },
    ],
};


pub const NA: LanguageCode = LanguageCode {
    name: "Nauru",
    code: "na",
    code_2t: "nau",
    code_2b: "nau",
    code_3: "nau",
    individual_languages: &[
    ],
};


pub const NV: LanguageCode = LanguageCode {
    name: "Navajo, Navaho",
    code: "nv",
    code_2t: "nav",
    code_2b: "nav",
    code_3: "nav",
    individual_languages: &[
    ],
};


pub const ND: LanguageCode = LanguageCode {
    name: "North Ndebele",
    code: "nd",
    code_2t: "nde",
    code_2b: "nde",
    code_3: "nde",
    individual_languages: &[
    ],
};


pub const NR: LanguageCode = LanguageCode {
    name: "South Ndebele",
    code: "nr",
    code_2t: "nbl",
    code_2b: "nbl",
    code_3: "nbl",
    individual_languages: &[
    ],
};


pub const NG: LanguageCode = LanguageCode {
    name: "Ndonga",
    code: "ng",
    code_2t: "ndo",
    code_2b: "ndo",
    code_3: "ndo",
    individual_languages: &[
    ],
};


pub const NE: LanguageCode = LanguageCode {
    name: "Nepali",
    code: "ne",
    code_2t: "nep",
    code_2b: "nep",
    code_3: "nep",
    individual_languages: &[
        IndividualLanguages {
            name: "Dotyali",
            code: "dty",
        },
        IndividualLanguages {
            name: "Nepali (individual language)",
            code: "npi",
        },
    ],
};


pub const NO: LanguageCode = LanguageCode {
    name: "Norwegian",
    code: "no",
    code_2t: "nor",
    code_2b: "nor",
    code_3: "nor",
    individual_languages: &[
        IndividualLanguages {
            name: "Norwegian Nynorsk",
            code: "nno",
        },
        IndividualLanguages {
            name: "Norwegian Bokmål",
            code: "nob",
        },
    ],
};


pub const NB: LanguageCode = LanguageCode {
    name: "Norwegian Bokmål",
    code: "nb",
    code_2t: "nob",
    code_2b: "nob",
    code_3: "nob",
    individual_languages: &[
    ],
};


pub const NN: LanguageCode = LanguageCode {
    name: "Norwegian Nynorsk",
    code: "nn",
    code_2t: "nno",
    code_2b: "nno",
    code_3: "nno",
    individual_languages: &[
    ],
};


pub const II: LanguageCode = LanguageCode {
    name: "Sichuan Yi, Nuosu",
    code: "ii",
    code_2t: "iii",
    code_2b: "iii",
    code_3: "iii",
    individual_languages: &[
    ],
};


pub const OC: LanguageCode = LanguageCode {
    name: "Occitan",
    code: "oc",
    code_2t: "oci",
    code_2b: "oci",
    code_3: "oci",
    individual_languages: &[
    ],
};


pub const OJ: LanguageCode = LanguageCode {
    name: "Ojibwa",
    code: "oj",
    code_2t: "oji",
    code_2b: "oji",
    code_3: "oji",
    individual_languages: &[
        IndividualLanguages {
            name: "Chippewa",
            code: "ciw",
        },
        IndividualLanguages {
            name: "Northwestern Ojibwa",
            code: "ojb",
        },
        IndividualLanguages {
            name: "Central Ojibwa",
            code: "ojc",
        },
        IndividualLanguages {
            name: "Eastern Ojibwa",
            code: "ojg",
        },
        IndividualLanguages {
            name: "Severn Ojibwa",
            code: "ojs",
        },
        IndividualLanguages {
            name: "Western Ojibwa",
            code: "ojw",
        },
        IndividualLanguages {
            name: "Ottawa",
            code: "otw",
        },
    ],
};


pub const OR: LanguageCode = LanguageCode {
    name: "Oriya",
    code: "or",
    code_2t: "ori",
    code_2b: "ori",
    code_3: "ori",
    individual_languages: &[
        IndividualLanguages {
            name: "Oriya (individual language)",
            code: "ory",
        },
        IndividualLanguages {
            name: "Sambalpuri",
            code: "spv",
        },
    ],
};


pub const OM: LanguageCode = LanguageCode {
    name: "Oromo",
    code: "om",
    code_2t: "orm",
    code_2b: "orm",
    code_3: "orm",
    individual_languages: &[
        IndividualLanguages {
            name: "Borana-Arsi-Guji Oromo",
            code: "gax",
        },
        IndividualLanguages {
            name: "West Central Oromo",
            code: "gaz",
        },
        IndividualLanguages {
            name: "Eastern Oromo",
            code: "hae",
        },
        IndividualLanguages {
            name: "Orma",
            code: "orc",
        },
    ],
};


pub const OS: LanguageCode = LanguageCode {
    name: "Ossetian, Ossetic",
    code: "os",
    code_2t: "oss",
    code_2b: "oss",
    code_3: "oss",
    individual_languages: &[
    ],
};


pub const PI: LanguageCode = LanguageCode {
    name: "Pali",
    code: "pi",
    code_2t: "pli",
    code_2b: "pli",
    code_3: "pli",
    individual_languages: &[
    ],
};


pub const PS: LanguageCode = LanguageCode {
    name: "Pashto, Pushto",
    code: "ps",
    code_2t: "pus",
    code_2b: "pus",
    code_3: "pus",
    individual_languages: &[
        IndividualLanguages {
            name: "Southern Pashto",
            code: "pbt",
        },
        IndividualLanguages {
            name: "Northern Pashto",
            code: "pbu",
        },
        IndividualLanguages {
            name: "Central Pashto",
            code: "pst",
        },
    ],
};


pub const FA: LanguageCode = LanguageCode {
    name: "Persian",
    code: "fa",
    code_2t: "fas",
    code_2b: "per",
    code_3: "fas",
    individual_languages: &[
        IndividualLanguages {
            name: "Iranian Persian",
            code: "pes",
        },
        IndividualLanguages {
            name: "Dari",
            code: "prs",
        },
    ],
};


pub const PL: LanguageCode = LanguageCode {
    name: "Polish",
    code: "pl",
    code_2t: "pol",
    code_2b: "pol",
    code_3: "pol",
    individual_languages: &[
    ],
};


pub const PT: LanguageCode = LanguageCode {
    name: "Portuguese",
    code: "pt",
    code_2t: "por",
    code_2b: "por",
    code_3: "por",
    individual_languages: &[
    ],
};


pub const PA: LanguageCode = LanguageCode {
    name: "Punjabi, Panjabi",
    code: "pa",
    code_2t: "pan",
    code_2b: "pan",
    code_3: "pan",
    individual_languages: &[
    ],
};


pub const QU: LanguageCode = LanguageCode {
    name: "Quechua",
    code: "qu",
    code_2t: "que",
    code_2b: "que",
    code_3: "que",
    individual_languages: &[
        IndividualLanguages {
            name: "",
            code: "cqu",
        },
        IndividualLanguages {
            name: "Huallaga Huánuco Quechua",
            code: "qub",
        },
        IndividualLanguages {
            name: "Calderón Highland Quichua",
            code: "qud",
        },
        IndividualLanguages {
            name: "Lambayeque Quechua",
            code: "quf",
        },
        IndividualLanguages {
            name: "Chimborazo Highland Quichua",
            code: "qug",
        },
        IndividualLanguages {
            name: "South Bolivian Quechua",
            code: "quh",
        },
        IndividualLanguages {
            name: "Chachapoyas Quechua",
            code: "quk",
        },
        IndividualLanguages {
            name: "North Bolivian Quechua",
            code: "qul",
        },
        IndividualLanguages {
            name: "Southern Pastaza Quechua",
            code: "qup",
        },
        IndividualLanguages {
            name: "Yanahuanca Pasco Quechua",
            code: "qur",
        },
        IndividualLanguages {
            name: "Santiago del Estero Quichua",
            code: "qus",
        },
        IndividualLanguages {
            name: "Tena Lowland Quichua",
            code: "quw",
        },
        IndividualLanguages {
            name: "Yauyos Quechua",
            code: "qux",
        },
        IndividualLanguages {
            name: "Ayacucho Quechua",
            code: "quy",
        },
        IndividualLanguages {
            name: "Cusco Quechua",
            code: "quz",
        },
        IndividualLanguages {
            name: "Ambo-Pasco Quechua",
            code: "qva",
        },
        IndividualLanguages {
            name: "Cajamarca Quechua",
            code: "qvc",
        },
        IndividualLanguages {
            name: "Eastern Apurímac Quechua",
            code: "qve",
        },
        IndividualLanguages {
            name: "Huamalíes-Dos de Mayo Huánuco Quechua",
            code: "qvh",
        },
        IndividualLanguages {
            name: "Imbabura Highland Quichua",
            code: "qvi",
        },
        IndividualLanguages {
            name: "Loja Highland Quichua",
            code: "qvj",
        },
        IndividualLanguages {
            name: "Cajatambo North Lima Quechua",
            code: "qvl",
        },
        IndividualLanguages {
            name: "Margos-Yarowilca-Lauricocha Quechua",
            code: "qvm",
        },
        IndividualLanguages {
            name: "North Junín Quechua",
            code: "qvn",
        },
        IndividualLanguages {
            name: "Napo Lowland Quechua",
            code: "qvo",
        },
        IndividualLanguages {
            name: "Pacaraos Quechua",
            code: "qvp",
        },
        IndividualLanguages {
            name: "San Martín Quechua",
            code: "qvs",
        },
        IndividualLanguages {
            name: "Huaylla Wanca Quechua",
            code: "qvw",
        },
        IndividualLanguages {
            name: "Northern Pastaza Quichua",
            code: "qvz",
        },
        IndividualLanguages {
            name: "Corongo Ancash Quechua",
            code: "qwa",
        },
        IndividualLanguages {
            name: "Classical Quechua",
            code: "qwc",
        },
        IndividualLanguages {
            name: "Huaylas Ancash Quechua",
            code: "qwh",
        },
        IndividualLanguages {
            name: "Sihuas Ancash Quechua",
            code: "qws",
        },
        IndividualLanguages {
            name: "Chiquián Ancash Quechua",
            code: "qxa",
        },
        IndividualLanguages {
            name: "Chincha Quechua",
            code: "qxc",
        },
        IndividualLanguages {
            name: "Panao Huánuco Quechua",
            code: "qxh",
        },
        IndividualLanguages {
            name: "Salasaca Highland Quichua",
            code: "qxl",
        },
        IndividualLanguages {
            name: "Northern Conchucos Ancash Quechua",
            code: "qxn",
        },
        IndividualLanguages {
            name: "Southern Conchucos Ancash Quechua",
            code: "qxo",
        },
        IndividualLanguages {
            name: "Puno Quechua",
            code: "qxp",
        },
        IndividualLanguages {
            name: "Cañar Highland Quichua",
            code: "qxr",
        },
        IndividualLanguages {
            name: "Santa Ana de Tusi Pasco Quechua",
            code: "qxt",
        },
        IndividualLanguages {
            name: "Arequipa-La Unión Quechua",
            code: "qxu",
        },
        IndividualLanguages {
            name: "Jauja Wanca Quechua",
            code: "qxw",
        },
    ],
};


pub const RO: LanguageCode = LanguageCode {
    name: "Moldavian, Moldovan",
    code: "ro",
    code_2t: "ron",
    code_2b: "rum",
    code_3: "ron",
    individual_languages: &[
    ],
};


pub const RM: LanguageCode = LanguageCode {
    name: "Romansh",
    code: "rm",
    code_2t: "roh",
    code_2b: "roh",
    code_3: "roh",
    individual_languages: &[
    ],
};


pub const RN: LanguageCode = LanguageCode {
    name: "Rundi",
    code: "rn",
    code_2t: "run",
    code_2b: "run",
    code_3: "run",
    individual_languages: &[
    ],
};


pub const RU: LanguageCode = LanguageCode {
    name: "Russian",
    code: "ru",
    code_2t: "rus",
    code_2b: "rus",
    code_3: "rus",
    individual_languages: &[
    ],
};


pub const SE: LanguageCode = LanguageCode {
    name: "Northern Sami",
    code: "se",
    code_2t: "sme",
    code_2b: "sme",
    code_3: "sme",
    individual_languages: &[
    ],
};


pub const SM: LanguageCode = LanguageCode {
    name: "Samoan",
    code: "sm",
    code_2t: "smo",
    code_2b: "smo",
    code_3: "smo",
    individual_languages: &[
    ],
};


pub const SG: LanguageCode = LanguageCode {
    name: "Sango",
    code: "sg",
    code_2t: "sag",
    code_2b: "sag",
    code_3: "sag",
    individual_languages: &[
    ],
};


pub const SA: LanguageCode = LanguageCode {
    name: "Sanskrit",
    code: "sa",
    code_2t: "san",
    code_2b: "san",
    code_3: "san",
    individual_languages: &[
    ],
};


pub const SC: LanguageCode = LanguageCode {
    name: "Sardinian",
    code: "sc",
    code_2t: "srd",
    code_2b: "srd",
    code_3: "srd",
    individual_languages: &[
        IndividualLanguages {
            name: "Sassarese Sardinian",
            code: "sdc",
        },
        IndividualLanguages {
            name: "Gallurese Sardinian",
            code: "sdn",
        },
        IndividualLanguages {
            name: "Logudorese Sardinian",
            code: "src",
        },
        IndividualLanguages {
            name: "Campidanese Sardinian",
            code: "sro",
        },
    ],
};


pub const SR: LanguageCode = LanguageCode {
    name: "Serbian",
    code: "sr",
    code_2t: "srp",
    code_2b: "srp",
    code_3: "srp",
    individual_languages: &[
    ],
};


pub const SN: LanguageCode = LanguageCode {
    name: "Shona",
    code: "sn",
    code_2t: "sna",
    code_2b: "sna",
    code_3: "sna",
    individual_languages: &[
    ],
};


pub const SD: LanguageCode = LanguageCode {
    name: "Sindhi",
    code: "sd",
    code_2t: "snd",
    code_2b: "snd",
    code_3: "snd",
    individual_languages: &[
    ],
};


pub const SI: LanguageCode = LanguageCode {
    name: "Sinhala, Sinhalese",
    code: "si",
    code_2t: "sin",
    code_2b: "sin",
    code_3: "sin",
    individual_languages: &[
    ],
};


pub const SK: LanguageCode = LanguageCode {
    name: "Slovak",
    code: "sk",
    code_2t: "slk",
    code_2b: "slo",
    code_3: "slk",
    individual_languages: &[
    ],
};


pub const SL: LanguageCode = LanguageCode {
    name: "Slovenian",
    code: "sl",
    code_2t: "slv",
    code_2b: "slv",
    code_3: "slv",
    individual_languages: &[
    ],
};


pub const SO: LanguageCode = LanguageCode {
    name: "Somali",
    code: "so",
    code_2t: "som",
    code_2b: "som",
    code_3: "som",
    individual_languages: &[
    ],
};


pub const ST: LanguageCode = LanguageCode {
    name: "Southern Sotho",
    code: "st",
    code_2t: "sot",
    code_2b: "sot",
    code_3: "sot",
    individual_languages: &[
    ],
};


pub const ES: LanguageCode = LanguageCode {
    name: "Spanish, Castilian",
    code: "es",
    code_2t: "spa",
    code_2b: "spa",
    code_3: "spa",
    individual_languages: &[
    ],
};


pub const SU: LanguageCode = LanguageCode {
    name: "Sundanese",
    code: "su",
    code_2t: "sun",
    code_2b: "sun",
    code_3: "sun",
    individual_languages: &[
    ],
};


pub const SW: LanguageCode = LanguageCode {
    name: "Swahili",
    code: "sw",
    code_2t: "swa",
    code_2b: "swa",
    code_3: "swa",
    individual_languages: &[
        IndividualLanguages {
            name: "Congo Swahili",
            code: "swc",
        },
        IndividualLanguages {
            name: "Swahili (individual language)",
            code: "swh",
        },
    ],
};


pub const SS: LanguageCode = LanguageCode {
    name: "Swati",
    code: "ss",
    code_2t: "ssw",
    code_2b: "ssw",
    code_3: "ssw",
    individual_languages: &[
    ],
};


pub const SV: LanguageCode = LanguageCode {
    name: "Swedish",
    code: "sv",
    code_2t: "swe",
    code_2b: "swe",
    code_3: "swe",
    individual_languages: &[
    ],
};


pub const TL: LanguageCode = LanguageCode {
    name: "Tagalog",
    code: "tl",
    code_2t: "tgl",
    code_2b: "tgl",
    code_3: "tgl",
    individual_languages: &[
    ],
};


pub const TY: LanguageCode = LanguageCode {
    name: "Tahitian",
    code: "ty",
    code_2t: "tah",
    code_2b: "tah",
    code_3: "tah",
    individual_languages: &[
    ],
};


pub const TG: LanguageCode = LanguageCode {
    name: "Tajik",
    code: "tg",
    code_2t: "tgk",
    code_2b: "tgk",
    code_3: "tgk",
    individual_languages: &[
    ],
};


pub const TA: LanguageCode = LanguageCode {
    name: "Tamil",
    code: "ta",
    code_2t: "tam",
    code_2b: "tam",
    code_3: "tam",
    individual_languages: &[
    ],
};


pub const TT: LanguageCode = LanguageCode {
    name: "Tatar",
    code: "tt",
    code_2t: "tat",
    code_2b: "tat",
    code_3: "tat",
    individual_languages: &[
    ],
};


pub const TE: LanguageCode = LanguageCode {
    name: "Telugu",
    code: "te",
    code_2t: "tel",
    code_2b: "tel",
    code_3: "tel",
    individual_languages: &[
    ],
};


pub const TH: LanguageCode = LanguageCode {
    name: "Thai",
    code: "th",
    code_2t: "tha",
    code_2b: "tha",
    code_3: "tha",
    individual_languages: &[
    ],
};


pub const BO: LanguageCode = LanguageCode {
    name: "Tibetan",
    code: "bo",
    code_2t: "bod",
    code_2b: "tib",
    code_3: "bod",
    individual_languages: &[
    ],
};


pub const TI: LanguageCode = LanguageCode {
    name: "Tigrinya",
    code: "ti",
    code_2t: "tir",
    code_2b: "tir",
    code_3: "tir",
    individual_languages: &[
    ],
};


pub const TO: LanguageCode = LanguageCode {
    name: "Tonga (Tonga Islands)",
    code: "to",
    code_2t: "ton",
    code_2b: "ton",
    code_3: "ton",
    individual_languages: &[
    ],
};


pub const TS: LanguageCode = LanguageCode {
    name: "Tsonga",
    code: "ts",
    code_2t: "tso",
    code_2b: "tso",
    code_3: "tso",
    individual_languages: &[
    ],
};


pub const TN: LanguageCode = LanguageCode {
    name: "Tswana",
    code: "tn",
    code_2t: "tsn",
    code_2b: "tsn",
    code_3: "tsn",
    individual_languages: &[
    ],
};


pub const TR: LanguageCode = LanguageCode {
    name: "Turkish",
    code: "tr",
    code_2t: "tur",
    code_2b: "tur",
    code_3: "tur",
    individual_languages: &[
    ],
};


pub const TK: LanguageCode = LanguageCode {
    name: "Turkmen",
    code: "tk",
    code_2t: "tuk",
    code_2b: "tuk",
    code_3: "tuk",
    individual_languages: &[
    ],
};


pub const TW: LanguageCode = LanguageCode {
    name: "Twi",
    code: "tw",
    code_2t: "twi",
    code_2b: "twi",
    code_3: "twi",
    individual_languages: &[
    ],
};


pub const UG: LanguageCode = LanguageCode {
    name: "Uighur, Uyghur",
    code: "ug",
    code_2t: "uig",
    code_2b: "uig",
    code_3: "uig",
    individual_languages: &[
    ],
};


pub const UK: LanguageCode = LanguageCode {
    name: "Ukrainian",
    code: "uk",
    code_2t: "ukr",
    code_2b: "ukr",
    code_3: "ukr",
    individual_languages: &[
    ],
};


pub const UR: LanguageCode = LanguageCode {
    name: "Urdu",
    code: "ur",
    code_2t: "urd",
    code_2b: "urd",
    code_3: "urd",
    individual_languages: &[
    ],
};


pub const UZ: LanguageCode = LanguageCode {
    name: "Uzbek",
    code: "uz",
    code_2t: "uzb",
    code_2b: "uzb",
    code_3: "uzb",
    individual_languages: &[
        IndividualLanguages {
            name: "Northern Uzbek",
            code: "uzn",
        },
        IndividualLanguages {
            name: "Southern Uzbek",
            code: "uzs",
        },
    ],
};


pub const VE: LanguageCode = LanguageCode {
    name: "Venda",
    code: "ve",
    code_2t: "ven",
    code_2b: "ven",
    code_3: "ven",
    individual_languages: &[
    ],
};


pub const VI: LanguageCode = LanguageCode {
    name: "Vietnamese",
    code: "vi",
    code_2t: "vie",
    code_2b: "vie",
    code_3: "vie",
    individual_languages: &[
    ],
};


pub const VO: LanguageCode = LanguageCode {
    name: "Volapük",
    code: "vo",
    code_2t: "vol",
    code_2b: "vol",
    code_3: "vol",
    individual_languages: &[
    ],
};


pub const WA: LanguageCode = LanguageCode {
    name: "Walloon",
    code: "wa",
    code_2t: "wln",
    code_2b: "wln",
    code_3: "wln",
    individual_languages: &[
    ],
};


pub const CY: LanguageCode = LanguageCode {
    name: "Welsh",
    code: "cy",
    code_2t: "cym",
    code_2b: "wel",
    code_3: "cym",
    individual_languages: &[
    ],
};


pub const WO: LanguageCode = LanguageCode {
    name: "Wolof",
    code: "wo",
    code_2t: "wol",
    code_2b: "wol",
    code_3: "wol",
    individual_languages: &[
    ],
};


pub const XH: LanguageCode = LanguageCode {
    name: "Xhosa",
    code: "xh",
    code_2t: "xho",
    code_2b: "xho",
    code_3: "xho",
    individual_languages: &[
    ],
};


pub const YI: LanguageCode = LanguageCode {
    name: "Yiddish",
    code: "yi",
    code_2t: "yid",
    code_2b: "yid",
    code_3: "yid",
    individual_languages: &[
        IndividualLanguages {
            name: "Eastern Yiddish",
            code: "ydd",
        },
        IndividualLanguages {
            name: "Western Yiddish",
            code: "yih",
        },
    ],
};


pub const YO: LanguageCode = LanguageCode {
    name: "Yoruba",
    code: "yo",
    code_2t: "yor",
    code_2b: "yor",
    code_3: "yor",
    individual_languages: &[
    ],
};


pub const ZA: LanguageCode = LanguageCode {
    name: "Zhuang, Chuang",
    code: "za",
    code_2t: "zha",
    code_2b: "zha",
    code_3: "zha",
    individual_languages: &[
        IndividualLanguages {
            name: "",
            code: "ccx",
        },
        IndividualLanguages {
            name: "",
            code: "ccy",
        },
        IndividualLanguages {
            name: "Central Hongshuihe Zhuang",
            code: "zch",
        },
        IndividualLanguages {
            name: "Eastern Hongshuihe Zhuang",
            code: "zeh",
        },
        IndividualLanguages {
            name: "Guibei Zhuang",
            code: "zgb",
        },
        IndividualLanguages {
            name: "Minz Zhuang",
            code: "zgm",
        },
        IndividualLanguages {
            name: "Guibian Zhuang",
            code: "zgn",
        },
        IndividualLanguages {
            name: "Dai Zhuang",
            code: "zhd",
        },
        IndividualLanguages {
            name: "Nong Zhuang",
            code: "zhn",
        },
        IndividualLanguages {
            name: "Liujiang Zhuang",
            code: "zlj",
        },
        IndividualLanguages {
            name: "Lianshan Zhuang",
            code: "zln",
        },
        IndividualLanguages {
            name: "Liuqian Zhuang",
            code: "zlq",
        },
        IndividualLanguages {
            name: "Qiubei Zhuang",
            code: "zqe",
        },
        IndividualLanguages {
            name: "Yongbei Zhuang",
            code: "zyb",
        },
        IndividualLanguages {
            name: "Yang Zhuang",
            code: "zyg",
        },
        IndividualLanguages {
            name: "Youjiang Zhuang",
            code: "zyj",
        },
        IndividualLanguages {
            name: "Yongnan Zhuang",
            code: "zyn",
        },
        IndividualLanguages {
            name: "Zuojiang Zhuang",
            code: "zzj",
        },
    ],
};


pub const ZU: LanguageCode = LanguageCode {
    name: "Zulu",
    code: "zu",
    code_2t: "zul",
    code_2b: "zul",
    code_3: "zul",
    individual_languages: &[
    ],
};


pub const ACE: LanguageCode = LanguageCode {
    name: "Achinese",
    code: "",
    code_2t: "ace",
    code_2b: "ace",
    code_3: "ace",
    individual_languages: &[
    ],
};


pub const ACH: LanguageCode = LanguageCode {
    name: "Acoli",
    code: "",
    code_2t: "ach",
    code_2b: "ach",
    code_3: "ach",
    individual_languages: &[
    ],
};


pub const ADA: LanguageCode = LanguageCode {
    name: "Adangme",
    code: "",
    code_2t: "ada",
    code_2b: "ada",
    code_3: "ada",
    individual_languages: &[
    ],
};


pub const ADY: LanguageCode = LanguageCode {
    name: "Adyghe; Adygei",
    code: "",
    code_2t: "ady",
    code_2b: "ady",
    code_3: "ady",
    individual_languages: &[
    ],
};


pub const AFA: LanguageCode = LanguageCode {
    name: "Afroasiatic languages|Afro-Asiatic languages",
    code: "",
    code_2t: "afa",
    code_2b: "afa",
    code_3: "",
    individual_languages: &[
    ],
};


pub const AFH: LanguageCode = LanguageCode {
    name: "Afrihili",
    code: "",
    code_2t: "afh",
    code_2b: "afh",
    code_3: "afh",
    individual_languages: &[
    ],
};


pub const AIN: LanguageCode = LanguageCode {
    name: "Ainu",
    code: "",
    code_2t: "ain",
    code_2b: "ain",
    code_3: "ain",
    individual_languages: &[
    ],
};


pub const AKK: LanguageCode = LanguageCode {
    name: "Akkadian",
    code: "",
    code_2t: "akk",
    code_2b: "akk",
    code_3: "akk",
    individual_languages: &[
    ],
};


pub const ALE: LanguageCode = LanguageCode {
    name: "Aleut",
    code: "",
    code_2t: "ale",
    code_2b: "ale",
    code_3: "ale",
    individual_languages: &[
    ],
};


pub const ALG: LanguageCode = LanguageCode {
    name: "Algonquian languages",
    code: "",
    code_2t: "alg",
    code_2b: "alg",
    code_3: "",
    individual_languages: &[
    ],
};


pub const ALT: LanguageCode = LanguageCode {
    name: "Southern Southern Altai",
    code: "",
    code_2t: "alt",
    code_2b: "alt",
    code_3: "alt",
    individual_languages: &[
    ],
};


pub const ANG: LanguageCode = LanguageCode {
    name: "Old English|English, Old (ca.450–1100)",
    code: "",
    code_2t: "ang",
    code_2b: "ang",
    code_3: "ang",
    individual_languages: &[
    ],
};


pub const ANP: LanguageCode = LanguageCode {
    name: "Angika",
    code: "",
    code_2t: "anp",
    code_2b: "anp",
    code_3: "anp",
    individual_languages: &[
    ],
};


pub const APA: LanguageCode = LanguageCode {
    name: "Southern Athabaskan languages|Apache languages",
    code: "",
    code_2t: "apa",
    code_2b: "apa",
    code_3: "",
    individual_languages: &[
    ],
};


pub const ARC: LanguageCode = LanguageCode {
    name: "Old Official Aramaic (700–300 BCE); {{nowrap|Imperial Imperial Aramaic (700–300 BCE)}}",
    code: "",
    code_2t: "arc",
    code_2b: "arc",
    code_3: "arc",
    individual_languages: &[
    ],
};


pub const ARN: LanguageCode = LanguageCode {
    name: "Mapudungun; Mapuche",
    code: "",
    code_2t: "arn",
    code_2b: "arn",
    code_3: "arn",
    individual_languages: &[
    ],
};


pub const ARP: LanguageCode = LanguageCode {
    name: "Arapaho",
    code: "",
    code_2t: "arp",
    code_2b: "arp",
    code_3: "arp",
    individual_languages: &[
    ],
};


pub const ART: LanguageCode = LanguageCode {
    name: "Artificial languages",
    code: "",
    code_2t: "art",
    code_2b: "art",
    code_3: "",
    individual_languages: &[
    ],
};


pub const ARW: LanguageCode = LanguageCode {
    name: "Arawak",
    code: "",
    code_2t: "arw",
    code_2b: "arw",
    code_3: "arw",
    individual_languages: &[
    ],
};


pub const AST: LanguageCode = LanguageCode {
    name: "Asturian; Bable; Leonese; Asturleonese",
    code: "",
    code_2t: "ast",
    code_2b: "ast",
    code_3: "ast",
    individual_languages: &[
    ],
};


pub const ATH: LanguageCode = LanguageCode {
    name: "Athapascan languages",
    code: "",
    code_2t: "ath",
    code_2b: "ath",
    code_3: "",
    individual_languages: &[
    ],
};


pub const AUS: LanguageCode = LanguageCode {
    name: "Australian languages",
    code: "",
    code_2t: "aus",
    code_2b: "aus",
    code_3: "",
    individual_languages: &[
    ],
};


pub const AWA: LanguageCode = LanguageCode {
    name: "Awadhi",
    code: "",
    code_2t: "awa",
    code_2b: "awa",
    code_3: "awa",
    individual_languages: &[
    ],
};


pub const BAD: LanguageCode = LanguageCode {
    name: "Banda languages",
    code: "",
    code_2t: "bad",
    code_2b: "bad",
    code_3: "",
    individual_languages: &[
    ],
};


pub const BAI: LanguageCode = LanguageCode {
    name: "Bamileke languages",
    code: "",
    code_2t: "bai",
    code_2b: "bai",
    code_3: "",
    individual_languages: &[
    ],
};


pub const BAL: LanguageCode = LanguageCode {
    name: "Baluchi",
    code: "",
    code_2t: "bal",
    code_2b: "bal",
    code_3: "bal",
    individual_languages: &[
        IndividualLanguages {
            name: "Southern Balochi",
            code: "bcc",
        },
        IndividualLanguages {
            name: "Western Balochi",
            code: "bgn",
        },
        IndividualLanguages {
            name: "Eastern Balochi",
            code: "bgp",
        },
    ],
};


pub const BAN: LanguageCode = LanguageCode {
    name: "Balinese",
    code: "",
    code_2t: "ban",
    code_2b: "ban",
    code_3: "ban",
    individual_languages: &[
    ],
};


pub const BAS: LanguageCode = LanguageCode {
    name: "Basa",
    code: "",
    code_2t: "bas",
    code_2b: "bas",
    code_3: "bas",
    individual_languages: &[
    ],
};


pub const BAT: LanguageCode = LanguageCode {
    name: "Baltic languages",
    code: "",
    code_2t: "bat",
    code_2b: "bat",
    code_3: "",
    individual_languages: &[
    ],
};


pub const BEJ: LanguageCode = LanguageCode {
    name: "Beja; Bedawiyet",
    code: "",
    code_2t: "bej",
    code_2b: "bej",
    code_3: "bej",
    individual_languages: &[
    ],
};


pub const BEM: LanguageCode = LanguageCode {
    name: "Bemba",
    code: "",
    code_2t: "bem",
    code_2b: "bem",
    code_3: "bem",
    individual_languages: &[
    ],
};


pub const BER: LanguageCode = LanguageCode {
    name: "Berber languages",
    code: "",
    code_2t: "ber",
    code_2b: "ber",
    code_3: "",
    individual_languages: &[
    ],
};


pub const BHO: LanguageCode = LanguageCode {
    name: "Bhojpuri",
    code: "",
    code_2t: "bho",
    code_2b: "bho",
    code_3: "bho",
    individual_languages: &[
    ],
};


pub const BIH: LanguageCode = LanguageCode {
    name: "Bihari languages",
    code: "",
    code_2t: "bih",
    code_2b: "bih",
    code_3: "",
    individual_languages: &[
    ],
};


pub const BIK: LanguageCode = LanguageCode {
    name: "Bikol",
    code: "",
    code_2t: "bik",
    code_2b: "bik",
    code_3: "bik",
    individual_languages: &[
        IndividualLanguages {
            name: "Central Bikol",
            code: "bcl",
        },
        IndividualLanguages {
            name: "",
            code: "bhk",
        },
        IndividualLanguages {
            name: "Southern Catanduanes Bikol",
            code: "bln",
        },
        IndividualLanguages {
            name: "Rinconada Bikol",
            code: "bto",
        },
        IndividualLanguages {
            name: "Northern Catanduanes Bikol",
            code: "cts",
        },
        IndividualLanguages {
            name: "West Albay Bikol",
            code: "fbl",
        },
        IndividualLanguages {
            name: "Libon Bikol",
            code: "lbl",
        },
        IndividualLanguages {
            name: "Miraya Bikol",
            code: "rbl",
        },
        IndividualLanguages {
            name: "Buhi'non Bikol",
            code: "ubl",
        },
    ],
};


pub const BIN: LanguageCode = LanguageCode {
    name: "Bini; Edo",
    code: "",
    code_2t: "bin",
    code_2b: "bin",
    code_3: "bin",
    individual_languages: &[
    ],
};


pub const BLA: LanguageCode = LanguageCode {
    name: "Siksika",
    code: "",
    code_2t: "bla",
    code_2b: "bla",
    code_3: "bla",
    individual_languages: &[
    ],
};


pub const BNT: LanguageCode = LanguageCode {
    name: "Bantu languages",
    code: "",
    code_2t: "bnt",
    code_2b: "bnt",
    code_3: "",
    individual_languages: &[
    ],
};


pub const BRA: LanguageCode = LanguageCode {
    name: "Braj",
    code: "",
    code_2t: "bra",
    code_2b: "bra",
    code_3: "bra",
    individual_languages: &[
    ],
};


pub const BTK: LanguageCode = LanguageCode {
    name: "Batak languages",
    code: "",
    code_2t: "btk",
    code_2b: "btk",
    code_3: "",
    individual_languages: &[
    ],
};


pub const BUA: LanguageCode = LanguageCode {
    name: "Buriat",
    code: "",
    code_2t: "bua",
    code_2b: "bua",
    code_3: "bua",
    individual_languages: &[
        IndividualLanguages {
            name: "Mongolia Buriat",
            code: "bxm",
        },
        IndividualLanguages {
            name: "Russia Buriat",
            code: "bxr",
        },
        IndividualLanguages {
            name: "China Buriat",
            code: "bxu",
        },
    ],
};


pub const BUG: LanguageCode = LanguageCode {
    name: "Buginese",
    code: "",
    code_2t: "bug",
    code_2b: "bug",
    code_3: "bug",
    individual_languages: &[
    ],
};


pub const BYN: LanguageCode = LanguageCode {
    name: "Blin; Bilin",
    code: "",
    code_2t: "byn",
    code_2b: "byn",
    code_3: "byn",
    individual_languages: &[
    ],
};


pub const CAD: LanguageCode = LanguageCode {
    name: "Caddo",
    code: "",
    code_2t: "cad",
    code_2b: "cad",
    code_3: "cad",
    individual_languages: &[
    ],
};


pub const CAI: LanguageCode = LanguageCode {
    name: "Central American Indian languages",
    code: "",
    code_2t: "cai",
    code_2b: "cai",
    code_3: "",
    individual_languages: &[
    ],
};


pub const CAR: LanguageCode = LanguageCode {
    name: "Galibi Galibi Carib",
    code: "",
    code_2t: "car",
    code_2b: "car",
    code_3: "car",
    individual_languages: &[
    ],
};


pub const CAU: LanguageCode = LanguageCode {
    name: "Caucasian languages",
    code: "",
    code_2t: "cau",
    code_2b: "cau",
    code_3: "",
    individual_languages: &[
    ],
};


pub const CEB: LanguageCode = LanguageCode {
    name: "Cebuano",
    code: "",
    code_2t: "ceb",
    code_2b: "ceb",
    code_3: "ceb",
    individual_languages: &[
    ],
};


pub const CEL: LanguageCode = LanguageCode {
    name: "Celtic languages",
    code: "",
    code_2t: "cel",
    code_2b: "cel",
    code_3: "",
    individual_languages: &[
    ],
};


pub const CHB: LanguageCode = LanguageCode {
    name: "Chibcha",
    code: "",
    code_2t: "chb",
    code_2b: "chb",
    code_3: "chb",
    individual_languages: &[
    ],
};


pub const CHG: LanguageCode = LanguageCode {
    name: "Chagatai",
    code: "",
    code_2t: "chg",
    code_2b: "chg",
    code_3: "chg",
    individual_languages: &[
    ],
};


pub const CHK: LanguageCode = LanguageCode {
    name: "Chuukese",
    code: "",
    code_2t: "chk",
    code_2b: "chk",
    code_3: "chk",
    individual_languages: &[
    ],
};


pub const CHM: LanguageCode = LanguageCode {
    name: "Mari",
    code: "",
    code_2t: "chm",
    code_2b: "chm",
    code_3: "chm",
    individual_languages: &[
        IndividualLanguages {
            name: "Eastern Mari",
            code: "mhr",
        },
        IndividualLanguages {
            name: "Western Mari",
            code: "mrj",
        },
    ],
};


pub const CHN: LanguageCode = LanguageCode {
    name: "Chinook Chinook jargon",
    code: "",
    code_2t: "chn",
    code_2b: "chn",
    code_3: "chn",
    individual_languages: &[
    ],
};


pub const CHO: LanguageCode = LanguageCode {
    name: "Choctaw",
    code: "",
    code_2t: "cho",
    code_2b: "cho",
    code_3: "cho",
    individual_languages: &[
    ],
};


pub const CHP: LanguageCode = LanguageCode {
    name: "Chipewyan; Dene Dene Suline",
    code: "",
    code_2t: "chp",
    code_2b: "chp",
    code_3: "chp",
    individual_languages: &[
    ],
};


pub const CHR: LanguageCode = LanguageCode {
    name: "Cherokee",
    code: "",
    code_2t: "chr",
    code_2b: "chr",
    code_3: "chr",
    individual_languages: &[
    ],
};


pub const CHY: LanguageCode = LanguageCode {
    name: "Cheyenne",
    code: "",
    code_2t: "chy",
    code_2b: "chy",
    code_3: "chy",
    individual_languages: &[
    ],
};


pub const CMC: LanguageCode = LanguageCode {
    name: "Chamic languages",
    code: "",
    code_2t: "cmc",
    code_2b: "cmc",
    code_3: "",
    individual_languages: &[
    ],
};


pub const CNR: LanguageCode = LanguageCode {
    name: "Montenegrin",
    code: "",
    code_2t: "cnr",
    code_2b: "cnr",
    code_3: "cnr",
    individual_languages: &[
    ],
};


pub const COP: LanguageCode = LanguageCode {
    name: "Coptic",
    code: "",
    code_2t: "cop",
    code_2b: "cop",
    code_3: "cop",
    individual_languages: &[
    ],
};


pub const CPE: LanguageCode = LanguageCode {
    name: "English-based Creoles and List of English-based pidgins|pidgins, English based",
    code: "",
    code_2t: "cpe",
    code_2b: "cpe",
    code_3: "",
    individual_languages: &[
    ],
};


pub const CPF: LanguageCode = LanguageCode {
    name: "French-based creole languages|Creoles and pidgins, French-based",
    code: "",
    code_2t: "cpf",
    code_2b: "cpf",
    code_3: "",
    individual_languages: &[
    ],
};


pub const CPP: LanguageCode = LanguageCode {
    name: "Portuguese-based creole languages|Creoles and pidgins, Portuguese-based",
    code: "",
    code_2t: "cpp",
    code_2b: "cpp",
    code_3: "",
    individual_languages: &[
    ],
};


pub const CRH: LanguageCode = LanguageCode {
    name: "Crimean Crimean Tatar; Crimean Crimean Turkish",
    code: "",
    code_2t: "crh",
    code_2b: "crh",
    code_3: "crh",
    individual_languages: &[
    ],
};


pub const CRP: LanguageCode = LanguageCode {
    name: "Creoles and pidgins",
    code: "",
    code_2t: "crp",
    code_2b: "crp",
    code_3: "",
    individual_languages: &[
    ],
};


pub const CSB: LanguageCode = LanguageCode {
    name: "Kashubian",
    code: "",
    code_2t: "csb",
    code_2b: "csb",
    code_3: "csb",
    individual_languages: &[
    ],
};


pub const CUS: LanguageCode = LanguageCode {
    name: "Cushitic languages",
    code: "",
    code_2t: "cus",
    code_2b: "cus",
    code_3: "",
    individual_languages: &[
    ],
};


pub const DAK: LanguageCode = LanguageCode {
    name: "Dakota",
    code: "",
    code_2t: "dak",
    code_2b: "dak",
    code_3: "dak",
    individual_languages: &[
    ],
};


pub const DAR: LanguageCode = LanguageCode {
    name: "Dargwa",
    code: "",
    code_2t: "dar",
    code_2b: "dar",
    code_3: "dar",
    individual_languages: &[
    ],
};


pub const DAY: LanguageCode = LanguageCode {
    name: "Land Dayak languages",
    code: "",
    code_2t: "day",
    code_2b: "day",
    code_3: "",
    individual_languages: &[
    ],
};


pub const DEL: LanguageCode = LanguageCode {
    name: "Delaware",
    code: "",
    code_2t: "del",
    code_2b: "del",
    code_3: "del",
    individual_languages: &[
        IndividualLanguages {
            name: "Munsee",
            code: "umu",
        },
        IndividualLanguages {
            name: "Unami",
            code: "unm",
        },
    ],
};


pub const DEN: LanguageCode = LanguageCode {
    name: "Slave language (Athapascan)|Slave (Athapascan)",
    code: "",
    code_2t: "den",
    code_2b: "den",
    code_3: "den",
    individual_languages: &[
        IndividualLanguages {
            name: "North Slavey",
            code: "scs",
        },
        IndividualLanguages {
            name: "South Slavey",
            code: "xsl",
        },
    ],
};


pub const DGR: LanguageCode = LanguageCode {
    name: "Dogrib",
    code: "",
    code_2t: "dgr",
    code_2b: "dgr",
    code_3: "dgr",
    individual_languages: &[
    ],
};


pub const DIN: LanguageCode = LanguageCode {
    name: "Dinka",
    code: "",
    code_2t: "din",
    code_2b: "din",
    code_3: "din",
    individual_languages: &[
        IndividualLanguages {
            name: "South Central Dinka",
            code: "dib",
        },
        IndividualLanguages {
            name: "Southwestern Dinka",
            code: "dik",
        },
        IndividualLanguages {
            name: "Northeastern Dinka",
            code: "dip",
        },
        IndividualLanguages {
            name: "Northwestern Dinka",
            code: "diw",
        },
        IndividualLanguages {
            name: "Southeastern Dinka",
            code: "dks",
        },
    ],
};


pub const DOI: LanguageCode = LanguageCode {
    name: "Dogri",
    code: "",
    code_2t: "doi",
    code_2b: "doi",
    code_3: "doi",
    individual_languages: &[
        IndividualLanguages {
            name: "Dogri (individual language)",
            code: "dgo",
        },
        IndividualLanguages {
            name: "Kangri",
            code: "xnr",
        },
    ],
};


pub const DRA: LanguageCode = LanguageCode {
    name: "Dravidian languages",
    code: "",
    code_2t: "dra",
    code_2b: "dra",
    code_3: "",
    individual_languages: &[
    ],
};


pub const DSB: LanguageCode = LanguageCode {
    name: "Lower Lower Sorbian",
    code: "",
    code_2t: "dsb",
    code_2b: "dsb",
    code_3: "dsb",
    individual_languages: &[
    ],
};


pub const DUA: LanguageCode = LanguageCode {
    name: "Duala",
    code: "",
    code_2t: "dua",
    code_2b: "dua",
    code_3: "dua",
    individual_languages: &[
    ],
};


pub const DUM: LanguageCode = LanguageCode {
    name: "Middle Dutch|Dutch, Middle (ca. 1050–1350)",
    code: "",
    code_2t: "dum",
    code_2b: "dum",
    code_3: "dum",
    individual_languages: &[
    ],
};


pub const DYU: LanguageCode = LanguageCode {
    name: "Dyula",
    code: "",
    code_2t: "dyu",
    code_2b: "dyu",
    code_3: "dyu",
    individual_languages: &[
    ],
};


pub const EFI: LanguageCode = LanguageCode {
    name: "Efik",
    code: "",
    code_2t: "efi",
    code_2b: "efi",
    code_3: "efi",
    individual_languages: &[
    ],
};


pub const EGY: LanguageCode = LanguageCode {
    name: "Egyptian language (Ancient)|Egyptian (Ancient)",
    code: "",
    code_2t: "egy",
    code_2b: "egy",
    code_3: "egy",
    individual_languages: &[
    ],
};


pub const EKA: LanguageCode = LanguageCode {
    name: "Ekajuk",
    code: "",
    code_2t: "eka",
    code_2b: "eka",
    code_3: "eka",
    individual_languages: &[
    ],
};


pub const ELX: LanguageCode = LanguageCode {
    name: "Elamite",
    code: "",
    code_2t: "elx",
    code_2b: "elx",
    code_3: "elx",
    individual_languages: &[
    ],
};


pub const ENM: LanguageCode = LanguageCode {
    name: "Middle English|English, Middle (1100–1500)",
    code: "",
    code_2t: "enm",
    code_2b: "enm",
    code_3: "enm",
    individual_languages: &[
    ],
};


pub const EWO: LanguageCode = LanguageCode {
    name: "Ewondo",
    code: "",
    code_2t: "ewo",
    code_2b: "ewo",
    code_3: "ewo",
    individual_languages: &[
    ],
};


pub const FAN: LanguageCode = LanguageCode {
    name: "Fang",
    code: "",
    code_2t: "fan",
    code_2b: "fan",
    code_3: "fan",
    individual_languages: &[
    ],
};


pub const FAT: LanguageCode = LanguageCode {
    name: "Fanti",
    code: "",
    code_2t: "fat",
    code_2b: "fat",
    code_3: "fat",
    individual_languages: &[
    ],
};


pub const FIL: LanguageCode = LanguageCode {
    name: "Filipino; Pilipino",
    code: "",
    code_2t: "fil",
    code_2b: "fil",
    code_3: "fil",
    individual_languages: &[
    ],
};


pub const FIU: LanguageCode = LanguageCode {
    name: "Finno-Ugrian languages",
    code: "",
    code_2t: "fiu",
    code_2b: "fiu",
    code_3: "",
    individual_languages: &[
    ],
};


pub const FON: LanguageCode = LanguageCode {
    name: "Fon",
    code: "",
    code_2t: "fon",
    code_2b: "fon",
    code_3: "fon",
    individual_languages: &[
    ],
};


pub const FRM: LanguageCode = LanguageCode {
    name: "Middle French|French, Middle (ca. 1400–1600)",
    code: "",
    code_2t: "frm",
    code_2b: "frm",
    code_3: "frm",
    individual_languages: &[
    ],
};


pub const FRO: LanguageCode = LanguageCode {
    name: "Old French|French, Old (842–ca. 1400)",
    code: "",
    code_2t: "fro",
    code_2b: "fro",
    code_3: "fro",
    individual_languages: &[
    ],
};


pub const FRR: LanguageCode = LanguageCode {
    name: "Northern Northern Frisian",
    code: "",
    code_2t: "frr",
    code_2b: "frr",
    code_3: "frr",
    individual_languages: &[
    ],
};


pub const FRS: LanguageCode = LanguageCode {
    name: "East Frisian Low Saxon",
    code: "",
    code_2t: "frs",
    code_2b: "frs",
    code_3: "frs",
    individual_languages: &[
    ],
};


pub const FUR: LanguageCode = LanguageCode {
    name: "Friulian",
    code: "",
    code_2t: "fur",
    code_2b: "fur",
    code_3: "fur",
    individual_languages: &[
    ],
};


pub const GAA: LanguageCode = LanguageCode {
    name: "Ga",
    code: "",
    code_2t: "gaa",
    code_2b: "gaa",
    code_3: "gaa",
    individual_languages: &[
    ],
};


pub const GAY: LanguageCode = LanguageCode {
    name: "Gayo",
    code: "",
    code_2t: "gay",
    code_2b: "gay",
    code_3: "gay",
    individual_languages: &[
    ],
};


pub const GBA: LanguageCode = LanguageCode {
    name: "Gbaya",
    code: "",
    code_2t: "gba",
    code_2b: "gba",
    code_3: "gba",
    individual_languages: &[
        IndividualLanguages {
            name: "Bokoto",
            code: "bdt",
        },
        IndividualLanguages {
            name: "Gbaya-Bossangoa",
            code: "gbp",
        },
        IndividualLanguages {
            name: "Gbaya-Bozoum",
            code: "gbq",
        },
        IndividualLanguages {
            name: "Gbaya-Mbodomo",
            code: "gmm",
        },
        IndividualLanguages {
            name: "Southwest Gbaya",
            code: "gso",
        },
        IndividualLanguages {
            name: "Northwest Gbaya",
            code: "gya",
        },
        IndividualLanguages {
            name: "",
            code: "mdo",
        },
    ],
};


pub const GEM: LanguageCode = LanguageCode {
    name: "Germanic languages",
    code: "",
    code_2t: "gem",
    code_2b: "gem",
    code_3: "",
    individual_languages: &[
    ],
};


pub const GEZ: LanguageCode = LanguageCode {
    name: "Geez",
    code: "",
    code_2t: "gez",
    code_2b: "gez",
    code_3: "gez",
    individual_languages: &[
    ],
};


pub const GIL: LanguageCode = LanguageCode {
    name: "Gilbertese",
    code: "",
    code_2t: "gil",
    code_2b: "gil",
    code_3: "gil",
    individual_languages: &[
    ],
};


pub const GMH: LanguageCode = LanguageCode {
    name: "Middle High German|German, Middle High (ca. 1050–1500)",
    code: "",
    code_2t: "gmh",
    code_2b: "gmh",
    code_3: "gmh",
    individual_languages: &[
    ],
};


pub const GOH: LanguageCode = LanguageCode {
    name: "Old High German|German, Old High (ca. 750–1050)",
    code: "",
    code_2t: "goh",
    code_2b: "goh",
    code_3: "goh",
    individual_languages: &[
    ],
};


pub const GON: LanguageCode = LanguageCode {
    name: "Gondi",
    code: "",
    code_2t: "gon",
    code_2b: "gon",
    code_3: "gon",
    individual_languages: &[
        IndividualLanguages {
            name: "Aheri Gondi",
            code: "esg",
        },
        IndividualLanguages {
            name: "",
            code: "ggo",
        },
        IndividualLanguages {
            name: "Northern Gondi",
            code: "gno",
        },
        IndividualLanguages {
            name: "Adilabad Gondi",
            code: "wsg",
        },
    ],
};


pub const GOR: LanguageCode = LanguageCode {
    name: "Gorontalo",
    code: "",
    code_2t: "gor",
    code_2b: "gor",
    code_3: "gor",
    individual_languages: &[
    ],
};


pub const GOT: LanguageCode = LanguageCode {
    name: "Gothic",
    code: "",
    code_2t: "got",
    code_2b: "got",
    code_3: "got",
    individual_languages: &[
    ],
};


pub const GRB: LanguageCode = LanguageCode {
    name: "Grebo",
    code: "",
    code_2t: "grb",
    code_2b: "grb",
    code_3: "grb",
    individual_languages: &[
        IndividualLanguages {
            name: "Northern Grebo",
            code: "gbo",
        },
        IndividualLanguages {
            name: "Gboloo Grebo",
            code: "gec",
        },
        IndividualLanguages {
            name: "Southern Grebo",
            code: "grj",
        },
        IndividualLanguages {
            name: "Central Grebo",
            code: "grv",
        },
        IndividualLanguages {
            name: "Barclayville Grebo",
            code: "gry",
        },
    ],
};


pub const GRC: LanguageCode = LanguageCode {
    name: "Ancient Greek|Greek, Ancient (to 1453)",
    code: "",
    code_2t: "grc",
    code_2b: "grc",
    code_3: "grc",
    individual_languages: &[
    ],
};


pub const GSW: LanguageCode = LanguageCode {
    name: "Swiss Swiss German; Alemannic; Alsatian",
    code: "",
    code_2t: "gsw",
    code_2b: "gsw",
    code_3: "gsw",
    individual_languages: &[
    ],
};


pub const GWI: LanguageCode = LanguageCode {
    name: "Gwich’Gwich'in",
    code: "",
    code_2t: "gwi",
    code_2b: "gwi",
    code_3: "gwi",
    individual_languages: &[
    ],
};


pub const HAI: LanguageCode = LanguageCode {
    name: "Haida",
    code: "",
    code_2t: "hai",
    code_2b: "hai",
    code_3: "hai",
    individual_languages: &[
        IndividualLanguages {
            name: "Southern Haida",
            code: "hax",
        },
        IndividualLanguages {
            name: "Northern Haida",
            code: "hdn",
        },
    ],
};


pub const HAW: LanguageCode = LanguageCode {
    name: "Hawaiian",
    code: "",
    code_2t: "haw",
    code_2b: "haw",
    code_3: "haw",
    individual_languages: &[
    ],
};


pub const HIL: LanguageCode = LanguageCode {
    name: "Hiligaynon",
    code: "",
    code_2t: "hil",
    code_2b: "hil",
    code_3: "hil",
    individual_languages: &[
    ],
};


pub const HIM: LanguageCode = LanguageCode {
    name: "Western Pahari|Himachali&nbsp;languages; Pahari&nbsp;languages",
    code: "",
    code_2t: "him",
    code_2b: "him",
    code_3: "",
    individual_languages: &[
    ],
};


pub const HIT: LanguageCode = LanguageCode {
    name: "Hittite",
    code: "",
    code_2t: "hit",
    code_2b: "hit",
    code_3: "hit",
    individual_languages: &[
    ],
};


pub const HMN: LanguageCode = LanguageCode {
    name: "Hmong; Mong",
    code: "",
    code_2t: "hmn",
    code_2b: "hmn",
    code_3: "hmn",
    individual_languages: &[
        IndividualLanguages {
            name: "",
            code: "blu",
        },
        IndividualLanguages {
            name: "Chuanqiandian Cluster Miao",
            code: "cqd",
        },
        IndividualLanguages {
            name: "Northern Qiandong Miao",
            code: "hea",
        },
        IndividualLanguages {
            name: "Southern Mashan Miao",
            code: "hma",
        },
        IndividualLanguages {
            name: "Central Huishui Miao",
            code: "hmc",
        },
        IndividualLanguages {
            name: "Large Flowery Miao",
            code: "hmd",
        },
        IndividualLanguages {
            name: "Eastern Huishui Miao",
            code: "hme",
        },
        IndividualLanguages {
            name: "Southwestern Guiyang Hmong",
            code: "hmg",
        },
        IndividualLanguages {
            name: "Southwestern Huishui Miao",
            code: "hmh",
        },
        IndividualLanguages {
            name: "Northern Huishui Miao",
            code: "hmi",
        },
        IndividualLanguages {
            name: "Gejia",
            code: "hmj",
        },
        IndividualLanguages {
            name: "Luopohe Miao",
            code: "hml",
        },
        IndividualLanguages {
            name: "Central Mashan Miao",
            code: "hmm",
        },
        IndividualLanguages {
            name: "Northern Mashan Miao",
            code: "hmp",
        },
        IndividualLanguages {
            name: "Eastern Qiandong Miao",
            code: "hmq",
        },
        IndividualLanguages {
            name: "Southern Qiandong Miao",
            code: "hms",
        },
        IndividualLanguages {
            name: "Western Mashan Miao",
            code: "hmw",
        },
        IndividualLanguages {
            name: "Southern Guiyang Miao",
            code: "hmy",
        },
        IndividualLanguages {
            name: "Sinicized Miao",
            code: "hmz",
        },
        IndividualLanguages {
            name: "Mong Njua",
            code: "hnj",
        },
        IndividualLanguages {
            name: "Horned Miao",
            code: "hrm",
        },
        IndividualLanguages {
            name: "Northern Guiyang Miao",
            code: "huj",
        },
        IndividualLanguages {
            name: "Western Xiangxi Miao",
            code: "mmr",
        },
        IndividualLanguages {
            name: "Eastern Xiangxi Miao",
            code: "muq",
        },
        IndividualLanguages {
            name: "Hmong Daw",
            code: "mww",
        },
        IndividualLanguages {
            name: "Small Flowery Miao",
            code: "sfm",
        },
    ],
};


pub const HSB: LanguageCode = LanguageCode {
    name: "Upper Upper Sorbian",
    code: "",
    code_2t: "hsb",
    code_2b: "hsb",
    code_3: "hsb",
    individual_languages: &[
    ],
};


pub const HUP: LanguageCode = LanguageCode {
    name: "Hupa",
    code: "",
    code_2t: "hup",
    code_2b: "hup",
    code_3: "hup",
    individual_languages: &[
    ],
};


pub const IBA: LanguageCode = LanguageCode {
    name: "Iban",
    code: "",
    code_2t: "iba",
    code_2b: "iba",
    code_3: "iba",
    individual_languages: &[
    ],
};


pub const IJO: LanguageCode = LanguageCode {
    name: "Ijo languages",
    code: "",
    code_2t: "ijo",
    code_2b: "ijo",
    code_3: "",
    individual_languages: &[
    ],
};


pub const ILO: LanguageCode = LanguageCode {
    name: "Iloko",
    code: "",
    code_2t: "ilo",
    code_2b: "ilo",
    code_3: "ilo",
    individual_languages: &[
    ],
};


pub const INC: LanguageCode = LanguageCode {
    name: "Indo-Aryan languages",
    code: "",
    code_2t: "inc",
    code_2b: "inc",
    code_3: "",
    individual_languages: &[
    ],
};


pub const INE: LanguageCode = LanguageCode {
    name: "Indo-European languages",
    code: "",
    code_2t: "ine",
    code_2b: "ine",
    code_3: "",
    individual_languages: &[
    ],
};


pub const INH: LanguageCode = LanguageCode {
    name: "Ingush",
    code: "",
    code_2t: "inh",
    code_2b: "inh",
    code_3: "inh",
    individual_languages: &[
    ],
};


pub const IRA: LanguageCode = LanguageCode {
    name: "Iranian languages",
    code: "",
    code_2t: "ira",
    code_2b: "ira",
    code_3: "",
    individual_languages: &[
    ],
};


pub const IRO: LanguageCode = LanguageCode {
    name: "Iroquoian languages",
    code: "",
    code_2t: "iro",
    code_2b: "iro",
    code_3: "",
    individual_languages: &[
    ],
};


pub const JBO: LanguageCode = LanguageCode {
    name: "Lojban",
    code: "",
    code_2t: "jbo",
    code_2b: "jbo",
    code_3: "jbo",
    individual_languages: &[
    ],
};


pub const JPR: LanguageCode = LanguageCode {
    name: "Judeo-Judeo-Persian",
    code: "",
    code_2t: "jpr",
    code_2b: "jpr",
    code_3: "jpr",
    individual_languages: &[
    ],
};


pub const JRB: LanguageCode = LanguageCode {
    name: "Judeo-Judeo-Arabic",
    code: "",
    code_2t: "jrb",
    code_2b: "jrb",
    code_3: "jrb",
    individual_languages: &[
        IndividualLanguages {
            name: "",
            code: "ajt",
        },
        IndividualLanguages {
            name: "Judeo-Moroccan Arabic",
            code: "aju",
        },
        IndividualLanguages {
            name: "Judeo-Yemeni Arabic",
            code: "jye",
        },
        IndividualLanguages {
            name: "Judeo-Iraqi Arabic",
            code: "yhd",
        },
        IndividualLanguages {
            name: "Judeo-Tripolitanian Arabic",
            code: "yud",
        },
    ],
};


pub const KAA: LanguageCode = LanguageCode {
    name: "Kara-Kara-Kalpak",
    code: "",
    code_2t: "kaa",
    code_2b: "kaa",
    code_3: "kaa",
    individual_languages: &[
    ],
};


pub const KAB: LanguageCode = LanguageCode {
    name: "Kabyle",
    code: "",
    code_2t: "kab",
    code_2b: "kab",
    code_3: "kab",
    individual_languages: &[
    ],
};


pub const KAC: LanguageCode = LanguageCode {
    name: "Kachin; Jingpho",
    code: "",
    code_2t: "kac",
    code_2b: "kac",
    code_3: "kac",
    individual_languages: &[
    ],
};


pub const KAM: LanguageCode = LanguageCode {
    name: "Kamba",
    code: "",
    code_2t: "kam",
    code_2b: "kam",
    code_3: "kam",
    individual_languages: &[
    ],
};


pub const KAR: LanguageCode = LanguageCode {
    name: "Karen languages",
    code: "",
    code_2t: "kar",
    code_2b: "kar",
    code_3: "",
    individual_languages: &[
    ],
};


pub const KAW: LanguageCode = LanguageCode {
    name: "Kawi",
    code: "",
    code_2t: "kaw",
    code_2b: "kaw",
    code_3: "kaw",
    individual_languages: &[
    ],
};


pub const KBD: LanguageCode = LanguageCode {
    name: "Kabardian",
    code: "",
    code_2t: "kbd",
    code_2b: "kbd",
    code_3: "kbd",
    individual_languages: &[
    ],
};


pub const KHA: LanguageCode = LanguageCode {
    name: "Khasi",
    code: "",
    code_2t: "kha",
    code_2b: "kha",
    code_3: "kha",
    individual_languages: &[
    ],
};


pub const KHI: LanguageCode = LanguageCode {
    name: "Khoisan languages",
    code: "",
    code_2t: "khi",
    code_2b: "khi",
    code_3: "",
    individual_languages: &[
    ],
};


pub const KHO: LanguageCode = LanguageCode {
    name: "Khotanese; Sakan",
    code: "",
    code_2t: "kho",
    code_2b: "kho",
    code_3: "kho",
    individual_languages: &[
    ],
};


pub const KMB: LanguageCode = LanguageCode {
    name: "Kimbundu",
    code: "",
    code_2t: "kmb",
    code_2b: "kmb",
    code_3: "kmb",
    individual_languages: &[
    ],
};


pub const KOK: LanguageCode = LanguageCode {
    name: "Konkani",
    code: "",
    code_2t: "kok",
    code_2b: "kok",
    code_3: "kok",
    individual_languages: &[
        IndividualLanguages {
            name: "Goan Konkani",
            code: "gom",
        },
        IndividualLanguages {
            name: "Konkani (individual language)",
            code: "knn",
        },
    ],
};


pub const KOS: LanguageCode = LanguageCode {
    name: "Kosraean",
    code: "",
    code_2t: "kos",
    code_2b: "kos",
    code_3: "kos",
    individual_languages: &[
    ],
};


pub const KPE: LanguageCode = LanguageCode {
    name: "Kpelle",
    code: "",
    code_2t: "kpe",
    code_2b: "kpe",
    code_3: "kpe",
    individual_languages: &[
        IndividualLanguages {
            name: "Guinea Kpelle",
            code: "gkp",
        },
        IndividualLanguages {
            name: "Liberia Kpelle",
            code: "xpe",
        },
    ],
};


pub const KRC: LanguageCode = LanguageCode {
    name: "Karachay-Karachay-Balkar",
    code: "",
    code_2t: "krc",
    code_2b: "krc",
    code_3: "krc",
    individual_languages: &[
    ],
};


pub const KRL: LanguageCode = LanguageCode {
    name: "Karelian",
    code: "",
    code_2t: "krl",
    code_2b: "krl",
    code_3: "krl",
    individual_languages: &[
    ],
};


pub const KRO: LanguageCode = LanguageCode {
    name: "Kru languages",
    code: "",
    code_2t: "kro",
    code_2b: "kro",
    code_3: "",
    individual_languages: &[
    ],
};


pub const KRU: LanguageCode = LanguageCode {
    name: "Kurukh",
    code: "",
    code_2t: "kru",
    code_2b: "kru",
    code_3: "kru",
    individual_languages: &[
    ],
};


pub const KUM: LanguageCode = LanguageCode {
    name: "Kumyk",
    code: "",
    code_2t: "kum",
    code_2b: "kum",
    code_3: "kum",
    individual_languages: &[
    ],
};


pub const KUT: LanguageCode = LanguageCode {
    name: "Kutenai",
    code: "",
    code_2t: "kut",
    code_2b: "kut",
    code_3: "kut",
    individual_languages: &[
    ],
};


pub const LAD: LanguageCode = LanguageCode {
    name: "Ladino",
    code: "",
    code_2t: "lad",
    code_2b: "lad",
    code_3: "lad",
    individual_languages: &[
    ],
};


pub const LAH: LanguageCode = LanguageCode {
    name: "Lahnda",
    code: "",
    code_2t: "lah",
    code_2b: "lah",
    code_3: "lah",
    individual_languages: &[
        IndividualLanguages {
            name: "Southern Hindko",
            code: "hnd",
        },
        IndividualLanguages {
            name: "Northern Hindko",
            code: "hno",
        },
        IndividualLanguages {
            name: "Jakati",
            code: "jat",
        },
        IndividualLanguages {
            name: "Pahari-Potwari",
            code: "phr",
        },
        IndividualLanguages {
            name: "",
            code: "pmu",
        },
        IndividualLanguages {
            name: "Western Panjabi",
            code: "pnb",
        },
        IndividualLanguages {
            name: "Seraiki",
            code: "skr",
        },
        IndividualLanguages {
            name: "Khetrani",
            code: "xhe",
        },
    ],
};


pub const LAM: LanguageCode = LanguageCode {
    name: "Lamba",
    code: "",
    code_2t: "lam",
    code_2b: "lam",
    code_3: "lam",
    individual_languages: &[
    ],
};


pub const LEZ: LanguageCode = LanguageCode {
    name: "Lezghian",
    code: "",
    code_2t: "lez",
    code_2b: "lez",
    code_3: "lez",
    individual_languages: &[
    ],
};


pub const LOL: LanguageCode = LanguageCode {
    name: "Mongo",
    code: "",
    code_2t: "lol",
    code_2b: "lol",
    code_3: "lol",
    individual_languages: &[
    ],
};


pub const LOZ: LanguageCode = LanguageCode {
    name: "Lozi",
    code: "",
    code_2t: "loz",
    code_2b: "loz",
    code_3: "loz",
    individual_languages: &[
    ],
};


pub const LUA: LanguageCode = LanguageCode {
    name: "Luba-Luba-Lulua",
    code: "",
    code_2t: "lua",
    code_2b: "lua",
    code_3: "lua",
    individual_languages: &[
    ],
};


pub const LUI: LanguageCode = LanguageCode {
    name: "Luiseno",
    code: "",
    code_2t: "lui",
    code_2b: "lui",
    code_3: "lui",
    individual_languages: &[
    ],
};


pub const LUN: LanguageCode = LanguageCode {
    name: "Lunda",
    code: "",
    code_2t: "lun",
    code_2b: "lun",
    code_3: "lun",
    individual_languages: &[
    ],
};


pub const LUO: LanguageCode = LanguageCode {
    name: "Luo language (Kenya and Tanzania)|Luo (Kenya and Tanzania)",
    code: "",
    code_2t: "luo",
    code_2b: "luo",
    code_3: "luo",
    individual_languages: &[
    ],
};


pub const LUS: LanguageCode = LanguageCode {
    name: "Lushai",
    code: "",
    code_2t: "lus",
    code_2b: "lus",
    code_3: "lus",
    individual_languages: &[
    ],
};


pub const MAD: LanguageCode = LanguageCode {
    name: "Madurese",
    code: "",
    code_2t: "mad",
    code_2b: "mad",
    code_3: "mad",
    individual_languages: &[
    ],
};


pub const MAG: LanguageCode = LanguageCode {
    name: "Magahi",
    code: "",
    code_2t: "mag",
    code_2b: "mag",
    code_3: "mag",
    individual_languages: &[
    ],
};


pub const MAI: LanguageCode = LanguageCode {
    name: "Maithili",
    code: "",
    code_2t: "mai",
    code_2b: "mai",
    code_3: "mai",
    individual_languages: &[
    ],
};


pub const MAK: LanguageCode = LanguageCode {
    name: "Makasar",
    code: "",
    code_2t: "mak",
    code_2b: "mak",
    code_3: "mak",
    individual_languages: &[
    ],
};


pub const MAN: LanguageCode = LanguageCode {
    name: "Mandingo",
    code: "",
    code_2t: "man",
    code_2b: "man",
    code_3: "man",
    individual_languages: &[
        IndividualLanguages {
            name: "Eastern Maninkakan",
            code: "emk",
        },
        IndividualLanguages {
            name: "Konyanka Maninka",
            code: "mku",
        },
        IndividualLanguages {
            name: "Western Maninkakan",
            code: "mlq",
        },
        IndividualLanguages {
            name: "Mandinka",
            code: "mnk",
        },
        IndividualLanguages {
            name: "Sankaran Maninka",
            code: "msc",
        },
        IndividualLanguages {
            name: "Kita Maninkakan",
            code: "mwk",
        },
        IndividualLanguages {
            name: "",
            code: "myq",
        },
    ],
};


pub const MAP: LanguageCode = LanguageCode {
    name: "Austronesian languages",
    code: "",
    code_2t: "map",
    code_2b: "map",
    code_3: "",
    individual_languages: &[
    ],
};


pub const MAS: LanguageCode = LanguageCode {
    name: "Masai",
    code: "",
    code_2t: "mas",
    code_2b: "mas",
    code_3: "mas",
    individual_languages: &[
    ],
};


pub const MDF: LanguageCode = LanguageCode {
    name: "Moksha",
    code: "",
    code_2t: "mdf",
    code_2b: "mdf",
    code_3: "mdf",
    individual_languages: &[
    ],
};


pub const MDR: LanguageCode = LanguageCode {
    name: "Mandar",
    code: "",
    code_2t: "mdr",
    code_2b: "mdr",
    code_3: "mdr",
    individual_languages: &[
    ],
};


pub const MEN: LanguageCode = LanguageCode {
    name: "Mende",
    code: "",
    code_2t: "men",
    code_2b: "men",
    code_3: "men",
    individual_languages: &[
    ],
};


pub const MGA: LanguageCode = LanguageCode {
    name: "Middle Irish|Irish, Middle (900–1200)",
    code: "",
    code_2t: "mga",
    code_2b: "mga",
    code_3: "mga",
    individual_languages: &[
    ],
};


pub const MIC: LanguageCode = LanguageCode {
    name: "Mi'Mi'kmaq; Micmac",
    code: "",
    code_2t: "mic",
    code_2b: "mic",
    code_3: "mic",
    individual_languages: &[
    ],
};


pub const MIN: LanguageCode = LanguageCode {
    name: "Minangkabau",
    code: "",
    code_2t: "min",
    code_2b: "min",
    code_3: "min",
    individual_languages: &[
    ],
};


pub const MIS: LanguageCode = LanguageCode {
    name: "Uncoded languages",
    code: "",
    code_2t: "mis",
    code_2b: "mis",
    code_3: "mis",
    individual_languages: &[
    ],
};


pub const MKH: LanguageCode = LanguageCode {
    name: "Mon-Khmer languages",
    code: "",
    code_2t: "mkh",
    code_2b: "mkh",
    code_3: "",
    individual_languages: &[
    ],
};


pub const MNC: LanguageCode = LanguageCode {
    name: "Manchu",
    code: "",
    code_2t: "mnc",
    code_2b: "mnc",
    code_3: "mnc",
    individual_languages: &[
    ],
};


pub const MNI: LanguageCode = LanguageCode {
    name: "Manipuri",
    code: "",
    code_2t: "mni",
    code_2b: "mni",
    code_3: "mni",
    individual_languages: &[
    ],
};


pub const MNO: LanguageCode = LanguageCode {
    name: "Manobo languages",
    code: "",
    code_2t: "mno",
    code_2b: "mno",
    code_3: "",
    individual_languages: &[
    ],
};


pub const MOH: LanguageCode = LanguageCode {
    name: "Mohawk",
    code: "",
    code_2t: "moh",
    code_2b: "moh",
    code_3: "moh",
    individual_languages: &[
    ],
};


pub const MOS: LanguageCode = LanguageCode {
    name: "Mossi",
    code: "",
    code_2t: "mos",
    code_2b: "mos",
    code_3: "mos",
    individual_languages: &[
    ],
};


pub const MUL: LanguageCode = LanguageCode {
    name: "Multiple languages",
    code: "",
    code_2t: "mul",
    code_2b: "mul",
    code_3: "mul",
    individual_languages: &[
    ],
};


pub const MUN: LanguageCode = LanguageCode {
    name: "Munda languages",
    code: "",
    code_2t: "mun",
    code_2b: "mun",
    code_3: "",
    individual_languages: &[
    ],
};


pub const MUS: LanguageCode = LanguageCode {
    name: "Creek",
    code: "",
    code_2t: "mus",
    code_2b: "mus",
    code_3: "mus",
    individual_languages: &[
    ],
};


pub const MWL: LanguageCode = LanguageCode {
    name: "Mirandese",
    code: "",
    code_2t: "mwl",
    code_2b: "mwl",
    code_3: "mwl",
    individual_languages: &[
    ],
};


pub const MWR: LanguageCode = LanguageCode {
    name: "Marwari",
    code: "",
    code_2t: "mwr",
    code_2b: "mwr",
    code_3: "mwr",
    individual_languages: &[
        IndividualLanguages {
            name: "Dhundari",
            code: "dhd",
        },
        IndividualLanguages {
            name: "Mewari",
            code: "mtr",
        },
        IndividualLanguages {
            name: "Marwari (Pakistan)",
            code: "mve",
        },
        IndividualLanguages {
            name: "Marwari (India)",
            code: "rwr",
        },
        IndividualLanguages {
            name: "Shekhawati",
            code: "swv",
        },
        IndividualLanguages {
            name: "Merwari",
            code: "wry",
        },
    ],
};


pub const MYN: LanguageCode = LanguageCode {
    name: "Mayan languages",
    code: "",
    code_2t: "myn",
    code_2b: "myn",
    code_3: "",
    individual_languages: &[
    ],
};


pub const MYV: LanguageCode = LanguageCode {
    name: "Erzya",
    code: "",
    code_2t: "myv",
    code_2b: "myv",
    code_3: "myv",
    individual_languages: &[
    ],
};


pub const NAH: LanguageCode = LanguageCode {
    name: "Nahuatl languages",
    code: "",
    code_2t: "nah",
    code_2b: "nah",
    code_3: "",
    individual_languages: &[
    ],
};


pub const NAI: LanguageCode = LanguageCode {
    name: "North American Indian languages",
    code: "",
    code_2t: "nai",
    code_2b: "nai",
    code_3: "",
    individual_languages: &[
    ],
};


pub const NAP: LanguageCode = LanguageCode {
    name: "Neapolitan",
    code: "",
    code_2t: "nap",
    code_2b: "nap",
    code_3: "nap",
    individual_languages: &[
    ],
};


pub const NDS: LanguageCode = LanguageCode {
    name: "Low German; Low Saxon; German, Low; Saxon, Low",
    code: "",
    code_2t: "nds",
    code_2b: "nds",
    code_3: "nds",
    individual_languages: &[
    ],
};


pub const NEW: LanguageCode = LanguageCode {
    name: "Nepal Nepal Bhasa; Newari",
    code: "",
    code_2t: "new",
    code_2b: "new",
    code_3: "new",
    individual_languages: &[
    ],
};


pub const NIA: LanguageCode = LanguageCode {
    name: "Nias",
    code: "",
    code_2t: "nia",
    code_2b: "nia",
    code_3: "nia",
    individual_languages: &[
    ],
};


pub const NIC: LanguageCode = LanguageCode {
    name: "Niger-Kordofanian languages",
    code: "",
    code_2t: "nic",
    code_2b: "nic",
    code_3: "",
    individual_languages: &[
    ],
};


pub const NIU: LanguageCode = LanguageCode {
    name: "Niuean",
    code: "",
    code_2t: "niu",
    code_2b: "niu",
    code_3: "niu",
    individual_languages: &[
    ],
};


pub const NOG: LanguageCode = LanguageCode {
    name: "Nogai",
    code: "",
    code_2t: "nog",
    code_2b: "nog",
    code_3: "nog",
    individual_languages: &[
    ],
};


pub const NON: LanguageCode = LanguageCode {
    name: "Old Norse, Old",
    code: "",
    code_2t: "non",
    code_2b: "non",
    code_3: "non",
    individual_languages: &[
    ],
};


pub const NQO: LanguageCode = LanguageCode {
    name: "N'N'Ko",
    code: "",
    code_2t: "nqo",
    code_2b: "nqo",
    code_3: "nqo",
    individual_languages: &[
    ],
};


pub const NSO: LanguageCode = LanguageCode {
    name: "Pedi; Sepedi; Northern Northern Sotho",
    code: "",
    code_2t: "nso",
    code_2b: "nso",
    code_3: "nso",
    individual_languages: &[
    ],
};


pub const NUB: LanguageCode = LanguageCode {
    name: "Nubian languages",
    code: "",
    code_2t: "nub",
    code_2b: "nub",
    code_3: "",
    individual_languages: &[
    ],
};


pub const NWC: LanguageCode = LanguageCode {
    name: "Classical Classical&nbsp;Newari; Old Old&nbsp;Newari; Classical Nepal Classical&nbsp;Nepal&nbsp;Bhasa",
    code: "",
    code_2t: "nwc",
    code_2b: "nwc",
    code_3: "nwc",
    individual_languages: &[
    ],
};


pub const NYM: LanguageCode = LanguageCode {
    name: "Nyamwezi",
    code: "",
    code_2t: "nym",
    code_2b: "nym",
    code_3: "nym",
    individual_languages: &[
    ],
};


pub const NYN: LanguageCode = LanguageCode {
    name: "Nyankole",
    code: "",
    code_2t: "nyn",
    code_2b: "nyn",
    code_3: "nyn",
    individual_languages: &[
    ],
};


pub const NYO: LanguageCode = LanguageCode {
    name: "Nyoro",
    code: "",
    code_2t: "nyo",
    code_2b: "nyo",
    code_3: "nyo",
    individual_languages: &[
    ],
};


pub const NZI: LanguageCode = LanguageCode {
    name: "Nzima",
    code: "",
    code_2t: "nzi",
    code_2b: "nzi",
    code_3: "nzi",
    individual_languages: &[
    ],
};


pub const OSA: LanguageCode = LanguageCode {
    name: "Osage",
    code: "",
    code_2t: "osa",
    code_2b: "osa",
    code_3: "osa",
    individual_languages: &[
    ],
};


pub const OTA: LanguageCode = LanguageCode {
    name: "Ottoman Turkish, Ottoman (1500–1928)",
    code: "",
    code_2t: "ota",
    code_2b: "ota",
    code_3: "ota",
    individual_languages: &[
    ],
};


pub const OTO: LanguageCode = LanguageCode {
    name: "Otomian languages",
    code: "",
    code_2t: "oto",
    code_2b: "oto",
    code_3: "",
    individual_languages: &[
    ],
};


pub const PAA: LanguageCode = LanguageCode {
    name: "Papuan languages",
    code: "",
    code_2t: "paa",
    code_2b: "paa",
    code_3: "",
    individual_languages: &[
    ],
};


pub const PAG: LanguageCode = LanguageCode {
    name: "Pangasinan",
    code: "",
    code_2t: "pag",
    code_2b: "pag",
    code_3: "pag",
    individual_languages: &[
    ],
};


pub const PAL: LanguageCode = LanguageCode {
    name: "Pahlavi",
    code: "",
    code_2t: "pal",
    code_2b: "pal",
    code_3: "pal",
    individual_languages: &[
    ],
};


pub const PAM: LanguageCode = LanguageCode {
    name: "Pampanga; Kapampangan",
    code: "",
    code_2t: "pam",
    code_2b: "pam",
    code_3: "pam",
    individual_languages: &[
    ],
};


pub const PAP: LanguageCode = LanguageCode {
    name: "Papiamento",
    code: "",
    code_2t: "pap",
    code_2b: "pap",
    code_3: "pap",
    individual_languages: &[
    ],
};


pub const PAU: LanguageCode = LanguageCode {
    name: "Palauan",
    code: "",
    code_2t: "pau",
    code_2b: "pau",
    code_3: "pau",
    individual_languages: &[
    ],
};


pub const PEO: LanguageCode = LanguageCode {
    name: "Old Persian, Old (ca. 600–400 B.C.)",
    code: "",
    code_2t: "peo",
    code_2b: "peo",
    code_3: "peo",
    individual_languages: &[
    ],
};


pub const PHI: LanguageCode = LanguageCode {
    name: "Philippine languages",
    code: "",
    code_2t: "phi",
    code_2b: "phi",
    code_3: "",
    individual_languages: &[
    ],
};


pub const PHN: LanguageCode = LanguageCode {
    name: "Phoenician",
    code: "",
    code_2t: "phn",
    code_2b: "phn",
    code_3: "phn",
    individual_languages: &[
    ],
};


pub const PON: LanguageCode = LanguageCode {
    name: "Pohnpeian",
    code: "",
    code_2t: "pon",
    code_2b: "pon",
    code_3: "pon",
    individual_languages: &[
    ],
};


pub const PRA: LanguageCode = LanguageCode {
    name: "Prakrit languages",
    code: "",
    code_2t: "pra",
    code_2b: "pra",
    code_3: "",
    individual_languages: &[
    ],
};


pub const PRO: LanguageCode = LanguageCode {
    name: "Old Provençal, Old (to 1500); Old Old&nbsp;Occitan&nbsp;(to&nbsp;1500)",
    code: "",
    code_2t: "pro",
    code_2b: "pro",
    code_3: "pro",
    individual_languages: &[
    ],
};


pub const QAA: LanguageCode = LanguageCode {
    name: "Reserved for local use",
    code: "",
    code_2t: "qaa-qtz",
    code_2b: "qaa-qtz",
    code_3: "qaa-qtz",
    individual_languages: &[
    ],
};


pub const RAJ: LanguageCode = LanguageCode {
    name: "Rajasthani",
    code: "",
    code_2t: "raj",
    code_2b: "raj",
    code_3: "raj",
    individual_languages: &[
        IndividualLanguages {
            name: "Bagri",
            code: "bgq",
        },
        IndividualLanguages {
            name: "Gade Lohar",
            code: "gda",
        },
        IndividualLanguages {
            name: "Gujari",
            code: "gju",
        },
        IndividualLanguages {
            name: "Haroti",
            code: "hoj",
        },
        IndividualLanguages {
            name: "Malvi",
            code: "mup",
        },
        IndividualLanguages {
            name: "Wagdi",
            code: "wbr",
        },
    ],
};


pub const RAP: LanguageCode = LanguageCode {
    name: "Rapanui",
    code: "",
    code_2t: "rap",
    code_2b: "rap",
    code_3: "rap",
    individual_languages: &[
    ],
};


pub const RAR: LanguageCode = LanguageCode {
    name: "Rarotongan; Cook Islands Cook Islands Māori",
    code: "",
    code_2t: "rar",
    code_2b: "rar",
    code_3: "rar",
    individual_languages: &[
    ],
};


pub const ROA: LanguageCode = LanguageCode {
    name: "Romance languages",
    code: "",
    code_2t: "roa",
    code_2b: "roa",
    code_3: "",
    individual_languages: &[
    ],
};


pub const ROM: LanguageCode = LanguageCode {
    name: "Romany",
    code: "",
    code_2t: "rom",
    code_2b: "rom",
    code_3: "rom",
    individual_languages: &[
        IndividualLanguages {
            name: "Carpathian Romani",
            code: "rmc",
        },
        IndividualLanguages {
            name: "Kalo Finnish Romani",
            code: "rmf",
        },
        IndividualLanguages {
            name: "Baltic Romani",
            code: "rml",
        },
        IndividualLanguages {
            name: "Balkan Romani",
            code: "rmn",
        },
        IndividualLanguages {
            name: "Sinte Romani",
            code: "rmo",
        },
        IndividualLanguages {
            name: "Welsh Romani",
            code: "rmw",
        },
        IndividualLanguages {
            name: "Vlax Romani",
            code: "rmy",
        },
    ],
};


pub const RUP: LanguageCode = LanguageCode {
    name: "Aromanian; Arumanian; Macedo-Romanian{{efn|The term \"Macedo-Romanian\" might also be used for the Megleno-Romanians and Megleno-their language.<ref>{{cite journal|url=https://www.ceeol.com/search/article-detail?id=87227|title=Some topics of the traditional wedding customs of the Macedo–Romanians (Aromanians and Megleno–Romanians)|first=Emil|last=Țîrcomnicu|journal=Romanian Journal of Population Studies|issue=3|pages=141–152|year=2009|volume=3 }}</ref>}}",
    code: "",
    code_2t: "rup",
    code_2b: "rup",
    code_3: "rup",
    individual_languages: &[
    ],
};


pub const SAD: LanguageCode = LanguageCode {
    name: "Sandawe",
    code: "",
    code_2t: "sad",
    code_2b: "sad",
    code_3: "sad",
    individual_languages: &[
    ],
};


pub const SAH: LanguageCode = LanguageCode {
    name: "Yakut",
    code: "",
    code_2t: "sah",
    code_2b: "sah",
    code_3: "sah",
    individual_languages: &[
    ],
};


pub const SAI: LanguageCode = LanguageCode {
    name: "South American Indian languages",
    code: "",
    code_2t: "sai",
    code_2b: "sai",
    code_3: "",
    individual_languages: &[
    ],
};


pub const SAL: LanguageCode = LanguageCode {
    name: "Salishan languages",
    code: "",
    code_2t: "sal",
    code_2b: "sal",
    code_3: "",
    individual_languages: &[
    ],
};


pub const SAM: LanguageCode = LanguageCode {
    name: "Samaritan Samaritan Aramaic",
    code: "",
    code_2t: "sam",
    code_2b: "sam",
    code_3: "sam",
    individual_languages: &[
    ],
};


pub const SAS: LanguageCode = LanguageCode {
    name: "Sasak",
    code: "",
    code_2t: "sas",
    code_2b: "sas",
    code_3: "sas",
    individual_languages: &[
    ],
};


pub const SAT: LanguageCode = LanguageCode {
    name: "Santali",
    code: "",
    code_2t: "sat",
    code_2b: "sat",
    code_3: "sat",
    individual_languages: &[
    ],
};


pub const SCN: LanguageCode = LanguageCode {
    name: "Sicilian",
    code: "",
    code_2t: "scn",
    code_2b: "scn",
    code_3: "scn",
    individual_languages: &[
    ],
};


pub const SCO: LanguageCode = LanguageCode {
    name: "Scots",
    code: "",
    code_2t: "sco",
    code_2b: "sco",
    code_3: "sco",
    individual_languages: &[
    ],
};


pub const SEL: LanguageCode = LanguageCode {
    name: "Selkup",
    code: "",
    code_2t: "sel",
    code_2b: "sel",
    code_3: "sel",
    individual_languages: &[
    ],
};


pub const SEM: LanguageCode = LanguageCode {
    name: "Semitic languages",
    code: "",
    code_2t: "sem",
    code_2b: "sem",
    code_3: "",
    individual_languages: &[
    ],
};


pub const SGA: LanguageCode = LanguageCode {
    name: "Old Irish, Old (to 900)",
    code: "",
    code_2t: "sga",
    code_2b: "sga",
    code_3: "sga",
    individual_languages: &[
    ],
};


pub const SGN: LanguageCode = LanguageCode {
    name: "Sign Languages",
    code: "",
    code_2t: "sgn",
    code_2b: "sgn",
    code_3: "",
    individual_languages: &[
    ],
};


pub const SHN: LanguageCode = LanguageCode {
    name: "Shan",
    code: "",
    code_2t: "shn",
    code_2b: "shn",
    code_3: "shn",
    individual_languages: &[
    ],
};


pub const SID: LanguageCode = LanguageCode {
    name: "Sidamo",
    code: "",
    code_2t: "sid",
    code_2b: "sid",
    code_3: "sid",
    individual_languages: &[
    ],
};


pub const SIO: LanguageCode = LanguageCode {
    name: "Siouan languages",
    code: "",
    code_2t: "sio",
    code_2b: "sio",
    code_3: "",
    individual_languages: &[
    ],
};


pub const SIT: LanguageCode = LanguageCode {
    name: "Sino-Tibetan languages",
    code: "",
    code_2t: "sit",
    code_2b: "sit",
    code_3: "",
    individual_languages: &[
    ],
};


pub const SLA: LanguageCode = LanguageCode {
    name: "Slavic languages",
    code: "",
    code_2t: "sla",
    code_2b: "sla",
    code_3: "",
    individual_languages: &[
    ],
};


pub const SMA: LanguageCode = LanguageCode {
    name: "Southern Southern Sami",
    code: "",
    code_2t: "sma",
    code_2b: "sma",
    code_3: "sma",
    individual_languages: &[
    ],
};


pub const SMI: LanguageCode = LanguageCode {
    name: "Sami languages",
    code: "",
    code_2t: "smi",
    code_2b: "smi",
    code_3: "",
    individual_languages: &[
    ],
};


pub const SMJ: LanguageCode = LanguageCode {
    name: "Lule Lule Sami",
    code: "",
    code_2t: "smj",
    code_2b: "smj",
    code_3: "smj",
    individual_languages: &[
    ],
};


pub const SMN: LanguageCode = LanguageCode {
    name: "Inari Inari Sami",
    code: "",
    code_2t: "smn",
    code_2b: "smn",
    code_3: "smn",
    individual_languages: &[
    ],
};


pub const SMS: LanguageCode = LanguageCode {
    name: "Skolt Skolt Sami",
    code: "",
    code_2t: "sms",
    code_2b: "sms",
    code_3: "sms",
    individual_languages: &[
    ],
};


pub const SNK: LanguageCode = LanguageCode {
    name: "Soninke",
    code: "",
    code_2t: "snk",
    code_2b: "snk",
    code_3: "snk",
    individual_languages: &[
    ],
};


pub const SOG: LanguageCode = LanguageCode {
    name: "Sogdian",
    code: "",
    code_2t: "sog",
    code_2b: "sog",
    code_3: "sog",
    individual_languages: &[
    ],
};


pub const SON: LanguageCode = LanguageCode {
    name: "Songhai languages",
    code: "",
    code_2t: "son",
    code_2b: "son",
    code_3: "",
    individual_languages: &[
    ],
};


pub const SRN: LanguageCode = LanguageCode {
    name: "Sranan Sranan Tongo",
    code: "",
    code_2t: "srn",
    code_2b: "srn",
    code_3: "srn",
    individual_languages: &[
    ],
};


pub const SRR: LanguageCode = LanguageCode {
    name: "Serer",
    code: "",
    code_2t: "srr",
    code_2b: "srr",
    code_3: "srr",
    individual_languages: &[
    ],
};


pub const SSA: LanguageCode = LanguageCode {
    name: "Nilo-Saharan languages",
    code: "",
    code_2t: "ssa",
    code_2b: "ssa",
    code_3: "",
    individual_languages: &[
    ],
};


pub const SUK: LanguageCode = LanguageCode {
    name: "Sukuma",
    code: "",
    code_2t: "suk",
    code_2b: "suk",
    code_3: "suk",
    individual_languages: &[
    ],
};


pub const SUS: LanguageCode = LanguageCode {
    name: "Susu",
    code: "",
    code_2t: "sus",
    code_2b: "sus",
    code_3: "sus",
    individual_languages: &[
    ],
};


pub const SUX: LanguageCode = LanguageCode {
    name: "Sumerian",
    code: "",
    code_2t: "sux",
    code_2b: "sux",
    code_3: "sux",
    individual_languages: &[
    ],
};


pub const SYC: LanguageCode = LanguageCode {
    name: "Classical Classical Syriac",
    code: "",
    code_2t: "syc",
    code_2b: "syc",
    code_3: "syc",
    individual_languages: &[
    ],
};


pub const SYR: LanguageCode = LanguageCode {
    name: "Syriac",
    code: "",
    code_2t: "syr",
    code_2b: "syr",
    code_3: "syr",
    individual_languages: &[
        IndividualLanguages {
            name: "Assyrian Neo-Aramaic",
            code: "aii",
        },
        IndividualLanguages {
            name: "Chaldean Neo-Aramaic",
            code: "cld",
        },
    ],
};


pub const TAI: LanguageCode = LanguageCode {
    name: "Tai languages",
    code: "",
    code_2t: "tai",
    code_2b: "tai",
    code_3: "",
    individual_languages: &[
    ],
};


pub const TEM: LanguageCode = LanguageCode {
    name: "Timne",
    code: "",
    code_2t: "tem",
    code_2b: "tem",
    code_3: "tem",
    individual_languages: &[
    ],
};


pub const TER: LanguageCode = LanguageCode {
    name: "Tereno",
    code: "",
    code_2t: "ter",
    code_2b: "ter",
    code_3: "ter",
    individual_languages: &[
    ],
};


pub const TET: LanguageCode = LanguageCode {
    name: "Tetum",
    code: "",
    code_2t: "tet",
    code_2b: "tet",
    code_3: "tet",
    individual_languages: &[
    ],
};


pub const TIG: LanguageCode = LanguageCode {
    name: "Tigre",
    code: "",
    code_2t: "tig",
    code_2b: "tig",
    code_3: "tig",
    individual_languages: &[
    ],
};


pub const TIV: LanguageCode = LanguageCode {
    name: "Tiv",
    code: "",
    code_2t: "tiv",
    code_2b: "tiv",
    code_3: "tiv",
    individual_languages: &[
    ],
};


pub const TKL: LanguageCode = LanguageCode {
    name: "Tokelau",
    code: "",
    code_2t: "tkl",
    code_2b: "tkl",
    code_3: "tkl",
    individual_languages: &[
    ],
};


pub const TLH: LanguageCode = LanguageCode {
    name: "Klingon; tlhIngan-tlhIngan-Hol<!--starts with lowercase-->",
    code: "",
    code_2t: "tlh",
    code_2b: "tlh",
    code_3: "tlh",
    individual_languages: &[
    ],
};


pub const TLI: LanguageCode = LanguageCode {
    name: "Tlingit",
    code: "",
    code_2t: "tli",
    code_2b: "tli",
    code_3: "tli",
    individual_languages: &[
    ],
};


pub const TMH: LanguageCode = LanguageCode {
    name: "Tamashek",
    code: "",
    code_2t: "tmh",
    code_2b: "tmh",
    code_3: "tmh",
    individual_languages: &[
        IndividualLanguages {
            name: "Tamasheq",
            code: "taq",
        },
        IndividualLanguages {
            name: "Tahaggart Tamahaq",
            code: "thv",
        },
        IndividualLanguages {
            name: "Tayart Tamajeq",
            code: "thz",
        },
        IndividualLanguages {
            name: "Tawallammat Tamajaq",
            code: "ttq",
        },
    ],
};


pub const TOG: LanguageCode = LanguageCode {
    name: "Tonga language (Nyasa)|Tonga (Nyasa)",
    code: "",
    code_2t: "tog",
    code_2b: "tog",
    code_3: "tog",
    individual_languages: &[
    ],
};


pub const TPI: LanguageCode = LanguageCode {
    name: "Tok Tok Pisin",
    code: "",
    code_2t: "tpi",
    code_2b: "tpi",
    code_3: "tpi",
    individual_languages: &[
    ],
};


pub const TSI: LanguageCode = LanguageCode {
    name: "Tsimshian",
    code: "",
    code_2t: "tsi",
    code_2b: "tsi",
    code_3: "tsi",
    individual_languages: &[
    ],
};


pub const TUM: LanguageCode = LanguageCode {
    name: "Tumbuka",
    code: "",
    code_2t: "tum",
    code_2b: "tum",
    code_3: "tum",
    individual_languages: &[
    ],
};


pub const TUP: LanguageCode = LanguageCode {
    name: "Tupi languages",
    code: "",
    code_2t: "tup",
    code_2b: "tup",
    code_3: "",
    individual_languages: &[
    ],
};


pub const TUT: LanguageCode = LanguageCode {
    name: "Altaic languages",
    code: "",
    code_2t: "tut",
    code_2b: "tut",
    code_3: "",
    individual_languages: &[
    ],
};


pub const TVL: LanguageCode = LanguageCode {
    name: "Tuvalu",
    code: "",
    code_2t: "tvl",
    code_2b: "tvl",
    code_3: "tvl",
    individual_languages: &[
    ],
};


pub const TYV: LanguageCode = LanguageCode {
    name: "Tuvinian",
    code: "",
    code_2t: "tyv",
    code_2b: "tyv",
    code_3: "tyv",
    individual_languages: &[
    ],
};


pub const UDM: LanguageCode = LanguageCode {
    name: "Udmurt",
    code: "",
    code_2t: "udm",
    code_2b: "udm",
    code_3: "udm",
    individual_languages: &[
    ],
};


pub const UGA: LanguageCode = LanguageCode {
    name: "Ugaritic",
    code: "",
    code_2t: "uga",
    code_2b: "uga",
    code_3: "uga",
    individual_languages: &[
    ],
};


pub const UMB: LanguageCode = LanguageCode {
    name: "Umbundu",
    code: "",
    code_2t: "umb",
    code_2b: "umb",
    code_3: "umb",
    individual_languages: &[
    ],
};


pub const UND: LanguageCode = LanguageCode {
    name: "Undetermined",
    code: "",
    code_2t: "und",
    code_2b: "und",
    code_3: "und",
    individual_languages: &[
    ],
};


pub const VAI: LanguageCode = LanguageCode {
    name: "Vai",
    code: "",
    code_2t: "vai",
    code_2b: "vai",
    code_3: "vai",
    individual_languages: &[
    ],
};


pub const VOT: LanguageCode = LanguageCode {
    name: "Votic",
    code: "",
    code_2t: "vot",
    code_2b: "vot",
    code_3: "vot",
    individual_languages: &[
    ],
};


pub const WAK: LanguageCode = LanguageCode {
    name: "Wakashan languages",
    code: "",
    code_2t: "wak",
    code_2b: "wak",
    code_3: "",
    individual_languages: &[
    ],
};


pub const WAL: LanguageCode = LanguageCode {
    name: "Wolaitta; Wolaytta",
    code: "",
    code_2t: "wal",
    code_2b: "wal",
    code_3: "wal",
    individual_languages: &[
    ],
};


pub const WAR: LanguageCode = LanguageCode {
    name: "Waray",
    code: "",
    code_2t: "war",
    code_2b: "war",
    code_3: "war",
    individual_languages: &[
    ],
};


pub const WAS: LanguageCode = LanguageCode {
    name: "Washo",
    code: "",
    code_2t: "was",
    code_2b: "was",
    code_3: "was",
    individual_languages: &[
    ],
};


pub const WEN: LanguageCode = LanguageCode {
    name: "Sorbian languages",
    code: "",
    code_2t: "wen",
    code_2b: "wen",
    code_3: "",
    individual_languages: &[
    ],
};


pub const XAL: LanguageCode = LanguageCode {
    name: "Kalmyk; Oirat",
    code: "",
    code_2t: "xal",
    code_2b: "xal",
    code_3: "xal",
    individual_languages: &[
    ],
};


pub const YAO: LanguageCode = LanguageCode {
    name: "Yao",
    code: "",
    code_2t: "yao",
    code_2b: "yao",
    code_3: "yao",
    individual_languages: &[
    ],
};


pub const YAP: LanguageCode = LanguageCode {
    name: "Yapese",
    code: "",
    code_2t: "yap",
    code_2b: "yap",
    code_3: "yap",
    individual_languages: &[
    ],
};


pub const YPK: LanguageCode = LanguageCode {
    name: "Yupik languages",
    code: "",
    code_2t: "ypk",
    code_2b: "ypk",
    code_3: "",
    individual_languages: &[
    ],
};


pub const ZAP: LanguageCode = LanguageCode {
    name: "Zapotec",
    code: "",
    code_2t: "zap",
    code_2b: "zap",
    code_3: "zap",
    individual_languages: &[
        IndividualLanguages {
            name: "Sierra de Juárez Zapotec",
            code: "zaa",
        },
        IndividualLanguages {
            name: "Western Tlacolula Valley Zapotec",
            code: "zab",
        },
        IndividualLanguages {
            name: "Ocotlán Zapotec",
            code: "zac",
        },
        IndividualLanguages {
            name: "Cajonos Zapotec",
            code: "zad",
        },
        IndividualLanguages {
            name: "Yareni Zapotec",
            code: "zae",
        },
        IndividualLanguages {
            name: "Ayoquesco Zapotec",
            code: "zaf",
        },
        IndividualLanguages {
            name: "Isthmus Zapotec",
            code: "zai",
        },
        IndividualLanguages {
            name: "Miahuatlán Zapotec",
            code: "zam",
        },
        IndividualLanguages {
            name: "Ozolotepec Zapotec",
            code: "zao",
        },
        IndividualLanguages {
            name: "Aloápam Zapotec",
            code: "zaq",
        },
        IndividualLanguages {
            name: "Rincón Zapotec",
            code: "zar",
        },
        IndividualLanguages {
            name: "Santo Domingo Albarradas Zapotec",
            code: "zas",
        },
        IndividualLanguages {
            name: "Tabaa Zapotec",
            code: "zat",
        },
        IndividualLanguages {
            name: "Yatzachi Zapotec",
            code: "zav",
        },
        IndividualLanguages {
            name: "Mitla Zapotec",
            code: "zaw",
        },
        IndividualLanguages {
            name: "Xadani Zapotec",
            code: "zax",
        },
        IndividualLanguages {
            name: "Coatecas Altas Zapotec",
            code: "zca",
        },
        IndividualLanguages {
            name: "Las Delicias Zapotec",
            code: "zcd",
        },
        IndividualLanguages {
            name: "Asunción Mixtepec Zapotec",
            code: "zoo",
        },
        IndividualLanguages {
            name: "Lachiguiri Zapotec",
            code: "zpa",
        },
        IndividualLanguages {
            name: "Yautepec Zapotec",
            code: "zpb",
        },
        IndividualLanguages {
            name: "Choapan Zapotec",
            code: "zpc",
        },
        IndividualLanguages {
            name: "Southeastern Ixtlán Zapotec",
            code: "zpd",
        },
        IndividualLanguages {
            name: "Petapa Zapotec",
            code: "zpe",
        },
        IndividualLanguages {
            name: "San Pedro Quiatoni Zapotec",
            code: "zpf",
        },
        IndividualLanguages {
            name: "Guevea De Humboldt Zapotec",
            code: "zpg",
        },
        IndividualLanguages {
            name: "Totomachapan Zapotec",
            code: "zph",
        },
        IndividualLanguages {
            name: "Santa María Quiegolani Zapotec",
            code: "zpi",
        },
        IndividualLanguages {
            name: "Quiavicuzas Zapotec",
            code: "zpj",
        },
        IndividualLanguages {
            name: "Tlacolulita Zapotec",
            code: "zpk",
        },
        IndividualLanguages {
            name: "Lachixío Zapotec",
            code: "zpl",
        },
        IndividualLanguages {
            name: "Mixtepec Zapotec",
            code: "zpm",
        },
        IndividualLanguages {
            name: "Santa Inés Yatzechi Zapotec",
            code: "zpn",
        },
        IndividualLanguages {
            name: "Amatlán Zapotec",
            code: "zpo",
        },
        IndividualLanguages {
            name: "El Alto Zapotec",
            code: "zpp",
        },
        IndividualLanguages {
            name: "Zoogocho Zapotec",
            code: "zpq",
        },
        IndividualLanguages {
            name: "Santiago Xanica Zapotec",
            code: "zpr",
        },
        IndividualLanguages {
            name: "Coatlán Zapotec",
            code: "zps",
        },
        IndividualLanguages {
            name: "San Vicente Coatlán Zapotec",
            code: "zpt",
        },
        IndividualLanguages {
            name: "Yalálag Zapotec",
            code: "zpu",
        },
        IndividualLanguages {
            name: "Chichicapan Zapotec",
            code: "zpv",
        },
        IndividualLanguages {
            name: "Zaniza Zapotec",
            code: "zpw",
        },
        IndividualLanguages {
            name: "San Baltazar Loxicha Zapotec",
            code: "zpx",
        },
        IndividualLanguages {
            name: "Mazaltepec Zapotec",
            code: "zpy",
        },
        IndividualLanguages {
            name: "Texmelucan Zapotec",
            code: "zpz",
        },
        IndividualLanguages {
            name: "Southern Rincon Zapotec",
            code: "zsr",
        },
        IndividualLanguages {
            name: "",
            code: "ztc",
        },
        IndividualLanguages {
            name: "Elotepec Zapotec",
            code: "zte",
        },
        IndividualLanguages {
            name: "Xanaguía Zapotec",
            code: "ztg",
        },
        IndividualLanguages {
            name: "Lapaguía-Guivini Zapotec",
            code: "ztl",
        },
        IndividualLanguages {
            name: "San Agustín Mixtepec Zapotec",
            code: "ztm",
        },
        IndividualLanguages {
            name: "Santa Catarina Albarradas Zapotec",
            code: "ztn",
        },
        IndividualLanguages {
            name: "Loxicha Zapotec",
            code: "ztp",
        },
        IndividualLanguages {
            name: "Quioquitani-Quierí Zapotec",
            code: "ztq",
        },
        IndividualLanguages {
            name: "Tilquiapan Zapotec",
            code: "zts",
        },
        IndividualLanguages {
            name: "Tejalapan Zapotec",
            code: "ztt",
        },
        IndividualLanguages {
            name: "Güilá Zapotec",
            code: "ztu",
        },
        IndividualLanguages {
            name: "Zaachila Zapotec",
            code: "ztx",
        },
        IndividualLanguages {
            name: "Yatee Zapotec",
            code: "zty",
        },
    ],
};


pub const ZBL: LanguageCode = LanguageCode {
    name: "Blissymbols; Blissymbolics; Bliss",
    code: "",
    code_2t: "zbl",
    code_2b: "zbl",
    code_3: "zbl",
    individual_languages: &[
    ],
};


pub const ZEN: LanguageCode = LanguageCode {
    name: "Zenaga",
    code: "",
    code_2t: "zen",
    code_2b: "zen",
    code_3: "zen",
    individual_languages: &[
    ],
};


pub const ZGH: LanguageCode = LanguageCode {
    name: "Standard Moroccan Standard Moroccan Tamazight",
    code: "",
    code_2t: "zgh",
    code_2b: "zgh",
    code_3: "zgh",
    individual_languages: &[
    ],
};


pub const ZND: LanguageCode = LanguageCode {
    name: "Zande languages",
    code: "",
    code_2t: "znd",
    code_2b: "znd",
    code_3: "",
    individual_languages: &[
    ],
};


pub const ZUN: LanguageCode = LanguageCode {
    name: "Zuni",
    code: "",
    code_2t: "zun",
    code_2b: "zun",
    code_3: "zun",
    individual_languages: &[
    ],
};


pub const ZXX: LanguageCode = LanguageCode {
    name: "No linguistic content; Not applicable",
    code: "",
    code_2t: "zxx",
    code_2b: "zxx",
    code_3: "zxx",
    individual_languages: &[
    ],
};


pub const ZZA: LanguageCode = LanguageCode {
    name: "Zaza; Dimili; Dimli; Kirdki; Kirmanjki; Zazaki",
    code: "",
    code_2t: "zza",
    code_2b: "zza",
    code_3: "zza",
    individual_languages: &[
        IndividualLanguages {
            name: "Dimli (individual language)",
            code: "diq",
        },
        IndividualLanguages {
            name: "Kirmanjki (individual language)",
            code: "kiu",
        },
    ],
};


pub const AAA: LanguageCode = LanguageCode {
    name: "Ghotuo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aaa",
    individual_languages: &[
    ],
};


pub const AAB: LanguageCode = LanguageCode {
    name: "Alumu-Tesu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aab",
    individual_languages: &[
    ],
};


pub const AAC: LanguageCode = LanguageCode {
    name: "Ari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aac",
    individual_languages: &[
    ],
};


pub const AAD: LanguageCode = LanguageCode {
    name: "Amal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aad",
    individual_languages: &[
    ],
};


pub const AAE: LanguageCode = LanguageCode {
    name: "Arbëreshë Albanian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aae",
    individual_languages: &[
    ],
};


pub const AAF: LanguageCode = LanguageCode {
    name: "Aranadan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aaf",
    individual_languages: &[
    ],
};


pub const AAG: LanguageCode = LanguageCode {
    name: "Ambrak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aag",
    individual_languages: &[
    ],
};


pub const AAH: LanguageCode = LanguageCode {
    name: "Abu' Arapesh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aah",
    individual_languages: &[
    ],
};


pub const AAI: LanguageCode = LanguageCode {
    name: "Arifama-Miniafia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aai",
    individual_languages: &[
    ],
};


pub const AAK: LanguageCode = LanguageCode {
    name: "Ankave",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aak",
    individual_languages: &[
    ],
};


pub const AAL: LanguageCode = LanguageCode {
    name: "Afade",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aal",
    individual_languages: &[
    ],
};


pub const AAN: LanguageCode = LanguageCode {
    name: "Anambé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aan",
    individual_languages: &[
    ],
};


pub const AAO: LanguageCode = LanguageCode {
    name: "Algerian Saharan Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aao",
    individual_languages: &[
    ],
};


pub const AAP: LanguageCode = LanguageCode {
    name: "Pará Arára",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aap",
    individual_languages: &[
    ],
};


pub const AAQ: LanguageCode = LanguageCode {
    name: "Eastern Abnaki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aaq",
    individual_languages: &[
    ],
};


pub const AAS: LanguageCode = LanguageCode {
    name: "Aasáx",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aas",
    individual_languages: &[
    ],
};


pub const AAT: LanguageCode = LanguageCode {
    name: "Arvanitika Albanian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aat",
    individual_languages: &[
    ],
};


pub const AAU: LanguageCode = LanguageCode {
    name: "Abau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aau",
    individual_languages: &[
    ],
};


pub const AAW: LanguageCode = LanguageCode {
    name: "Solong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aaw",
    individual_languages: &[
    ],
};


pub const AAX: LanguageCode = LanguageCode {
    name: "Mandobo Atas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aax",
    individual_languages: &[
    ],
};


pub const AAZ: LanguageCode = LanguageCode {
    name: "Amarasi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aaz",
    individual_languages: &[
    ],
};


pub const ABA: LanguageCode = LanguageCode {
    name: "Abé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aba",
    individual_languages: &[
    ],
};


pub const ABB: LanguageCode = LanguageCode {
    name: "Bankon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abb",
    individual_languages: &[
    ],
};


pub const ABC: LanguageCode = LanguageCode {
    name: "Ambala Ayta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abc",
    individual_languages: &[
    ],
};


pub const ABD: LanguageCode = LanguageCode {
    name: "Manide",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abd",
    individual_languages: &[
    ],
};


pub const ABE: LanguageCode = LanguageCode {
    name: "Western Abnaki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abe",
    individual_languages: &[
    ],
};


pub const ABF: LanguageCode = LanguageCode {
    name: "Abai Sungai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abf",
    individual_languages: &[
    ],
};


pub const ABG: LanguageCode = LanguageCode {
    name: "Abaga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abg",
    individual_languages: &[
    ],
};


pub const ABH: LanguageCode = LanguageCode {
    name: "Tajiki Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abh",
    individual_languages: &[
    ],
};


pub const ABI: LanguageCode = LanguageCode {
    name: "Abidji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abi",
    individual_languages: &[
    ],
};


pub const ABJ: LanguageCode = LanguageCode {
    name: "Aka-Bea",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abj",
    individual_languages: &[
    ],
};


pub const ABL: LanguageCode = LanguageCode {
    name: "Lampung Nyo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abl",
    individual_languages: &[
    ],
};


pub const ABM: LanguageCode = LanguageCode {
    name: "Abanyom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abm",
    individual_languages: &[
    ],
};


pub const ABN: LanguageCode = LanguageCode {
    name: "Abua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abn",
    individual_languages: &[
    ],
};


pub const ABO: LanguageCode = LanguageCode {
    name: "Abon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abo",
    individual_languages: &[
    ],
};


pub const ABP: LanguageCode = LanguageCode {
    name: "Abellen Ayta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abp",
    individual_languages: &[
    ],
};


pub const ABQ: LanguageCode = LanguageCode {
    name: "Abaza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abq",
    individual_languages: &[
    ],
};


pub const ABR: LanguageCode = LanguageCode {
    name: "Abron",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abr",
    individual_languages: &[
    ],
};


pub const ABS: LanguageCode = LanguageCode {
    name: "Ambonese Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abs",
    individual_languages: &[
    ],
};


pub const ABT: LanguageCode = LanguageCode {
    name: "Ambulas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abt",
    individual_languages: &[
    ],
};


pub const ABU: LanguageCode = LanguageCode {
    name: "Abure",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abu",
    individual_languages: &[
    ],
};


pub const ABV: LanguageCode = LanguageCode {
    name: "Baharna Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abv",
    individual_languages: &[
    ],
};


pub const ABW: LanguageCode = LanguageCode {
    name: "Pal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abw",
    individual_languages: &[
    ],
};


pub const ABX: LanguageCode = LanguageCode {
    name: "Inabaknon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abx",
    individual_languages: &[
    ],
};


pub const ABY: LanguageCode = LanguageCode {
    name: "Aneme Wake",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aby",
    individual_languages: &[
    ],
};


pub const ABZ: LanguageCode = LanguageCode {
    name: "Abui",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "abz",
    individual_languages: &[
    ],
};


pub const ACA: LanguageCode = LanguageCode {
    name: "Achagua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aca",
    individual_languages: &[
    ],
};


pub const ACB: LanguageCode = LanguageCode {
    name: "Áncá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acb",
    individual_languages: &[
    ],
};


pub const ACD: LanguageCode = LanguageCode {
    name: "Gikyode",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acd",
    individual_languages: &[
    ],
};


pub const ACF: LanguageCode = LanguageCode {
    name: "Saint Lucian Creole French",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acf",
    individual_languages: &[
    ],
};


pub const ACI: LanguageCode = LanguageCode {
    name: "Aka-Cari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aci",
    individual_languages: &[
    ],
};


pub const ACK: LanguageCode = LanguageCode {
    name: "Aka-Kora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ack",
    individual_languages: &[
    ],
};


pub const ACL: LanguageCode = LanguageCode {
    name: "Akar-Bale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acl",
    individual_languages: &[
    ],
};


pub const ACM: LanguageCode = LanguageCode {
    name: "Mesopotamian Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acm",
    individual_languages: &[
    ],
};


pub const ACN: LanguageCode = LanguageCode {
    name: "Achang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acn",
    individual_languages: &[
    ],
};


pub const ACP: LanguageCode = LanguageCode {
    name: "Eastern Acipa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acp",
    individual_languages: &[
    ],
};


pub const ACQ: LanguageCode = LanguageCode {
    name: "Ta'izzi-Adeni Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acq",
    individual_languages: &[
    ],
};


pub const ACR: LanguageCode = LanguageCode {
    name: "Achi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acr",
    individual_languages: &[
    ],
};


pub const ACS: LanguageCode = LanguageCode {
    name: "Acroá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acs",
    individual_languages: &[
    ],
};


pub const ACT: LanguageCode = LanguageCode {
    name: "Achterhoeks",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "act",
    individual_languages: &[
    ],
};


pub const ACU: LanguageCode = LanguageCode {
    name: "Achuar-Shiwiar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acu",
    individual_languages: &[
    ],
};


pub const ACV: LanguageCode = LanguageCode {
    name: "Achumawi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acv",
    individual_languages: &[
    ],
};


pub const ACW: LanguageCode = LanguageCode {
    name: "Hijazi Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acw",
    individual_languages: &[
    ],
};


pub const ACX: LanguageCode = LanguageCode {
    name: "Omani Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acx",
    individual_languages: &[
    ],
};


pub const ACY: LanguageCode = LanguageCode {
    name: "Cypriot Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acy",
    individual_languages: &[
    ],
};


pub const ACZ: LanguageCode = LanguageCode {
    name: "Acheron",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "acz",
    individual_languages: &[
    ],
};


pub const ADB: LanguageCode = LanguageCode {
    name: "Atauran",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adb",
    individual_languages: &[
    ],
};


pub const ADD: LanguageCode = LanguageCode {
    name: "Lidzonka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "add",
    individual_languages: &[
    ],
};


pub const ADE: LanguageCode = LanguageCode {
    name: "Adele",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ade",
    individual_languages: &[
    ],
};


pub const ADF: LanguageCode = LanguageCode {
    name: "Dhofari Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adf",
    individual_languages: &[
    ],
};


pub const ADG: LanguageCode = LanguageCode {
    name: "Andegerebinha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adg",
    individual_languages: &[
    ],
};


pub const ADH: LanguageCode = LanguageCode {
    name: "Adhola",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adh",
    individual_languages: &[
    ],
};


pub const ADI: LanguageCode = LanguageCode {
    name: "Adi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adi",
    individual_languages: &[
    ],
};


pub const ADJ: LanguageCode = LanguageCode {
    name: "Adioukrou",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adj",
    individual_languages: &[
    ],
};


pub const ADL: LanguageCode = LanguageCode {
    name: "Galo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adl",
    individual_languages: &[
    ],
};


pub const ADN: LanguageCode = LanguageCode {
    name: "Adang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adn",
    individual_languages: &[
    ],
};


pub const ADO: LanguageCode = LanguageCode {
    name: "Abu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ado",
    individual_languages: &[
    ],
};


pub const ADQ: LanguageCode = LanguageCode {
    name: "Adangbe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adq",
    individual_languages: &[
    ],
};


pub const ADR: LanguageCode = LanguageCode {
    name: "Adonara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adr",
    individual_languages: &[
    ],
};


pub const ADS: LanguageCode = LanguageCode {
    name: "Adamorobe Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ads",
    individual_languages: &[
    ],
};


pub const ADT: LanguageCode = LanguageCode {
    name: "Adnyamathanha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adt",
    individual_languages: &[
    ],
};


pub const ADU: LanguageCode = LanguageCode {
    name: "Aduge",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adu",
    individual_languages: &[
    ],
};


pub const ADW: LanguageCode = LanguageCode {
    name: "Amundava",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adw",
    individual_languages: &[
    ],
};


pub const ADX: LanguageCode = LanguageCode {
    name: "Amdo Tibetan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adx",
    individual_languages: &[
    ],
};


pub const ADZ: LanguageCode = LanguageCode {
    name: "Adzera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "adz",
    individual_languages: &[
    ],
};


pub const AEA: LanguageCode = LanguageCode {
    name: "Areba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aea",
    individual_languages: &[
    ],
};


pub const AEB: LanguageCode = LanguageCode {
    name: "Tunisian Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aeb",
    individual_languages: &[
    ],
};


pub const AEC: LanguageCode = LanguageCode {
    name: "Saidi Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aec",
    individual_languages: &[
    ],
};


pub const AED: LanguageCode = LanguageCode {
    name: "Argentine Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aed",
    individual_languages: &[
    ],
};


pub const AEE: LanguageCode = LanguageCode {
    name: "Northeast Pashai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aee",
    individual_languages: &[
    ],
};


pub const AEK: LanguageCode = LanguageCode {
    name: "Haeke",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aek",
    individual_languages: &[
    ],
};


pub const AEL: LanguageCode = LanguageCode {
    name: "Ambele",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ael",
    individual_languages: &[
    ],
};


pub const AEM: LanguageCode = LanguageCode {
    name: "Arem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aem",
    individual_languages: &[
    ],
};


pub const AEN: LanguageCode = LanguageCode {
    name: "Armenian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aen",
    individual_languages: &[
    ],
};


pub const AEQ: LanguageCode = LanguageCode {
    name: "Aer",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aeq",
    individual_languages: &[
    ],
};


pub const AER: LanguageCode = LanguageCode {
    name: "Eastern Arrernte",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aer",
    individual_languages: &[
    ],
};


pub const AES: LanguageCode = LanguageCode {
    name: "Alsea",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aes",
    individual_languages: &[
    ],
};


pub const AEU: LanguageCode = LanguageCode {
    name: "Akeu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aeu",
    individual_languages: &[
    ],
};


pub const AEW: LanguageCode = LanguageCode {
    name: "Ambakich",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aew",
    individual_languages: &[
    ],
};


pub const AEY: LanguageCode = LanguageCode {
    name: "Amele",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aey",
    individual_languages: &[
    ],
};


pub const AEZ: LanguageCode = LanguageCode {
    name: "Aeka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aez",
    individual_languages: &[
    ],
};


pub const AFB: LanguageCode = LanguageCode {
    name: "Gulf Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "afb",
    individual_languages: &[
    ],
};


pub const AFD: LanguageCode = LanguageCode {
    name: "Andai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "afd",
    individual_languages: &[
    ],
};


pub const AFE: LanguageCode = LanguageCode {
    name: "Putukwam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "afe",
    individual_languages: &[
    ],
};


pub const AFG: LanguageCode = LanguageCode {
    name: "Afghan Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "afg",
    individual_languages: &[
    ],
};


pub const AFI: LanguageCode = LanguageCode {
    name: "Akrukay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "afi",
    individual_languages: &[
    ],
};


pub const AFK: LanguageCode = LanguageCode {
    name: "Nanubae",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "afk",
    individual_languages: &[
    ],
};


pub const AFN: LanguageCode = LanguageCode {
    name: "Defaka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "afn",
    individual_languages: &[
    ],
};


pub const AFO: LanguageCode = LanguageCode {
    name: "Eloyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "afo",
    individual_languages: &[
    ],
};


pub const AFP: LanguageCode = LanguageCode {
    name: "Tapei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "afp",
    individual_languages: &[
    ],
};


pub const AFS: LanguageCode = LanguageCode {
    name: "Afro-Seminole Creole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "afs",
    individual_languages: &[
    ],
};


pub const AFT: LanguageCode = LanguageCode {
    name: "Afitti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aft",
    individual_languages: &[
    ],
};


pub const AFU: LanguageCode = LanguageCode {
    name: "Awutu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "afu",
    individual_languages: &[
    ],
};


pub const AFZ: LanguageCode = LanguageCode {
    name: "Obokuitai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "afz",
    individual_languages: &[
    ],
};


pub const AGA: LanguageCode = LanguageCode {
    name: "Aguano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aga",
    individual_languages: &[
    ],
};


pub const AGB: LanguageCode = LanguageCode {
    name: "Legbo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agb",
    individual_languages: &[
    ],
};


pub const AGC: LanguageCode = LanguageCode {
    name: "Agatu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agc",
    individual_languages: &[
    ],
};


pub const AGD: LanguageCode = LanguageCode {
    name: "Agarabi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agd",
    individual_languages: &[
    ],
};


pub const AGE: LanguageCode = LanguageCode {
    name: "Angal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "age",
    individual_languages: &[
    ],
};


pub const AGF: LanguageCode = LanguageCode {
    name: "Arguni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agf",
    individual_languages: &[
    ],
};


pub const AGG: LanguageCode = LanguageCode {
    name: "Angor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agg",
    individual_languages: &[
    ],
};


pub const AGH: LanguageCode = LanguageCode {
    name: "Ngelima",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agh",
    individual_languages: &[
    ],
};


pub const AGI: LanguageCode = LanguageCode {
    name: "Agariya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agi",
    individual_languages: &[
    ],
};


pub const AGJ: LanguageCode = LanguageCode {
    name: "Argobba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agj",
    individual_languages: &[
    ],
};


pub const AGK: LanguageCode = LanguageCode {
    name: "Isarog Agta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agk",
    individual_languages: &[
    ],
};


pub const AGL: LanguageCode = LanguageCode {
    name: "Fembe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agl",
    individual_languages: &[
    ],
};


pub const AGM: LanguageCode = LanguageCode {
    name: "Angaataha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agm",
    individual_languages: &[
    ],
};


pub const AGN: LanguageCode = LanguageCode {
    name: "Agutaynen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agn",
    individual_languages: &[
    ],
};


pub const AGO: LanguageCode = LanguageCode {
    name: "Tainae",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ago",
    individual_languages: &[
    ],
};


pub const AGQ: LanguageCode = LanguageCode {
    name: "Aghem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agq",
    individual_languages: &[
    ],
};


pub const AGR: LanguageCode = LanguageCode {
    name: "Aguaruna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agr",
    individual_languages: &[
    ],
};


pub const AGS: LanguageCode = LanguageCode {
    name: "Esimbi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ags",
    individual_languages: &[
    ],
};


pub const AGT: LanguageCode = LanguageCode {
    name: "Central Cagayan Agta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agt",
    individual_languages: &[
    ],
};


pub const AGU: LanguageCode = LanguageCode {
    name: "Aguacateco",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agu",
    individual_languages: &[
    ],
};


pub const AGV: LanguageCode = LanguageCode {
    name: "Remontado Dumagat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agv",
    individual_languages: &[
    ],
};


pub const AGW: LanguageCode = LanguageCode {
    name: "Kahua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agw",
    individual_languages: &[
    ],
};


pub const AGX: LanguageCode = LanguageCode {
    name: "Aghul",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agx",
    individual_languages: &[
    ],
};


pub const AGY: LanguageCode = LanguageCode {
    name: "Southern Alta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agy",
    individual_languages: &[
    ],
};


pub const AGZ: LanguageCode = LanguageCode {
    name: "Mt. Iriga Agta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "agz",
    individual_languages: &[
    ],
};


pub const AHA: LanguageCode = LanguageCode {
    name: "Ahanta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aha",
    individual_languages: &[
    ],
};


pub const AHB: LanguageCode = LanguageCode {
    name: "Axamb",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ahb",
    individual_languages: &[
    ],
};


pub const AHG: LanguageCode = LanguageCode {
    name: "Qimant",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ahg",
    individual_languages: &[
    ],
};


pub const AHH: LanguageCode = LanguageCode {
    name: "Aghu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ahh",
    individual_languages: &[
    ],
};


pub const AHI: LanguageCode = LanguageCode {
    name: "Tiagbamrin Aizi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ahi",
    individual_languages: &[
    ],
};


pub const AHK: LanguageCode = LanguageCode {
    name: "Akha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ahk",
    individual_languages: &[
    ],
};


pub const AHL: LanguageCode = LanguageCode {
    name: "Igo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ahl",
    individual_languages: &[
    ],
};


pub const AHM: LanguageCode = LanguageCode {
    name: "Mobumrin Aizi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ahm",
    individual_languages: &[
    ],
};


pub const AHN: LanguageCode = LanguageCode {
    name: "Àhàn",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ahn",
    individual_languages: &[
    ],
};


pub const AHO: LanguageCode = LanguageCode {
    name: "Ahom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aho",
    individual_languages: &[
    ],
};


pub const AHP: LanguageCode = LanguageCode {
    name: "Aproumu Aizi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ahp",
    individual_languages: &[
    ],
};


pub const AHR: LanguageCode = LanguageCode {
    name: "Ahirani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ahr",
    individual_languages: &[
    ],
};


pub const AHS: LanguageCode = LanguageCode {
    name: "Ashe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ahs",
    individual_languages: &[
    ],
};


pub const AHT: LanguageCode = LanguageCode {
    name: "Ahtena",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aht",
    individual_languages: &[
    ],
};


pub const AIA: LanguageCode = LanguageCode {
    name: "Arosi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aia",
    individual_languages: &[
    ],
};


pub const AIB: LanguageCode = LanguageCode {
    name: "Ainu (China)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aib",
    individual_languages: &[
    ],
};


pub const AIC: LanguageCode = LanguageCode {
    name: "Ainbai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aic",
    individual_languages: &[
    ],
};


pub const AID: LanguageCode = LanguageCode {
    name: "Alngith",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aid",
    individual_languages: &[
    ],
};


pub const AIE: LanguageCode = LanguageCode {
    name: "Amara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aie",
    individual_languages: &[
    ],
};


pub const AIF: LanguageCode = LanguageCode {
    name: "Agi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aif",
    individual_languages: &[
    ],
};


pub const AIG: LanguageCode = LanguageCode {
    name: "Antigua and Barbuda Creole English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aig",
    individual_languages: &[
    ],
};


pub const AIH: LanguageCode = LanguageCode {
    name: "Ai-Cham",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aih",
    individual_languages: &[
    ],
};


pub const AII: LanguageCode = LanguageCode {
    name: "Assyrian Neo-Aramaic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aii",
    individual_languages: &[
    ],
};


pub const AIJ: LanguageCode = LanguageCode {
    name: "Lishanid Noshan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aij",
    individual_languages: &[
    ],
};


pub const AIK: LanguageCode = LanguageCode {
    name: "Ake",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aik",
    individual_languages: &[
    ],
};


pub const AIL: LanguageCode = LanguageCode {
    name: "Aimele",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ail",
    individual_languages: &[
    ],
};


pub const AIM: LanguageCode = LanguageCode {
    name: "Aimol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aim",
    individual_languages: &[
    ],
};


pub const AIO: LanguageCode = LanguageCode {
    name: "Aiton",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aio",
    individual_languages: &[
    ],
};


pub const AIP: LanguageCode = LanguageCode {
    name: "Burumakok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aip",
    individual_languages: &[
    ],
};


pub const AIQ: LanguageCode = LanguageCode {
    name: "Aimaq",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aiq",
    individual_languages: &[
    ],
};


pub const AIR: LanguageCode = LanguageCode {
    name: "Airoran",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "air",
    individual_languages: &[
    ],
};


pub const AIT: LanguageCode = LanguageCode {
    name: "Arikem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ait",
    individual_languages: &[
    ],
};


pub const AIW: LanguageCode = LanguageCode {
    name: "Aari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aiw",
    individual_languages: &[
    ],
};


pub const AIX: LanguageCode = LanguageCode {
    name: "Aighon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aix",
    individual_languages: &[
    ],
};


pub const AIY: LanguageCode = LanguageCode {
    name: "Ali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aiy",
    individual_languages: &[
    ],
};


pub const AJA: LanguageCode = LanguageCode {
    name: "Aja (South Sudan)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aja",
    individual_languages: &[
    ],
};


pub const AJG: LanguageCode = LanguageCode {
    name: "Aja (Benin)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ajg",
    individual_languages: &[
    ],
};


pub const AJI: LanguageCode = LanguageCode {
    name: "Ajië",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aji",
    individual_languages: &[
    ],
};


pub const AJN: LanguageCode = LanguageCode {
    name: "Andajin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ajn",
    individual_languages: &[
    ],
};


pub const AJP: LanguageCode = LanguageCode {
    name: "South Levantine Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ajp",
    individual_languages: &[
    ],
};


pub const AJS: LanguageCode = LanguageCode {
    name: "Algerian Jewish Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ajs",
    individual_languages: &[
    ],
};


pub const AJU: LanguageCode = LanguageCode {
    name: "Judeo-Moroccan Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aju",
    individual_languages: &[
    ],
};


pub const AJW: LanguageCode = LanguageCode {
    name: "Ajawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ajw",
    individual_languages: &[
    ],
};


pub const AJZ: LanguageCode = LanguageCode {
    name: "Amri Karbi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ajz",
    individual_languages: &[
    ],
};


pub const AKB: LanguageCode = LanguageCode {
    name: "Batak Angkola",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akb",
    individual_languages: &[
    ],
};


pub const AKC: LanguageCode = LanguageCode {
    name: "Mpur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akc",
    individual_languages: &[
    ],
};


pub const AKD: LanguageCode = LanguageCode {
    name: "Ukpet-Ehom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akd",
    individual_languages: &[
    ],
};


pub const AKE: LanguageCode = LanguageCode {
    name: "Akawaio",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ake",
    individual_languages: &[
    ],
};


pub const AKF: LanguageCode = LanguageCode {
    name: "Akpa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akf",
    individual_languages: &[
    ],
};


pub const AKG: LanguageCode = LanguageCode {
    name: "Anakalangu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akg",
    individual_languages: &[
    ],
};


pub const AKH: LanguageCode = LanguageCode {
    name: "Angal Heneng",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akh",
    individual_languages: &[
    ],
};


pub const AKI: LanguageCode = LanguageCode {
    name: "Aiome",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aki",
    individual_languages: &[
    ],
};


pub const AKJ: LanguageCode = LanguageCode {
    name: "Aka-Jeru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akj",
    individual_languages: &[
    ],
};


pub const AKL: LanguageCode = LanguageCode {
    name: "Aklanon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akl",
    individual_languages: &[
    ],
};


pub const AKM: LanguageCode = LanguageCode {
    name: "Aka-Bo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akm",
    individual_languages: &[
    ],
};


pub const AKO: LanguageCode = LanguageCode {
    name: "Akurio",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ako",
    individual_languages: &[
    ],
};


pub const AKP: LanguageCode = LanguageCode {
    name: "Siwu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akp",
    individual_languages: &[
    ],
};


pub const AKQ: LanguageCode = LanguageCode {
    name: "Ak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akq",
    individual_languages: &[
    ],
};


pub const AKR: LanguageCode = LanguageCode {
    name: "Araki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akr",
    individual_languages: &[
    ],
};


pub const AKS: LanguageCode = LanguageCode {
    name: "Akaselem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aks",
    individual_languages: &[
    ],
};


pub const AKT: LanguageCode = LanguageCode {
    name: "Akolet",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akt",
    individual_languages: &[
    ],
};


pub const AKU: LanguageCode = LanguageCode {
    name: "Akum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aku",
    individual_languages: &[
    ],
};


pub const AKV: LanguageCode = LanguageCode {
    name: "Akhvakh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akv",
    individual_languages: &[
    ],
};


pub const AKW: LanguageCode = LanguageCode {
    name: "Akwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akw",
    individual_languages: &[
    ],
};


pub const AKX: LanguageCode = LanguageCode {
    name: "Aka-Kede",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akx",
    individual_languages: &[
    ],
};


pub const AKY: LanguageCode = LanguageCode {
    name: "Aka-Kol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aky",
    individual_languages: &[
    ],
};


pub const AKZ: LanguageCode = LanguageCode {
    name: "Alabama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "akz",
    individual_languages: &[
    ],
};


pub const ALA: LanguageCode = LanguageCode {
    name: "Alago",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ala",
    individual_languages: &[
    ],
};


pub const ALC: LanguageCode = LanguageCode {
    name: "Qawasqar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alc",
    individual_languages: &[
    ],
};


pub const ALD: LanguageCode = LanguageCode {
    name: "Alladian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ald",
    individual_languages: &[
    ],
};


pub const ALF: LanguageCode = LanguageCode {
    name: "Alege",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alf",
    individual_languages: &[
    ],
};


pub const ALH: LanguageCode = LanguageCode {
    name: "Alawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alh",
    individual_languages: &[
    ],
};


pub const ALI: LanguageCode = LanguageCode {
    name: "Amaimon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ali",
    individual_languages: &[
    ],
};


pub const ALJ: LanguageCode = LanguageCode {
    name: "Alangan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alj",
    individual_languages: &[
    ],
};


pub const ALK: LanguageCode = LanguageCode {
    name: "Alak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alk",
    individual_languages: &[
    ],
};


pub const ALL: LanguageCode = LanguageCode {
    name: "Allar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "all",
    individual_languages: &[
    ],
};


pub const ALM: LanguageCode = LanguageCode {
    name: "Amblong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alm",
    individual_languages: &[
    ],
};


pub const ALN: LanguageCode = LanguageCode {
    name: "Gheg Albanian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aln",
    individual_languages: &[
    ],
};


pub const ALO: LanguageCode = LanguageCode {
    name: "Larike-Wakasihu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alo",
    individual_languages: &[
    ],
};


pub const ALP: LanguageCode = LanguageCode {
    name: "Alune",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alp",
    individual_languages: &[
    ],
};


pub const ALQ: LanguageCode = LanguageCode {
    name: "Algonquin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alq",
    individual_languages: &[
    ],
};


pub const ALR: LanguageCode = LanguageCode {
    name: "Alutor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alr",
    individual_languages: &[
    ],
};


pub const ALS: LanguageCode = LanguageCode {
    name: "Tosk Albanian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "als",
    individual_languages: &[
    ],
};


pub const ALU: LanguageCode = LanguageCode {
    name: "'Are'are",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alu",
    individual_languages: &[
    ],
};


pub const ALW: LanguageCode = LanguageCode {
    name: "Alaba-K’abeena",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alw",
    individual_languages: &[
    ],
};


pub const ALX: LanguageCode = LanguageCode {
    name: "Amol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alx",
    individual_languages: &[
    ],
};


pub const ALY: LanguageCode = LanguageCode {
    name: "Alyawarr",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aly",
    individual_languages: &[
    ],
};


pub const ALZ: LanguageCode = LanguageCode {
    name: "Alur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "alz",
    individual_languages: &[
    ],
};


pub const AMA: LanguageCode = LanguageCode {
    name: "Amanayé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ama",
    individual_languages: &[
    ],
};


pub const AMB: LanguageCode = LanguageCode {
    name: "Ambo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amb",
    individual_languages: &[
    ],
};


pub const AMC: LanguageCode = LanguageCode {
    name: "Amahuaca",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amc",
    individual_languages: &[
    ],
};


pub const AME: LanguageCode = LanguageCode {
    name: "Yanesha'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ame",
    individual_languages: &[
    ],
};


pub const AMF: LanguageCode = LanguageCode {
    name: "Hamer-Banna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amf",
    individual_languages: &[
    ],
};


pub const AMG: LanguageCode = LanguageCode {
    name: "Amurdak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amg",
    individual_languages: &[
    ],
};


pub const AMI: LanguageCode = LanguageCode {
    name: "Amis",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ami",
    individual_languages: &[
    ],
};


pub const AMJ: LanguageCode = LanguageCode {
    name: "Amdang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amj",
    individual_languages: &[
    ],
};


pub const AMK: LanguageCode = LanguageCode {
    name: "Ambai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amk",
    individual_languages: &[
    ],
};


pub const AML: LanguageCode = LanguageCode {
    name: "War-Jaintia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aml",
    individual_languages: &[
    ],
};


pub const AMM: LanguageCode = LanguageCode {
    name: "Ama (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amm",
    individual_languages: &[
    ],
};


pub const AMN: LanguageCode = LanguageCode {
    name: "Amanab",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amn",
    individual_languages: &[
    ],
};


pub const AMO: LanguageCode = LanguageCode {
    name: "Amo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amo",
    individual_languages: &[
    ],
};


pub const AMP: LanguageCode = LanguageCode {
    name: "Alamblak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amp",
    individual_languages: &[
    ],
};


pub const AMQ: LanguageCode = LanguageCode {
    name: "Amahai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amq",
    individual_languages: &[
    ],
};


pub const AMR: LanguageCode = LanguageCode {
    name: "Amarakaeri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amr",
    individual_languages: &[
    ],
};


pub const AMS: LanguageCode = LanguageCode {
    name: "Southern Amami-Oshima",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ams",
    individual_languages: &[
    ],
};


pub const AMT: LanguageCode = LanguageCode {
    name: "Amto",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amt",
    individual_languages: &[
    ],
};


pub const AMU: LanguageCode = LanguageCode {
    name: "Guerrero Amuzgo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amu",
    individual_languages: &[
    ],
};


pub const AMV: LanguageCode = LanguageCode {
    name: "Ambelau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amv",
    individual_languages: &[
    ],
};


pub const AMW: LanguageCode = LanguageCode {
    name: "Western Neo-Aramaic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amw",
    individual_languages: &[
    ],
};


pub const AMX: LanguageCode = LanguageCode {
    name: "Anmatyerre",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amx",
    individual_languages: &[
    ],
};


pub const AMY: LanguageCode = LanguageCode {
    name: "Ami",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amy",
    individual_languages: &[
    ],
};


pub const AMZ: LanguageCode = LanguageCode {
    name: "Atampaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "amz",
    individual_languages: &[
    ],
};


pub const ANA: LanguageCode = LanguageCode {
    name: "Andaqui",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ana",
    individual_languages: &[
    ],
};


pub const ANB: LanguageCode = LanguageCode {
    name: "Andoa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anb",
    individual_languages: &[
    ],
};


pub const ANC: LanguageCode = LanguageCode {
    name: "Ngas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anc",
    individual_languages: &[
    ],
};


pub const AND: LanguageCode = LanguageCode {
    name: "Ansus",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "and",
    individual_languages: &[
    ],
};


pub const ANE: LanguageCode = LanguageCode {
    name: "Xârâcùù",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ane",
    individual_languages: &[
    ],
};


pub const ANF: LanguageCode = LanguageCode {
    name: "Animere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anf",
    individual_languages: &[
    ],
};


pub const ANH: LanguageCode = LanguageCode {
    name: "Nend",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anh",
    individual_languages: &[
    ],
};


pub const ANI: LanguageCode = LanguageCode {
    name: "Andi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ani",
    individual_languages: &[
    ],
};


pub const ANJ: LanguageCode = LanguageCode {
    name: "Anor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anj",
    individual_languages: &[
    ],
};


pub const ANK: LanguageCode = LanguageCode {
    name: "Goemai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ank",
    individual_languages: &[
    ],
};


pub const ANL: LanguageCode = LanguageCode {
    name: "Anu-Hkongso Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anl",
    individual_languages: &[
    ],
};


pub const ANM: LanguageCode = LanguageCode {
    name: "Anal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anm",
    individual_languages: &[
    ],
};


pub const ANN: LanguageCode = LanguageCode {
    name: "Obolo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ann",
    individual_languages: &[
    ],
};


pub const ANO: LanguageCode = LanguageCode {
    name: "Andoque",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ano",
    individual_languages: &[
    ],
};


pub const ANQ: LanguageCode = LanguageCode {
    name: "Jarawa (India)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anq",
    individual_languages: &[
    ],
};


pub const ANR: LanguageCode = LanguageCode {
    name: "Andh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anr",
    individual_languages: &[
    ],
};


pub const ANS: LanguageCode = LanguageCode {
    name: "Anserma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ans",
    individual_languages: &[
    ],
};


pub const ANT: LanguageCode = LanguageCode {
    name: "Antakarinya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ant",
    individual_languages: &[
    ],
};


pub const ANU: LanguageCode = LanguageCode {
    name: "Anuak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anu",
    individual_languages: &[
    ],
};


pub const ANV: LanguageCode = LanguageCode {
    name: "Denya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anv",
    individual_languages: &[
    ],
};


pub const ANW: LanguageCode = LanguageCode {
    name: "Anaang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anw",
    individual_languages: &[
    ],
};


pub const ANX: LanguageCode = LanguageCode {
    name: "Andra-Hus",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anx",
    individual_languages: &[
    ],
};


pub const ANY: LanguageCode = LanguageCode {
    name: "Anyin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "any",
    individual_languages: &[
    ],
};


pub const ANZ: LanguageCode = LanguageCode {
    name: "Anem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "anz",
    individual_languages: &[
    ],
};


pub const AOA: LanguageCode = LanguageCode {
    name: "Angolar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aoa",
    individual_languages: &[
    ],
};


pub const AOB: LanguageCode = LanguageCode {
    name: "Abom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aob",
    individual_languages: &[
    ],
};


pub const AOC: LanguageCode = LanguageCode {
    name: "Pemon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aoc",
    individual_languages: &[
    ],
};


pub const AOD: LanguageCode = LanguageCode {
    name: "Andarum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aod",
    individual_languages: &[
    ],
};


pub const AOE: LanguageCode = LanguageCode {
    name: "Angal Enen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aoe",
    individual_languages: &[
    ],
};


pub const AOF: LanguageCode = LanguageCode {
    name: "Bragat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aof",
    individual_languages: &[
    ],
};


pub const AOG: LanguageCode = LanguageCode {
    name: "Angoram",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aog",
    individual_languages: &[
    ],
};


pub const AOI: LanguageCode = LanguageCode {
    name: "Anindilyakwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aoi",
    individual_languages: &[
    ],
};


pub const AOJ: LanguageCode = LanguageCode {
    name: "Mufian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aoj",
    individual_languages: &[
    ],
};


pub const AOK: LanguageCode = LanguageCode {
    name: "Arhö",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aok",
    individual_languages: &[
    ],
};


pub const AOL: LanguageCode = LanguageCode {
    name: "Alor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aol",
    individual_languages: &[
    ],
};


pub const AOM: LanguageCode = LanguageCode {
    name: "Ömie",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aom",
    individual_languages: &[
    ],
};


pub const AON: LanguageCode = LanguageCode {
    name: "Bumbita Arapesh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aon",
    individual_languages: &[
    ],
};


pub const AOR: LanguageCode = LanguageCode {
    name: "Aore",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aor",
    individual_languages: &[
    ],
};


pub const AOS: LanguageCode = LanguageCode {
    name: "Taikat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aos",
    individual_languages: &[
    ],
};


pub const AOT: LanguageCode = LanguageCode {
    name: "Atong (India)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aot",
    individual_languages: &[
    ],
};


pub const AOU: LanguageCode = LanguageCode {
    name: "A'ou",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aou",
    individual_languages: &[
    ],
};


pub const AOX: LanguageCode = LanguageCode {
    name: "Atorada",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aox",
    individual_languages: &[
    ],
};


pub const AOZ: LanguageCode = LanguageCode {
    name: "Uab Meto",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aoz",
    individual_languages: &[
    ],
};


pub const APB: LanguageCode = LanguageCode {
    name: "Sa'a",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apb",
    individual_languages: &[
    ],
};


pub const APC: LanguageCode = LanguageCode {
    name: "North Levantine Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apc",
    individual_languages: &[
    ],
};


pub const APD: LanguageCode = LanguageCode {
    name: "Sudanese Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apd",
    individual_languages: &[
    ],
};


pub const APE: LanguageCode = LanguageCode {
    name: "Bukiyip",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ape",
    individual_languages: &[
    ],
};


pub const APF: LanguageCode = LanguageCode {
    name: "Pahanan Agta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apf",
    individual_languages: &[
    ],
};


pub const APG: LanguageCode = LanguageCode {
    name: "Ampanang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apg",
    individual_languages: &[
    ],
};


pub const APH: LanguageCode = LanguageCode {
    name: "Athpariya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aph",
    individual_languages: &[
    ],
};


pub const API: LanguageCode = LanguageCode {
    name: "Apiaká",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "api",
    individual_languages: &[
    ],
};


pub const APJ: LanguageCode = LanguageCode {
    name: "Jicarilla Apache",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apj",
    individual_languages: &[
    ],
};


pub const APK: LanguageCode = LanguageCode {
    name: "Kiowa Apache",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apk",
    individual_languages: &[
    ],
};


pub const APL: LanguageCode = LanguageCode {
    name: "Lipan Apache",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apl",
    individual_languages: &[
    ],
};


pub const APM: LanguageCode = LanguageCode {
    name: "Mescalero-Chiricahua Apache",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apm",
    individual_languages: &[
    ],
};


pub const APN: LanguageCode = LanguageCode {
    name: "Apinayé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apn",
    individual_languages: &[
    ],
};


pub const APO: LanguageCode = LanguageCode {
    name: "Ambul",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apo",
    individual_languages: &[
    ],
};


pub const APP: LanguageCode = LanguageCode {
    name: "Apma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "app",
    individual_languages: &[
    ],
};


pub const APQ: LanguageCode = LanguageCode {
    name: "A-Pucikwar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apq",
    individual_languages: &[
    ],
};


pub const APR: LanguageCode = LanguageCode {
    name: "Arop-Lokep",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apr",
    individual_languages: &[
    ],
};


pub const APS: LanguageCode = LanguageCode {
    name: "Arop-Sissano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aps",
    individual_languages: &[
    ],
};


pub const APT: LanguageCode = LanguageCode {
    name: "Apatani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apt",
    individual_languages: &[
    ],
};


pub const APU: LanguageCode = LanguageCode {
    name: "Apurinã",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apu",
    individual_languages: &[
    ],
};


pub const APV: LanguageCode = LanguageCode {
    name: "Alapmunte",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apv",
    individual_languages: &[
    ],
};


pub const APW: LanguageCode = LanguageCode {
    name: "Western Apache",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apw",
    individual_languages: &[
    ],
};


pub const APX: LanguageCode = LanguageCode {
    name: "Aputai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apx",
    individual_languages: &[
    ],
};


pub const APY: LanguageCode = LanguageCode {
    name: "Apalaí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apy",
    individual_languages: &[
    ],
};


pub const APZ: LanguageCode = LanguageCode {
    name: "Safeyoka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "apz",
    individual_languages: &[
    ],
};


pub const AQC: LanguageCode = LanguageCode {
    name: "Archi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aqc",
    individual_languages: &[
    ],
};


pub const AQD: LanguageCode = LanguageCode {
    name: "Ampari Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aqd",
    individual_languages: &[
    ],
};


pub const AQG: LanguageCode = LanguageCode {
    name: "Arigidi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aqg",
    individual_languages: &[
    ],
};


pub const AQK: LanguageCode = LanguageCode {
    name: "Aninka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aqk",
    individual_languages: &[
    ],
};


pub const AQM: LanguageCode = LanguageCode {
    name: "Atohwaim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aqm",
    individual_languages: &[
    ],
};


pub const AQN: LanguageCode = LanguageCode {
    name: "Northern Alta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aqn",
    individual_languages: &[
    ],
};


pub const AQP: LanguageCode = LanguageCode {
    name: "Atakapa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aqp",
    individual_languages: &[
    ],
};


pub const AQR: LanguageCode = LanguageCode {
    name: "Arhâ",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aqr",
    individual_languages: &[
    ],
};


pub const AQT: LanguageCode = LanguageCode {
    name: "Angaité",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aqt",
    individual_languages: &[
    ],
};


pub const AQZ: LanguageCode = LanguageCode {
    name: "Akuntsu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aqz",
    individual_languages: &[
    ],
};


pub const ARB: LanguageCode = LanguageCode {
    name: "Standard Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "arb",
    individual_languages: &[
    ],
};


pub const ARD: LanguageCode = LanguageCode {
    name: "Arabana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ard",
    individual_languages: &[
    ],
};


pub const ARE: LanguageCode = LanguageCode {
    name: "Western Arrarnta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "are",
    individual_languages: &[
    ],
};


pub const ARH: LanguageCode = LanguageCode {
    name: "Arhuaco",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "arh",
    individual_languages: &[
    ],
};


pub const ARI: LanguageCode = LanguageCode {
    name: "Arikara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ari",
    individual_languages: &[
    ],
};


pub const ARJ: LanguageCode = LanguageCode {
    name: "Arapaso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "arj",
    individual_languages: &[
    ],
};


pub const ARK: LanguageCode = LanguageCode {
    name: "Arikapú",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ark",
    individual_languages: &[
    ],
};


pub const ARL: LanguageCode = LanguageCode {
    name: "Arabela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "arl",
    individual_languages: &[
    ],
};


pub const ARO: LanguageCode = LanguageCode {
    name: "Araona",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aro",
    individual_languages: &[
    ],
};


pub const ARQ: LanguageCode = LanguageCode {
    name: "Algerian Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "arq",
    individual_languages: &[
    ],
};


pub const ARR: LanguageCode = LanguageCode {
    name: "Karo (Brazil)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "arr",
    individual_languages: &[
    ],
};


pub const ARS: LanguageCode = LanguageCode {
    name: "Najdi Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ars",
    individual_languages: &[
    ],
};


pub const ARU: LanguageCode = LanguageCode {
    name: "Aruá (Amazonas State)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aru",
    individual_languages: &[
    ],
};


pub const ARV: LanguageCode = LanguageCode {
    name: "Arbore",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "arv",
    individual_languages: &[
    ],
};


pub const ARX: LanguageCode = LanguageCode {
    name: "Aruá (Rodonia State)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "arx",
    individual_languages: &[
    ],
};


pub const ARY: LanguageCode = LanguageCode {
    name: "Moroccan Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ary",
    individual_languages: &[
    ],
};


pub const ARZ: LanguageCode = LanguageCode {
    name: "Egyptian Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "arz",
    individual_languages: &[
    ],
};


pub const ASA: LanguageCode = LanguageCode {
    name: "Asu (Tanzania)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asa",
    individual_languages: &[
    ],
};


pub const ASB: LanguageCode = LanguageCode {
    name: "Assiniboine",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asb",
    individual_languages: &[
    ],
};


pub const ASC: LanguageCode = LanguageCode {
    name: "Casuarina Coast Asmat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asc",
    individual_languages: &[
    ],
};


pub const ASE: LanguageCode = LanguageCode {
    name: "American Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ase",
    individual_languages: &[
    ],
};


pub const ASF: LanguageCode = LanguageCode {
    name: "Auslan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asf",
    individual_languages: &[
    ],
};


pub const ASG: LanguageCode = LanguageCode {
    name: "Cishingini",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asg",
    individual_languages: &[
    ],
};


pub const ASH: LanguageCode = LanguageCode {
    name: "Abishira",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ash",
    individual_languages: &[
    ],
};


pub const ASI: LanguageCode = LanguageCode {
    name: "Buruwai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asi",
    individual_languages: &[
    ],
};


pub const ASJ: LanguageCode = LanguageCode {
    name: "Sari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asj",
    individual_languages: &[
    ],
};


pub const ASK: LanguageCode = LanguageCode {
    name: "Ashkun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ask",
    individual_languages: &[
    ],
};


pub const ASL: LanguageCode = LanguageCode {
    name: "Asilulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asl",
    individual_languages: &[
    ],
};


pub const ASN: LanguageCode = LanguageCode {
    name: "Xingú Asuriní",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asn",
    individual_languages: &[
    ],
};


pub const ASO: LanguageCode = LanguageCode {
    name: "Dano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aso",
    individual_languages: &[
    ],
};


pub const ASP: LanguageCode = LanguageCode {
    name: "Algerian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asp",
    individual_languages: &[
    ],
};


pub const ASQ: LanguageCode = LanguageCode {
    name: "Austrian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asq",
    individual_languages: &[
    ],
};


pub const ASR: LanguageCode = LanguageCode {
    name: "Asuri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asr",
    individual_languages: &[
    ],
};


pub const ASS: LanguageCode = LanguageCode {
    name: "Ipulo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ass",
    individual_languages: &[
    ],
};


pub const ASU: LanguageCode = LanguageCode {
    name: "Tocantins Asurini",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asu",
    individual_languages: &[
    ],
};


pub const ASV: LanguageCode = LanguageCode {
    name: "Asoa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asv",
    individual_languages: &[
    ],
};


pub const ASW: LanguageCode = LanguageCode {
    name: "Australian Aborigines Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asw",
    individual_languages: &[
    ],
};


pub const ASX: LanguageCode = LanguageCode {
    name: "Muratayak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asx",
    individual_languages: &[
    ],
};


pub const ASY: LanguageCode = LanguageCode {
    name: "Yaosakor Asmat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asy",
    individual_languages: &[
    ],
};


pub const ASZ: LanguageCode = LanguageCode {
    name: "As",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "asz",
    individual_languages: &[
    ],
};


pub const ATA: LanguageCode = LanguageCode {
    name: "Pele-Ata",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ata",
    individual_languages: &[
    ],
};


pub const ATB: LanguageCode = LanguageCode {
    name: "Zaiwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atb",
    individual_languages: &[
    ],
};


pub const ATC: LanguageCode = LanguageCode {
    name: "Atsahuaca",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atc",
    individual_languages: &[
    ],
};


pub const ATD: LanguageCode = LanguageCode {
    name: "Ata Manobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atd",
    individual_languages: &[
    ],
};


pub const ATE: LanguageCode = LanguageCode {
    name: "Atemble",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ate",
    individual_languages: &[
    ],
};


pub const ATG: LanguageCode = LanguageCode {
    name: "Ivbie North-Okpela-Arhe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atg",
    individual_languages: &[
    ],
};


pub const ATI: LanguageCode = LanguageCode {
    name: "Attié",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ati",
    individual_languages: &[
    ],
};


pub const ATJ: LanguageCode = LanguageCode {
    name: "Atikamekw",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atj",
    individual_languages: &[
    ],
};


pub const ATK: LanguageCode = LanguageCode {
    name: "Ati",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atk",
    individual_languages: &[
    ],
};


pub const ATL: LanguageCode = LanguageCode {
    name: "Mt. Iraya Agta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atl",
    individual_languages: &[
    ],
};


pub const ATM: LanguageCode = LanguageCode {
    name: "Ata",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atm",
    individual_languages: &[
    ],
};


pub const ATN: LanguageCode = LanguageCode {
    name: "Ashtiani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atn",
    individual_languages: &[
    ],
};


pub const ATO: LanguageCode = LanguageCode {
    name: "Atong (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ato",
    individual_languages: &[
    ],
};


pub const ATP: LanguageCode = LanguageCode {
    name: "Pudtol Atta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atp",
    individual_languages: &[
    ],
};


pub const ATQ: LanguageCode = LanguageCode {
    name: "Aralle-Tabulahan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atq",
    individual_languages: &[
    ],
};


pub const ATR: LanguageCode = LanguageCode {
    name: "Waimiri-Atroari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atr",
    individual_languages: &[
    ],
};


pub const ATS: LanguageCode = LanguageCode {
    name: "Gros Ventre",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ats",
    individual_languages: &[
    ],
};


pub const ATT: LanguageCode = LanguageCode {
    name: "Pamplona Atta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "att",
    individual_languages: &[
    ],
};


pub const ATU: LanguageCode = LanguageCode {
    name: "Reel",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atu",
    individual_languages: &[
    ],
};


pub const ATV: LanguageCode = LanguageCode {
    name: "Northern Altai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atv",
    individual_languages: &[
    ],
};


pub const ATW: LanguageCode = LanguageCode {
    name: "Atsugewi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atw",
    individual_languages: &[
    ],
};


pub const ATX: LanguageCode = LanguageCode {
    name: "Arutani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atx",
    individual_languages: &[
    ],
};


pub const ATY: LanguageCode = LanguageCode {
    name: "Aneityum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aty",
    individual_languages: &[
    ],
};


pub const ATZ: LanguageCode = LanguageCode {
    name: "Arta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "atz",
    individual_languages: &[
    ],
};


pub const AUA: LanguageCode = LanguageCode {
    name: "Asumboa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aua",
    individual_languages: &[
    ],
};


pub const AUB: LanguageCode = LanguageCode {
    name: "Alugu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aub",
    individual_languages: &[
    ],
};


pub const AUC: LanguageCode = LanguageCode {
    name: "Waorani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "auc",
    individual_languages: &[
    ],
};


pub const AUD: LanguageCode = LanguageCode {
    name: "Anuta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aud",
    individual_languages: &[
    ],
};


pub const AUG: LanguageCode = LanguageCode {
    name: "Aguna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aug",
    individual_languages: &[
    ],
};


pub const AUH: LanguageCode = LanguageCode {
    name: "Aushi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "auh",
    individual_languages: &[
    ],
};


pub const AUI: LanguageCode = LanguageCode {
    name: "Anuki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aui",
    individual_languages: &[
    ],
};


pub const AUJ: LanguageCode = LanguageCode {
    name: "Awjilah",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "auj",
    individual_languages: &[
    ],
};


pub const AUK: LanguageCode = LanguageCode {
    name: "Heyo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "auk",
    individual_languages: &[
    ],
};


pub const AUL: LanguageCode = LanguageCode {
    name: "Aulua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aul",
    individual_languages: &[
    ],
};


pub const AUM: LanguageCode = LanguageCode {
    name: "Asu (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aum",
    individual_languages: &[
    ],
};


pub const AUN: LanguageCode = LanguageCode {
    name: "Molmo One",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aun",
    individual_languages: &[
    ],
};


pub const AUO: LanguageCode = LanguageCode {
    name: "Auyokawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "auo",
    individual_languages: &[
    ],
};


pub const AUP: LanguageCode = LanguageCode {
    name: "Makayam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aup",
    individual_languages: &[
    ],
};


pub const AUQ: LanguageCode = LanguageCode {
    name: "Anus",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "auq",
    individual_languages: &[
    ],
};


pub const AUR: LanguageCode = LanguageCode {
    name: "Aruek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aur",
    individual_languages: &[
    ],
};


pub const AUT: LanguageCode = LanguageCode {
    name: "Austral",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aut",
    individual_languages: &[
    ],
};


pub const AUU: LanguageCode = LanguageCode {
    name: "Auye",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "auu",
    individual_languages: &[
    ],
};


pub const AUW: LanguageCode = LanguageCode {
    name: "Awyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "auw",
    individual_languages: &[
    ],
};


pub const AUX: LanguageCode = LanguageCode {
    name: "Aurá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aux",
    individual_languages: &[
    ],
};


pub const AUY: LanguageCode = LanguageCode {
    name: "Awiyaana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "auy",
    individual_languages: &[
    ],
};


pub const AUZ: LanguageCode = LanguageCode {
    name: "Uzbeki Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "auz",
    individual_languages: &[
    ],
};


pub const AVB: LanguageCode = LanguageCode {
    name: "Avau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "avb",
    individual_languages: &[
    ],
};


pub const AVD: LanguageCode = LanguageCode {
    name: "Alviri-Vidari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "avd",
    individual_languages: &[
    ],
};


pub const AVI: LanguageCode = LanguageCode {
    name: "Avikam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "avi",
    individual_languages: &[
    ],
};


pub const AVK: LanguageCode = LanguageCode {
    name: "Kotava",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "avk",
    individual_languages: &[
    ],
};


pub const AVL: LanguageCode = LanguageCode {
    name: "Eastern Egyptian Bedawi Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "avl",
    individual_languages: &[
    ],
};


pub const AVM: LanguageCode = LanguageCode {
    name: "Angkamuthi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "avm",
    individual_languages: &[
    ],
};


pub const AVN: LanguageCode = LanguageCode {
    name: "Avatime",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "avn",
    individual_languages: &[
    ],
};


pub const AVO: LanguageCode = LanguageCode {
    name: "Agavotaguerra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "avo",
    individual_languages: &[
    ],
};


pub const AVS: LanguageCode = LanguageCode {
    name: "Aushiri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "avs",
    individual_languages: &[
    ],
};


pub const AVT: LanguageCode = LanguageCode {
    name: "Au",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "avt",
    individual_languages: &[
    ],
};


pub const AVU: LanguageCode = LanguageCode {
    name: "Avokaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "avu",
    individual_languages: &[
    ],
};


pub const AVV: LanguageCode = LanguageCode {
    name: "Avá-Canoeiro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "avv",
    individual_languages: &[
    ],
};


pub const AWB: LanguageCode = LanguageCode {
    name: "Awa (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awb",
    individual_languages: &[
    ],
};


pub const AWC: LanguageCode = LanguageCode {
    name: "Cicipu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awc",
    individual_languages: &[
    ],
};


pub const AWE: LanguageCode = LanguageCode {
    name: "Awetí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awe",
    individual_languages: &[
    ],
};


pub const AWG: LanguageCode = LanguageCode {
    name: "Anguthimri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awg",
    individual_languages: &[
    ],
};


pub const AWH: LanguageCode = LanguageCode {
    name: "Awbono",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awh",
    individual_languages: &[
    ],
};


pub const AWI: LanguageCode = LanguageCode {
    name: "Aekyom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awi",
    individual_languages: &[
    ],
};


pub const AWK: LanguageCode = LanguageCode {
    name: "Awabakal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awk",
    individual_languages: &[
    ],
};


pub const AWM: LanguageCode = LanguageCode {
    name: "Arawum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awm",
    individual_languages: &[
    ],
};


pub const AWN: LanguageCode = LanguageCode {
    name: "Awngi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awn",
    individual_languages: &[
    ],
};


pub const AWO: LanguageCode = LanguageCode {
    name: "Awak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awo",
    individual_languages: &[
    ],
};


pub const AWR: LanguageCode = LanguageCode {
    name: "Awera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awr",
    individual_languages: &[
    ],
};


pub const AWS: LanguageCode = LanguageCode {
    name: "South Awyu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aws",
    individual_languages: &[
    ],
};


pub const AWT: LanguageCode = LanguageCode {
    name: "Araweté",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awt",
    individual_languages: &[
    ],
};


pub const AWU: LanguageCode = LanguageCode {
    name: "Central Awyu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awu",
    individual_languages: &[
    ],
};


pub const AWV: LanguageCode = LanguageCode {
    name: "Jair Awyu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awv",
    individual_languages: &[
    ],
};


pub const AWW: LanguageCode = LanguageCode {
    name: "Awun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aww",
    individual_languages: &[
    ],
};


pub const AWX: LanguageCode = LanguageCode {
    name: "Awara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awx",
    individual_languages: &[
    ],
};


pub const AWY: LanguageCode = LanguageCode {
    name: "Edera Awyu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "awy",
    individual_languages: &[
    ],
};


pub const AXB: LanguageCode = LanguageCode {
    name: "Abipon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "axb",
    individual_languages: &[
    ],
};


pub const AXE: LanguageCode = LanguageCode {
    name: "Ayerrerenge",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "axe",
    individual_languages: &[
    ],
};


pub const AXG: LanguageCode = LanguageCode {
    name: "Mato Grosso Arára",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "axg",
    individual_languages: &[
    ],
};


pub const AXK: LanguageCode = LanguageCode {
    name: "Yaka (Central African Republic)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "axk",
    individual_languages: &[
    ],
};


pub const AXL: LanguageCode = LanguageCode {
    name: "Lower Southern Aranda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "axl",
    individual_languages: &[
    ],
};


pub const AXM: LanguageCode = LanguageCode {
    name: "Middle Armenian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "axm",
    individual_languages: &[
    ],
};


pub const AXX: LanguageCode = LanguageCode {
    name: "Xârâgurè",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "axx",
    individual_languages: &[
    ],
};


pub const AYA: LanguageCode = LanguageCode {
    name: "Awar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aya",
    individual_languages: &[
    ],
};


pub const AYB: LanguageCode = LanguageCode {
    name: "Ayizo Gbe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayb",
    individual_languages: &[
    ],
};


pub const AYC: LanguageCode = LanguageCode {
    name: "Southern Aymara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayc",
    individual_languages: &[
    ],
};


pub const AYD: LanguageCode = LanguageCode {
    name: "Ayabadhu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayd",
    individual_languages: &[
    ],
};


pub const AYE: LanguageCode = LanguageCode {
    name: "Ayere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aye",
    individual_languages: &[
    ],
};


pub const AYG: LanguageCode = LanguageCode {
    name: "Ginyanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayg",
    individual_languages: &[
    ],
};


pub const AYH: LanguageCode = LanguageCode {
    name: "Hadrami Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayh",
    individual_languages: &[
    ],
};


pub const AYI: LanguageCode = LanguageCode {
    name: "Leyigha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayi",
    individual_languages: &[
    ],
};


pub const AYK: LanguageCode = LanguageCode {
    name: "Akuku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayk",
    individual_languages: &[
    ],
};


pub const AYL: LanguageCode = LanguageCode {
    name: "Libyan Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayl",
    individual_languages: &[
    ],
};


pub const AYN: LanguageCode = LanguageCode {
    name: "Sanaani Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayn",
    individual_languages: &[
    ],
};


pub const AYO: LanguageCode = LanguageCode {
    name: "Ayoreo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayo",
    individual_languages: &[
    ],
};


pub const AYP: LanguageCode = LanguageCode {
    name: "North Mesopotamian Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayp",
    individual_languages: &[
    ],
};


pub const AYQ: LanguageCode = LanguageCode {
    name: "Ayi (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayq",
    individual_languages: &[
    ],
};


pub const AYR: LanguageCode = LanguageCode {
    name: "Central Aymara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayr",
    individual_languages: &[
    ],
};


pub const AYS: LanguageCode = LanguageCode {
    name: "Sorsogon Ayta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ays",
    individual_languages: &[
    ],
};


pub const AYT: LanguageCode = LanguageCode {
    name: "Magbukun Ayta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayt",
    individual_languages: &[
    ],
};


pub const AYU: LanguageCode = LanguageCode {
    name: "Ayu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayu",
    individual_languages: &[
    ],
};


pub const AYZ: LanguageCode = LanguageCode {
    name: "Mai Brat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ayz",
    individual_languages: &[
    ],
};


pub const AZA: LanguageCode = LanguageCode {
    name: "Azha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "aza",
    individual_languages: &[
    ],
};


pub const AZB: LanguageCode = LanguageCode {
    name: "South Azerbaijani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "azb",
    individual_languages: &[
    ],
};


pub const AZD: LanguageCode = LanguageCode {
    name: "Eastern Durango Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "azd",
    individual_languages: &[
    ],
};


pub const AZG: LanguageCode = LanguageCode {
    name: "San Pedro Amuzgos Amuzgo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "azg",
    individual_languages: &[
    ],
};


pub const AZJ: LanguageCode = LanguageCode {
    name: "North Azerbaijani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "azj",
    individual_languages: &[
    ],
};


pub const AZM: LanguageCode = LanguageCode {
    name: "Ipalapa Amuzgo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "azm",
    individual_languages: &[
    ],
};


pub const AZN: LanguageCode = LanguageCode {
    name: "Western Durango Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "azn",
    individual_languages: &[
    ],
};


pub const AZO: LanguageCode = LanguageCode {
    name: "Awing",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "azo",
    individual_languages: &[
    ],
};


pub const AZT: LanguageCode = LanguageCode {
    name: "Faire Atta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "azt",
    individual_languages: &[
    ],
};


pub const AZZ: LanguageCode = LanguageCode {
    name: "Highland Puebla Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "azz",
    individual_languages: &[
    ],
};


pub const BAA: LanguageCode = LanguageCode {
    name: "Babatana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "baa",
    individual_languages: &[
    ],
};


pub const BAB: LanguageCode = LanguageCode {
    name: "Bainouk-Gunyuño",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bab",
    individual_languages: &[
    ],
};


pub const BAC: LanguageCode = LanguageCode {
    name: "Badui",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bac",
    individual_languages: &[
    ],
};


pub const BAE: LanguageCode = LanguageCode {
    name: "Baré",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bae",
    individual_languages: &[
    ],
};


pub const BAF: LanguageCode = LanguageCode {
    name: "Nubaca",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "baf",
    individual_languages: &[
    ],
};


pub const BAG: LanguageCode = LanguageCode {
    name: "Tuki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bag",
    individual_languages: &[
    ],
};


pub const BAH: LanguageCode = LanguageCode {
    name: "Bahamas Creole English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bah",
    individual_languages: &[
    ],
};


pub const BAJ: LanguageCode = LanguageCode {
    name: "Barakai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "baj",
    individual_languages: &[
    ],
};


pub const BAO: LanguageCode = LanguageCode {
    name: "Waimaha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bao",
    individual_languages: &[
    ],
};


pub const BAP: LanguageCode = LanguageCode {
    name: "Bantawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bap",
    individual_languages: &[
    ],
};


pub const BAR: LanguageCode = LanguageCode {
    name: "Bavarian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bar",
    individual_languages: &[
    ],
};


pub const BAU: LanguageCode = LanguageCode {
    name: "Bada (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bau",
    individual_languages: &[
    ],
};


pub const BAV: LanguageCode = LanguageCode {
    name: "Vengo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bav",
    individual_languages: &[
    ],
};


pub const BAW: LanguageCode = LanguageCode {
    name: "Bambili-Bambui",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "baw",
    individual_languages: &[
    ],
};


pub const BAX: LanguageCode = LanguageCode {
    name: "Bamun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bax",
    individual_languages: &[
    ],
};


pub const BAY: LanguageCode = LanguageCode {
    name: "Batuley",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bay",
    individual_languages: &[
    ],
};


pub const BBA: LanguageCode = LanguageCode {
    name: "Baatonum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bba",
    individual_languages: &[
    ],
};


pub const BBB: LanguageCode = LanguageCode {
    name: "Barai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbb",
    individual_languages: &[
    ],
};


pub const BBC: LanguageCode = LanguageCode {
    name: "Batak Toba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbc",
    individual_languages: &[
    ],
};


pub const BBD: LanguageCode = LanguageCode {
    name: "Bau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbd",
    individual_languages: &[
    ],
};


pub const BBE: LanguageCode = LanguageCode {
    name: "Bangba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbe",
    individual_languages: &[
    ],
};


pub const BBF: LanguageCode = LanguageCode {
    name: "Baibai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbf",
    individual_languages: &[
    ],
};


pub const BBG: LanguageCode = LanguageCode {
    name: "Barama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbg",
    individual_languages: &[
    ],
};


pub const BBH: LanguageCode = LanguageCode {
    name: "Bugan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbh",
    individual_languages: &[
    ],
};


pub const BBI: LanguageCode = LanguageCode {
    name: "Barombi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbi",
    individual_languages: &[
    ],
};


pub const BBJ: LanguageCode = LanguageCode {
    name: "Ghomálá'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbj",
    individual_languages: &[
    ],
};


pub const BBK: LanguageCode = LanguageCode {
    name: "Babanki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbk",
    individual_languages: &[
    ],
};


pub const BBL: LanguageCode = LanguageCode {
    name: "Bats",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbl",
    individual_languages: &[
    ],
};


pub const BBM: LanguageCode = LanguageCode {
    name: "Babango",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbm",
    individual_languages: &[
    ],
};


pub const BBN: LanguageCode = LanguageCode {
    name: "Uneapa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbn",
    individual_languages: &[
    ],
};


pub const BBO: LanguageCode = LanguageCode {
    name: "Northern Bobo Madaré",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbo",
    individual_languages: &[
    ],
};


pub const BBP: LanguageCode = LanguageCode {
    name: "West Central Banda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbp",
    individual_languages: &[
    ],
};


pub const BBQ: LanguageCode = LanguageCode {
    name: "Bamali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbq",
    individual_languages: &[
    ],
};


pub const BBR: LanguageCode = LanguageCode {
    name: "Girawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbr",
    individual_languages: &[
    ],
};


pub const BBS: LanguageCode = LanguageCode {
    name: "Bakpinka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbs",
    individual_languages: &[
    ],
};


pub const BBT: LanguageCode = LanguageCode {
    name: "Mburku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbt",
    individual_languages: &[
    ],
};


pub const BBU: LanguageCode = LanguageCode {
    name: "Kulung (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbu",
    individual_languages: &[
    ],
};


pub const BBV: LanguageCode = LanguageCode {
    name: "Karnai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbv",
    individual_languages: &[
    ],
};


pub const BBW: LanguageCode = LanguageCode {
    name: "Baba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbw",
    individual_languages: &[
    ],
};


pub const BBX: LanguageCode = LanguageCode {
    name: "Bubia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bbx",
    individual_languages: &[
    ],
};


pub const BBY: LanguageCode = LanguageCode {
    name: "Befang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bby",
    individual_languages: &[
    ],
};


pub const BCA: LanguageCode = LanguageCode {
    name: "Central Bai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bca",
    individual_languages: &[
    ],
};


pub const BCB: LanguageCode = LanguageCode {
    name: "Bainouk-Samik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcb",
    individual_languages: &[
    ],
};


pub const BCC: LanguageCode = LanguageCode {
    name: "Southern Balochi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcc",
    individual_languages: &[
    ],
};


pub const BCD: LanguageCode = LanguageCode {
    name: "North Babar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcd",
    individual_languages: &[
    ],
};


pub const BCE: LanguageCode = LanguageCode {
    name: "Bamenyam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bce",
    individual_languages: &[
    ],
};


pub const BCF: LanguageCode = LanguageCode {
    name: "Bamu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcf",
    individual_languages: &[
    ],
};


pub const BCG: LanguageCode = LanguageCode {
    name: "Baga Pokur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcg",
    individual_languages: &[
    ],
};


pub const BCH: LanguageCode = LanguageCode {
    name: "Bariai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bch",
    individual_languages: &[
    ],
};


pub const BCI: LanguageCode = LanguageCode {
    name: "Baoulé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bci",
    individual_languages: &[
    ],
};


pub const BCJ: LanguageCode = LanguageCode {
    name: "Bardi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcj",
    individual_languages: &[
    ],
};


pub const BCK: LanguageCode = LanguageCode {
    name: "Bunuba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bck",
    individual_languages: &[
    ],
};


pub const BCL: LanguageCode = LanguageCode {
    name: "Central Bikol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcl",
    individual_languages: &[
    ],
};


pub const BCM: LanguageCode = LanguageCode {
    name: "Bannoni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcm",
    individual_languages: &[
    ],
};


pub const BCN: LanguageCode = LanguageCode {
    name: "Bali (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcn",
    individual_languages: &[
    ],
};


pub const BCO: LanguageCode = LanguageCode {
    name: "Kaluli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bco",
    individual_languages: &[
    ],
};


pub const BCP: LanguageCode = LanguageCode {
    name: "Bali (Democratic Republic of Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcp",
    individual_languages: &[
    ],
};


pub const BCQ: LanguageCode = LanguageCode {
    name: "Bench",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcq",
    individual_languages: &[
    ],
};


pub const BCR: LanguageCode = LanguageCode {
    name: "Babine",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcr",
    individual_languages: &[
    ],
};


pub const BCS: LanguageCode = LanguageCode {
    name: "Kohumono",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcs",
    individual_languages: &[
    ],
};


pub const BCT: LanguageCode = LanguageCode {
    name: "Bendi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bct",
    individual_languages: &[
    ],
};


pub const BCU: LanguageCode = LanguageCode {
    name: "Awad Bing",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcu",
    individual_languages: &[
    ],
};


pub const BCV: LanguageCode = LanguageCode {
    name: "Shoo-Minda-Nye",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcv",
    individual_languages: &[
    ],
};


pub const BCW: LanguageCode = LanguageCode {
    name: "Bana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcw",
    individual_languages: &[
    ],
};


pub const BCY: LanguageCode = LanguageCode {
    name: "Bacama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcy",
    individual_languages: &[
    ],
};


pub const BCZ: LanguageCode = LanguageCode {
    name: "Bainouk-Gunyaamolo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bcz",
    individual_languages: &[
    ],
};


pub const BDA: LanguageCode = LanguageCode {
    name: "Bayot",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bda",
    individual_languages: &[
    ],
};


pub const BDB: LanguageCode = LanguageCode {
    name: "Basap",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdb",
    individual_languages: &[
    ],
};


pub const BDC: LanguageCode = LanguageCode {
    name: "Emberá-Baudó",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdc",
    individual_languages: &[
    ],
};


pub const BDD: LanguageCode = LanguageCode {
    name: "Bunama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdd",
    individual_languages: &[
    ],
};


pub const BDE: LanguageCode = LanguageCode {
    name: "Bade",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bde",
    individual_languages: &[
    ],
};


pub const BDF: LanguageCode = LanguageCode {
    name: "Biage",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdf",
    individual_languages: &[
    ],
};


pub const BDG: LanguageCode = LanguageCode {
    name: "Bonggi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdg",
    individual_languages: &[
    ],
};


pub const BDH: LanguageCode = LanguageCode {
    name: "Baka (South Sudan)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdh",
    individual_languages: &[
    ],
};


pub const BDI: LanguageCode = LanguageCode {
    name: "Burun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdi",
    individual_languages: &[
    ],
};


pub const BDJ: LanguageCode = LanguageCode {
    name: "Bai (South Sudan)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdj",
    individual_languages: &[
    ],
};


pub const BDK: LanguageCode = LanguageCode {
    name: "Budukh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdk",
    individual_languages: &[
    ],
};


pub const BDL: LanguageCode = LanguageCode {
    name: "Indonesian Bajau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdl",
    individual_languages: &[
    ],
};


pub const BDM: LanguageCode = LanguageCode {
    name: "Buduma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdm",
    individual_languages: &[
    ],
};


pub const BDN: LanguageCode = LanguageCode {
    name: "Baldemu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdn",
    individual_languages: &[
    ],
};


pub const BDO: LanguageCode = LanguageCode {
    name: "Morom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdo",
    individual_languages: &[
    ],
};


pub const BDP: LanguageCode = LanguageCode {
    name: "Bende",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdp",
    individual_languages: &[
    ],
};


pub const BDQ: LanguageCode = LanguageCode {
    name: "Bahnar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdq",
    individual_languages: &[
    ],
};


pub const BDR: LanguageCode = LanguageCode {
    name: "West Coast Bajau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdr",
    individual_languages: &[
    ],
};


pub const BDS: LanguageCode = LanguageCode {
    name: "Burunge",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bds",
    individual_languages: &[
    ],
};


pub const BDT: LanguageCode = LanguageCode {
    name: "Bokoto",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdt",
    individual_languages: &[
    ],
};


pub const BDU: LanguageCode = LanguageCode {
    name: "Oroko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdu",
    individual_languages: &[
    ],
};


pub const BDV: LanguageCode = LanguageCode {
    name: "Bodo Parja",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdv",
    individual_languages: &[
    ],
};


pub const BDW: LanguageCode = LanguageCode {
    name: "Baham",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdw",
    individual_languages: &[
    ],
};


pub const BDX: LanguageCode = LanguageCode {
    name: "Budong-Budong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdx",
    individual_languages: &[
    ],
};


pub const BDY: LanguageCode = LanguageCode {
    name: "Bandjalang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdy",
    individual_languages: &[
    ],
};


pub const BDZ: LanguageCode = LanguageCode {
    name: "Badeshi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bdz",
    individual_languages: &[
    ],
};


pub const BEA: LanguageCode = LanguageCode {
    name: "Beaver",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bea",
    individual_languages: &[
    ],
};


pub const BEB: LanguageCode = LanguageCode {
    name: "Bebele",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "beb",
    individual_languages: &[
    ],
};


pub const BEC: LanguageCode = LanguageCode {
    name: "Iceve-Maci",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bec",
    individual_languages: &[
    ],
};


pub const BED: LanguageCode = LanguageCode {
    name: "Bedoanas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bed",
    individual_languages: &[
    ],
};


pub const BEE: LanguageCode = LanguageCode {
    name: "Byangsi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bee",
    individual_languages: &[
    ],
};


pub const BEF: LanguageCode = LanguageCode {
    name: "Benabena",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bef",
    individual_languages: &[
    ],
};


pub const BEG: LanguageCode = LanguageCode {
    name: "Belait",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "beg",
    individual_languages: &[
    ],
};


pub const BEH: LanguageCode = LanguageCode {
    name: "Biali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "beh",
    individual_languages: &[
    ],
};


pub const BEI: LanguageCode = LanguageCode {
    name: "Bekati'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bei",
    individual_languages: &[
    ],
};


pub const BEK: LanguageCode = LanguageCode {
    name: "Bebeli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bek",
    individual_languages: &[
    ],
};


pub const BEO: LanguageCode = LanguageCode {
    name: "Beami",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "beo",
    individual_languages: &[
    ],
};


pub const BEP: LanguageCode = LanguageCode {
    name: "Besoa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bep",
    individual_languages: &[
    ],
};


pub const BEQ: LanguageCode = LanguageCode {
    name: "Beembe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "beq",
    individual_languages: &[
    ],
};


pub const BES: LanguageCode = LanguageCode {
    name: "Besme",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bes",
    individual_languages: &[
    ],
};


pub const BET: LanguageCode = LanguageCode {
    name: "Guiberoua Béte",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bet",
    individual_languages: &[
    ],
};


pub const BEU: LanguageCode = LanguageCode {
    name: "Blagar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "beu",
    individual_languages: &[
    ],
};


pub const BEV: LanguageCode = LanguageCode {
    name: "Daloa Bété",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bev",
    individual_languages: &[
    ],
};


pub const BEW: LanguageCode = LanguageCode {
    name: "Betawi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bew",
    individual_languages: &[
    ],
};


pub const BEX: LanguageCode = LanguageCode {
    name: "Jur Modo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bex",
    individual_languages: &[
    ],
};


pub const BEY: LanguageCode = LanguageCode {
    name: "Beli (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bey",
    individual_languages: &[
    ],
};


pub const BEZ: LanguageCode = LanguageCode {
    name: "Bena (Tanzania)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bez",
    individual_languages: &[
    ],
};


pub const BFA: LanguageCode = LanguageCode {
    name: "Bari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfa",
    individual_languages: &[
    ],
};


pub const BFB: LanguageCode = LanguageCode {
    name: "Pauri Bareli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfb",
    individual_languages: &[
    ],
};


pub const BFC: LanguageCode = LanguageCode {
    name: "Panyi Bai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfc",
    individual_languages: &[
    ],
};


pub const BFD: LanguageCode = LanguageCode {
    name: "Bafut",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfd",
    individual_languages: &[
    ],
};


pub const BFE: LanguageCode = LanguageCode {
    name: "Betaf",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfe",
    individual_languages: &[
    ],
};


pub const BFF: LanguageCode = LanguageCode {
    name: "Bofi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bff",
    individual_languages: &[
    ],
};


pub const BFG: LanguageCode = LanguageCode {
    name: "Busang Kayan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfg",
    individual_languages: &[
    ],
};


pub const BFH: LanguageCode = LanguageCode {
    name: "Blafe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfh",
    individual_languages: &[
    ],
};


pub const BFI: LanguageCode = LanguageCode {
    name: "British Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfi",
    individual_languages: &[
    ],
};


pub const BFJ: LanguageCode = LanguageCode {
    name: "Bafanji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfj",
    individual_languages: &[
    ],
};


pub const BFK: LanguageCode = LanguageCode {
    name: "Ban Khor Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfk",
    individual_languages: &[
    ],
};


pub const BFL: LanguageCode = LanguageCode {
    name: "Banda-Ndélé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfl",
    individual_languages: &[
    ],
};


pub const BFM: LanguageCode = LanguageCode {
    name: "Mmen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfm",
    individual_languages: &[
    ],
};


pub const BFN: LanguageCode = LanguageCode {
    name: "Bunak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfn",
    individual_languages: &[
    ],
};


pub const BFO: LanguageCode = LanguageCode {
    name: "Malba Birifor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfo",
    individual_languages: &[
    ],
};


pub const BFP: LanguageCode = LanguageCode {
    name: "Beba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfp",
    individual_languages: &[
    ],
};


pub const BFQ: LanguageCode = LanguageCode {
    name: "Badaga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfq",
    individual_languages: &[
    ],
};


pub const BFR: LanguageCode = LanguageCode {
    name: "Bazigar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfr",
    individual_languages: &[
    ],
};


pub const BFS: LanguageCode = LanguageCode {
    name: "Southern Bai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfs",
    individual_languages: &[
    ],
};


pub const BFT: LanguageCode = LanguageCode {
    name: "Balti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bft",
    individual_languages: &[
    ],
};


pub const BFU: LanguageCode = LanguageCode {
    name: "Gahri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfu",
    individual_languages: &[
    ],
};


pub const BFW: LanguageCode = LanguageCode {
    name: "Bondo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfw",
    individual_languages: &[
    ],
};


pub const BFX: LanguageCode = LanguageCode {
    name: "Bantayanon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfx",
    individual_languages: &[
    ],
};


pub const BFY: LanguageCode = LanguageCode {
    name: "Bagheli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfy",
    individual_languages: &[
    ],
};


pub const BFZ: LanguageCode = LanguageCode {
    name: "Mahasu Pahari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bfz",
    individual_languages: &[
    ],
};


pub const BGA: LanguageCode = LanguageCode {
    name: "Gwamhi-Wuri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bga",
    individual_languages: &[
    ],
};


pub const BGB: LanguageCode = LanguageCode {
    name: "Bobongko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgb",
    individual_languages: &[
    ],
};


pub const BGC: LanguageCode = LanguageCode {
    name: "Haryanvi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgc",
    individual_languages: &[
    ],
};


pub const BGD: LanguageCode = LanguageCode {
    name: "Rathwi Bareli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgd",
    individual_languages: &[
    ],
};


pub const BGE: LanguageCode = LanguageCode {
    name: "Bauria",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bge",
    individual_languages: &[
    ],
};


pub const BGF: LanguageCode = LanguageCode {
    name: "Bangandu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgf",
    individual_languages: &[
    ],
};


pub const BGG: LanguageCode = LanguageCode {
    name: "Bugun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgg",
    individual_languages: &[
    ],
};


pub const BGI: LanguageCode = LanguageCode {
    name: "Giangan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgi",
    individual_languages: &[
    ],
};


pub const BGJ: LanguageCode = LanguageCode {
    name: "Bangolan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgj",
    individual_languages: &[
    ],
};


pub const BGK: LanguageCode = LanguageCode {
    name: "Bit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgk",
    individual_languages: &[
    ],
};


pub const BGL: LanguageCode = LanguageCode {
    name: "Bo (Laos)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgl",
    individual_languages: &[
    ],
};


pub const BGN: LanguageCode = LanguageCode {
    name: "Western Balochi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgn",
    individual_languages: &[
    ],
};


pub const BGO: LanguageCode = LanguageCode {
    name: "Baga Koga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgo",
    individual_languages: &[
    ],
};


pub const BGP: LanguageCode = LanguageCode {
    name: "Eastern Balochi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgp",
    individual_languages: &[
    ],
};


pub const BGQ: LanguageCode = LanguageCode {
    name: "Bagri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgq",
    individual_languages: &[
    ],
};


pub const BGR: LanguageCode = LanguageCode {
    name: "Bawm Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgr",
    individual_languages: &[
    ],
};


pub const BGS: LanguageCode = LanguageCode {
    name: "Tagabawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgs",
    individual_languages: &[
    ],
};


pub const BGT: LanguageCode = LanguageCode {
    name: "Bughotu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgt",
    individual_languages: &[
    ],
};


pub const BGU: LanguageCode = LanguageCode {
    name: "Mbongno",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgu",
    individual_languages: &[
    ],
};


pub const BGV: LanguageCode = LanguageCode {
    name: "Warkay-Bipim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgv",
    individual_languages: &[
    ],
};


pub const BGW: LanguageCode = LanguageCode {
    name: "Bhatri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgw",
    individual_languages: &[
    ],
};


pub const BGX: LanguageCode = LanguageCode {
    name: "Balkan Gagauz Turkish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgx",
    individual_languages: &[
    ],
};


pub const BGY: LanguageCode = LanguageCode {
    name: "Benggoi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgy",
    individual_languages: &[
    ],
};


pub const BGZ: LanguageCode = LanguageCode {
    name: "Banggai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bgz",
    individual_languages: &[
    ],
};


pub const BHA: LanguageCode = LanguageCode {
    name: "Bharia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bha",
    individual_languages: &[
    ],
};


pub const BHB: LanguageCode = LanguageCode {
    name: "Bhili",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhb",
    individual_languages: &[
    ],
};


pub const BHC: LanguageCode = LanguageCode {
    name: "Biga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhc",
    individual_languages: &[
    ],
};


pub const BHD: LanguageCode = LanguageCode {
    name: "Bhadrawahi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhd",
    individual_languages: &[
    ],
};


pub const BHE: LanguageCode = LanguageCode {
    name: "Bhaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhe",
    individual_languages: &[
    ],
};


pub const BHF: LanguageCode = LanguageCode {
    name: "Odiai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhf",
    individual_languages: &[
    ],
};


pub const BHG: LanguageCode = LanguageCode {
    name: "Binandere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhg",
    individual_languages: &[
    ],
};


pub const BHH: LanguageCode = LanguageCode {
    name: "Bukharic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhh",
    individual_languages: &[
    ],
};


pub const BHI: LanguageCode = LanguageCode {
    name: "Bhilali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhi",
    individual_languages: &[
    ],
};


pub const BHJ: LanguageCode = LanguageCode {
    name: "Bahing",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhj",
    individual_languages: &[
    ],
};


pub const BHL: LanguageCode = LanguageCode {
    name: "Bimin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhl",
    individual_languages: &[
    ],
};


pub const BHM: LanguageCode = LanguageCode {
    name: "Bathari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhm",
    individual_languages: &[
    ],
};


pub const BHN: LanguageCode = LanguageCode {
    name: "Bohtan Neo-Aramaic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhn",
    individual_languages: &[
    ],
};


pub const BHP: LanguageCode = LanguageCode {
    name: "Bima",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhp",
    individual_languages: &[
    ],
};


pub const BHQ: LanguageCode = LanguageCode {
    name: "Tukang Besi South",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhq",
    individual_languages: &[
    ],
};


pub const BHR: LanguageCode = LanguageCode {
    name: "Bara Malagasy",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhr",
    individual_languages: &[
    ],
};


pub const BHS: LanguageCode = LanguageCode {
    name: "Buwal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhs",
    individual_languages: &[
    ],
};


pub const BHT: LanguageCode = LanguageCode {
    name: "Bhattiyali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bht",
    individual_languages: &[
    ],
};


pub const BHU: LanguageCode = LanguageCode {
    name: "Bhunjia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhu",
    individual_languages: &[
    ],
};


pub const BHV: LanguageCode = LanguageCode {
    name: "Bahau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhv",
    individual_languages: &[
    ],
};


pub const BHW: LanguageCode = LanguageCode {
    name: "Biak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhw",
    individual_languages: &[
    ],
};


pub const BHX: LanguageCode = LanguageCode {
    name: "Bhalay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhx",
    individual_languages: &[
    ],
};


pub const BHY: LanguageCode = LanguageCode {
    name: "Bhele",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhy",
    individual_languages: &[
    ],
};


pub const BHZ: LanguageCode = LanguageCode {
    name: "Bada (Indonesia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bhz",
    individual_languages: &[
    ],
};


pub const BIA: LanguageCode = LanguageCode {
    name: "Badimaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bia",
    individual_languages: &[
    ],
};


pub const BIB: LanguageCode = LanguageCode {
    name: "Bissa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bib",
    individual_languages: &[
    ],
};


pub const BID: LanguageCode = LanguageCode {
    name: "Bidiyo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bid",
    individual_languages: &[
    ],
};


pub const BIE: LanguageCode = LanguageCode {
    name: "Bepour",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bie",
    individual_languages: &[
    ],
};


pub const BIF: LanguageCode = LanguageCode {
    name: "Biafada",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bif",
    individual_languages: &[
    ],
};


pub const BIG: LanguageCode = LanguageCode {
    name: "Biangai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "big",
    individual_languages: &[
    ],
};


pub const BIL: LanguageCode = LanguageCode {
    name: "Bile",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bil",
    individual_languages: &[
    ],
};


pub const BIM: LanguageCode = LanguageCode {
    name: "Bimoba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bim",
    individual_languages: &[
    ],
};


pub const BIO: LanguageCode = LanguageCode {
    name: "Nai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bio",
    individual_languages: &[
    ],
};


pub const BIP: LanguageCode = LanguageCode {
    name: "Bila",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bip",
    individual_languages: &[
    ],
};


pub const BIQ: LanguageCode = LanguageCode {
    name: "Bipi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "biq",
    individual_languages: &[
    ],
};


pub const BIR: LanguageCode = LanguageCode {
    name: "Bisorio",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bir",
    individual_languages: &[
    ],
};


pub const BIT: LanguageCode = LanguageCode {
    name: "Berinomo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bit",
    individual_languages: &[
    ],
};


pub const BIU: LanguageCode = LanguageCode {
    name: "Biete",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "biu",
    individual_languages: &[
    ],
};


pub const BIV: LanguageCode = LanguageCode {
    name: "Southern Birifor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "biv",
    individual_languages: &[
    ],
};


pub const BIW: LanguageCode = LanguageCode {
    name: "Kol (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "biw",
    individual_languages: &[
    ],
};


pub const BIX: LanguageCode = LanguageCode {
    name: "Bijori",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bix",
    individual_languages: &[
    ],
};


pub const BIY: LanguageCode = LanguageCode {
    name: "Birhor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "biy",
    individual_languages: &[
    ],
};


pub const BIZ: LanguageCode = LanguageCode {
    name: "Baloi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "biz",
    individual_languages: &[
    ],
};


pub const BJA: LanguageCode = LanguageCode {
    name: "Budza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bja",
    individual_languages: &[
    ],
};


pub const BJB: LanguageCode = LanguageCode {
    name: "Banggarla",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjb",
    individual_languages: &[
    ],
};


pub const BJC: LanguageCode = LanguageCode {
    name: "Bariji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjc",
    individual_languages: &[
    ],
};


pub const BJE: LanguageCode = LanguageCode {
    name: "Biao-Jiao Mien",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bje",
    individual_languages: &[
    ],
};


pub const BJF: LanguageCode = LanguageCode {
    name: "Barzani Jewish Neo-Aramaic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjf",
    individual_languages: &[
    ],
};


pub const BJG: LanguageCode = LanguageCode {
    name: "Bidyogo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjg",
    individual_languages: &[
    ],
};


pub const BJH: LanguageCode = LanguageCode {
    name: "Bahinemo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjh",
    individual_languages: &[
    ],
};


pub const BJI: LanguageCode = LanguageCode {
    name: "Burji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bji",
    individual_languages: &[
    ],
};


pub const BJJ: LanguageCode = LanguageCode {
    name: "Kanauji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjj",
    individual_languages: &[
    ],
};


pub const BJK: LanguageCode = LanguageCode {
    name: "Barok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjk",
    individual_languages: &[
    ],
};


pub const BJL: LanguageCode = LanguageCode {
    name: "Bulu (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjl",
    individual_languages: &[
    ],
};


pub const BJM: LanguageCode = LanguageCode {
    name: "Bajelani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjm",
    individual_languages: &[
    ],
};


pub const BJN: LanguageCode = LanguageCode {
    name: "Banjar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjn",
    individual_languages: &[
    ],
};


pub const BJO: LanguageCode = LanguageCode {
    name: "Mid-Southern Banda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjo",
    individual_languages: &[
    ],
};


pub const BJP: LanguageCode = LanguageCode {
    name: "Fanamaket",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjp",
    individual_languages: &[
    ],
};


pub const BJR: LanguageCode = LanguageCode {
    name: "Binumarien",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjr",
    individual_languages: &[
    ],
};


pub const BJS: LanguageCode = LanguageCode {
    name: "Bajan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjs",
    individual_languages: &[
    ],
};


pub const BJT: LanguageCode = LanguageCode {
    name: "Balanta-Ganja",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjt",
    individual_languages: &[
    ],
};


pub const BJU: LanguageCode = LanguageCode {
    name: "Busuu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bju",
    individual_languages: &[
    ],
};


pub const BJV: LanguageCode = LanguageCode {
    name: "Bedjond",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjv",
    individual_languages: &[
    ],
};


pub const BJW: LanguageCode = LanguageCode {
    name: "Bakwé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjw",
    individual_languages: &[
    ],
};


pub const BJX: LanguageCode = LanguageCode {
    name: "Banao Itneg",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjx",
    individual_languages: &[
    ],
};


pub const BJY: LanguageCode = LanguageCode {
    name: "Bayali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjy",
    individual_languages: &[
    ],
};


pub const BJZ: LanguageCode = LanguageCode {
    name: "Baruga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bjz",
    individual_languages: &[
    ],
};


pub const BKA: LanguageCode = LanguageCode {
    name: "Kyak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bka",
    individual_languages: &[
    ],
};


pub const BKC: LanguageCode = LanguageCode {
    name: "Baka (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkc",
    individual_languages: &[
    ],
};


pub const BKD: LanguageCode = LanguageCode {
    name: "Binukid",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkd",
    individual_languages: &[
    ],
};


pub const BKF: LanguageCode = LanguageCode {
    name: "Beeke",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkf",
    individual_languages: &[
    ],
};


pub const BKG: LanguageCode = LanguageCode {
    name: "Buraka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkg",
    individual_languages: &[
    ],
};


pub const BKH: LanguageCode = LanguageCode {
    name: "Bakoko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkh",
    individual_languages: &[
    ],
};


pub const BKI: LanguageCode = LanguageCode {
    name: "Baki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bki",
    individual_languages: &[
    ],
};


pub const BKJ: LanguageCode = LanguageCode {
    name: "Pande",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkj",
    individual_languages: &[
    ],
};


pub const BKK: LanguageCode = LanguageCode {
    name: "Brokskat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkk",
    individual_languages: &[
    ],
};


pub const BKL: LanguageCode = LanguageCode {
    name: "Berik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkl",
    individual_languages: &[
    ],
};


pub const BKM: LanguageCode = LanguageCode {
    name: "Kom (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkm",
    individual_languages: &[
    ],
};


pub const BKN: LanguageCode = LanguageCode {
    name: "Bukitan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkn",
    individual_languages: &[
    ],
};


pub const BKO: LanguageCode = LanguageCode {
    name: "Kwa'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bko",
    individual_languages: &[
    ],
};


pub const BKP: LanguageCode = LanguageCode {
    name: "Boko (Democratic Republic of Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkp",
    individual_languages: &[
    ],
};


pub const BKQ: LanguageCode = LanguageCode {
    name: "Bakairí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkq",
    individual_languages: &[
    ],
};


pub const BKR: LanguageCode = LanguageCode {
    name: "Bakumpai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkr",
    individual_languages: &[
    ],
};


pub const BKS: LanguageCode = LanguageCode {
    name: "Northern Sorsoganon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bks",
    individual_languages: &[
    ],
};


pub const BKT: LanguageCode = LanguageCode {
    name: "Boloki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkt",
    individual_languages: &[
    ],
};


pub const BKU: LanguageCode = LanguageCode {
    name: "Buhid",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bku",
    individual_languages: &[
    ],
};


pub const BKV: LanguageCode = LanguageCode {
    name: "Bekwarra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkv",
    individual_languages: &[
    ],
};


pub const BKW: LanguageCode = LanguageCode {
    name: "Bekwel",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkw",
    individual_languages: &[
    ],
};


pub const BKX: LanguageCode = LanguageCode {
    name: "Baikeno",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkx",
    individual_languages: &[
    ],
};


pub const BKY: LanguageCode = LanguageCode {
    name: "Bokyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bky",
    individual_languages: &[
    ],
};


pub const BKZ: LanguageCode = LanguageCode {
    name: "Bungku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bkz",
    individual_languages: &[
    ],
};


pub const BLB: LanguageCode = LanguageCode {
    name: "Bilua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blb",
    individual_languages: &[
    ],
};


pub const BLC: LanguageCode = LanguageCode {
    name: "Bella Coola",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blc",
    individual_languages: &[
    ],
};


pub const BLD: LanguageCode = LanguageCode {
    name: "Bolango",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bld",
    individual_languages: &[
    ],
};


pub const BLE: LanguageCode = LanguageCode {
    name: "Balanta-Kentohe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ble",
    individual_languages: &[
    ],
};


pub const BLF: LanguageCode = LanguageCode {
    name: "Buol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blf",
    individual_languages: &[
    ],
};


pub const BLH: LanguageCode = LanguageCode {
    name: "Kuwaa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blh",
    individual_languages: &[
    ],
};


pub const BLI: LanguageCode = LanguageCode {
    name: "Bolia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bli",
    individual_languages: &[
    ],
};


pub const BLJ: LanguageCode = LanguageCode {
    name: "Bolongan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blj",
    individual_languages: &[
    ],
};


pub const BLK: LanguageCode = LanguageCode {
    name: "Pa'o Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blk",
    individual_languages: &[
    ],
};


pub const BLL: LanguageCode = LanguageCode {
    name: "Biloxi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bll",
    individual_languages: &[
    ],
};


pub const BLM: LanguageCode = LanguageCode {
    name: "Beli (South Sudan)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blm",
    individual_languages: &[
    ],
};


pub const BLN: LanguageCode = LanguageCode {
    name: "Southern Catanduanes Bikol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bln",
    individual_languages: &[
    ],
};


pub const BLO: LanguageCode = LanguageCode {
    name: "Anii",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blo",
    individual_languages: &[
    ],
};


pub const BLP: LanguageCode = LanguageCode {
    name: "Blablanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blp",
    individual_languages: &[
    ],
};


pub const BLQ: LanguageCode = LanguageCode {
    name: "Baluan-Pam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blq",
    individual_languages: &[
    ],
};


pub const BLR: LanguageCode = LanguageCode {
    name: "Blang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blr",
    individual_languages: &[
    ],
};


pub const BLS: LanguageCode = LanguageCode {
    name: "Balaesang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bls",
    individual_languages: &[
    ],
};


pub const BLT: LanguageCode = LanguageCode {
    name: "Tai Dam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blt",
    individual_languages: &[
    ],
};


pub const BLV: LanguageCode = LanguageCode {
    name: "Kibala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blv",
    individual_languages: &[
    ],
};


pub const BLW: LanguageCode = LanguageCode {
    name: "Balangao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blw",
    individual_languages: &[
    ],
};


pub const BLX: LanguageCode = LanguageCode {
    name: "Mag-Indi Ayta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blx",
    individual_languages: &[
    ],
};


pub const BLY: LanguageCode = LanguageCode {
    name: "Notre",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bly",
    individual_languages: &[
    ],
};


pub const BLZ: LanguageCode = LanguageCode {
    name: "Balantak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "blz",
    individual_languages: &[
    ],
};


pub const BMA: LanguageCode = LanguageCode {
    name: "Lame",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bma",
    individual_languages: &[
    ],
};


pub const BMB: LanguageCode = LanguageCode {
    name: "Bembe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmb",
    individual_languages: &[
    ],
};


pub const BMC: LanguageCode = LanguageCode {
    name: "Biem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmc",
    individual_languages: &[
    ],
};


pub const BMD: LanguageCode = LanguageCode {
    name: "Baga Manduri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmd",
    individual_languages: &[
    ],
};


pub const BME: LanguageCode = LanguageCode {
    name: "Limassa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bme",
    individual_languages: &[
    ],
};


pub const BMF: LanguageCode = LanguageCode {
    name: "Bom-Kim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmf",
    individual_languages: &[
    ],
};


pub const BMG: LanguageCode = LanguageCode {
    name: "Bamwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmg",
    individual_languages: &[
    ],
};


pub const BMH: LanguageCode = LanguageCode {
    name: "Kein",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmh",
    individual_languages: &[
    ],
};


pub const BMI: LanguageCode = LanguageCode {
    name: "Bagirmi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmi",
    individual_languages: &[
    ],
};


pub const BMJ: LanguageCode = LanguageCode {
    name: "Bote-Majhi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmj",
    individual_languages: &[
    ],
};


pub const BMK: LanguageCode = LanguageCode {
    name: "Ghayavi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmk",
    individual_languages: &[
    ],
};


pub const BML: LanguageCode = LanguageCode {
    name: "Bomboli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bml",
    individual_languages: &[
    ],
};


pub const BMM: LanguageCode = LanguageCode {
    name: "Northern Betsimisaraka Malagasy",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmm",
    individual_languages: &[
    ],
};


pub const BMN: LanguageCode = LanguageCode {
    name: "Bina (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmn",
    individual_languages: &[
    ],
};


pub const BMO: LanguageCode = LanguageCode {
    name: "Bambalang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmo",
    individual_languages: &[
    ],
};


pub const BMP: LanguageCode = LanguageCode {
    name: "Bulgebi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmp",
    individual_languages: &[
    ],
};


pub const BMQ: LanguageCode = LanguageCode {
    name: "Bomu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmq",
    individual_languages: &[
    ],
};


pub const BMR: LanguageCode = LanguageCode {
    name: "Muinane",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmr",
    individual_languages: &[
    ],
};


pub const BMS: LanguageCode = LanguageCode {
    name: "Bilma Kanuri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bms",
    individual_languages: &[
    ],
};


pub const BMT: LanguageCode = LanguageCode {
    name: "Biao Mon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmt",
    individual_languages: &[
    ],
};


pub const BMU: LanguageCode = LanguageCode {
    name: "Somba-Siawari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmu",
    individual_languages: &[
    ],
};


pub const BMV: LanguageCode = LanguageCode {
    name: "Bum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmv",
    individual_languages: &[
    ],
};


pub const BMW: LanguageCode = LanguageCode {
    name: "Bomwali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmw",
    individual_languages: &[
    ],
};


pub const BMX: LanguageCode = LanguageCode {
    name: "Baimak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmx",
    individual_languages: &[
    ],
};


pub const BMZ: LanguageCode = LanguageCode {
    name: "Baramu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bmz",
    individual_languages: &[
    ],
};


pub const BNA: LanguageCode = LanguageCode {
    name: "Bonerate",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bna",
    individual_languages: &[
    ],
};


pub const BNB: LanguageCode = LanguageCode {
    name: "Bookan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnb",
    individual_languages: &[
    ],
};


pub const BNC: LanguageCode = LanguageCode {
    name: "Bontok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnc",
    individual_languages: &[
        IndividualLanguages {
            name: "Eastern Bontok",
            code: "ebk",
        },
        IndividualLanguages {
            name: "Central Bontok",
            code: "lbk",
        },
        IndividualLanguages {
            name: "Southern Bontok",
            code: "obk",
        },
        IndividualLanguages {
            name: "Northern Bontok",
            code: "rbk",
        },
        IndividualLanguages {
            name: "Southwestern Bontok",
            code: "vbk",
        },
    ],
};


pub const BND: LanguageCode = LanguageCode {
    name: "Banda (Indonesia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnd",
    individual_languages: &[
    ],
};


pub const BNE: LanguageCode = LanguageCode {
    name: "Bintauna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bne",
    individual_languages: &[
    ],
};


pub const BNF: LanguageCode = LanguageCode {
    name: "Masiwang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnf",
    individual_languages: &[
    ],
};


pub const BNG: LanguageCode = LanguageCode {
    name: "Benga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bng",
    individual_languages: &[
    ],
};


pub const BNI: LanguageCode = LanguageCode {
    name: "Bangi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bni",
    individual_languages: &[
    ],
};


pub const BNJ: LanguageCode = LanguageCode {
    name: "Eastern Tawbuid",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnj",
    individual_languages: &[
    ],
};


pub const BNK: LanguageCode = LanguageCode {
    name: "Bierebo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnk",
    individual_languages: &[
    ],
};


pub const BNL: LanguageCode = LanguageCode {
    name: "Boon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnl",
    individual_languages: &[
    ],
};


pub const BNM: LanguageCode = LanguageCode {
    name: "Batanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnm",
    individual_languages: &[
    ],
};


pub const BNN: LanguageCode = LanguageCode {
    name: "Bunun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnn",
    individual_languages: &[
    ],
};


pub const BNO: LanguageCode = LanguageCode {
    name: "Bantoanon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bno",
    individual_languages: &[
    ],
};


pub const BNP: LanguageCode = LanguageCode {
    name: "Bola",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnp",
    individual_languages: &[
    ],
};


pub const BNQ: LanguageCode = LanguageCode {
    name: "Bantik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnq",
    individual_languages: &[
    ],
};


pub const BNR: LanguageCode = LanguageCode {
    name: "Butmas-Tur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnr",
    individual_languages: &[
    ],
};


pub const BNS: LanguageCode = LanguageCode {
    name: "Bundeli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bns",
    individual_languages: &[
    ],
};


pub const BNU: LanguageCode = LanguageCode {
    name: "Bentong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnu",
    individual_languages: &[
    ],
};


pub const BNV: LanguageCode = LanguageCode {
    name: "Bonerif",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnv",
    individual_languages: &[
    ],
};


pub const BNW: LanguageCode = LanguageCode {
    name: "Bisis",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnw",
    individual_languages: &[
    ],
};


pub const BNX: LanguageCode = LanguageCode {
    name: "Bangubangu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnx",
    individual_languages: &[
    ],
};


pub const BNY: LanguageCode = LanguageCode {
    name: "Bintulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bny",
    individual_languages: &[
    ],
};


pub const BNZ: LanguageCode = LanguageCode {
    name: "Beezen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bnz",
    individual_languages: &[
    ],
};


pub const BOA: LanguageCode = LanguageCode {
    name: "Bora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "boa",
    individual_languages: &[
    ],
};


pub const BOB: LanguageCode = LanguageCode {
    name: "Aweer",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bob",
    individual_languages: &[
    ],
};


pub const BOE: LanguageCode = LanguageCode {
    name: "Mundabli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "boe",
    individual_languages: &[
    ],
};


pub const BOF: LanguageCode = LanguageCode {
    name: "Bolon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bof",
    individual_languages: &[
    ],
};


pub const BOG: LanguageCode = LanguageCode {
    name: "Bamako Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bog",
    individual_languages: &[
    ],
};


pub const BOH: LanguageCode = LanguageCode {
    name: "Boma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "boh",
    individual_languages: &[
    ],
};


pub const BOI: LanguageCode = LanguageCode {
    name: "Barbareño",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "boi",
    individual_languages: &[
    ],
};


pub const BOJ: LanguageCode = LanguageCode {
    name: "Anjam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "boj",
    individual_languages: &[
    ],
};


pub const BOK: LanguageCode = LanguageCode {
    name: "Bonjo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bok",
    individual_languages: &[
    ],
};


pub const BOL: LanguageCode = LanguageCode {
    name: "Bole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bol",
    individual_languages: &[
    ],
};


pub const BOM: LanguageCode = LanguageCode {
    name: "Berom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bom",
    individual_languages: &[
    ],
};


pub const BON: LanguageCode = LanguageCode {
    name: "Bine",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bon",
    individual_languages: &[
    ],
};


pub const BOO: LanguageCode = LanguageCode {
    name: "Tiemacèwè Bozo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "boo",
    individual_languages: &[
    ],
};


pub const BOP: LanguageCode = LanguageCode {
    name: "Bonkiman",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bop",
    individual_languages: &[
    ],
};


pub const BOQ: LanguageCode = LanguageCode {
    name: "Bogaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "boq",
    individual_languages: &[
    ],
};


pub const BOR: LanguageCode = LanguageCode {
    name: "Borôro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bor",
    individual_languages: &[
    ],
};


pub const BOT: LanguageCode = LanguageCode {
    name: "Bongo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bot",
    individual_languages: &[
    ],
};


pub const BOU: LanguageCode = LanguageCode {
    name: "Bondei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bou",
    individual_languages: &[
    ],
};


pub const BOV: LanguageCode = LanguageCode {
    name: "Tuwuli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bov",
    individual_languages: &[
    ],
};


pub const BOW: LanguageCode = LanguageCode {
    name: "Rema",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bow",
    individual_languages: &[
    ],
};


pub const BOX: LanguageCode = LanguageCode {
    name: "Buamu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "box",
    individual_languages: &[
    ],
};


pub const BOY: LanguageCode = LanguageCode {
    name: "Bodo (Central African Republic)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "boy",
    individual_languages: &[
    ],
};


pub const BOZ: LanguageCode = LanguageCode {
    name: "Tiéyaxo Bozo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "boz",
    individual_languages: &[
    ],
};


pub const BPA: LanguageCode = LanguageCode {
    name: "Daakaka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpa",
    individual_languages: &[
    ],
};


pub const BPC: LanguageCode = LanguageCode {
    name: "Mbuk",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpc",
    individual_languages: &[
    ],
};


pub const BPD: LanguageCode = LanguageCode {
    name: "Banda-Banda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpd",
    individual_languages: &[
    ],
};


pub const BPE: LanguageCode = LanguageCode {
    name: "Bauni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpe",
    individual_languages: &[
    ],
};


pub const BPG: LanguageCode = LanguageCode {
    name: "Bonggo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpg",
    individual_languages: &[
    ],
};


pub const BPH: LanguageCode = LanguageCode {
    name: "Botlikh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bph",
    individual_languages: &[
    ],
};


pub const BPI: LanguageCode = LanguageCode {
    name: "Bagupi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpi",
    individual_languages: &[
    ],
};


pub const BPJ: LanguageCode = LanguageCode {
    name: "Binji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpj",
    individual_languages: &[
    ],
};


pub const BPK: LanguageCode = LanguageCode {
    name: "Orowe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpk",
    individual_languages: &[
    ],
};


pub const BPL: LanguageCode = LanguageCode {
    name: "Broome Pearling Lugger Pidgin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpl",
    individual_languages: &[
    ],
};


pub const BPM: LanguageCode = LanguageCode {
    name: "Biyom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpm",
    individual_languages: &[
    ],
};


pub const BPN: LanguageCode = LanguageCode {
    name: "Dzao Min",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpn",
    individual_languages: &[
    ],
};


pub const BPO: LanguageCode = LanguageCode {
    name: "Anasi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpo",
    individual_languages: &[
    ],
};


pub const BPP: LanguageCode = LanguageCode {
    name: "Kaure",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpp",
    individual_languages: &[
    ],
};


pub const BPQ: LanguageCode = LanguageCode {
    name: "Banda Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpq",
    individual_languages: &[
    ],
};


pub const BPR: LanguageCode = LanguageCode {
    name: "Koronadal Blaan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpr",
    individual_languages: &[
    ],
};


pub const BPS: LanguageCode = LanguageCode {
    name: "Sarangani Blaan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bps",
    individual_languages: &[
    ],
};


pub const BPT: LanguageCode = LanguageCode {
    name: "Barrow Point",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpt",
    individual_languages: &[
    ],
};


pub const BPU: LanguageCode = LanguageCode {
    name: "Bongu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpu",
    individual_languages: &[
    ],
};


pub const BPV: LanguageCode = LanguageCode {
    name: "Bian Marind",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpv",
    individual_languages: &[
    ],
};


pub const BPW: LanguageCode = LanguageCode {
    name: "Bo (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpw",
    individual_languages: &[
    ],
};


pub const BPX: LanguageCode = LanguageCode {
    name: "Palya Bareli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpx",
    individual_languages: &[
    ],
};


pub const BPY: LanguageCode = LanguageCode {
    name: "Bishnupriya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpy",
    individual_languages: &[
    ],
};


pub const BPZ: LanguageCode = LanguageCode {
    name: "Bilba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bpz",
    individual_languages: &[
    ],
};


pub const BQA: LanguageCode = LanguageCode {
    name: "Tchumbuli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqa",
    individual_languages: &[
    ],
};


pub const BQB: LanguageCode = LanguageCode {
    name: "Bagusa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqb",
    individual_languages: &[
    ],
};


pub const BQC: LanguageCode = LanguageCode {
    name: "Boko (Benin)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqc",
    individual_languages: &[
    ],
};


pub const BQD: LanguageCode = LanguageCode {
    name: "Bung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqd",
    individual_languages: &[
    ],
};


pub const BQF: LanguageCode = LanguageCode {
    name: "Baga Kaloum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqf",
    individual_languages: &[
    ],
};


pub const BQG: LanguageCode = LanguageCode {
    name: "Bago-Kusuntu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqg",
    individual_languages: &[
    ],
};


pub const BQH: LanguageCode = LanguageCode {
    name: "Baima",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqh",
    individual_languages: &[
    ],
};


pub const BQI: LanguageCode = LanguageCode {
    name: "Bakhtiari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqi",
    individual_languages: &[
    ],
};


pub const BQJ: LanguageCode = LanguageCode {
    name: "Bandial",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqj",
    individual_languages: &[
    ],
};


pub const BQK: LanguageCode = LanguageCode {
    name: "Banda-Mbrès",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqk",
    individual_languages: &[
    ],
};


pub const BQL: LanguageCode = LanguageCode {
    name: "Bilakura",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bql",
    individual_languages: &[
    ],
};


pub const BQM: LanguageCode = LanguageCode {
    name: "Wumboko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqm",
    individual_languages: &[
    ],
};


pub const BQN: LanguageCode = LanguageCode {
    name: "Bulgarian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqn",
    individual_languages: &[
    ],
};


pub const BQO: LanguageCode = LanguageCode {
    name: "Balo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqo",
    individual_languages: &[
    ],
};


pub const BQP: LanguageCode = LanguageCode {
    name: "Busa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqp",
    individual_languages: &[
    ],
};


pub const BQQ: LanguageCode = LanguageCode {
    name: "Biritai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqq",
    individual_languages: &[
    ],
};


pub const BQR: LanguageCode = LanguageCode {
    name: "Burusu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqr",
    individual_languages: &[
    ],
};


pub const BQS: LanguageCode = LanguageCode {
    name: "Bosngun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqs",
    individual_languages: &[
    ],
};


pub const BQT: LanguageCode = LanguageCode {
    name: "Bamukumbit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqt",
    individual_languages: &[
    ],
};


pub const BQU: LanguageCode = LanguageCode {
    name: "Boguru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqu",
    individual_languages: &[
    ],
};


pub const BQV: LanguageCode = LanguageCode {
    name: "Koro Wachi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqv",
    individual_languages: &[
    ],
};


pub const BQW: LanguageCode = LanguageCode {
    name: "Buru (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqw",
    individual_languages: &[
    ],
};


pub const BQX: LanguageCode = LanguageCode {
    name: "Baangi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqx",
    individual_languages: &[
    ],
};


pub const BQY: LanguageCode = LanguageCode {
    name: "Bengkala Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqy",
    individual_languages: &[
    ],
};


pub const BQZ: LanguageCode = LanguageCode {
    name: "Bakaka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bqz",
    individual_languages: &[
    ],
};


pub const BRB: LanguageCode = LanguageCode {
    name: "Brao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brb",
    individual_languages: &[
    ],
};


pub const BRC: LanguageCode = LanguageCode {
    name: "Berbice Creole Dutch",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brc",
    individual_languages: &[
    ],
};


pub const BRD: LanguageCode = LanguageCode {
    name: "Baraamu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brd",
    individual_languages: &[
    ],
};


pub const BRF: LanguageCode = LanguageCode {
    name: "Bira",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brf",
    individual_languages: &[
    ],
};


pub const BRG: LanguageCode = LanguageCode {
    name: "Baure",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brg",
    individual_languages: &[
    ],
};


pub const BRH: LanguageCode = LanguageCode {
    name: "Brahui",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brh",
    individual_languages: &[
    ],
};


pub const BRI: LanguageCode = LanguageCode {
    name: "Mokpwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bri",
    individual_languages: &[
    ],
};


pub const BRJ: LanguageCode = LanguageCode {
    name: "Bieria",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brj",
    individual_languages: &[
    ],
};


pub const BRK: LanguageCode = LanguageCode {
    name: "Birked",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brk",
    individual_languages: &[
    ],
};


pub const BRL: LanguageCode = LanguageCode {
    name: "Birwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brl",
    individual_languages: &[
    ],
};


pub const BRM: LanguageCode = LanguageCode {
    name: "Barambu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brm",
    individual_languages: &[
    ],
};


pub const BRN: LanguageCode = LanguageCode {
    name: "Boruca",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brn",
    individual_languages: &[
    ],
};


pub const BRO: LanguageCode = LanguageCode {
    name: "Brokkat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bro",
    individual_languages: &[
    ],
};


pub const BRP: LanguageCode = LanguageCode {
    name: "Barapasi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brp",
    individual_languages: &[
    ],
};


pub const BRQ: LanguageCode = LanguageCode {
    name: "Breri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brq",
    individual_languages: &[
    ],
};


pub const BRR: LanguageCode = LanguageCode {
    name: "Birao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brr",
    individual_languages: &[
    ],
};


pub const BRS: LanguageCode = LanguageCode {
    name: "Baras",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brs",
    individual_languages: &[
    ],
};


pub const BRT: LanguageCode = LanguageCode {
    name: "Bitare",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brt",
    individual_languages: &[
    ],
};


pub const BRU: LanguageCode = LanguageCode {
    name: "Eastern Bru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bru",
    individual_languages: &[
    ],
};


pub const BRV: LanguageCode = LanguageCode {
    name: "Western Bru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brv",
    individual_languages: &[
    ],
};


pub const BRW: LanguageCode = LanguageCode {
    name: "Bellari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brw",
    individual_languages: &[
    ],
};


pub const BRX: LanguageCode = LanguageCode {
    name: "Bodo (India)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brx",
    individual_languages: &[
    ],
};


pub const BRY: LanguageCode = LanguageCode {
    name: "Burui",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bry",
    individual_languages: &[
    ],
};


pub const BRZ: LanguageCode = LanguageCode {
    name: "Bilbil",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "brz",
    individual_languages: &[
    ],
};


pub const BSA: LanguageCode = LanguageCode {
    name: "Abinomn",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsa",
    individual_languages: &[
    ],
};


pub const BSB: LanguageCode = LanguageCode {
    name: "Brunei Bisaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsb",
    individual_languages: &[
    ],
};


pub const BSC: LanguageCode = LanguageCode {
    name: "Bassari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsc",
    individual_languages: &[
    ],
};


pub const BSE: LanguageCode = LanguageCode {
    name: "Wushi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bse",
    individual_languages: &[
    ],
};


pub const BSF: LanguageCode = LanguageCode {
    name: "Bauchi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsf",
    individual_languages: &[
    ],
};


pub const BSG: LanguageCode = LanguageCode {
    name: "Bashkardi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsg",
    individual_languages: &[
    ],
};


pub const BSH: LanguageCode = LanguageCode {
    name: "Kati",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsh",
    individual_languages: &[
    ],
};


pub const BSI: LanguageCode = LanguageCode {
    name: "Bassossi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsi",
    individual_languages: &[
    ],
};


pub const BSJ: LanguageCode = LanguageCode {
    name: "Bangwinji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsj",
    individual_languages: &[
    ],
};


pub const BSK: LanguageCode = LanguageCode {
    name: "Burushaski",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsk",
    individual_languages: &[
    ],
};


pub const BSL: LanguageCode = LanguageCode {
    name: "Basa-Gumna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsl",
    individual_languages: &[
    ],
};


pub const BSM: LanguageCode = LanguageCode {
    name: "Busami",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsm",
    individual_languages: &[
    ],
};


pub const BSN: LanguageCode = LanguageCode {
    name: "Barasana-Eduria",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsn",
    individual_languages: &[
    ],
};


pub const BSO: LanguageCode = LanguageCode {
    name: "Buso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bso",
    individual_languages: &[
    ],
};


pub const BSP: LanguageCode = LanguageCode {
    name: "Baga Sitemu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsp",
    individual_languages: &[
    ],
};


pub const BSQ: LanguageCode = LanguageCode {
    name: "Bassa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsq",
    individual_languages: &[
    ],
};


pub const BSR: LanguageCode = LanguageCode {
    name: "Bassa-Kontagora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsr",
    individual_languages: &[
    ],
};


pub const BSS: LanguageCode = LanguageCode {
    name: "Akoose",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bss",
    individual_languages: &[
    ],
};


pub const BST: LanguageCode = LanguageCode {
    name: "Basketo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bst",
    individual_languages: &[
    ],
};


pub const BSU: LanguageCode = LanguageCode {
    name: "Bahonsuai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsu",
    individual_languages: &[
    ],
};


pub const BSV: LanguageCode = LanguageCode {
    name: "Baga Sobané",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsv",
    individual_languages: &[
    ],
};


pub const BSW: LanguageCode = LanguageCode {
    name: "Baiso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsw",
    individual_languages: &[
    ],
};


pub const BSX: LanguageCode = LanguageCode {
    name: "Yangkam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsx",
    individual_languages: &[
    ],
};


pub const BSY: LanguageCode = LanguageCode {
    name: "Sabah Bisaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bsy",
    individual_languages: &[
    ],
};


pub const BTA: LanguageCode = LanguageCode {
    name: "Bata",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bta",
    individual_languages: &[
    ],
};


pub const BTC: LanguageCode = LanguageCode {
    name: "Bati (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btc",
    individual_languages: &[
    ],
};


pub const BTD: LanguageCode = LanguageCode {
    name: "Batak Dairi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btd",
    individual_languages: &[
    ],
};


pub const BTE: LanguageCode = LanguageCode {
    name: "Gamo-Ningi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bte",
    individual_languages: &[
    ],
};


pub const BTF: LanguageCode = LanguageCode {
    name: "Birgit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btf",
    individual_languages: &[
    ],
};


pub const BTG: LanguageCode = LanguageCode {
    name: "Gagnoa Bété",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btg",
    individual_languages: &[
    ],
};


pub const BTH: LanguageCode = LanguageCode {
    name: "Biatah Bidayuh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bth",
    individual_languages: &[
    ],
};


pub const BTI: LanguageCode = LanguageCode {
    name: "Burate",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bti",
    individual_languages: &[
    ],
};


pub const BTJ: LanguageCode = LanguageCode {
    name: "Bacanese Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btj",
    individual_languages: &[
    ],
};


pub const BTM: LanguageCode = LanguageCode {
    name: "Batak Mandailing",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btm",
    individual_languages: &[
    ],
};


pub const BTN: LanguageCode = LanguageCode {
    name: "Ratagnon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btn",
    individual_languages: &[
    ],
};


pub const BTO: LanguageCode = LanguageCode {
    name: "Rinconada Bikol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bto",
    individual_languages: &[
    ],
};


pub const BTP: LanguageCode = LanguageCode {
    name: "Budibud",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btp",
    individual_languages: &[
    ],
};


pub const BTQ: LanguageCode = LanguageCode {
    name: "Batek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btq",
    individual_languages: &[
    ],
};


pub const BTR: LanguageCode = LanguageCode {
    name: "Baetora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btr",
    individual_languages: &[
    ],
};


pub const BTS: LanguageCode = LanguageCode {
    name: "Batak Simalungun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bts",
    individual_languages: &[
    ],
};


pub const BTT: LanguageCode = LanguageCode {
    name: "Bete-Bendi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btt",
    individual_languages: &[
    ],
};


pub const BTU: LanguageCode = LanguageCode {
    name: "Batu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btu",
    individual_languages: &[
    ],
};


pub const BTV: LanguageCode = LanguageCode {
    name: "Bateri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btv",
    individual_languages: &[
    ],
};


pub const BTW: LanguageCode = LanguageCode {
    name: "Butuanon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btw",
    individual_languages: &[
    ],
};


pub const BTX: LanguageCode = LanguageCode {
    name: "Batak Karo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btx",
    individual_languages: &[
    ],
};


pub const BTY: LanguageCode = LanguageCode {
    name: "Bobot",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bty",
    individual_languages: &[
    ],
};


pub const BTZ: LanguageCode = LanguageCode {
    name: "Batak Alas-Kluet",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "btz",
    individual_languages: &[
    ],
};


pub const BUB: LanguageCode = LanguageCode {
    name: "Bua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bub",
    individual_languages: &[
    ],
};


pub const BUC: LanguageCode = LanguageCode {
    name: "Bushi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "buc",
    individual_languages: &[
    ],
};


pub const BUD: LanguageCode = LanguageCode {
    name: "Ntcham",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bud",
    individual_languages: &[
    ],
};


pub const BUE: LanguageCode = LanguageCode {
    name: "Beothuk",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bue",
    individual_languages: &[
    ],
};


pub const BUF: LanguageCode = LanguageCode {
    name: "Bushoong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "buf",
    individual_languages: &[
    ],
};


pub const BUH: LanguageCode = LanguageCode {
    name: "Younuo Bunu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "buh",
    individual_languages: &[
    ],
};


pub const BUI: LanguageCode = LanguageCode {
    name: "Bongili",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bui",
    individual_languages: &[
    ],
};


pub const BUJ: LanguageCode = LanguageCode {
    name: "Basa-Gurmana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "buj",
    individual_languages: &[
    ],
};


pub const BUK: LanguageCode = LanguageCode {
    name: "Bugawac",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "buk",
    individual_languages: &[
    ],
};


pub const BUM: LanguageCode = LanguageCode {
    name: "Bulu (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bum",
    individual_languages: &[
    ],
};


pub const BUN: LanguageCode = LanguageCode {
    name: "Sherbro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bun",
    individual_languages: &[
    ],
};


pub const BUO: LanguageCode = LanguageCode {
    name: "Terei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "buo",
    individual_languages: &[
    ],
};


pub const BUP: LanguageCode = LanguageCode {
    name: "Busoa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bup",
    individual_languages: &[
    ],
};


pub const BUQ: LanguageCode = LanguageCode {
    name: "Brem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "buq",
    individual_languages: &[
    ],
};


pub const BUS: LanguageCode = LanguageCode {
    name: "Bokobaru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bus",
    individual_languages: &[
    ],
};


pub const BUT: LanguageCode = LanguageCode {
    name: "Bungain",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "but",
    individual_languages: &[
    ],
};


pub const BUU: LanguageCode = LanguageCode {
    name: "Budu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "buu",
    individual_languages: &[
    ],
};


pub const BUV: LanguageCode = LanguageCode {
    name: "Bun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "buv",
    individual_languages: &[
    ],
};


pub const BUW: LanguageCode = LanguageCode {
    name: "Bubi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "buw",
    individual_languages: &[
    ],
};


pub const BUX: LanguageCode = LanguageCode {
    name: "Boghom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bux",
    individual_languages: &[
    ],
};


pub const BUY: LanguageCode = LanguageCode {
    name: "Bullom So",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "buy",
    individual_languages: &[
    ],
};


pub const BUZ: LanguageCode = LanguageCode {
    name: "Bukwen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "buz",
    individual_languages: &[
    ],
};


pub const BVA: LanguageCode = LanguageCode {
    name: "Barein",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bva",
    individual_languages: &[
    ],
};


pub const BVB: LanguageCode = LanguageCode {
    name: "Bube",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvb",
    individual_languages: &[
    ],
};


pub const BVC: LanguageCode = LanguageCode {
    name: "Baelelea",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvc",
    individual_languages: &[
    ],
};


pub const BVD: LanguageCode = LanguageCode {
    name: "Baeggu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvd",
    individual_languages: &[
    ],
};


pub const BVE: LanguageCode = LanguageCode {
    name: "Berau Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bve",
    individual_languages: &[
    ],
};


pub const BVF: LanguageCode = LanguageCode {
    name: "Boor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvf",
    individual_languages: &[
    ],
};


pub const BVG: LanguageCode = LanguageCode {
    name: "Bonkeng",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvg",
    individual_languages: &[
    ],
};


pub const BVH: LanguageCode = LanguageCode {
    name: "Bure",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvh",
    individual_languages: &[
    ],
};


pub const BVI: LanguageCode = LanguageCode {
    name: "Belanda Viri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvi",
    individual_languages: &[
    ],
};


pub const BVJ: LanguageCode = LanguageCode {
    name: "Baan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvj",
    individual_languages: &[
    ],
};


pub const BVK: LanguageCode = LanguageCode {
    name: "Bukat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvk",
    individual_languages: &[
    ],
};


pub const BVL: LanguageCode = LanguageCode {
    name: "Bolivian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvl",
    individual_languages: &[
    ],
};


pub const BVM: LanguageCode = LanguageCode {
    name: "Bamunka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvm",
    individual_languages: &[
    ],
};


pub const BVN: LanguageCode = LanguageCode {
    name: "Buna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvn",
    individual_languages: &[
    ],
};


pub const BVO: LanguageCode = LanguageCode {
    name: "Bolgo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvo",
    individual_languages: &[
    ],
};


pub const BVP: LanguageCode = LanguageCode {
    name: "Bumang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvp",
    individual_languages: &[
    ],
};


pub const BVQ: LanguageCode = LanguageCode {
    name: "Birri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvq",
    individual_languages: &[
    ],
};


pub const BVR: LanguageCode = LanguageCode {
    name: "Burarra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvr",
    individual_languages: &[
    ],
};


pub const BVT: LanguageCode = LanguageCode {
    name: "Bati (Indonesia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvt",
    individual_languages: &[
    ],
};


pub const BVU: LanguageCode = LanguageCode {
    name: "Bukit Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvu",
    individual_languages: &[
    ],
};


pub const BVV: LanguageCode = LanguageCode {
    name: "Baniva",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvv",
    individual_languages: &[
    ],
};


pub const BVW: LanguageCode = LanguageCode {
    name: "Boga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvw",
    individual_languages: &[
    ],
};


pub const BVX: LanguageCode = LanguageCode {
    name: "Dibole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvx",
    individual_languages: &[
    ],
};


pub const BVY: LanguageCode = LanguageCode {
    name: "Baybayanon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvy",
    individual_languages: &[
    ],
};


pub const BVZ: LanguageCode = LanguageCode {
    name: "Bauzi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bvz",
    individual_languages: &[
    ],
};


pub const BWA: LanguageCode = LanguageCode {
    name: "Bwatoo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwa",
    individual_languages: &[
    ],
};


pub const BWB: LanguageCode = LanguageCode {
    name: "Namosi-Naitasiri-Serua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwb",
    individual_languages: &[
    ],
};


pub const BWC: LanguageCode = LanguageCode {
    name: "Bwile",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwc",
    individual_languages: &[
    ],
};


pub const BWD: LanguageCode = LanguageCode {
    name: "Bwaidoka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwd",
    individual_languages: &[
    ],
};


pub const BWE: LanguageCode = LanguageCode {
    name: "Bwe Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwe",
    individual_languages: &[
    ],
};


pub const BWF: LanguageCode = LanguageCode {
    name: "Boselewa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwf",
    individual_languages: &[
    ],
};


pub const BWG: LanguageCode = LanguageCode {
    name: "Barwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwg",
    individual_languages: &[
    ],
};


pub const BWH: LanguageCode = LanguageCode {
    name: "Bishuo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwh",
    individual_languages: &[
    ],
};


pub const BWI: LanguageCode = LanguageCode {
    name: "Baniwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwi",
    individual_languages: &[
    ],
};


pub const BWJ: LanguageCode = LanguageCode {
    name: "Láá Láá Bwamu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwj",
    individual_languages: &[
    ],
};


pub const BWK: LanguageCode = LanguageCode {
    name: "Bauwaki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwk",
    individual_languages: &[
    ],
};


pub const BWL: LanguageCode = LanguageCode {
    name: "Bwela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwl",
    individual_languages: &[
    ],
};


pub const BWM: LanguageCode = LanguageCode {
    name: "Biwat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwm",
    individual_languages: &[
    ],
};


pub const BWN: LanguageCode = LanguageCode {
    name: "Wunai Bunu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwn",
    individual_languages: &[
    ],
};


pub const BWO: LanguageCode = LanguageCode {
    name: "Boro (Ethiopia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwo",
    individual_languages: &[
    ],
};


pub const BWP: LanguageCode = LanguageCode {
    name: "Mandobo Bawah",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwp",
    individual_languages: &[
    ],
};


pub const BWQ: LanguageCode = LanguageCode {
    name: "Southern Bobo Madaré",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwq",
    individual_languages: &[
    ],
};


pub const BWR: LanguageCode = LanguageCode {
    name: "Bura-Pabir",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwr",
    individual_languages: &[
    ],
};


pub const BWS: LanguageCode = LanguageCode {
    name: "Bomboma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bws",
    individual_languages: &[
    ],
};


pub const BWT: LanguageCode = LanguageCode {
    name: "Bafaw-Balong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwt",
    individual_languages: &[
    ],
};


pub const BWU: LanguageCode = LanguageCode {
    name: "Buli (Ghana)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwu",
    individual_languages: &[
    ],
};


pub const BWW: LanguageCode = LanguageCode {
    name: "Bwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bww",
    individual_languages: &[
    ],
};


pub const BWX: LanguageCode = LanguageCode {
    name: "Bu-Nao Bunu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwx",
    individual_languages: &[
    ],
};


pub const BWY: LanguageCode = LanguageCode {
    name: "Cwi Bwamu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwy",
    individual_languages: &[
    ],
};


pub const BWZ: LanguageCode = LanguageCode {
    name: "Bwisi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bwz",
    individual_languages: &[
    ],
};


pub const BXA: LanguageCode = LanguageCode {
    name: "Tairaha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxa",
    individual_languages: &[
    ],
};


pub const BXB: LanguageCode = LanguageCode {
    name: "Belanda Bor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxb",
    individual_languages: &[
    ],
};


pub const BXC: LanguageCode = LanguageCode {
    name: "Molengue",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxc",
    individual_languages: &[
    ],
};


pub const BXD: LanguageCode = LanguageCode {
    name: "Pela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxd",
    individual_languages: &[
    ],
};


pub const BXE: LanguageCode = LanguageCode {
    name: "Birale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxe",
    individual_languages: &[
    ],
};


pub const BXF: LanguageCode = LanguageCode {
    name: "Bilur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxf",
    individual_languages: &[
    ],
};


pub const BXG: LanguageCode = LanguageCode {
    name: "Bangala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxg",
    individual_languages: &[
    ],
};


pub const BXH: LanguageCode = LanguageCode {
    name: "Buhutu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxh",
    individual_languages: &[
    ],
};


pub const BXI: LanguageCode = LanguageCode {
    name: "Pirlatapa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxi",
    individual_languages: &[
    ],
};


pub const BXJ: LanguageCode = LanguageCode {
    name: "Bayungu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxj",
    individual_languages: &[
    ],
};


pub const BXK: LanguageCode = LanguageCode {
    name: "Bukusu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxk",
    individual_languages: &[
    ],
};


pub const BXL: LanguageCode = LanguageCode {
    name: "Jalkunan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxl",
    individual_languages: &[
    ],
};


pub const BXM: LanguageCode = LanguageCode {
    name: "Mongolia Buriat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxm",
    individual_languages: &[
    ],
};


pub const BXN: LanguageCode = LanguageCode {
    name: "Burduna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxn",
    individual_languages: &[
    ],
};


pub const BXO: LanguageCode = LanguageCode {
    name: "Barikanchi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxo",
    individual_languages: &[
    ],
};


pub const BXP: LanguageCode = LanguageCode {
    name: "Bebil",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxp",
    individual_languages: &[
    ],
};


pub const BXQ: LanguageCode = LanguageCode {
    name: "Beele",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxq",
    individual_languages: &[
    ],
};


pub const BXR: LanguageCode = LanguageCode {
    name: "Russia Buriat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxr",
    individual_languages: &[
    ],
};


pub const BXS: LanguageCode = LanguageCode {
    name: "Busam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxs",
    individual_languages: &[
    ],
};


pub const BXU: LanguageCode = LanguageCode {
    name: "China Buriat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxu",
    individual_languages: &[
    ],
};


pub const BXV: LanguageCode = LanguageCode {
    name: "Berakou",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxv",
    individual_languages: &[
    ],
};


pub const BXW: LanguageCode = LanguageCode {
    name: "Bankagooma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxw",
    individual_languages: &[
    ],
};


pub const BXZ: LanguageCode = LanguageCode {
    name: "Binahari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bxz",
    individual_languages: &[
    ],
};


pub const BYA: LanguageCode = LanguageCode {
    name: "Batak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bya",
    individual_languages: &[
    ],
};


pub const BYB: LanguageCode = LanguageCode {
    name: "Bikya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byb",
    individual_languages: &[
    ],
};


pub const BYC: LanguageCode = LanguageCode {
    name: "Ubaghara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byc",
    individual_languages: &[
    ],
};


pub const BYD: LanguageCode = LanguageCode {
    name: "Benyadu'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byd",
    individual_languages: &[
    ],
};


pub const BYE: LanguageCode = LanguageCode {
    name: "Pouye",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bye",
    individual_languages: &[
    ],
};


pub const BYF: LanguageCode = LanguageCode {
    name: "Bete",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byf",
    individual_languages: &[
    ],
};


pub const BYG: LanguageCode = LanguageCode {
    name: "Baygo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byg",
    individual_languages: &[
    ],
};


pub const BYH: LanguageCode = LanguageCode {
    name: "Bhujel",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byh",
    individual_languages: &[
    ],
};


pub const BYI: LanguageCode = LanguageCode {
    name: "Buyu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byi",
    individual_languages: &[
    ],
};


pub const BYJ: LanguageCode = LanguageCode {
    name: "Bina (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byj",
    individual_languages: &[
    ],
};


pub const BYK: LanguageCode = LanguageCode {
    name: "Biao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byk",
    individual_languages: &[
    ],
};


pub const BYL: LanguageCode = LanguageCode {
    name: "Bayono",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byl",
    individual_languages: &[
    ],
};


pub const BYM: LanguageCode = LanguageCode {
    name: "Bidjara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bym",
    individual_languages: &[
    ],
};


pub const BYO: LanguageCode = LanguageCode {
    name: "Biyo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byo",
    individual_languages: &[
    ],
};


pub const BYP: LanguageCode = LanguageCode {
    name: "Bumaji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byp",
    individual_languages: &[
    ],
};


pub const BYQ: LanguageCode = LanguageCode {
    name: "Basay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byq",
    individual_languages: &[
    ],
};


pub const BYR: LanguageCode = LanguageCode {
    name: "Baruya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byr",
    individual_languages: &[
    ],
};


pub const BYS: LanguageCode = LanguageCode {
    name: "Burak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bys",
    individual_languages: &[
    ],
};


pub const BYT: LanguageCode = LanguageCode {
    name: "Berti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byt",
    individual_languages: &[
    ],
};


pub const BYV: LanguageCode = LanguageCode {
    name: "Medumba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byv",
    individual_languages: &[
    ],
};


pub const BYW: LanguageCode = LanguageCode {
    name: "Belhariya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byw",
    individual_languages: &[
    ],
};


pub const BYX: LanguageCode = LanguageCode {
    name: "Qaqet",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byx",
    individual_languages: &[
    ],
};


pub const BYZ: LanguageCode = LanguageCode {
    name: "Banaro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "byz",
    individual_languages: &[
    ],
};


pub const BZA: LanguageCode = LanguageCode {
    name: "Bandi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bza",
    individual_languages: &[
    ],
};


pub const BZB: LanguageCode = LanguageCode {
    name: "Andio",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzb",
    individual_languages: &[
    ],
};


pub const BZC: LanguageCode = LanguageCode {
    name: "Southern Betsimisaraka Malagasy",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzc",
    individual_languages: &[
    ],
};


pub const BZD: LanguageCode = LanguageCode {
    name: "Bribri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzd",
    individual_languages: &[
    ],
};


pub const BZE: LanguageCode = LanguageCode {
    name: "Jenaama Bozo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bze",
    individual_languages: &[
    ],
};


pub const BZF: LanguageCode = LanguageCode {
    name: "Boikin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzf",
    individual_languages: &[
    ],
};


pub const BZG: LanguageCode = LanguageCode {
    name: "Babuza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzg",
    individual_languages: &[
    ],
};


pub const BZH: LanguageCode = LanguageCode {
    name: "Mapos Buang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzh",
    individual_languages: &[
    ],
};


pub const BZI: LanguageCode = LanguageCode {
    name: "Bisu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzi",
    individual_languages: &[
    ],
};


pub const BZJ: LanguageCode = LanguageCode {
    name: "Belize Kriol English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzj",
    individual_languages: &[
    ],
};


pub const BZK: LanguageCode = LanguageCode {
    name: "Nicaragua Creole English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzk",
    individual_languages: &[
    ],
};


pub const BZL: LanguageCode = LanguageCode {
    name: "Boano (Sulawesi)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzl",
    individual_languages: &[
    ],
};


pub const BZM: LanguageCode = LanguageCode {
    name: "Bolondo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzm",
    individual_languages: &[
    ],
};


pub const BZN: LanguageCode = LanguageCode {
    name: "Boano (Maluku)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzn",
    individual_languages: &[
    ],
};


pub const BZO: LanguageCode = LanguageCode {
    name: "Bozaba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzo",
    individual_languages: &[
    ],
};


pub const BZP: LanguageCode = LanguageCode {
    name: "Kemberano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzp",
    individual_languages: &[
    ],
};


pub const BZQ: LanguageCode = LanguageCode {
    name: "Buli (Indonesia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzq",
    individual_languages: &[
    ],
};


pub const BZR: LanguageCode = LanguageCode {
    name: "Biri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzr",
    individual_languages: &[
    ],
};


pub const BZS: LanguageCode = LanguageCode {
    name: "Brazilian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzs",
    individual_languages: &[
    ],
};


pub const BZT: LanguageCode = LanguageCode {
    name: "Brithenig",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzt",
    individual_languages: &[
    ],
};


pub const BZU: LanguageCode = LanguageCode {
    name: "Burmeso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzu",
    individual_languages: &[
    ],
};


pub const BZV: LanguageCode = LanguageCode {
    name: "Naami",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzv",
    individual_languages: &[
    ],
};


pub const BZW: LanguageCode = LanguageCode {
    name: "Basa (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzw",
    individual_languages: &[
    ],
};


pub const BZX: LanguageCode = LanguageCode {
    name: "Kɛlɛngaxo Bozo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzx",
    individual_languages: &[
    ],
};


pub const BZY: LanguageCode = LanguageCode {
    name: "Obanliku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzy",
    individual_languages: &[
    ],
};


pub const BZZ: LanguageCode = LanguageCode {
    name: "Evant",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "bzz",
    individual_languages: &[
    ],
};


pub const CAA: LanguageCode = LanguageCode {
    name: "Chortí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "caa",
    individual_languages: &[
    ],
};


pub const CAB: LanguageCode = LanguageCode {
    name: "Garifuna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cab",
    individual_languages: &[
    ],
};


pub const CAC: LanguageCode = LanguageCode {
    name: "Chuj",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cac",
    individual_languages: &[
    ],
};


pub const CAE: LanguageCode = LanguageCode {
    name: "Lehar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cae",
    individual_languages: &[
    ],
};


pub const CAF: LanguageCode = LanguageCode {
    name: "Southern Carrier",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "caf",
    individual_languages: &[
    ],
};


pub const CAG: LanguageCode = LanguageCode {
    name: "Nivaclé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cag",
    individual_languages: &[
    ],
};


pub const CAH: LanguageCode = LanguageCode {
    name: "Cahuarano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cah",
    individual_languages: &[
    ],
};


pub const CAJ: LanguageCode = LanguageCode {
    name: "Chané",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "caj",
    individual_languages: &[
    ],
};


pub const CAK: LanguageCode = LanguageCode {
    name: "Kaqchikel",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cak",
    individual_languages: &[
    ],
};


pub const CAL: LanguageCode = LanguageCode {
    name: "Carolinian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cal",
    individual_languages: &[
    ],
};


pub const CAM: LanguageCode = LanguageCode {
    name: "Cemuhî",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cam",
    individual_languages: &[
    ],
};


pub const CAN: LanguageCode = LanguageCode {
    name: "Chambri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "can",
    individual_languages: &[
    ],
};


pub const CAO: LanguageCode = LanguageCode {
    name: "Chácobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cao",
    individual_languages: &[
    ],
};


pub const CAP: LanguageCode = LanguageCode {
    name: "Chipaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cap",
    individual_languages: &[
    ],
};


pub const CAQ: LanguageCode = LanguageCode {
    name: "Car Nicobarese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "caq",
    individual_languages: &[
    ],
};


pub const CAS: LanguageCode = LanguageCode {
    name: "Tsimané",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cas",
    individual_languages: &[
    ],
};


pub const CAV: LanguageCode = LanguageCode {
    name: "Cavineña",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cav",
    individual_languages: &[
    ],
};


pub const CAW: LanguageCode = LanguageCode {
    name: "Callawalla",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "caw",
    individual_languages: &[
    ],
};


pub const CAX: LanguageCode = LanguageCode {
    name: "Chiquitano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cax",
    individual_languages: &[
    ],
};


pub const CAY: LanguageCode = LanguageCode {
    name: "Cayuga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cay",
    individual_languages: &[
    ],
};


pub const CAZ: LanguageCode = LanguageCode {
    name: "Canichana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "caz",
    individual_languages: &[
    ],
};


pub const CBB: LanguageCode = LanguageCode {
    name: "Cabiyarí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbb",
    individual_languages: &[
    ],
};


pub const CBC: LanguageCode = LanguageCode {
    name: "Carapana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbc",
    individual_languages: &[
    ],
};


pub const CBD: LanguageCode = LanguageCode {
    name: "Carijona",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbd",
    individual_languages: &[
    ],
};


pub const CBG: LanguageCode = LanguageCode {
    name: "Chimila",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbg",
    individual_languages: &[
    ],
};


pub const CBI: LanguageCode = LanguageCode {
    name: "Chachi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbi",
    individual_languages: &[
    ],
};


pub const CBJ: LanguageCode = LanguageCode {
    name: "Ede Cabe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbj",
    individual_languages: &[
    ],
};


pub const CBK: LanguageCode = LanguageCode {
    name: "Chavacano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbk",
    individual_languages: &[
    ],
};


pub const CBL: LanguageCode = LanguageCode {
    name: "Bualkhaw Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbl",
    individual_languages: &[
    ],
};


pub const CBN: LanguageCode = LanguageCode {
    name: "Nyahkur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbn",
    individual_languages: &[
    ],
};


pub const CBO: LanguageCode = LanguageCode {
    name: "Izora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbo",
    individual_languages: &[
    ],
};


pub const CBQ: LanguageCode = LanguageCode {
    name: "Tsucuba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbq",
    individual_languages: &[
    ],
};


pub const CBR: LanguageCode = LanguageCode {
    name: "Cashibo-Cacataibo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbr",
    individual_languages: &[
    ],
};


pub const CBS: LanguageCode = LanguageCode {
    name: "Cashinahua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbs",
    individual_languages: &[
    ],
};


pub const CBT: LanguageCode = LanguageCode {
    name: "Chayahuita",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbt",
    individual_languages: &[
    ],
};


pub const CBU: LanguageCode = LanguageCode {
    name: "Candoshi-Shapra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbu",
    individual_languages: &[
    ],
};


pub const CBV: LanguageCode = LanguageCode {
    name: "Cacua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbv",
    individual_languages: &[
    ],
};


pub const CBW: LanguageCode = LanguageCode {
    name: "Kinabalian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cbw",
    individual_languages: &[
    ],
};


pub const CBY: LanguageCode = LanguageCode {
    name: "Carabayo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cby",
    individual_languages: &[
    ],
};


pub const CCC: LanguageCode = LanguageCode {
    name: "Chamicuro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ccc",
    individual_languages: &[
    ],
};


pub const CCD: LanguageCode = LanguageCode {
    name: "Cafundo Creole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ccd",
    individual_languages: &[
    ],
};


pub const CCE: LanguageCode = LanguageCode {
    name: "Chopi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cce",
    individual_languages: &[
    ],
};


pub const CCG: LanguageCode = LanguageCode {
    name: "Samba Daka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ccg",
    individual_languages: &[
    ],
};


pub const CCH: LanguageCode = LanguageCode {
    name: "Atsam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cch",
    individual_languages: &[
    ],
};


pub const CCJ: LanguageCode = LanguageCode {
    name: "Kasanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ccj",
    individual_languages: &[
    ],
};


pub const CCL: LanguageCode = LanguageCode {
    name: "Cutchi-Swahili",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ccl",
    individual_languages: &[
    ],
};


pub const CCM: LanguageCode = LanguageCode {
    name: "Malaccan Creole Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ccm",
    individual_languages: &[
    ],
};


pub const CCO: LanguageCode = LanguageCode {
    name: "Comaltepec Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cco",
    individual_languages: &[
    ],
};


pub const CCP: LanguageCode = LanguageCode {
    name: "Chakma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ccp",
    individual_languages: &[
    ],
};


pub const CCR: LanguageCode = LanguageCode {
    name: "Cacaopera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ccr",
    individual_languages: &[
    ],
};


pub const CDA: LanguageCode = LanguageCode {
    name: "Choni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cda",
    individual_languages: &[
    ],
};


pub const CDE: LanguageCode = LanguageCode {
    name: "Chenchu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cde",
    individual_languages: &[
    ],
};


pub const CDF: LanguageCode = LanguageCode {
    name: "Chiru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cdf",
    individual_languages: &[
    ],
};


pub const CDH: LanguageCode = LanguageCode {
    name: "Chambeali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cdh",
    individual_languages: &[
    ],
};


pub const CDI: LanguageCode = LanguageCode {
    name: "Chodri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cdi",
    individual_languages: &[
    ],
};


pub const CDJ: LanguageCode = LanguageCode {
    name: "Churahi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cdj",
    individual_languages: &[
    ],
};


pub const CDM: LanguageCode = LanguageCode {
    name: "Chepang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cdm",
    individual_languages: &[
    ],
};


pub const CDN: LanguageCode = LanguageCode {
    name: "Chaudangsi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cdn",
    individual_languages: &[
    ],
};


pub const CDO: LanguageCode = LanguageCode {
    name: "Min Dong Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cdo",
    individual_languages: &[
    ],
};


pub const CDR: LanguageCode = LanguageCode {
    name: "Cinda-Regi-Tiyal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cdr",
    individual_languages: &[
    ],
};


pub const CDS: LanguageCode = LanguageCode {
    name: "Chadian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cds",
    individual_languages: &[
    ],
};


pub const CDY: LanguageCode = LanguageCode {
    name: "Chadong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cdy",
    individual_languages: &[
    ],
};


pub const CDZ: LanguageCode = LanguageCode {
    name: "Koda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cdz",
    individual_languages: &[
    ],
};


pub const CEA: LanguageCode = LanguageCode {
    name: "Lower Chehalis",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cea",
    individual_languages: &[
    ],
};


pub const CEG: LanguageCode = LanguageCode {
    name: "Chamacoco",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ceg",
    individual_languages: &[
    ],
};


pub const CEK: LanguageCode = LanguageCode {
    name: "Eastern Khumi Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cek",
    individual_languages: &[
    ],
};


pub const CEN: LanguageCode = LanguageCode {
    name: "Cen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cen",
    individual_languages: &[
    ],
};


pub const CET: LanguageCode = LanguageCode {
    name: "Centúúm",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cet",
    individual_languages: &[
    ],
};


pub const CEY: LanguageCode = LanguageCode {
    name: "Ekai Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cey",
    individual_languages: &[
    ],
};


pub const CFA: LanguageCode = LanguageCode {
    name: "Dijim-Bwilim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cfa",
    individual_languages: &[
    ],
};


pub const CFD: LanguageCode = LanguageCode {
    name: "Cara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cfd",
    individual_languages: &[
    ],
};


pub const CFG: LanguageCode = LanguageCode {
    name: "Como Karim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cfg",
    individual_languages: &[
    ],
};


pub const CFM: LanguageCode = LanguageCode {
    name: "Falam Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cfm",
    individual_languages: &[
    ],
};


pub const CGA: LanguageCode = LanguageCode {
    name: "Changriwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cga",
    individual_languages: &[
    ],
};


pub const CGC: LanguageCode = LanguageCode {
    name: "Kagayanen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cgc",
    individual_languages: &[
    ],
};


pub const CGG: LanguageCode = LanguageCode {
    name: "Chiga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cgg",
    individual_languages: &[
    ],
};


pub const CGK: LanguageCode = LanguageCode {
    name: "Chocangacakha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cgk",
    individual_languages: &[
    ],
};


pub const CHC: LanguageCode = LanguageCode {
    name: "Catawba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "chc",
    individual_languages: &[
    ],
};


pub const CHD: LanguageCode = LanguageCode {
    name: "Highland Oaxaca Chontal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "chd",
    individual_languages: &[
    ],
};


pub const CHF: LanguageCode = LanguageCode {
    name: "Tabasco Chontal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "chf",
    individual_languages: &[
    ],
};


pub const CHH: LanguageCode = LanguageCode {
    name: "Chinook",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "chh",
    individual_languages: &[
    ],
};


pub const CHJ: LanguageCode = LanguageCode {
    name: "Ojitlán Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "chj",
    individual_languages: &[
    ],
};


pub const CHL: LanguageCode = LanguageCode {
    name: "Cahuilla",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "chl",
    individual_languages: &[
    ],
};


pub const CHQ: LanguageCode = LanguageCode {
    name: "Quiotepec Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "chq",
    individual_languages: &[
    ],
};


pub const CHT: LanguageCode = LanguageCode {
    name: "Cholón",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cht",
    individual_languages: &[
    ],
};


pub const CHW: LanguageCode = LanguageCode {
    name: "Chuwabu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "chw",
    individual_languages: &[
    ],
};


pub const CHX: LanguageCode = LanguageCode {
    name: "Chantyal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "chx",
    individual_languages: &[
    ],
};


pub const CHZ: LanguageCode = LanguageCode {
    name: "Ozumacín Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "chz",
    individual_languages: &[
    ],
};


pub const CIA: LanguageCode = LanguageCode {
    name: "Cia-Cia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cia",
    individual_languages: &[
    ],
};


pub const CIB: LanguageCode = LanguageCode {
    name: "Ci Gbe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cib",
    individual_languages: &[
    ],
};


pub const CIC: LanguageCode = LanguageCode {
    name: "Chickasaw",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cic",
    individual_languages: &[
    ],
};


pub const CID: LanguageCode = LanguageCode {
    name: "Chimariko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cid",
    individual_languages: &[
    ],
};


pub const CIE: LanguageCode = LanguageCode {
    name: "Cineni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cie",
    individual_languages: &[
    ],
};


pub const CIH: LanguageCode = LanguageCode {
    name: "Chinali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cih",
    individual_languages: &[
    ],
};


pub const CIK: LanguageCode = LanguageCode {
    name: "Chitkuli Kinnauri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cik",
    individual_languages: &[
    ],
};


pub const CIM: LanguageCode = LanguageCode {
    name: "Cimbrian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cim",
    individual_languages: &[
    ],
};


pub const CIN: LanguageCode = LanguageCode {
    name: "Cinta Larga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cin",
    individual_languages: &[
    ],
};


pub const CIP: LanguageCode = LanguageCode {
    name: "Chiapanec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cip",
    individual_languages: &[
    ],
};


pub const CIR: LanguageCode = LanguageCode {
    name: "Tiri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cir",
    individual_languages: &[
    ],
};


pub const CIW: LanguageCode = LanguageCode {
    name: "Chippewa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ciw",
    individual_languages: &[
    ],
};


pub const CIY: LanguageCode = LanguageCode {
    name: "Chaima",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ciy",
    individual_languages: &[
    ],
};


pub const CJA: LanguageCode = LanguageCode {
    name: "Western Cham",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cja",
    individual_languages: &[
    ],
};


pub const CJE: LanguageCode = LanguageCode {
    name: "Chru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cje",
    individual_languages: &[
    ],
};


pub const CJH: LanguageCode = LanguageCode {
    name: "Upper Chehalis",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cjh",
    individual_languages: &[
    ],
};


pub const CJI: LanguageCode = LanguageCode {
    name: "Chamalal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cji",
    individual_languages: &[
    ],
};


pub const CJK: LanguageCode = LanguageCode {
    name: "Chokwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cjk",
    individual_languages: &[
    ],
};


pub const CJM: LanguageCode = LanguageCode {
    name: "Eastern Cham",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cjm",
    individual_languages: &[
    ],
};


pub const CJN: LanguageCode = LanguageCode {
    name: "Chenapian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cjn",
    individual_languages: &[
    ],
};


pub const CJO: LanguageCode = LanguageCode {
    name: "Ashéninka Pajonal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cjo",
    individual_languages: &[
    ],
};


pub const CJP: LanguageCode = LanguageCode {
    name: "Cabécar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cjp",
    individual_languages: &[
    ],
};


pub const CJS: LanguageCode = LanguageCode {
    name: "Shor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cjs",
    individual_languages: &[
    ],
};


pub const CJV: LanguageCode = LanguageCode {
    name: "Chuave",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cjv",
    individual_languages: &[
    ],
};


pub const CJY: LanguageCode = LanguageCode {
    name: "Jinyu Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cjy",
    individual_languages: &[
    ],
};


pub const CKB: LanguageCode = LanguageCode {
    name: "Central Kurdish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ckb",
    individual_languages: &[
    ],
};


pub const CKH: LanguageCode = LanguageCode {
    name: "Chak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ckh",
    individual_languages: &[
    ],
};


pub const CKL: LanguageCode = LanguageCode {
    name: "Cibak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ckl",
    individual_languages: &[
    ],
};


pub const CKM: LanguageCode = LanguageCode {
    name: "Chakavian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ckm",
    individual_languages: &[
    ],
};


pub const CKN: LanguageCode = LanguageCode {
    name: "Kaang Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ckn",
    individual_languages: &[
    ],
};


pub const CKO: LanguageCode = LanguageCode {
    name: "Anufo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cko",
    individual_languages: &[
    ],
};


pub const CKQ: LanguageCode = LanguageCode {
    name: "Kajakse",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ckq",
    individual_languages: &[
    ],
};


pub const CKR: LanguageCode = LanguageCode {
    name: "Kairak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ckr",
    individual_languages: &[
    ],
};


pub const CKS: LanguageCode = LanguageCode {
    name: "Tayo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cks",
    individual_languages: &[
    ],
};


pub const CKT: LanguageCode = LanguageCode {
    name: "Chukot",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ckt",
    individual_languages: &[
    ],
};


pub const CKU: LanguageCode = LanguageCode {
    name: "Koasati",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cku",
    individual_languages: &[
    ],
};


pub const CKV: LanguageCode = LanguageCode {
    name: "Kavalan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ckv",
    individual_languages: &[
    ],
};


pub const CKX: LanguageCode = LanguageCode {
    name: "Caka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ckx",
    individual_languages: &[
    ],
};


pub const CKY: LanguageCode = LanguageCode {
    name: "Cakfem-Mushere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cky",
    individual_languages: &[
    ],
};


pub const CKZ: LanguageCode = LanguageCode {
    name: "Cakchiquel-Quiché Mixed Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ckz",
    individual_languages: &[
    ],
};


pub const CLA: LanguageCode = LanguageCode {
    name: "Ron",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cla",
    individual_languages: &[
    ],
};


pub const CLC: LanguageCode = LanguageCode {
    name: "Chilcotin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "clc",
    individual_languages: &[
    ],
};


pub const CLD: LanguageCode = LanguageCode {
    name: "Chaldean Neo-Aramaic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cld",
    individual_languages: &[
    ],
};


pub const CLE: LanguageCode = LanguageCode {
    name: "Lealao Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cle",
    individual_languages: &[
    ],
};


pub const CLH: LanguageCode = LanguageCode {
    name: "Chilisso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "clh",
    individual_languages: &[
    ],
};


pub const CLI: LanguageCode = LanguageCode {
    name: "Chakali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cli",
    individual_languages: &[
    ],
};


pub const CLJ: LanguageCode = LanguageCode {
    name: "Laitu Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "clj",
    individual_languages: &[
    ],
};


pub const CLK: LanguageCode = LanguageCode {
    name: "Idu-Mishmi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "clk",
    individual_languages: &[
    ],
};


pub const CLL: LanguageCode = LanguageCode {
    name: "Chala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cll",
    individual_languages: &[
    ],
};


pub const CLM: LanguageCode = LanguageCode {
    name: "Clallam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "clm",
    individual_languages: &[
    ],
};


pub const CLO: LanguageCode = LanguageCode {
    name: "Lowland Oaxaca Chontal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "clo",
    individual_languages: &[
    ],
};


pub const CLT: LanguageCode = LanguageCode {
    name: "Lautu Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "clt",
    individual_languages: &[
    ],
};


pub const CLU: LanguageCode = LanguageCode {
    name: "Caluyanun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "clu",
    individual_languages: &[
    ],
};


pub const CLW: LanguageCode = LanguageCode {
    name: "Chulym",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "clw",
    individual_languages: &[
    ],
};


pub const CLY: LanguageCode = LanguageCode {
    name: "Eastern Highland Chatino",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cly",
    individual_languages: &[
    ],
};


pub const CMA: LanguageCode = LanguageCode {
    name: "Maa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cma",
    individual_languages: &[
    ],
};


pub const CME: LanguageCode = LanguageCode {
    name: "Cerma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cme",
    individual_languages: &[
    ],
};


pub const CMG: LanguageCode = LanguageCode {
    name: "Classical Mongolian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cmg",
    individual_languages: &[
    ],
};


pub const CMI: LanguageCode = LanguageCode {
    name: "Emberá-Chamí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cmi",
    individual_languages: &[
    ],
};


pub const CML: LanguageCode = LanguageCode {
    name: "Campalagian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cml",
    individual_languages: &[
    ],
};


pub const CMM: LanguageCode = LanguageCode {
    name: "Michigamea",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cmm",
    individual_languages: &[
    ],
};


pub const CMN: LanguageCode = LanguageCode {
    name: "Mandarin Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cmn",
    individual_languages: &[
    ],
};


pub const CMO: LanguageCode = LanguageCode {
    name: "Central Mnong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cmo",
    individual_languages: &[
    ],
};


pub const CMR: LanguageCode = LanguageCode {
    name: "Mro-Khimi Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cmr",
    individual_languages: &[
    ],
};


pub const CMS: LanguageCode = LanguageCode {
    name: "Messapic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cms",
    individual_languages: &[
    ],
};


pub const CMT: LanguageCode = LanguageCode {
    name: "Camtho",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cmt",
    individual_languages: &[
    ],
};


pub const CNA: LanguageCode = LanguageCode {
    name: "Changthang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cna",
    individual_languages: &[
    ],
};


pub const CNB: LanguageCode = LanguageCode {
    name: "Chinbon Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cnb",
    individual_languages: &[
    ],
};


pub const CNC: LanguageCode = LanguageCode {
    name: "Côông",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cnc",
    individual_languages: &[
    ],
};


pub const CNG: LanguageCode = LanguageCode {
    name: "Northern Qiang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cng",
    individual_languages: &[
    ],
};


pub const CNH: LanguageCode = LanguageCode {
    name: "Hakha Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cnh",
    individual_languages: &[
    ],
};


pub const CNI: LanguageCode = LanguageCode {
    name: "Asháninka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cni",
    individual_languages: &[
    ],
};


pub const CNK: LanguageCode = LanguageCode {
    name: "Khumi Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cnk",
    individual_languages: &[
    ],
};


pub const CNL: LanguageCode = LanguageCode {
    name: "Lalana Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cnl",
    individual_languages: &[
    ],
};


pub const CNO: LanguageCode = LanguageCode {
    name: "Con",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cno",
    individual_languages: &[
    ],
};


pub const CNP: LanguageCode = LanguageCode {
    name: "Northern Ping Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cnp",
    individual_languages: &[
    ],
};


pub const CNQ: LanguageCode = LanguageCode {
    name: "Chung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cnq",
    individual_languages: &[
    ],
};


pub const CNS: LanguageCode = LanguageCode {
    name: "Central Asmat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cns",
    individual_languages: &[
    ],
};


pub const CNT: LanguageCode = LanguageCode {
    name: "Tepetotutla Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cnt",
    individual_languages: &[
    ],
};


pub const CNU: LanguageCode = LanguageCode {
    name: "Chenoua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cnu",
    individual_languages: &[
    ],
};


pub const CNW: LanguageCode = LanguageCode {
    name: "Ngawn Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cnw",
    individual_languages: &[
    ],
};


pub const CNX: LanguageCode = LanguageCode {
    name: "Middle Cornish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cnx",
    individual_languages: &[
    ],
};


pub const COA: LanguageCode = LanguageCode {
    name: "Cocos Islands Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "coa",
    individual_languages: &[
    ],
};


pub const COB: LanguageCode = LanguageCode {
    name: "Chicomuceltec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cob",
    individual_languages: &[
    ],
};


pub const COC: LanguageCode = LanguageCode {
    name: "Cocopa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "coc",
    individual_languages: &[
    ],
};


pub const COD: LanguageCode = LanguageCode {
    name: "Cocama-Cocamilla",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cod",
    individual_languages: &[
    ],
};


pub const COE: LanguageCode = LanguageCode {
    name: "Koreguaje",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "coe",
    individual_languages: &[
    ],
};


pub const COF: LanguageCode = LanguageCode {
    name: "Colorado",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cof",
    individual_languages: &[
    ],
};


pub const COG: LanguageCode = LanguageCode {
    name: "Chong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cog",
    individual_languages: &[
    ],
};


pub const COH: LanguageCode = LanguageCode {
    name: "Chonyi-Dzihana-Kauma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "coh",
    individual_languages: &[
    ],
};


pub const COJ: LanguageCode = LanguageCode {
    name: "Cochimi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "coj",
    individual_languages: &[
    ],
};


pub const COK: LanguageCode = LanguageCode {
    name: "Santa Teresa Cora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cok",
    individual_languages: &[
    ],
};


pub const COL: LanguageCode = LanguageCode {
    name: "Columbia-Wenatchi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "col",
    individual_languages: &[
    ],
};


pub const COM: LanguageCode = LanguageCode {
    name: "Comanche",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "com",
    individual_languages: &[
    ],
};


pub const CON: LanguageCode = LanguageCode {
    name: "Cofán",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "con",
    individual_languages: &[
    ],
};


pub const COO: LanguageCode = LanguageCode {
    name: "Comox",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "coo",
    individual_languages: &[
    ],
};


pub const COQ: LanguageCode = LanguageCode {
    name: "Coquille",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "coq",
    individual_languages: &[
    ],
};


pub const COT: LanguageCode = LanguageCode {
    name: "Caquinte",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cot",
    individual_languages: &[
    ],
};


pub const COU: LanguageCode = LanguageCode {
    name: "Wamey",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cou",
    individual_languages: &[
    ],
};


pub const COV: LanguageCode = LanguageCode {
    name: "Cao Miao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cov",
    individual_languages: &[
    ],
};


pub const COW: LanguageCode = LanguageCode {
    name: "Cowlitz",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cow",
    individual_languages: &[
    ],
};


pub const COX: LanguageCode = LanguageCode {
    name: "Nanti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cox",
    individual_languages: &[
    ],
};


pub const COZ: LanguageCode = LanguageCode {
    name: "Chochotec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "coz",
    individual_languages: &[
    ],
};


pub const CPA: LanguageCode = LanguageCode {
    name: "Palantla Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cpa",
    individual_languages: &[
    ],
};


pub const CPB: LanguageCode = LanguageCode {
    name: "Ucayali-Yurúa Ashéninka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cpb",
    individual_languages: &[
    ],
};


pub const CPC: LanguageCode = LanguageCode {
    name: "Ajyíninka Apurucayali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cpc",
    individual_languages: &[
    ],
};


pub const CPG: LanguageCode = LanguageCode {
    name: "Cappadocian Greek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cpg",
    individual_languages: &[
    ],
};


pub const CPI: LanguageCode = LanguageCode {
    name: "Chinese Pidgin English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cpi",
    individual_languages: &[
    ],
};


pub const CPN: LanguageCode = LanguageCode {
    name: "Cherepon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cpn",
    individual_languages: &[
    ],
};


pub const CPO: LanguageCode = LanguageCode {
    name: "Kpeego",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cpo",
    individual_languages: &[
    ],
};


pub const CPS: LanguageCode = LanguageCode {
    name: "Capiznon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cps",
    individual_languages: &[
    ],
};


pub const CPU: LanguageCode = LanguageCode {
    name: "Pichis Ashéninka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cpu",
    individual_languages: &[
    ],
};


pub const CPX: LanguageCode = LanguageCode {
    name: "Pu-Xian Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cpx",
    individual_languages: &[
    ],
};


pub const CPY: LanguageCode = LanguageCode {
    name: "South Ucayali Ashéninka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cpy",
    individual_languages: &[
    ],
};


pub const CQD: LanguageCode = LanguageCode {
    name: "Chuanqiandian Cluster Miao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cqd",
    individual_languages: &[
    ],
};


pub const CRA: LanguageCode = LanguageCode {
    name: "Chara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cra",
    individual_languages: &[
    ],
};


pub const CRB: LanguageCode = LanguageCode {
    name: "Island Carib",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crb",
    individual_languages: &[
    ],
};


pub const CRC: LanguageCode = LanguageCode {
    name: "Lonwolwol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crc",
    individual_languages: &[
    ],
};


pub const CRD: LanguageCode = LanguageCode {
    name: "Coeur d'Alene",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crd",
    individual_languages: &[
    ],
};


pub const CRF: LanguageCode = LanguageCode {
    name: "Caramanta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crf",
    individual_languages: &[
    ],
};


pub const CRG: LanguageCode = LanguageCode {
    name: "Michif",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crg",
    individual_languages: &[
    ],
};


pub const CRI: LanguageCode = LanguageCode {
    name: "Sãotomense",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cri",
    individual_languages: &[
    ],
};


pub const CRJ: LanguageCode = LanguageCode {
    name: "Southern East Cree",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crj",
    individual_languages: &[
    ],
};


pub const CRK: LanguageCode = LanguageCode {
    name: "Plains Cree",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crk",
    individual_languages: &[
    ],
};


pub const CRL: LanguageCode = LanguageCode {
    name: "Northern East Cree",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crl",
    individual_languages: &[
    ],
};


pub const CRM: LanguageCode = LanguageCode {
    name: "Moose Cree",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crm",
    individual_languages: &[
    ],
};


pub const CRN: LanguageCode = LanguageCode {
    name: "El Nayar Cora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crn",
    individual_languages: &[
    ],
};


pub const CRO: LanguageCode = LanguageCode {
    name: "Crow",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cro",
    individual_languages: &[
    ],
};


pub const CRQ: LanguageCode = LanguageCode {
    name: "Iyo'wujwa Chorote",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crq",
    individual_languages: &[
    ],
};


pub const CRR: LanguageCode = LanguageCode {
    name: "Carolina Algonquian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crr",
    individual_languages: &[
    ],
};


pub const CRS: LanguageCode = LanguageCode {
    name: "Seselwa Creole French",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crs",
    individual_languages: &[
    ],
};


pub const CRT: LanguageCode = LanguageCode {
    name: "Iyojwa'ja Chorote",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crt",
    individual_languages: &[
    ],
};


pub const CRV: LanguageCode = LanguageCode {
    name: "Chaura",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crv",
    individual_languages: &[
    ],
};


pub const CRW: LanguageCode = LanguageCode {
    name: "Chrau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crw",
    individual_languages: &[
    ],
};


pub const CRX: LanguageCode = LanguageCode {
    name: "Carrier",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crx",
    individual_languages: &[
    ],
};


pub const CRY: LanguageCode = LanguageCode {
    name: "Cori",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cry",
    individual_languages: &[
    ],
};


pub const CRZ: LanguageCode = LanguageCode {
    name: "Cruzeño",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "crz",
    individual_languages: &[
    ],
};


pub const CSA: LanguageCode = LanguageCode {
    name: "Chiltepec Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csa",
    individual_languages: &[
    ],
};


pub const CSC: LanguageCode = LanguageCode {
    name: "Catalan Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csc",
    individual_languages: &[
    ],
};


pub const CSD: LanguageCode = LanguageCode {
    name: "Chiangmai Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csd",
    individual_languages: &[
    ],
};


pub const CSE: LanguageCode = LanguageCode {
    name: "Czech Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cse",
    individual_languages: &[
    ],
};


pub const CSF: LanguageCode = LanguageCode {
    name: "Cuba Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csf",
    individual_languages: &[
    ],
};


pub const CSG: LanguageCode = LanguageCode {
    name: "Chilean Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csg",
    individual_languages: &[
    ],
};


pub const CSH: LanguageCode = LanguageCode {
    name: "Asho Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csh",
    individual_languages: &[
    ],
};


pub const CSI: LanguageCode = LanguageCode {
    name: "Coast Miwok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csi",
    individual_languages: &[
    ],
};


pub const CSJ: LanguageCode = LanguageCode {
    name: "Songlai Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csj",
    individual_languages: &[
    ],
};


pub const CSK: LanguageCode = LanguageCode {
    name: "Jola-Kasa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csk",
    individual_languages: &[
    ],
};


pub const CSL: LanguageCode = LanguageCode {
    name: "Chinese Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csl",
    individual_languages: &[
    ],
};


pub const CSM: LanguageCode = LanguageCode {
    name: "Central Sierra Miwok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csm",
    individual_languages: &[
    ],
};


pub const CSN: LanguageCode = LanguageCode {
    name: "Colombian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csn",
    individual_languages: &[
    ],
};


pub const CSO: LanguageCode = LanguageCode {
    name: "Sochiapam Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cso",
    individual_languages: &[
    ],
};


pub const CSP: LanguageCode = LanguageCode {
    name: "Southern Ping Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csp",
    individual_languages: &[
    ],
};


pub const CSQ: LanguageCode = LanguageCode {
    name: "Croatia Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csq",
    individual_languages: &[
    ],
};


pub const CSR: LanguageCode = LanguageCode {
    name: "Costa Rican Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csr",
    individual_languages: &[
    ],
};


pub const CSS: LanguageCode = LanguageCode {
    name: "Southern Ohlone",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "css",
    individual_languages: &[
    ],
};


pub const CST: LanguageCode = LanguageCode {
    name: "Northern Ohlone",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cst",
    individual_languages: &[
    ],
};


pub const CSV: LanguageCode = LanguageCode {
    name: "Sumtu Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csv",
    individual_languages: &[
    ],
};


pub const CSW: LanguageCode = LanguageCode {
    name: "Swampy Cree",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csw",
    individual_languages: &[
    ],
};


pub const CSX: LanguageCode = LanguageCode {
    name: "Cambodian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csx",
    individual_languages: &[
    ],
};


pub const CSY: LanguageCode = LanguageCode {
    name: "Siyin Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csy",
    individual_languages: &[
    ],
};


pub const CSZ: LanguageCode = LanguageCode {
    name: "Coos",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "csz",
    individual_languages: &[
    ],
};


pub const CTA: LanguageCode = LanguageCode {
    name: "Tataltepec Chatino",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cta",
    individual_languages: &[
    ],
};


pub const CTC: LanguageCode = LanguageCode {
    name: "Chetco",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ctc",
    individual_languages: &[
    ],
};


pub const CTD: LanguageCode = LanguageCode {
    name: "Tedim Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ctd",
    individual_languages: &[
    ],
};


pub const CTE: LanguageCode = LanguageCode {
    name: "Tepinapa Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cte",
    individual_languages: &[
    ],
};


pub const CTG: LanguageCode = LanguageCode {
    name: "Chittagonian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ctg",
    individual_languages: &[
    ],
};


pub const CTH: LanguageCode = LanguageCode {
    name: "Thaiphum Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cth",
    individual_languages: &[
    ],
};


pub const CTL: LanguageCode = LanguageCode {
    name: "Tlacoatzintepec Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ctl",
    individual_languages: &[
    ],
};


pub const CTM: LanguageCode = LanguageCode {
    name: "Chitimacha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ctm",
    individual_languages: &[
    ],
};


pub const CTN: LanguageCode = LanguageCode {
    name: "Chhintange",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ctn",
    individual_languages: &[
    ],
};


pub const CTO: LanguageCode = LanguageCode {
    name: "Emberá-Catío",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cto",
    individual_languages: &[
    ],
};


pub const CTP: LanguageCode = LanguageCode {
    name: "Western Highland Chatino",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ctp",
    individual_languages: &[
    ],
};


pub const CTS: LanguageCode = LanguageCode {
    name: "Northern Catanduanes Bikol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cts",
    individual_languages: &[
    ],
};


pub const CTT: LanguageCode = LanguageCode {
    name: "Wayanad Chetti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ctt",
    individual_languages: &[
    ],
};


pub const CTU: LanguageCode = LanguageCode {
    name: "Chol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ctu",
    individual_languages: &[
    ],
};


pub const CTY: LanguageCode = LanguageCode {
    name: "Moundadan Chetty",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cty",
    individual_languages: &[
    ],
};


pub const CTZ: LanguageCode = LanguageCode {
    name: "Zacatepec Chatino",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ctz",
    individual_languages: &[
    ],
};


pub const CUA: LanguageCode = LanguageCode {
    name: "Cua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cua",
    individual_languages: &[
    ],
};


pub const CUB: LanguageCode = LanguageCode {
    name: "Cubeo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cub",
    individual_languages: &[
    ],
};


pub const CUC: LanguageCode = LanguageCode {
    name: "Usila Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cuc",
    individual_languages: &[
    ],
};


pub const CUH: LanguageCode = LanguageCode {
    name: "Chuka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cuh",
    individual_languages: &[
    ],
};


pub const CUI: LanguageCode = LanguageCode {
    name: "Cuiba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cui",
    individual_languages: &[
    ],
};


pub const CUJ: LanguageCode = LanguageCode {
    name: "Mashco Piro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cuj",
    individual_languages: &[
    ],
};


pub const CUK: LanguageCode = LanguageCode {
    name: "San Blas Kuna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cuk",
    individual_languages: &[
    ],
};


pub const CUL: LanguageCode = LanguageCode {
    name: "Culina",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cul",
    individual_languages: &[
    ],
};


pub const CUO: LanguageCode = LanguageCode {
    name: "Cumanagoto",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cuo",
    individual_languages: &[
    ],
};


pub const CUP: LanguageCode = LanguageCode {
    name: "Cupeño",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cup",
    individual_languages: &[
    ],
};


pub const CUQ: LanguageCode = LanguageCode {
    name: "Cun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cuq",
    individual_languages: &[
    ],
};


pub const CUR: LanguageCode = LanguageCode {
    name: "Chhulung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cur",
    individual_languages: &[
    ],
};


pub const CUT: LanguageCode = LanguageCode {
    name: "Teutila Cuicatec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cut",
    individual_languages: &[
    ],
};


pub const CUU: LanguageCode = LanguageCode {
    name: "Tai Ya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cuu",
    individual_languages: &[
    ],
};


pub const CUV: LanguageCode = LanguageCode {
    name: "Cuvok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cuv",
    individual_languages: &[
    ],
};


pub const CUW: LanguageCode = LanguageCode {
    name: "Chukwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cuw",
    individual_languages: &[
    ],
};


pub const CUX: LanguageCode = LanguageCode {
    name: "Tepeuxila Cuicatec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cux",
    individual_languages: &[
    ],
};


pub const CUY: LanguageCode = LanguageCode {
    name: "Cuitlatec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cuy",
    individual_languages: &[
    ],
};


pub const CVG: LanguageCode = LanguageCode {
    name: "Chug",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cvg",
    individual_languages: &[
    ],
};


pub const CVN: LanguageCode = LanguageCode {
    name: "Valle Nacional Chinantec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cvn",
    individual_languages: &[
    ],
};


pub const CWA: LanguageCode = LanguageCode {
    name: "Kabwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cwa",
    individual_languages: &[
    ],
};


pub const CWB: LanguageCode = LanguageCode {
    name: "Maindo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cwb",
    individual_languages: &[
    ],
};


pub const CWD: LanguageCode = LanguageCode {
    name: "Woods Cree",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cwd",
    individual_languages: &[
    ],
};


pub const CWE: LanguageCode = LanguageCode {
    name: "Kwere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cwe",
    individual_languages: &[
    ],
};


pub const CWG: LanguageCode = LanguageCode {
    name: "Chewong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cwg",
    individual_languages: &[
    ],
};


pub const CWT: LanguageCode = LanguageCode {
    name: "Kuwaataay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cwt",
    individual_languages: &[
    ],
};


pub const CYA: LanguageCode = LanguageCode {
    name: "Nopala Chatino",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cya",
    individual_languages: &[
    ],
};


pub const CYB: LanguageCode = LanguageCode {
    name: "Cayubaba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cyb",
    individual_languages: &[
    ],
};


pub const CYO: LanguageCode = LanguageCode {
    name: "Cuyonon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "cyo",
    individual_languages: &[
    ],
};


pub const CZH: LanguageCode = LanguageCode {
    name: "Huizhou Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "czh",
    individual_languages: &[
    ],
};


pub const CZK: LanguageCode = LanguageCode {
    name: "Knaanic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "czk",
    individual_languages: &[
    ],
};


pub const CZN: LanguageCode = LanguageCode {
    name: "Zenzontepec Chatino",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "czn",
    individual_languages: &[
    ],
};


pub const CZO: LanguageCode = LanguageCode {
    name: "Min Zhong Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "czo",
    individual_languages: &[
    ],
};


pub const CZT: LanguageCode = LanguageCode {
    name: "Zotung Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "czt",
    individual_languages: &[
    ],
};


pub const DAA: LanguageCode = LanguageCode {
    name: "Dangaléat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "daa",
    individual_languages: &[
    ],
};


pub const DAC: LanguageCode = LanguageCode {
    name: "Dambi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dac",
    individual_languages: &[
    ],
};


pub const DAD: LanguageCode = LanguageCode {
    name: "Marik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dad",
    individual_languages: &[
    ],
};


pub const DAE: LanguageCode = LanguageCode {
    name: "Duupa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dae",
    individual_languages: &[
    ],
};


pub const DAG: LanguageCode = LanguageCode {
    name: "Dagbani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dag",
    individual_languages: &[
    ],
};


pub const DAH: LanguageCode = LanguageCode {
    name: "Gwahatike",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dah",
    individual_languages: &[
    ],
};


pub const DAI: LanguageCode = LanguageCode {
    name: "Day",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dai",
    individual_languages: &[
    ],
};


pub const DAJ: LanguageCode = LanguageCode {
    name: "Dar Fur Daju",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "daj",
    individual_languages: &[
    ],
};


pub const DAL: LanguageCode = LanguageCode {
    name: "Dahalo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dal",
    individual_languages: &[
    ],
};


pub const DAM: LanguageCode = LanguageCode {
    name: "Damakawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dam",
    individual_languages: &[
    ],
};


pub const DAO: LanguageCode = LanguageCode {
    name: "Daai Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dao",
    individual_languages: &[
    ],
};


pub const DAQ: LanguageCode = LanguageCode {
    name: "Dandami Maria",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "daq",
    individual_languages: &[
    ],
};


pub const DAS: LanguageCode = LanguageCode {
    name: "Daho-Doo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "das",
    individual_languages: &[
    ],
};


pub const DAU: LanguageCode = LanguageCode {
    name: "Dar Sila Daju",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dau",
    individual_languages: &[
    ],
};


pub const DAV: LanguageCode = LanguageCode {
    name: "Taita",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dav",
    individual_languages: &[
    ],
};


pub const DAW: LanguageCode = LanguageCode {
    name: "Davawenyo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "daw",
    individual_languages: &[
    ],
};


pub const DAX: LanguageCode = LanguageCode {
    name: "Dayi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dax",
    individual_languages: &[
    ],
};


pub const DAZ: LanguageCode = LanguageCode {
    name: "Dao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "daz",
    individual_languages: &[
    ],
};


pub const DBA: LanguageCode = LanguageCode {
    name: "Bangime",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dba",
    individual_languages: &[
    ],
};


pub const DBB: LanguageCode = LanguageCode {
    name: "Deno",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbb",
    individual_languages: &[
    ],
};


pub const DBD: LanguageCode = LanguageCode {
    name: "Dadiya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbd",
    individual_languages: &[
    ],
};


pub const DBE: LanguageCode = LanguageCode {
    name: "Dabe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbe",
    individual_languages: &[
    ],
};


pub const DBF: LanguageCode = LanguageCode {
    name: "Edopi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbf",
    individual_languages: &[
    ],
};


pub const DBG: LanguageCode = LanguageCode {
    name: "Dogul Dom Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbg",
    individual_languages: &[
    ],
};


pub const DBI: LanguageCode = LanguageCode {
    name: "Doka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbi",
    individual_languages: &[
    ],
};


pub const DBJ: LanguageCode = LanguageCode {
    name: "Ida'an",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbj",
    individual_languages: &[
    ],
};


pub const DBL: LanguageCode = LanguageCode {
    name: "Dyirbal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbl",
    individual_languages: &[
    ],
};


pub const DBM: LanguageCode = LanguageCode {
    name: "Duguri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbm",
    individual_languages: &[
    ],
};


pub const DBN: LanguageCode = LanguageCode {
    name: "Duriankere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbn",
    individual_languages: &[
    ],
};


pub const DBO: LanguageCode = LanguageCode {
    name: "Dulbu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbo",
    individual_languages: &[
    ],
};


pub const DBP: LanguageCode = LanguageCode {
    name: "Duwai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbp",
    individual_languages: &[
    ],
};


pub const DBQ: LanguageCode = LanguageCode {
    name: "Daba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbq",
    individual_languages: &[
    ],
};


pub const DBR: LanguageCode = LanguageCode {
    name: "Dabarre",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbr",
    individual_languages: &[
    ],
};


pub const DBT: LanguageCode = LanguageCode {
    name: "Ben Tey Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbt",
    individual_languages: &[
    ],
};


pub const DBU: LanguageCode = LanguageCode {
    name: "Bondum Dom Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbu",
    individual_languages: &[
    ],
};


pub const DBV: LanguageCode = LanguageCode {
    name: "Dungu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbv",
    individual_languages: &[
    ],
};


pub const DBW: LanguageCode = LanguageCode {
    name: "Bankan Tey Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dbw",
    individual_languages: &[
    ],
};


pub const DBY: LanguageCode = LanguageCode {
    name: "Dibiyaso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dby",
    individual_languages: &[
    ],
};


pub const DCC: LanguageCode = LanguageCode {
    name: "Deccan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dcc",
    individual_languages: &[
    ],
};


pub const DCR: LanguageCode = LanguageCode {
    name: "Negerhollands",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dcr",
    individual_languages: &[
    ],
};


pub const DDA: LanguageCode = LanguageCode {
    name: "Dadi Dadi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dda",
    individual_languages: &[
    ],
};


pub const DDD: LanguageCode = LanguageCode {
    name: "Dongotono",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ddd",
    individual_languages: &[
    ],
};


pub const DDE: LanguageCode = LanguageCode {
    name: "Doondo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dde",
    individual_languages: &[
    ],
};


pub const DDG: LanguageCode = LanguageCode {
    name: "Fataluku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ddg",
    individual_languages: &[
    ],
};


pub const DDI: LanguageCode = LanguageCode {
    name: "West Goodenough",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ddi",
    individual_languages: &[
    ],
};


pub const DDJ: LanguageCode = LanguageCode {
    name: "Jaru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ddj",
    individual_languages: &[
    ],
};


pub const DDN: LanguageCode = LanguageCode {
    name: "Dendi (Benin)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ddn",
    individual_languages: &[
    ],
};


pub const DDO: LanguageCode = LanguageCode {
    name: "Dido",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ddo",
    individual_languages: &[
    ],
};


pub const DDR: LanguageCode = LanguageCode {
    name: "Dhudhuroa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ddr",
    individual_languages: &[
    ],
};


pub const DDS: LanguageCode = LanguageCode {
    name: "Donno So Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dds",
    individual_languages: &[
    ],
};


pub const DDW: LanguageCode = LanguageCode {
    name: "Dawera-Daweloor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ddw",
    individual_languages: &[
    ],
};


pub const DEC: LanguageCode = LanguageCode {
    name: "Dagik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dec",
    individual_languages: &[
    ],
};


pub const DED: LanguageCode = LanguageCode {
    name: "Dedua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ded",
    individual_languages: &[
    ],
};


pub const DEE: LanguageCode = LanguageCode {
    name: "Dewoin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dee",
    individual_languages: &[
    ],
};


pub const DEF: LanguageCode = LanguageCode {
    name: "Dezfuli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "def",
    individual_languages: &[
    ],
};


pub const DEG: LanguageCode = LanguageCode {
    name: "Degema",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "deg",
    individual_languages: &[
    ],
};


pub const DEH: LanguageCode = LanguageCode {
    name: "Dehwari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "deh",
    individual_languages: &[
    ],
};


pub const DEI: LanguageCode = LanguageCode {
    name: "Demisa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dei",
    individual_languages: &[
    ],
};


pub const DEK: LanguageCode = LanguageCode {
    name: "Dek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dek",
    individual_languages: &[
    ],
};


pub const DEM: LanguageCode = LanguageCode {
    name: "Dem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dem",
    individual_languages: &[
    ],
};


pub const DEP: LanguageCode = LanguageCode {
    name: "Pidgin Delaware",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dep",
    individual_languages: &[
    ],
};


pub const DEQ: LanguageCode = LanguageCode {
    name: "Dendi (Central African Republic)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "deq",
    individual_languages: &[
    ],
};


pub const DER: LanguageCode = LanguageCode {
    name: "Deori",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "der",
    individual_languages: &[
    ],
};


pub const DES: LanguageCode = LanguageCode {
    name: "Desano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "des",
    individual_languages: &[
    ],
};


pub const DEV: LanguageCode = LanguageCode {
    name: "Domung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dev",
    individual_languages: &[
    ],
};


pub const DEZ: LanguageCode = LanguageCode {
    name: "Dengese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dez",
    individual_languages: &[
    ],
};


pub const DGA: LanguageCode = LanguageCode {
    name: "Southern Dagaare",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dga",
    individual_languages: &[
    ],
};


pub const DGB: LanguageCode = LanguageCode {
    name: "Bunoge Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgb",
    individual_languages: &[
    ],
};


pub const DGC: LanguageCode = LanguageCode {
    name: "Casiguran Dumagat Agta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgc",
    individual_languages: &[
    ],
};


pub const DGD: LanguageCode = LanguageCode {
    name: "Dagaari Dioula",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgd",
    individual_languages: &[
    ],
};


pub const DGE: LanguageCode = LanguageCode {
    name: "Degenan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dge",
    individual_languages: &[
    ],
};


pub const DGG: LanguageCode = LanguageCode {
    name: "Doga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgg",
    individual_languages: &[
    ],
};


pub const DGH: LanguageCode = LanguageCode {
    name: "Dghwede",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgh",
    individual_languages: &[
    ],
};


pub const DGI: LanguageCode = LanguageCode {
    name: "Northern Dagara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgi",
    individual_languages: &[
    ],
};


pub const DGK: LanguageCode = LanguageCode {
    name: "Dagba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgk",
    individual_languages: &[
    ],
};


pub const DGL: LanguageCode = LanguageCode {
    name: "Andaandi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgl",
    individual_languages: &[
    ],
};


pub const DGN: LanguageCode = LanguageCode {
    name: "Dagoman",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgn",
    individual_languages: &[
    ],
};


pub const DGO: LanguageCode = LanguageCode {
    name: "Dogri (individual language)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgo",
    individual_languages: &[
    ],
};


pub const DGS: LanguageCode = LanguageCode {
    name: "Dogoso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgs",
    individual_languages: &[
    ],
};


pub const DGT: LanguageCode = LanguageCode {
    name: "Ndra'ngith",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgt",
    individual_languages: &[
    ],
};


pub const DGW: LanguageCode = LanguageCode {
    name: "Daungwurrung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgw",
    individual_languages: &[
    ],
};


pub const DGX: LanguageCode = LanguageCode {
    name: "Doghoro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgx",
    individual_languages: &[
    ],
};


pub const DGZ: LanguageCode = LanguageCode {
    name: "Daga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dgz",
    individual_languages: &[
    ],
};


pub const DHD: LanguageCode = LanguageCode {
    name: "Dhundari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dhd",
    individual_languages: &[
    ],
};


pub const DHG: LanguageCode = LanguageCode {
    name: "Dhangu-Djangu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dhg",
    individual_languages: &[
    ],
};


pub const DHI: LanguageCode = LanguageCode {
    name: "Dhimal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dhi",
    individual_languages: &[
    ],
};


pub const DHL: LanguageCode = LanguageCode {
    name: "Dhalandji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dhl",
    individual_languages: &[
    ],
};


pub const DHM: LanguageCode = LanguageCode {
    name: "Zemba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dhm",
    individual_languages: &[
    ],
};


pub const DHN: LanguageCode = LanguageCode {
    name: "Dhanki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dhn",
    individual_languages: &[
    ],
};


pub const DHO: LanguageCode = LanguageCode {
    name: "Dhodia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dho",
    individual_languages: &[
    ],
};


pub const DHR: LanguageCode = LanguageCode {
    name: "Dhargari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dhr",
    individual_languages: &[
    ],
};


pub const DHS: LanguageCode = LanguageCode {
    name: "Dhaiso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dhs",
    individual_languages: &[
    ],
};


pub const DHU: LanguageCode = LanguageCode {
    name: "Dhurga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dhu",
    individual_languages: &[
    ],
};


pub const DHV: LanguageCode = LanguageCode {
    name: "Dehu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dhv",
    individual_languages: &[
    ],
};


pub const DHW: LanguageCode = LanguageCode {
    name: "Dhanwar (Nepal)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dhw",
    individual_languages: &[
    ],
};


pub const DHX: LanguageCode = LanguageCode {
    name: "Dhungaloo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dhx",
    individual_languages: &[
    ],
};


pub const DIA: LanguageCode = LanguageCode {
    name: "Dia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dia",
    individual_languages: &[
    ],
};


pub const DIB: LanguageCode = LanguageCode {
    name: "South Central Dinka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dib",
    individual_languages: &[
    ],
};


pub const DIC: LanguageCode = LanguageCode {
    name: "Lakota Dida",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dic",
    individual_languages: &[
    ],
};


pub const DID: LanguageCode = LanguageCode {
    name: "Didinga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "did",
    individual_languages: &[
    ],
};


pub const DIF: LanguageCode = LanguageCode {
    name: "Dieri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dif",
    individual_languages: &[
    ],
};


pub const DIG: LanguageCode = LanguageCode {
    name: "Digo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dig",
    individual_languages: &[
    ],
};


pub const DIH: LanguageCode = LanguageCode {
    name: "Kumiai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dih",
    individual_languages: &[
    ],
};


pub const DII: LanguageCode = LanguageCode {
    name: "Dimbong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dii",
    individual_languages: &[
    ],
};


pub const DIJ: LanguageCode = LanguageCode {
    name: "Dai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dij",
    individual_languages: &[
    ],
};


pub const DIK: LanguageCode = LanguageCode {
    name: "Southwestern Dinka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dik",
    individual_languages: &[
    ],
};


pub const DIL: LanguageCode = LanguageCode {
    name: "Dilling",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dil",
    individual_languages: &[
    ],
};


pub const DIM: LanguageCode = LanguageCode {
    name: "Dime",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dim",
    individual_languages: &[
    ],
};


pub const DIO: LanguageCode = LanguageCode {
    name: "Dibo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dio",
    individual_languages: &[
    ],
};


pub const DIP: LanguageCode = LanguageCode {
    name: "Northeastern Dinka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dip",
    individual_languages: &[
    ],
};


pub const DIQ: LanguageCode = LanguageCode {
    name: "Dimli (individual language)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "diq",
    individual_languages: &[
    ],
};


pub const DIR: LanguageCode = LanguageCode {
    name: "Dirim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dir",
    individual_languages: &[
    ],
};


pub const DIS: LanguageCode = LanguageCode {
    name: "Dimasa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dis",
    individual_languages: &[
    ],
};


pub const DIU: LanguageCode = LanguageCode {
    name: "Diriku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "diu",
    individual_languages: &[
    ],
};


pub const DIW: LanguageCode = LanguageCode {
    name: "Northwestern Dinka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "diw",
    individual_languages: &[
    ],
};


pub const DIX: LanguageCode = LanguageCode {
    name: "Dixon Reef",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dix",
    individual_languages: &[
    ],
};


pub const DIY: LanguageCode = LanguageCode {
    name: "Diuwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "diy",
    individual_languages: &[
    ],
};


pub const DIZ: LanguageCode = LanguageCode {
    name: "Ding",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "diz",
    individual_languages: &[
    ],
};


pub const DJA: LanguageCode = LanguageCode {
    name: "Djadjawurrung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dja",
    individual_languages: &[
    ],
};


pub const DJB: LanguageCode = LanguageCode {
    name: "Djinba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "djb",
    individual_languages: &[
    ],
};


pub const DJC: LanguageCode = LanguageCode {
    name: "Dar Daju Daju",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "djc",
    individual_languages: &[
    ],
};


pub const DJD: LanguageCode = LanguageCode {
    name: "Djamindjung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "djd",
    individual_languages: &[
    ],
};


pub const DJE: LanguageCode = LanguageCode {
    name: "Zarma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dje",
    individual_languages: &[
    ],
};


pub const DJF: LanguageCode = LanguageCode {
    name: "Djangun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "djf",
    individual_languages: &[
    ],
};


pub const DJI: LanguageCode = LanguageCode {
    name: "Djinang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dji",
    individual_languages: &[
    ],
};


pub const DJJ: LanguageCode = LanguageCode {
    name: "Djeebbana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "djj",
    individual_languages: &[
    ],
};


pub const DJK: LanguageCode = LanguageCode {
    name: "Eastern Maroon Creole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "djk",
    individual_languages: &[
    ],
};


pub const DJM: LanguageCode = LanguageCode {
    name: "Jamsay Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "djm",
    individual_languages: &[
    ],
};


pub const DJN: LanguageCode = LanguageCode {
    name: "Jawoyn",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "djn",
    individual_languages: &[
    ],
};


pub const DJO: LanguageCode = LanguageCode {
    name: "Jangkang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "djo",
    individual_languages: &[
    ],
};


pub const DJR: LanguageCode = LanguageCode {
    name: "Djambarrpuyngu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "djr",
    individual_languages: &[
    ],
};


pub const DJU: LanguageCode = LanguageCode {
    name: "Kapriman",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dju",
    individual_languages: &[
    ],
};


pub const DJW: LanguageCode = LanguageCode {
    name: "Djawi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "djw",
    individual_languages: &[
    ],
};


pub const DKA: LanguageCode = LanguageCode {
    name: "Dakpakha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dka",
    individual_languages: &[
    ],
};


pub const DKG: LanguageCode = LanguageCode {
    name: "Kadung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dkg",
    individual_languages: &[
    ],
};


pub const DKK: LanguageCode = LanguageCode {
    name: "Dakka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dkk",
    individual_languages: &[
    ],
};


pub const DKR: LanguageCode = LanguageCode {
    name: "Kuijau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dkr",
    individual_languages: &[
    ],
};


pub const DKS: LanguageCode = LanguageCode {
    name: "Southeastern Dinka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dks",
    individual_languages: &[
    ],
};


pub const DKX: LanguageCode = LanguageCode {
    name: "Mazagway",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dkx",
    individual_languages: &[
    ],
};


pub const DLG: LanguageCode = LanguageCode {
    name: "Dolgan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dlg",
    individual_languages: &[
    ],
};


pub const DLK: LanguageCode = LanguageCode {
    name: "Dahalik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dlk",
    individual_languages: &[
    ],
};


pub const DLM: LanguageCode = LanguageCode {
    name: "Dalmatian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dlm",
    individual_languages: &[
    ],
};


pub const DLN: LanguageCode = LanguageCode {
    name: "Darlong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dln",
    individual_languages: &[
    ],
};


pub const DMA: LanguageCode = LanguageCode {
    name: "Duma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dma",
    individual_languages: &[
    ],
};


pub const DMB: LanguageCode = LanguageCode {
    name: "Mombo Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmb",
    individual_languages: &[
    ],
};


pub const DMC: LanguageCode = LanguageCode {
    name: "Gavak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmc",
    individual_languages: &[
    ],
};


pub const DMD: LanguageCode = LanguageCode {
    name: "Madhi Madhi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmd",
    individual_languages: &[
    ],
};


pub const DME: LanguageCode = LanguageCode {
    name: "Dugwor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dme",
    individual_languages: &[
    ],
};


pub const DMF: LanguageCode = LanguageCode {
    name: "Medefaidrin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmf",
    individual_languages: &[
    ],
};


pub const DMG: LanguageCode = LanguageCode {
    name: "Upper Kinabatangan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmg",
    individual_languages: &[
    ],
};


pub const DMK: LanguageCode = LanguageCode {
    name: "Domaaki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmk",
    individual_languages: &[
    ],
};


pub const DML: LanguageCode = LanguageCode {
    name: "Dameli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dml",
    individual_languages: &[
    ],
};


pub const DMM: LanguageCode = LanguageCode {
    name: "Dama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmm",
    individual_languages: &[
    ],
};


pub const DMO: LanguageCode = LanguageCode {
    name: "Kemedzung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmo",
    individual_languages: &[
    ],
};


pub const DMR: LanguageCode = LanguageCode {
    name: "East Damar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmr",
    individual_languages: &[
    ],
};


pub const DMS: LanguageCode = LanguageCode {
    name: "Dampelas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dms",
    individual_languages: &[
    ],
};


pub const DMU: LanguageCode = LanguageCode {
    name: "Dubu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmu",
    individual_languages: &[
    ],
};


pub const DMV: LanguageCode = LanguageCode {
    name: "Dumpas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmv",
    individual_languages: &[
    ],
};


pub const DMW: LanguageCode = LanguageCode {
    name: "Mudburra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmw",
    individual_languages: &[
    ],
};


pub const DMX: LanguageCode = LanguageCode {
    name: "Dema",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmx",
    individual_languages: &[
    ],
};


pub const DMY: LanguageCode = LanguageCode {
    name: "Demta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dmy",
    individual_languages: &[
    ],
};


pub const DNA: LanguageCode = LanguageCode {
    name: "Upper Grand Valley Dani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dna",
    individual_languages: &[
    ],
};


pub const DND: LanguageCode = LanguageCode {
    name: "Daonda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dnd",
    individual_languages: &[
    ],
};


pub const DNE: LanguageCode = LanguageCode {
    name: "Ndendeule",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dne",
    individual_languages: &[
    ],
};


pub const DNG: LanguageCode = LanguageCode {
    name: "Dungan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dng",
    individual_languages: &[
    ],
};


pub const DNI: LanguageCode = LanguageCode {
    name: "Lower Grand Valley Dani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dni",
    individual_languages: &[
    ],
};


pub const DNJ: LanguageCode = LanguageCode {
    name: "Dan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dnj",
    individual_languages: &[
    ],
};


pub const DNK: LanguageCode = LanguageCode {
    name: "Dengka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dnk",
    individual_languages: &[
    ],
};


pub const DNN: LanguageCode = LanguageCode {
    name: "Dzùùngoo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dnn",
    individual_languages: &[
    ],
};


pub const DNO: LanguageCode = LanguageCode {
    name: "Ndrulo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dno",
    individual_languages: &[
    ],
};


pub const DNR: LanguageCode = LanguageCode {
    name: "Danaru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dnr",
    individual_languages: &[
    ],
};


pub const DNT: LanguageCode = LanguageCode {
    name: "Mid Grand Valley Dani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dnt",
    individual_languages: &[
    ],
};


pub const DNU: LanguageCode = LanguageCode {
    name: "Danau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dnu",
    individual_languages: &[
    ],
};


pub const DNV: LanguageCode = LanguageCode {
    name: "Danu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dnv",
    individual_languages: &[
    ],
};


pub const DNW: LanguageCode = LanguageCode {
    name: "Western Dani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dnw",
    individual_languages: &[
    ],
};


pub const DNY: LanguageCode = LanguageCode {
    name: "Dení",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dny",
    individual_languages: &[
    ],
};


pub const DOA: LanguageCode = LanguageCode {
    name: "Dom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "doa",
    individual_languages: &[
    ],
};


pub const DOB: LanguageCode = LanguageCode {
    name: "Dobu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dob",
    individual_languages: &[
    ],
};


pub const DOC: LanguageCode = LanguageCode {
    name: "Northern Dong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "doc",
    individual_languages: &[
    ],
};


pub const DOE: LanguageCode = LanguageCode {
    name: "Doe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "doe",
    individual_languages: &[
    ],
};


pub const DOF: LanguageCode = LanguageCode {
    name: "Domu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dof",
    individual_languages: &[
    ],
};


pub const DOH: LanguageCode = LanguageCode {
    name: "Dong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "doh",
    individual_languages: &[
    ],
};


pub const DOK: LanguageCode = LanguageCode {
    name: "Dondo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dok",
    individual_languages: &[
    ],
};


pub const DOL: LanguageCode = LanguageCode {
    name: "Doso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dol",
    individual_languages: &[
    ],
};


pub const DON: LanguageCode = LanguageCode {
    name: "Toura (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "don",
    individual_languages: &[
    ],
};


pub const DOO: LanguageCode = LanguageCode {
    name: "Dongo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "doo",
    individual_languages: &[
    ],
};


pub const DOP: LanguageCode = LanguageCode {
    name: "Lukpa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dop",
    individual_languages: &[
    ],
};


pub const DOQ: LanguageCode = LanguageCode {
    name: "Dominican Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "doq",
    individual_languages: &[
    ],
};


pub const DOR: LanguageCode = LanguageCode {
    name: "Dori'o",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dor",
    individual_languages: &[
    ],
};


pub const DOS: LanguageCode = LanguageCode {
    name: "Dogosé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dos",
    individual_languages: &[
    ],
};


pub const DOT: LanguageCode = LanguageCode {
    name: "Dass",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dot",
    individual_languages: &[
    ],
};


pub const DOV: LanguageCode = LanguageCode {
    name: "Dombe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dov",
    individual_languages: &[
    ],
};


pub const DOW: LanguageCode = LanguageCode {
    name: "Doyayo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dow",
    individual_languages: &[
    ],
};


pub const DOX: LanguageCode = LanguageCode {
    name: "Bussa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dox",
    individual_languages: &[
    ],
};


pub const DOY: LanguageCode = LanguageCode {
    name: "Dompo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "doy",
    individual_languages: &[
    ],
};


pub const DOZ: LanguageCode = LanguageCode {
    name: "Dorze",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "doz",
    individual_languages: &[
    ],
};


pub const DPP: LanguageCode = LanguageCode {
    name: "Papar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dpp",
    individual_languages: &[
    ],
};


pub const DRB: LanguageCode = LanguageCode {
    name: "Dair",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "drb",
    individual_languages: &[
    ],
};


pub const DRC: LanguageCode = LanguageCode {
    name: "Minderico",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "drc",
    individual_languages: &[
    ],
};


pub const DRD: LanguageCode = LanguageCode {
    name: "Darmiya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "drd",
    individual_languages: &[
    ],
};


pub const DRE: LanguageCode = LanguageCode {
    name: "Dolpo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dre",
    individual_languages: &[
    ],
};


pub const DRG: LanguageCode = LanguageCode {
    name: "Rungus",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "drg",
    individual_languages: &[
    ],
};


pub const DRI: LanguageCode = LanguageCode {
    name: "C'Lela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dri",
    individual_languages: &[
    ],
};


pub const DRL: LanguageCode = LanguageCode {
    name: "Paakantyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "drl",
    individual_languages: &[
    ],
};


pub const DRN: LanguageCode = LanguageCode {
    name: "West Damar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "drn",
    individual_languages: &[
    ],
};


pub const DRO: LanguageCode = LanguageCode {
    name: "Daro-Matu Melanau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dro",
    individual_languages: &[
    ],
};


pub const DRQ: LanguageCode = LanguageCode {
    name: "Dura",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "drq",
    individual_languages: &[
    ],
};


pub const DRS: LanguageCode = LanguageCode {
    name: "Gedeo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "drs",
    individual_languages: &[
    ],
};


pub const DRT: LanguageCode = LanguageCode {
    name: "Drents",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "drt",
    individual_languages: &[
    ],
};


pub const DRU: LanguageCode = LanguageCode {
    name: "Rukai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dru",
    individual_languages: &[
    ],
};


pub const DRY: LanguageCode = LanguageCode {
    name: "Darai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dry",
    individual_languages: &[
    ],
};


pub const DSE: LanguageCode = LanguageCode {
    name: "Dutch Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dse",
    individual_languages: &[
    ],
};


pub const DSH: LanguageCode = LanguageCode {
    name: "Daasanach",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dsh",
    individual_languages: &[
    ],
};


pub const DSI: LanguageCode = LanguageCode {
    name: "Disa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dsi",
    individual_languages: &[
    ],
};


pub const DSL: LanguageCode = LanguageCode {
    name: "Danish Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dsl",
    individual_languages: &[
    ],
};


pub const DSN: LanguageCode = LanguageCode {
    name: "Dusner",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dsn",
    individual_languages: &[
    ],
};


pub const DSO: LanguageCode = LanguageCode {
    name: "Desiya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dso",
    individual_languages: &[
    ],
};


pub const DSQ: LanguageCode = LanguageCode {
    name: "Tadaksahak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dsq",
    individual_languages: &[
    ],
};


pub const DSZ: LanguageCode = LanguageCode {
    name: "Mardin Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dsz",
    individual_languages: &[
    ],
};


pub const DTA: LanguageCode = LanguageCode {
    name: "Daur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dta",
    individual_languages: &[
    ],
};


pub const DTB: LanguageCode = LanguageCode {
    name: "Labuk-Kinabatangan Kadazan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dtb",
    individual_languages: &[
    ],
};


pub const DTD: LanguageCode = LanguageCode {
    name: "Ditidaht",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dtd",
    individual_languages: &[
    ],
};


pub const DTH: LanguageCode = LanguageCode {
    name: "Adithinngithigh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dth",
    individual_languages: &[
    ],
};


pub const DTI: LanguageCode = LanguageCode {
    name: "Ana Tinga Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dti",
    individual_languages: &[
    ],
};


pub const DTK: LanguageCode = LanguageCode {
    name: "Tene Kan Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dtk",
    individual_languages: &[
    ],
};


pub const DTM: LanguageCode = LanguageCode {
    name: "Tomo Kan Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dtm",
    individual_languages: &[
    ],
};


pub const DTN: LanguageCode = LanguageCode {
    name: "Daatsʼíin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dtn",
    individual_languages: &[
    ],
};


pub const DTO: LanguageCode = LanguageCode {
    name: "Tommo So Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dto",
    individual_languages: &[
    ],
};


pub const DTP: LanguageCode = LanguageCode {
    name: "Kadazan Dusun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dtp",
    individual_languages: &[
    ],
};


pub const DTR: LanguageCode = LanguageCode {
    name: "Lotud",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dtr",
    individual_languages: &[
    ],
};


pub const DTS: LanguageCode = LanguageCode {
    name: "Toro So Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dts",
    individual_languages: &[
    ],
};


pub const DTT: LanguageCode = LanguageCode {
    name: "Toro Tegu Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dtt",
    individual_languages: &[
    ],
};


pub const DTU: LanguageCode = LanguageCode {
    name: "Tebul Ure Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dtu",
    individual_languages: &[
    ],
};


pub const DTY: LanguageCode = LanguageCode {
    name: "Dotyali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dty",
    individual_languages: &[
    ],
};


pub const DUB: LanguageCode = LanguageCode {
    name: "Dubli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dub",
    individual_languages: &[
    ],
};


pub const DUC: LanguageCode = LanguageCode {
    name: "Duna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "duc",
    individual_languages: &[
    ],
};


pub const DUE: LanguageCode = LanguageCode {
    name: "Umiray Dumaget Agta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "due",
    individual_languages: &[
    ],
};


pub const DUF: LanguageCode = LanguageCode {
    name: "Dumbea",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "duf",
    individual_languages: &[
    ],
};


pub const DUG: LanguageCode = LanguageCode {
    name: "Duruma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dug",
    individual_languages: &[
    ],
};


pub const DUH: LanguageCode = LanguageCode {
    name: "Dungra Bhil",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "duh",
    individual_languages: &[
    ],
};


pub const DUI: LanguageCode = LanguageCode {
    name: "Dumun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dui",
    individual_languages: &[
    ],
};


pub const DUK: LanguageCode = LanguageCode {
    name: "Uyajitaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "duk",
    individual_languages: &[
    ],
};


pub const DUL: LanguageCode = LanguageCode {
    name: "Alabat Island Agta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dul",
    individual_languages: &[
    ],
};


pub const DUN: LanguageCode = LanguageCode {
    name: "Dusun Deyah",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dun",
    individual_languages: &[
    ],
};


pub const DUO: LanguageCode = LanguageCode {
    name: "Dupaninan Agta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "duo",
    individual_languages: &[
    ],
};


pub const DUP: LanguageCode = LanguageCode {
    name: "Duano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dup",
    individual_languages: &[
    ],
};


pub const DUQ: LanguageCode = LanguageCode {
    name: "Dusun Malang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "duq",
    individual_languages: &[
    ],
};


pub const DUR: LanguageCode = LanguageCode {
    name: "Dii",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dur",
    individual_languages: &[
    ],
};


pub const DUS: LanguageCode = LanguageCode {
    name: "Dumi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dus",
    individual_languages: &[
    ],
};


pub const DUU: LanguageCode = LanguageCode {
    name: "Drung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "duu",
    individual_languages: &[
    ],
};


pub const DUV: LanguageCode = LanguageCode {
    name: "Duvle",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "duv",
    individual_languages: &[
    ],
};


pub const DUW: LanguageCode = LanguageCode {
    name: "Dusun Witu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "duw",
    individual_languages: &[
    ],
};


pub const DUX: LanguageCode = LanguageCode {
    name: "Duungooma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dux",
    individual_languages: &[
    ],
};


pub const DUY: LanguageCode = LanguageCode {
    name: "Dicamay Agta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "duy",
    individual_languages: &[
    ],
};


pub const DUZ: LanguageCode = LanguageCode {
    name: "Duli-Gey",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "duz",
    individual_languages: &[
    ],
};


pub const DVA: LanguageCode = LanguageCode {
    name: "Duau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dva",
    individual_languages: &[
    ],
};


pub const DWA: LanguageCode = LanguageCode {
    name: "Diri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dwa",
    individual_languages: &[
    ],
};


pub const DWK: LanguageCode = LanguageCode {
    name: "Dawik Kui",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dwk",
    individual_languages: &[
    ],
};


pub const DWR: LanguageCode = LanguageCode {
    name: "Dawro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dwr",
    individual_languages: &[
    ],
};


pub const DWS: LanguageCode = LanguageCode {
    name: "Dutton World Speedwords",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dws",
    individual_languages: &[
    ],
};


pub const DWU: LanguageCode = LanguageCode {
    name: "Dhuwal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dwu",
    individual_languages: &[
    ],
};


pub const DWW: LanguageCode = LanguageCode {
    name: "Dawawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dww",
    individual_languages: &[
    ],
};


pub const DWY: LanguageCode = LanguageCode {
    name: "Dhuwaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dwy",
    individual_languages: &[
    ],
};


pub const DWZ: LanguageCode = LanguageCode {
    name: "Dewas Rai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dwz",
    individual_languages: &[
    ],
};


pub const DYA: LanguageCode = LanguageCode {
    name: "Dyan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dya",
    individual_languages: &[
    ],
};


pub const DYB: LanguageCode = LanguageCode {
    name: "Dyaberdyaber",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dyb",
    individual_languages: &[
    ],
};


pub const DYD: LanguageCode = LanguageCode {
    name: "Dyugun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dyd",
    individual_languages: &[
    ],
};


pub const DYG: LanguageCode = LanguageCode {
    name: "Villa Viciosa Agta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dyg",
    individual_languages: &[
    ],
};


pub const DYI: LanguageCode = LanguageCode {
    name: "Djimini Senoufo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dyi",
    individual_languages: &[
    ],
};


pub const DYM: LanguageCode = LanguageCode {
    name: "Yanda Dom Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dym",
    individual_languages: &[
    ],
};


pub const DYN: LanguageCode = LanguageCode {
    name: "Dyangadi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dyn",
    individual_languages: &[
    ],
};


pub const DYO: LanguageCode = LanguageCode {
    name: "Jola-Fonyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dyo",
    individual_languages: &[
    ],
};


pub const DYY: LanguageCode = LanguageCode {
    name: "Djabugay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dyy",
    individual_languages: &[
    ],
};


pub const DZA: LanguageCode = LanguageCode {
    name: "Tunzu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dza",
    individual_languages: &[
    ],
};


pub const DZE: LanguageCode = LanguageCode {
    name: "Djiwarli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dze",
    individual_languages: &[
    ],
};


pub const DZG: LanguageCode = LanguageCode {
    name: "Dazaga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dzg",
    individual_languages: &[
    ],
};


pub const DZL: LanguageCode = LanguageCode {
    name: "Dzalakha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dzl",
    individual_languages: &[
    ],
};


pub const DZN: LanguageCode = LanguageCode {
    name: "Dzando",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "dzn",
    individual_languages: &[
    ],
};


pub const EAA: LanguageCode = LanguageCode {
    name: "Karenggapa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eaa",
    individual_languages: &[
    ],
};


pub const EBC: LanguageCode = LanguageCode {
    name: "Beginci",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ebc",
    individual_languages: &[
    ],
};


pub const EBG: LanguageCode = LanguageCode {
    name: "Ebughu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ebg",
    individual_languages: &[
    ],
};


pub const EBK: LanguageCode = LanguageCode {
    name: "Eastern Bontok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ebk",
    individual_languages: &[
    ],
};


pub const EBO: LanguageCode = LanguageCode {
    name: "Teke-Ebo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ebo",
    individual_languages: &[
    ],
};


pub const EBR: LanguageCode = LanguageCode {
    name: "Ebrié",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ebr",
    individual_languages: &[
    ],
};


pub const EBU: LanguageCode = LanguageCode {
    name: "Embu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ebu",
    individual_languages: &[
    ],
};


pub const ECR: LanguageCode = LanguageCode {
    name: "Eteocretan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ecr",
    individual_languages: &[
    ],
};


pub const ECS: LanguageCode = LanguageCode {
    name: "Ecuadorian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ecs",
    individual_languages: &[
    ],
};


pub const ECY: LanguageCode = LanguageCode {
    name: "Eteocypriot",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ecy",
    individual_languages: &[
    ],
};


pub const EEE: LanguageCode = LanguageCode {
    name: "E",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eee",
    individual_languages: &[
    ],
};


pub const EFA: LanguageCode = LanguageCode {
    name: "Efai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "efa",
    individual_languages: &[
    ],
};


pub const EFE: LanguageCode = LanguageCode {
    name: "Efe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "efe",
    individual_languages: &[
    ],
};


pub const EGA: LanguageCode = LanguageCode {
    name: "Ega",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ega",
    individual_languages: &[
    ],
};


pub const EGL: LanguageCode = LanguageCode {
    name: "Emilian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "egl",
    individual_languages: &[
    ],
};


pub const EGM: LanguageCode = LanguageCode {
    name: "Benamanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "egm",
    individual_languages: &[
    ],
};


pub const EGO: LanguageCode = LanguageCode {
    name: "Eggon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ego",
    individual_languages: &[
    ],
};


pub const EHS: LanguageCode = LanguageCode {
    name: "Miyakubo Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ehs",
    individual_languages: &[
    ],
};


pub const EHU: LanguageCode = LanguageCode {
    name: "Ehueun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ehu",
    individual_languages: &[
    ],
};


pub const EIP: LanguageCode = LanguageCode {
    name: "Eipomek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eip",
    individual_languages: &[
    ],
};


pub const EIT: LanguageCode = LanguageCode {
    name: "Eitiep",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eit",
    individual_languages: &[
    ],
};


pub const EIV: LanguageCode = LanguageCode {
    name: "Askopan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eiv",
    individual_languages: &[
    ],
};


pub const EJA: LanguageCode = LanguageCode {
    name: "Ejamat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eja",
    individual_languages: &[
    ],
};


pub const EKE: LanguageCode = LanguageCode {
    name: "Ekit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eke",
    individual_languages: &[
    ],
};


pub const EKG: LanguageCode = LanguageCode {
    name: "Ekari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ekg",
    individual_languages: &[
    ],
};


pub const EKI: LanguageCode = LanguageCode {
    name: "Eki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eki",
    individual_languages: &[
    ],
};


pub const EKK: LanguageCode = LanguageCode {
    name: "Standard Estonian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ekk",
    individual_languages: &[
    ],
};


pub const EKL: LanguageCode = LanguageCode {
    name: "Kol (Bangladesh)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ekl",
    individual_languages: &[
    ],
};


pub const EKM: LanguageCode = LanguageCode {
    name: "Elip",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ekm",
    individual_languages: &[
    ],
};


pub const EKO: LanguageCode = LanguageCode {
    name: "Koti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eko",
    individual_languages: &[
    ],
};


pub const EKP: LanguageCode = LanguageCode {
    name: "Ekpeye",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ekp",
    individual_languages: &[
    ],
};


pub const EKR: LanguageCode = LanguageCode {
    name: "Yace",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ekr",
    individual_languages: &[
    ],
};


pub const EKY: LanguageCode = LanguageCode {
    name: "Eastern Kayah",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eky",
    individual_languages: &[
    ],
};


pub const ELE: LanguageCode = LanguageCode {
    name: "Elepi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ele",
    individual_languages: &[
    ],
};


pub const ELH: LanguageCode = LanguageCode {
    name: "El Hugeirat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "elh",
    individual_languages: &[
    ],
};


pub const ELI: LanguageCode = LanguageCode {
    name: "Nding",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eli",
    individual_languages: &[
    ],
};


pub const ELK: LanguageCode = LanguageCode {
    name: "Elkei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "elk",
    individual_languages: &[
    ],
};


pub const ELM: LanguageCode = LanguageCode {
    name: "Eleme",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "elm",
    individual_languages: &[
    ],
};


pub const ELO: LanguageCode = LanguageCode {
    name: "El Molo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "elo",
    individual_languages: &[
    ],
};


pub const ELU: LanguageCode = LanguageCode {
    name: "Elu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "elu",
    individual_languages: &[
    ],
};


pub const EMA: LanguageCode = LanguageCode {
    name: "Emai-Iuleha-Ora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ema",
    individual_languages: &[
    ],
};


pub const EMB: LanguageCode = LanguageCode {
    name: "Embaloh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emb",
    individual_languages: &[
    ],
};


pub const EME: LanguageCode = LanguageCode {
    name: "Emerillon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eme",
    individual_languages: &[
    ],
};


pub const EMG: LanguageCode = LanguageCode {
    name: "Eastern Meohang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emg",
    individual_languages: &[
    ],
};


pub const EMI: LanguageCode = LanguageCode {
    name: "Mussau-Emira",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emi",
    individual_languages: &[
    ],
};


pub const EMK: LanguageCode = LanguageCode {
    name: "Eastern Maninkakan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emk",
    individual_languages: &[
    ],
};


pub const EMM: LanguageCode = LanguageCode {
    name: "Mamulique",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emm",
    individual_languages: &[
    ],
};


pub const EMN: LanguageCode = LanguageCode {
    name: "Eman",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emn",
    individual_languages: &[
    ],
};


pub const EMP: LanguageCode = LanguageCode {
    name: "Northern Emberá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emp",
    individual_languages: &[
    ],
};


pub const EMQ: LanguageCode = LanguageCode {
    name: "Eastern Minyag",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emq",
    individual_languages: &[
    ],
};


pub const EMS: LanguageCode = LanguageCode {
    name: "Pacific Gulf Yupik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ems",
    individual_languages: &[
    ],
};


pub const EMU: LanguageCode = LanguageCode {
    name: "Eastern Muria",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emu",
    individual_languages: &[
    ],
};


pub const EMW: LanguageCode = LanguageCode {
    name: "Emplawas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emw",
    individual_languages: &[
    ],
};


pub const EMX: LanguageCode = LanguageCode {
    name: "Erromintxela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emx",
    individual_languages: &[
    ],
};


pub const EMY: LanguageCode = LanguageCode {
    name: "Epigraphic Mayan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emy",
    individual_languages: &[
    ],
};


pub const EMZ: LanguageCode = LanguageCode {
    name: "Mbessa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "emz",
    individual_languages: &[
    ],
};


pub const ENA: LanguageCode = LanguageCode {
    name: "Apali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ena",
    individual_languages: &[
    ],
};


pub const ENB: LanguageCode = LanguageCode {
    name: "Markweeta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "enb",
    individual_languages: &[
    ],
};


pub const ENC: LanguageCode = LanguageCode {
    name: "En",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "enc",
    individual_languages: &[
    ],
};


pub const END: LanguageCode = LanguageCode {
    name: "Ende",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "end",
    individual_languages: &[
    ],
};


pub const ENF: LanguageCode = LanguageCode {
    name: "Forest Enets",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "enf",
    individual_languages: &[
    ],
};


pub const ENH: LanguageCode = LanguageCode {
    name: "Tundra Enets",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "enh",
    individual_languages: &[
    ],
};


pub const ENL: LanguageCode = LanguageCode {
    name: "Enlhet",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "enl",
    individual_languages: &[
    ],
};


pub const ENN: LanguageCode = LanguageCode {
    name: "Engenni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "enn",
    individual_languages: &[
    ],
};


pub const ENO: LanguageCode = LanguageCode {
    name: "Enggano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eno",
    individual_languages: &[
    ],
};


pub const ENQ: LanguageCode = LanguageCode {
    name: "Enga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "enq",
    individual_languages: &[
    ],
};


pub const ENR: LanguageCode = LanguageCode {
    name: "Emumu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "enr",
    individual_languages: &[
    ],
};


pub const ENU: LanguageCode = LanguageCode {
    name: "Enu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "enu",
    individual_languages: &[
    ],
};


pub const ENV: LanguageCode = LanguageCode {
    name: "Enwan (Edo State)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "env",
    individual_languages: &[
    ],
};


pub const ENW: LanguageCode = LanguageCode {
    name: "Enwan (Akwa Ibom State)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "enw",
    individual_languages: &[
    ],
};


pub const ENX: LanguageCode = LanguageCode {
    name: "Enxet",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "enx",
    individual_languages: &[
    ],
};


pub const EOT: LanguageCode = LanguageCode {
    name: "Beti (Côte d'Ivoire)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eot",
    individual_languages: &[
    ],
};


pub const EPI: LanguageCode = LanguageCode {
    name: "Epie",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "epi",
    individual_languages: &[
    ],
};


pub const ERA: LanguageCode = LanguageCode {
    name: "Eravallan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "era",
    individual_languages: &[
    ],
};


pub const ERG: LanguageCode = LanguageCode {
    name: "Sie",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "erg",
    individual_languages: &[
    ],
};


pub const ERH: LanguageCode = LanguageCode {
    name: "Eruwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "erh",
    individual_languages: &[
    ],
};


pub const ERI: LanguageCode = LanguageCode {
    name: "Ogea",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eri",
    individual_languages: &[
    ],
};


pub const ERK: LanguageCode = LanguageCode {
    name: "South Efate",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "erk",
    individual_languages: &[
    ],
};


pub const ERO: LanguageCode = LanguageCode {
    name: "Horpa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ero",
    individual_languages: &[
    ],
};


pub const ERR: LanguageCode = LanguageCode {
    name: "Erre",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "err",
    individual_languages: &[
    ],
};


pub const ERS: LanguageCode = LanguageCode {
    name: "Ersu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ers",
    individual_languages: &[
    ],
};


pub const ERT: LanguageCode = LanguageCode {
    name: "Eritai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ert",
    individual_languages: &[
    ],
};


pub const ERW: LanguageCode = LanguageCode {
    name: "Erokwanas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "erw",
    individual_languages: &[
    ],
};


pub const ESE: LanguageCode = LanguageCode {
    name: "Ese Ejja",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ese",
    individual_languages: &[
    ],
};


pub const ESG: LanguageCode = LanguageCode {
    name: "Aheri Gondi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "esg",
    individual_languages: &[
    ],
};


pub const ESH: LanguageCode = LanguageCode {
    name: "Eshtehardi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "esh",
    individual_languages: &[
    ],
};


pub const ESI: LanguageCode = LanguageCode {
    name: "North Alaskan Inupiatun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "esi",
    individual_languages: &[
    ],
};


pub const ESK: LanguageCode = LanguageCode {
    name: "Northwest Alaska Inupiatun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "esk",
    individual_languages: &[
    ],
};


pub const ESL: LanguageCode = LanguageCode {
    name: "Egypt Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "esl",
    individual_languages: &[
    ],
};


pub const ESM: LanguageCode = LanguageCode {
    name: "Esuma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "esm",
    individual_languages: &[
    ],
};


pub const ESN: LanguageCode = LanguageCode {
    name: "Salvadoran Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "esn",
    individual_languages: &[
    ],
};


pub const ESO: LanguageCode = LanguageCode {
    name: "Estonian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eso",
    individual_languages: &[
    ],
};


pub const ESQ: LanguageCode = LanguageCode {
    name: "Esselen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "esq",
    individual_languages: &[
    ],
};


pub const ESS: LanguageCode = LanguageCode {
    name: "Central Siberian Yupik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ess",
    individual_languages: &[
    ],
};


pub const ESU: LanguageCode = LanguageCode {
    name: "Central Yupik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "esu",
    individual_languages: &[
    ],
};


pub const ESY: LanguageCode = LanguageCode {
    name: "Eskayan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "esy",
    individual_languages: &[
    ],
};


pub const ETB: LanguageCode = LanguageCode {
    name: "Etebi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "etb",
    individual_languages: &[
    ],
};


pub const ETC: LanguageCode = LanguageCode {
    name: "Etchemin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "etc",
    individual_languages: &[
    ],
};


pub const ETH: LanguageCode = LanguageCode {
    name: "Ethiopian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eth",
    individual_languages: &[
    ],
};


pub const ETN: LanguageCode = LanguageCode {
    name: "Eton (Vanuatu)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "etn",
    individual_languages: &[
    ],
};


pub const ETO: LanguageCode = LanguageCode {
    name: "Eton (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eto",
    individual_languages: &[
    ],
};


pub const ETR: LanguageCode = LanguageCode {
    name: "Edolo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "etr",
    individual_languages: &[
    ],
};


pub const ETS: LanguageCode = LanguageCode {
    name: "Yekhee",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ets",
    individual_languages: &[
    ],
};


pub const ETT: LanguageCode = LanguageCode {
    name: "Etruscan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ett",
    individual_languages: &[
    ],
};


pub const ETU: LanguageCode = LanguageCode {
    name: "Ejagham",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "etu",
    individual_languages: &[
    ],
};


pub const ETX: LanguageCode = LanguageCode {
    name: "Eten",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "etx",
    individual_languages: &[
    ],
};


pub const ETZ: LanguageCode = LanguageCode {
    name: "Semimi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "etz",
    individual_languages: &[
    ],
};


pub const EVE: LanguageCode = LanguageCode {
    name: "Even",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eve",
    individual_languages: &[
    ],
};


pub const EVH: LanguageCode = LanguageCode {
    name: "Uvbie",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "evh",
    individual_languages: &[
    ],
};


pub const EVN: LanguageCode = LanguageCode {
    name: "Evenki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "evn",
    individual_languages: &[
    ],
};


pub const EXT: LanguageCode = LanguageCode {
    name: "Extremaduran",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ext",
    individual_languages: &[
    ],
};


pub const EYA: LanguageCode = LanguageCode {
    name: "Eyak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eya",
    individual_languages: &[
    ],
};


pub const EYO: LanguageCode = LanguageCode {
    name: "Keiyo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eyo",
    individual_languages: &[
    ],
};


pub const EZA: LanguageCode = LanguageCode {
    name: "Ezaa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eza",
    individual_languages: &[
    ],
};


pub const EZE: LanguageCode = LanguageCode {
    name: "Uzekwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "eze",
    individual_languages: &[
    ],
};


pub const FAA: LanguageCode = LanguageCode {
    name: "Fasu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "faa",
    individual_languages: &[
    ],
};


pub const FAB: LanguageCode = LanguageCode {
    name: "Fa d'Ambu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fab",
    individual_languages: &[
    ],
};


pub const FAD: LanguageCode = LanguageCode {
    name: "Wagi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fad",
    individual_languages: &[
    ],
};


pub const FAF: LanguageCode = LanguageCode {
    name: "Fagani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "faf",
    individual_languages: &[
    ],
};


pub const FAG: LanguageCode = LanguageCode {
    name: "Finongan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fag",
    individual_languages: &[
    ],
};


pub const FAH: LanguageCode = LanguageCode {
    name: "Baissa Fali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fah",
    individual_languages: &[
    ],
};


pub const FAI: LanguageCode = LanguageCode {
    name: "Faiwol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fai",
    individual_languages: &[
    ],
};


pub const FAJ: LanguageCode = LanguageCode {
    name: "Faita",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "faj",
    individual_languages: &[
    ],
};


pub const FAK: LanguageCode = LanguageCode {
    name: "Fang (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fak",
    individual_languages: &[
    ],
};


pub const FAL: LanguageCode = LanguageCode {
    name: "South Fali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fal",
    individual_languages: &[
    ],
};


pub const FAM: LanguageCode = LanguageCode {
    name: "Fam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fam",
    individual_languages: &[
    ],
};


pub const FAP: LanguageCode = LanguageCode {
    name: "Paloor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fap",
    individual_languages: &[
    ],
};


pub const FAR: LanguageCode = LanguageCode {
    name: "Fataleka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "far",
    individual_languages: &[
    ],
};


pub const FAU: LanguageCode = LanguageCode {
    name: "Fayu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fau",
    individual_languages: &[
    ],
};


pub const FAX: LanguageCode = LanguageCode {
    name: "Fala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fax",
    individual_languages: &[
    ],
};


pub const FAY: LanguageCode = LanguageCode {
    name: "Southwestern Fars",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fay",
    individual_languages: &[
    ],
};


pub const FAZ: LanguageCode = LanguageCode {
    name: "Northwestern Fars",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "faz",
    individual_languages: &[
    ],
};


pub const FBL: LanguageCode = LanguageCode {
    name: "West Albay Bikol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fbl",
    individual_languages: &[
    ],
};


pub const FCS: LanguageCode = LanguageCode {
    name: "Quebec Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fcs",
    individual_languages: &[
    ],
};


pub const FER: LanguageCode = LanguageCode {
    name: "Feroge",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fer",
    individual_languages: &[
    ],
};


pub const FFI: LanguageCode = LanguageCode {
    name: "Foia Foia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ffi",
    individual_languages: &[
    ],
};


pub const FFM: LanguageCode = LanguageCode {
    name: "Maasina Fulfulde",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ffm",
    individual_languages: &[
    ],
};


pub const FGR: LanguageCode = LanguageCode {
    name: "Fongoro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fgr",
    individual_languages: &[
    ],
};


pub const FIA: LanguageCode = LanguageCode {
    name: "Nobiin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fia",
    individual_languages: &[
    ],
};


pub const FIE: LanguageCode = LanguageCode {
    name: "Fyer",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fie",
    individual_languages: &[
    ],
};


pub const FIF: LanguageCode = LanguageCode {
    name: "Faifi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fif",
    individual_languages: &[
    ],
};


pub const FIP: LanguageCode = LanguageCode {
    name: "Fipa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fip",
    individual_languages: &[
    ],
};


pub const FIR: LanguageCode = LanguageCode {
    name: "Firan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fir",
    individual_languages: &[
    ],
};


pub const FIT: LanguageCode = LanguageCode {
    name: "Tornedalen Finnish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fit",
    individual_languages: &[
    ],
};


pub const FIW: LanguageCode = LanguageCode {
    name: "Fiwaga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fiw",
    individual_languages: &[
    ],
};


pub const FKK: LanguageCode = LanguageCode {
    name: "Kirya-Konzəl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fkk",
    individual_languages: &[
    ],
};


pub const FKV: LanguageCode = LanguageCode {
    name: "Kven Finnish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fkv",
    individual_languages: &[
    ],
};


pub const FLA: LanguageCode = LanguageCode {
    name: "Kalispel-Pend d'Oreille",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fla",
    individual_languages: &[
    ],
};


pub const FLH: LanguageCode = LanguageCode {
    name: "Foau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "flh",
    individual_languages: &[
    ],
};


pub const FLI: LanguageCode = LanguageCode {
    name: "Fali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fli",
    individual_languages: &[
    ],
};


pub const FLL: LanguageCode = LanguageCode {
    name: "North Fali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fll",
    individual_languages: &[
    ],
};


pub const FLN: LanguageCode = LanguageCode {
    name: "Flinders Island",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fln",
    individual_languages: &[
    ],
};


pub const FLR: LanguageCode = LanguageCode {
    name: "Fuliiru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "flr",
    individual_languages: &[
    ],
};


pub const FLY: LanguageCode = LanguageCode {
    name: "Flaaitaal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fly",
    individual_languages: &[
    ],
};


pub const FMP: LanguageCode = LanguageCode {
    name: "Fe'fe'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fmp",
    individual_languages: &[
    ],
};


pub const FMU: LanguageCode = LanguageCode {
    name: "Far Western Muria",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fmu",
    individual_languages: &[
    ],
};


pub const FNB: LanguageCode = LanguageCode {
    name: "Fanbak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fnb",
    individual_languages: &[
    ],
};


pub const FNG: LanguageCode = LanguageCode {
    name: "Fanagalo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fng",
    individual_languages: &[
    ],
};


pub const FNI: LanguageCode = LanguageCode {
    name: "Fania",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fni",
    individual_languages: &[
    ],
};


pub const FOD: LanguageCode = LanguageCode {
    name: "Foodo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fod",
    individual_languages: &[
    ],
};


pub const FOI: LanguageCode = LanguageCode {
    name: "Foi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "foi",
    individual_languages: &[
    ],
};


pub const FOM: LanguageCode = LanguageCode {
    name: "Foma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fom",
    individual_languages: &[
    ],
};


pub const FOR: LanguageCode = LanguageCode {
    name: "Fore",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "for",
    individual_languages: &[
    ],
};


pub const FOS: LanguageCode = LanguageCode {
    name: "Siraya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fos",
    individual_languages: &[
    ],
};


pub const FPE: LanguageCode = LanguageCode {
    name: "Fernando Po Creole English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fpe",
    individual_languages: &[
    ],
};


pub const FQS: LanguageCode = LanguageCode {
    name: "Fas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fqs",
    individual_languages: &[
    ],
};


pub const FRC: LanguageCode = LanguageCode {
    name: "Cajun French",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "frc",
    individual_languages: &[
    ],
};


pub const FRD: LanguageCode = LanguageCode {
    name: "Fordata",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "frd",
    individual_languages: &[
    ],
};


pub const FRK: LanguageCode = LanguageCode {
    name: "Frankish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "frk",
    individual_languages: &[
    ],
};


pub const FRP: LanguageCode = LanguageCode {
    name: "Arpitan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "frp",
    individual_languages: &[
    ],
};


pub const FRQ: LanguageCode = LanguageCode {
    name: "Forak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "frq",
    individual_languages: &[
    ],
};


pub const FRT: LanguageCode = LanguageCode {
    name: "Fortsenal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "frt",
    individual_languages: &[
    ],
};


pub const FSE: LanguageCode = LanguageCode {
    name: "Finnish Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fse",
    individual_languages: &[
    ],
};


pub const FSL: LanguageCode = LanguageCode {
    name: "French Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fsl",
    individual_languages: &[
    ],
};


pub const FSS: LanguageCode = LanguageCode {
    name: "Finland-Swedish Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fss",
    individual_languages: &[
    ],
};


pub const FUB: LanguageCode = LanguageCode {
    name: "Adamawa Fulfulde",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fub",
    individual_languages: &[
    ],
};


pub const FUC: LanguageCode = LanguageCode {
    name: "Pulaar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fuc",
    individual_languages: &[
    ],
};


pub const FUD: LanguageCode = LanguageCode {
    name: "East Futuna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fud",
    individual_languages: &[
    ],
};


pub const FUE: LanguageCode = LanguageCode {
    name: "Borgu Fulfulde",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fue",
    individual_languages: &[
    ],
};


pub const FUF: LanguageCode = LanguageCode {
    name: "Pular",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fuf",
    individual_languages: &[
    ],
};


pub const FUH: LanguageCode = LanguageCode {
    name: "Western Niger Fulfulde",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fuh",
    individual_languages: &[
    ],
};


pub const FUI: LanguageCode = LanguageCode {
    name: "Bagirmi Fulfulde",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fui",
    individual_languages: &[
    ],
};


pub const FUJ: LanguageCode = LanguageCode {
    name: "Ko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fuj",
    individual_languages: &[
    ],
};


pub const FUM: LanguageCode = LanguageCode {
    name: "Fum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fum",
    individual_languages: &[
    ],
};


pub const FUN: LanguageCode = LanguageCode {
    name: "Fulniô",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fun",
    individual_languages: &[
    ],
};


pub const FUQ: LanguageCode = LanguageCode {
    name: "Central-Eastern Niger Fulfulde",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fuq",
    individual_languages: &[
    ],
};


pub const FUT: LanguageCode = LanguageCode {
    name: "Futuna-Aniwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fut",
    individual_languages: &[
    ],
};


pub const FUU: LanguageCode = LanguageCode {
    name: "Furu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fuu",
    individual_languages: &[
    ],
};


pub const FUV: LanguageCode = LanguageCode {
    name: "Nigerian Fulfulde",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fuv",
    individual_languages: &[
    ],
};


pub const FUY: LanguageCode = LanguageCode {
    name: "Fuyug",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fuy",
    individual_languages: &[
    ],
};


pub const FVR: LanguageCode = LanguageCode {
    name: "Fur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fvr",
    individual_languages: &[
    ],
};


pub const FWA: LanguageCode = LanguageCode {
    name: "Fwâi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fwa",
    individual_languages: &[
    ],
};


pub const FWE: LanguageCode = LanguageCode {
    name: "Fwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "fwe",
    individual_languages: &[
    ],
};


pub const GAB: LanguageCode = LanguageCode {
    name: "Gabri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gab",
    individual_languages: &[
    ],
};


pub const GAC: LanguageCode = LanguageCode {
    name: "Mixed Great Andamanese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gac",
    individual_languages: &[
    ],
};


pub const GAD: LanguageCode = LanguageCode {
    name: "Gaddang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gad",
    individual_languages: &[
    ],
};


pub const GAE: LanguageCode = LanguageCode {
    name: "Guarequena",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gae",
    individual_languages: &[
    ],
};


pub const GAF: LanguageCode = LanguageCode {
    name: "Gende",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gaf",
    individual_languages: &[
    ],
};


pub const GAG: LanguageCode = LanguageCode {
    name: "Gagauz",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gag",
    individual_languages: &[
    ],
};


pub const GAH: LanguageCode = LanguageCode {
    name: "Alekano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gah",
    individual_languages: &[
    ],
};


pub const GAI: LanguageCode = LanguageCode {
    name: "Borei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gai",
    individual_languages: &[
    ],
};


pub const GAJ: LanguageCode = LanguageCode {
    name: "Gadsup",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gaj",
    individual_languages: &[
    ],
};


pub const GAK: LanguageCode = LanguageCode {
    name: "Gamkonora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gak",
    individual_languages: &[
    ],
};


pub const GAL: LanguageCode = LanguageCode {
    name: "Galolen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gal",
    individual_languages: &[
    ],
};


pub const GAM: LanguageCode = LanguageCode {
    name: "Kandawo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gam",
    individual_languages: &[
    ],
};


pub const GAN: LanguageCode = LanguageCode {
    name: "Gan Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gan",
    individual_languages: &[
    ],
};


pub const GAO: LanguageCode = LanguageCode {
    name: "Gants",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gao",
    individual_languages: &[
    ],
};


pub const GAP: LanguageCode = LanguageCode {
    name: "Gal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gap",
    individual_languages: &[
    ],
};


pub const GAQ: LanguageCode = LanguageCode {
    name: "Gata'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gaq",
    individual_languages: &[
    ],
};


pub const GAR: LanguageCode = LanguageCode {
    name: "Galeya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gar",
    individual_languages: &[
    ],
};


pub const GAS: LanguageCode = LanguageCode {
    name: "Adiwasi Garasia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gas",
    individual_languages: &[
    ],
};


pub const GAT: LanguageCode = LanguageCode {
    name: "Kenati",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gat",
    individual_languages: &[
    ],
};


pub const GAU: LanguageCode = LanguageCode {
    name: "Mudhili Gadaba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gau",
    individual_languages: &[
    ],
};


pub const GAW: LanguageCode = LanguageCode {
    name: "Nobonob",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gaw",
    individual_languages: &[
    ],
};


pub const GAX: LanguageCode = LanguageCode {
    name: "Borana-Arsi-Guji Oromo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gax",
    individual_languages: &[
    ],
};


pub const GAZ: LanguageCode = LanguageCode {
    name: "West Central Oromo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gaz",
    individual_languages: &[
    ],
};


pub const GBB: LanguageCode = LanguageCode {
    name: "Kaytetye",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbb",
    individual_languages: &[
    ],
};


pub const GBD: LanguageCode = LanguageCode {
    name: "Karajarri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbd",
    individual_languages: &[
    ],
};


pub const GBE: LanguageCode = LanguageCode {
    name: "Niksek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbe",
    individual_languages: &[
    ],
};


pub const GBF: LanguageCode = LanguageCode {
    name: "Gaikundi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbf",
    individual_languages: &[
    ],
};


pub const GBG: LanguageCode = LanguageCode {
    name: "Gbanziri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbg",
    individual_languages: &[
    ],
};


pub const GBH: LanguageCode = LanguageCode {
    name: "Defi Gbe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbh",
    individual_languages: &[
    ],
};


pub const GBI: LanguageCode = LanguageCode {
    name: "Galela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbi",
    individual_languages: &[
    ],
};


pub const GBJ: LanguageCode = LanguageCode {
    name: "Bodo Gadaba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbj",
    individual_languages: &[
    ],
};


pub const GBK: LanguageCode = LanguageCode {
    name: "Gaddi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbk",
    individual_languages: &[
    ],
};


pub const GBL: LanguageCode = LanguageCode {
    name: "Gamit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbl",
    individual_languages: &[
    ],
};


pub const GBM: LanguageCode = LanguageCode {
    name: "Garhwali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbm",
    individual_languages: &[
    ],
};


pub const GBN: LanguageCode = LanguageCode {
    name: "Mo'da",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbn",
    individual_languages: &[
    ],
};


pub const GBO: LanguageCode = LanguageCode {
    name: "Northern Grebo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbo",
    individual_languages: &[
    ],
};


pub const GBP: LanguageCode = LanguageCode {
    name: "Gbaya-Bossangoa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbp",
    individual_languages: &[
    ],
};


pub const GBQ: LanguageCode = LanguageCode {
    name: "Gbaya-Bozoum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbq",
    individual_languages: &[
    ],
};


pub const GBR: LanguageCode = LanguageCode {
    name: "Gbagyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbr",
    individual_languages: &[
    ],
};


pub const GBS: LanguageCode = LanguageCode {
    name: "Gbesi Gbe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbs",
    individual_languages: &[
    ],
};


pub const GBU: LanguageCode = LanguageCode {
    name: "Gagadu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbu",
    individual_languages: &[
    ],
};


pub const GBV: LanguageCode = LanguageCode {
    name: "Gbanu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbv",
    individual_languages: &[
    ],
};


pub const GBW: LanguageCode = LanguageCode {
    name: "Gabi-Gabi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbw",
    individual_languages: &[
    ],
};


pub const GBX: LanguageCode = LanguageCode {
    name: "Eastern Xwla Gbe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbx",
    individual_languages: &[
    ],
};


pub const GBY: LanguageCode = LanguageCode {
    name: "Gbari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gby",
    individual_languages: &[
    ],
};


pub const GBZ: LanguageCode = LanguageCode {
    name: "Zoroastrian Dari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gbz",
    individual_languages: &[
    ],
};


pub const GCC: LanguageCode = LanguageCode {
    name: "Mali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gcc",
    individual_languages: &[
    ],
};


pub const GCD: LanguageCode = LanguageCode {
    name: "Ganggalida",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gcd",
    individual_languages: &[
    ],
};


pub const GCE: LanguageCode = LanguageCode {
    name: "Galice",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gce",
    individual_languages: &[
    ],
};


pub const GCF: LanguageCode = LanguageCode {
    name: "Guadeloupean Creole French",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gcf",
    individual_languages: &[
    ],
};


pub const GCL: LanguageCode = LanguageCode {
    name: "Grenadian Creole English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gcl",
    individual_languages: &[
    ],
};


pub const GCN: LanguageCode = LanguageCode {
    name: "Gaina",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gcn",
    individual_languages: &[
    ],
};


pub const GCR: LanguageCode = LanguageCode {
    name: "Guianese Creole French",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gcr",
    individual_languages: &[
    ],
};


pub const GCT: LanguageCode = LanguageCode {
    name: "Colonia Tovar German",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gct",
    individual_languages: &[
    ],
};


pub const GDA: LanguageCode = LanguageCode {
    name: "Gade Lohar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gda",
    individual_languages: &[
    ],
};


pub const GDB: LanguageCode = LanguageCode {
    name: "Pottangi Ollar Gadaba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdb",
    individual_languages: &[
    ],
};


pub const GDC: LanguageCode = LanguageCode {
    name: "Gugu Badhun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdc",
    individual_languages: &[
    ],
};


pub const GDD: LanguageCode = LanguageCode {
    name: "Gedaged",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdd",
    individual_languages: &[
    ],
};


pub const GDE: LanguageCode = LanguageCode {
    name: "Gude",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gde",
    individual_languages: &[
    ],
};


pub const GDF: LanguageCode = LanguageCode {
    name: "Guduf-Gava",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdf",
    individual_languages: &[
    ],
};


pub const GDG: LanguageCode = LanguageCode {
    name: "Ga'dang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdg",
    individual_languages: &[
    ],
};


pub const GDH: LanguageCode = LanguageCode {
    name: "Gadjerawang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdh",
    individual_languages: &[
    ],
};


pub const GDI: LanguageCode = LanguageCode {
    name: "Gundi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdi",
    individual_languages: &[
    ],
};


pub const GDJ: LanguageCode = LanguageCode {
    name: "Gurdjar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdj",
    individual_languages: &[
    ],
};


pub const GDK: LanguageCode = LanguageCode {
    name: "Gadang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdk",
    individual_languages: &[
    ],
};


pub const GDL: LanguageCode = LanguageCode {
    name: "Dirasha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdl",
    individual_languages: &[
    ],
};


pub const GDM: LanguageCode = LanguageCode {
    name: "Laal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdm",
    individual_languages: &[
    ],
};


pub const GDN: LanguageCode = LanguageCode {
    name: "Umanakaina",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdn",
    individual_languages: &[
    ],
};


pub const GDO: LanguageCode = LanguageCode {
    name: "Ghodoberi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdo",
    individual_languages: &[
    ],
};


pub const GDQ: LanguageCode = LanguageCode {
    name: "Mehri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdq",
    individual_languages: &[
    ],
};


pub const GDR: LanguageCode = LanguageCode {
    name: "Wipi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdr",
    individual_languages: &[
    ],
};


pub const GDS: LanguageCode = LanguageCode {
    name: "Ghandruk Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gds",
    individual_languages: &[
    ],
};


pub const GDT: LanguageCode = LanguageCode {
    name: "Kungardutyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdt",
    individual_languages: &[
    ],
};


pub const GDU: LanguageCode = LanguageCode {
    name: "Gudu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdu",
    individual_languages: &[
    ],
};


pub const GDX: LanguageCode = LanguageCode {
    name: "Godwari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gdx",
    individual_languages: &[
    ],
};


pub const GEA: LanguageCode = LanguageCode {
    name: "Geruma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gea",
    individual_languages: &[
    ],
};


pub const GEB: LanguageCode = LanguageCode {
    name: "Kire",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "geb",
    individual_languages: &[
    ],
};


pub const GEC: LanguageCode = LanguageCode {
    name: "Gboloo Grebo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gec",
    individual_languages: &[
    ],
};


pub const GED: LanguageCode = LanguageCode {
    name: "Gade",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ged",
    individual_languages: &[
    ],
};


pub const GEF: LanguageCode = LanguageCode {
    name: "Gerai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gef",
    individual_languages: &[
    ],
};


pub const GEG: LanguageCode = LanguageCode {
    name: "Gengle",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "geg",
    individual_languages: &[
    ],
};


pub const GEH: LanguageCode = LanguageCode {
    name: "Hutterite German",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "geh",
    individual_languages: &[
    ],
};


pub const GEI: LanguageCode = LanguageCode {
    name: "Gebe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gei",
    individual_languages: &[
    ],
};


pub const GEJ: LanguageCode = LanguageCode {
    name: "Gen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gej",
    individual_languages: &[
    ],
};


pub const GEK: LanguageCode = LanguageCode {
    name: "Ywom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gek",
    individual_languages: &[
    ],
};


pub const GEL: LanguageCode = LanguageCode {
    name: "ut-Ma'in",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gel",
    individual_languages: &[
    ],
};


pub const GEQ: LanguageCode = LanguageCode {
    name: "Geme",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "geq",
    individual_languages: &[
    ],
};


pub const GES: LanguageCode = LanguageCode {
    name: "Geser-Gorom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ges",
    individual_languages: &[
    ],
};


pub const GEV: LanguageCode = LanguageCode {
    name: "Eviya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gev",
    individual_languages: &[
    ],
};


pub const GEW: LanguageCode = LanguageCode {
    name: "Gera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gew",
    individual_languages: &[
    ],
};


pub const GEX: LanguageCode = LanguageCode {
    name: "Garre",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gex",
    individual_languages: &[
    ],
};


pub const GEY: LanguageCode = LanguageCode {
    name: "Enya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gey",
    individual_languages: &[
    ],
};


pub const GFK: LanguageCode = LanguageCode {
    name: "Patpatar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gfk",
    individual_languages: &[
    ],
};


pub const GFT: LanguageCode = LanguageCode {
    name: "Gafat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gft",
    individual_languages: &[
    ],
};


pub const GGA: LanguageCode = LanguageCode {
    name: "Gao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gga",
    individual_languages: &[
    ],
};


pub const GGB: LanguageCode = LanguageCode {
    name: "Gbii",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ggb",
    individual_languages: &[
    ],
};


pub const GGD: LanguageCode = LanguageCode {
    name: "Gugadj",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ggd",
    individual_languages: &[
    ],
};


pub const GGE: LanguageCode = LanguageCode {
    name: "Gurr-goni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gge",
    individual_languages: &[
    ],
};


pub const GGG: LanguageCode = LanguageCode {
    name: "Gurgula",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ggg",
    individual_languages: &[
    ],
};


pub const GGK: LanguageCode = LanguageCode {
    name: "Kungarakany",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ggk",
    individual_languages: &[
    ],
};


pub const GGL: LanguageCode = LanguageCode {
    name: "Ganglau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ggl",
    individual_languages: &[
    ],
};


pub const GGT: LanguageCode = LanguageCode {
    name: "Gitua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ggt",
    individual_languages: &[
    ],
};


pub const GGU: LanguageCode = LanguageCode {
    name: "Gagu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ggu",
    individual_languages: &[
    ],
};


pub const GGW: LanguageCode = LanguageCode {
    name: "Gogodala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ggw",
    individual_languages: &[
    ],
};


pub const GHA: LanguageCode = LanguageCode {
    name: "Ghadamès",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gha",
    individual_languages: &[
    ],
};


pub const GHC: LanguageCode = LanguageCode {
    name: "Hiberno-Scottish Gaelic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ghc",
    individual_languages: &[
    ],
};


pub const GHE: LanguageCode = LanguageCode {
    name: "Southern Ghale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ghe",
    individual_languages: &[
    ],
};


pub const GHH: LanguageCode = LanguageCode {
    name: "Northern Ghale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ghh",
    individual_languages: &[
    ],
};


pub const GHK: LanguageCode = LanguageCode {
    name: "Geko Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ghk",
    individual_languages: &[
    ],
};


pub const GHL: LanguageCode = LanguageCode {
    name: "Ghulfan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ghl",
    individual_languages: &[
    ],
};


pub const GHN: LanguageCode = LanguageCode {
    name: "Ghanongga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ghn",
    individual_languages: &[
    ],
};


pub const GHO: LanguageCode = LanguageCode {
    name: "Ghomara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gho",
    individual_languages: &[
    ],
};


pub const GHR: LanguageCode = LanguageCode {
    name: "Ghera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ghr",
    individual_languages: &[
    ],
};


pub const GHS: LanguageCode = LanguageCode {
    name: "Guhu-Samane",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ghs",
    individual_languages: &[
    ],
};


pub const GHT: LanguageCode = LanguageCode {
    name: "Kuke",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ght",
    individual_languages: &[
    ],
};


pub const GIA: LanguageCode = LanguageCode {
    name: "Kija",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gia",
    individual_languages: &[
    ],
};


pub const GIB: LanguageCode = LanguageCode {
    name: "Gibanawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gib",
    individual_languages: &[
    ],
};


pub const GIC: LanguageCode = LanguageCode {
    name: "Gail",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gic",
    individual_languages: &[
    ],
};


pub const GID: LanguageCode = LanguageCode {
    name: "Gidar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gid",
    individual_languages: &[
    ],
};


pub const GIE: LanguageCode = LanguageCode {
    name: "Gaɓogbo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gie",
    individual_languages: &[
    ],
};


pub const GIG: LanguageCode = LanguageCode {
    name: "Goaria",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gig",
    individual_languages: &[
    ],
};


pub const GIH: LanguageCode = LanguageCode {
    name: "Githabul",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gih",
    individual_languages: &[
    ],
};


pub const GII: LanguageCode = LanguageCode {
    name: "Girirra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gii",
    individual_languages: &[
    ],
};


pub const GIM: LanguageCode = LanguageCode {
    name: "Gimi (Eastern Highlands)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gim",
    individual_languages: &[
    ],
};


pub const GIN: LanguageCode = LanguageCode {
    name: "Hinukh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gin",
    individual_languages: &[
    ],
};


pub const GIP: LanguageCode = LanguageCode {
    name: "Gimi (West New Britain)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gip",
    individual_languages: &[
    ],
};


pub const GIQ: LanguageCode = LanguageCode {
    name: "Green Gelao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "giq",
    individual_languages: &[
    ],
};


pub const GIR: LanguageCode = LanguageCode {
    name: "Red Gelao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gir",
    individual_languages: &[
    ],
};


pub const GIS: LanguageCode = LanguageCode {
    name: "North Giziga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gis",
    individual_languages: &[
    ],
};


pub const GIT: LanguageCode = LanguageCode {
    name: "Gitxsan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "git",
    individual_languages: &[
    ],
};


pub const GIU: LanguageCode = LanguageCode {
    name: "Mulao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "giu",
    individual_languages: &[
    ],
};


pub const GIW: LanguageCode = LanguageCode {
    name: "White Gelao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "giw",
    individual_languages: &[
    ],
};


pub const GIX: LanguageCode = LanguageCode {
    name: "Gilima",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gix",
    individual_languages: &[
    ],
};


pub const GIY: LanguageCode = LanguageCode {
    name: "Giyug",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "giy",
    individual_languages: &[
    ],
};


pub const GIZ: LanguageCode = LanguageCode {
    name: "South Giziga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "giz",
    individual_languages: &[
    ],
};


pub const GJK: LanguageCode = LanguageCode {
    name: "Kachi Koli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gjk",
    individual_languages: &[
    ],
};


pub const GJM: LanguageCode = LanguageCode {
    name: "Gunditjmara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gjm",
    individual_languages: &[
    ],
};


pub const GJN: LanguageCode = LanguageCode {
    name: "Gonja",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gjn",
    individual_languages: &[
    ],
};


pub const GJR: LanguageCode = LanguageCode {
    name: "Gurindji Kriol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gjr",
    individual_languages: &[
    ],
};


pub const GJU: LanguageCode = LanguageCode {
    name: "Gujari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gju",
    individual_languages: &[
    ],
};


pub const GKA: LanguageCode = LanguageCode {
    name: "Guya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gka",
    individual_languages: &[
    ],
};


pub const GKD: LanguageCode = LanguageCode {
    name: "Magɨ (Madang Province)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gkd",
    individual_languages: &[
    ],
};


pub const GKE: LanguageCode = LanguageCode {
    name: "Ndai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gke",
    individual_languages: &[
    ],
};


pub const GKN: LanguageCode = LanguageCode {
    name: "Gokana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gkn",
    individual_languages: &[
    ],
};


pub const GKO: LanguageCode = LanguageCode {
    name: "Kok-Nar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gko",
    individual_languages: &[
    ],
};


pub const GKP: LanguageCode = LanguageCode {
    name: "Guinea Kpelle",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gkp",
    individual_languages: &[
    ],
};


pub const GKU: LanguageCode = LanguageCode {
    name: "ǂUngkue",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gku",
    individual_languages: &[
    ],
};


pub const GLB: LanguageCode = LanguageCode {
    name: "Belning",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "glb",
    individual_languages: &[
    ],
};


pub const GLC: LanguageCode = LanguageCode {
    name: "Bon Gula",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "glc",
    individual_languages: &[
    ],
};


pub const GLD: LanguageCode = LanguageCode {
    name: "Nanai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gld",
    individual_languages: &[
    ],
};


pub const GLH: LanguageCode = LanguageCode {
    name: "Northwest Pashai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "glh",
    individual_languages: &[
    ],
};


pub const GLJ: LanguageCode = LanguageCode {
    name: "Gula Iro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "glj",
    individual_languages: &[
    ],
};


pub const GLK: LanguageCode = LanguageCode {
    name: "Gilaki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "glk",
    individual_languages: &[
    ],
};


pub const GLL: LanguageCode = LanguageCode {
    name: "Garlali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gll",
    individual_languages: &[
    ],
};


pub const GLO: LanguageCode = LanguageCode {
    name: "Galambu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "glo",
    individual_languages: &[
    ],
};


pub const GLR: LanguageCode = LanguageCode {
    name: "Glaro-Twabo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "glr",
    individual_languages: &[
    ],
};


pub const GLU: LanguageCode = LanguageCode {
    name: "Gula (Chad)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "glu",
    individual_languages: &[
    ],
};


pub const GLW: LanguageCode = LanguageCode {
    name: "Glavda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "glw",
    individual_languages: &[
    ],
};


pub const GLY: LanguageCode = LanguageCode {
    name: "Gule",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gly",
    individual_languages: &[
    ],
};


pub const GMA: LanguageCode = LanguageCode {
    name: "Gambera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gma",
    individual_languages: &[
    ],
};


pub const GMB: LanguageCode = LanguageCode {
    name: "Gula'alaa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gmb",
    individual_languages: &[
    ],
};


pub const GMD: LanguageCode = LanguageCode {
    name: "Mághdì",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gmd",
    individual_languages: &[
    ],
};


pub const GMG: LanguageCode = LanguageCode {
    name: "Magɨyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gmg",
    individual_languages: &[
    ],
};


pub const GML: LanguageCode = LanguageCode {
    name: "Middle Low German",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gml",
    individual_languages: &[
    ],
};


pub const GMM: LanguageCode = LanguageCode {
    name: "Gbaya-Mbodomo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gmm",
    individual_languages: &[
    ],
};


pub const GMN: LanguageCode = LanguageCode {
    name: "Gimnime",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gmn",
    individual_languages: &[
    ],
};


pub const GMR: LanguageCode = LanguageCode {
    name: "Mirning",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gmr",
    individual_languages: &[
    ],
};


pub const GMU: LanguageCode = LanguageCode {
    name: "Gumalu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gmu",
    individual_languages: &[
    ],
};


pub const GMV: LanguageCode = LanguageCode {
    name: "Gamo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gmv",
    individual_languages: &[
    ],
};


pub const GMX: LanguageCode = LanguageCode {
    name: "Magoma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gmx",
    individual_languages: &[
    ],
};


pub const GMY: LanguageCode = LanguageCode {
    name: "Mycenaean Greek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gmy",
    individual_languages: &[
    ],
};


pub const GMZ: LanguageCode = LanguageCode {
    name: "Mgbolizhia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gmz",
    individual_languages: &[
    ],
};


pub const GNA: LanguageCode = LanguageCode {
    name: "Kaansa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gna",
    individual_languages: &[
    ],
};


pub const GNB: LanguageCode = LanguageCode {
    name: "Gangte",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnb",
    individual_languages: &[
    ],
};


pub const GNC: LanguageCode = LanguageCode {
    name: "Guanche",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnc",
    individual_languages: &[
    ],
};


pub const GND: LanguageCode = LanguageCode {
    name: "Zulgo-Gemzek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnd",
    individual_languages: &[
    ],
};


pub const GNE: LanguageCode = LanguageCode {
    name: "Ganang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gne",
    individual_languages: &[
    ],
};


pub const GNG: LanguageCode = LanguageCode {
    name: "Ngangam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gng",
    individual_languages: &[
    ],
};


pub const GNH: LanguageCode = LanguageCode {
    name: "Lere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnh",
    individual_languages: &[
    ],
};


pub const GNI: LanguageCode = LanguageCode {
    name: "Gooniyandi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gni",
    individual_languages: &[
    ],
};


pub const GNJ: LanguageCode = LanguageCode {
    name: "Ngen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnj",
    individual_languages: &[
    ],
};


pub const GNK: LanguageCode = LanguageCode {
    name: "ǁGana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnk",
    individual_languages: &[
    ],
};


pub const GNL: LanguageCode = LanguageCode {
    name: "Gangulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnl",
    individual_languages: &[
    ],
};


pub const GNM: LanguageCode = LanguageCode {
    name: "Ginuman",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnm",
    individual_languages: &[
    ],
};


pub const GNN: LanguageCode = LanguageCode {
    name: "Gumatj",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnn",
    individual_languages: &[
    ],
};


pub const GNO: LanguageCode = LanguageCode {
    name: "Northern Gondi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gno",
    individual_languages: &[
    ],
};


pub const GNQ: LanguageCode = LanguageCode {
    name: "Gana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnq",
    individual_languages: &[
    ],
};


pub const GNR: LanguageCode = LanguageCode {
    name: "Gureng Gureng",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnr",
    individual_languages: &[
    ],
};


pub const GNT: LanguageCode = LanguageCode {
    name: "Guntai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnt",
    individual_languages: &[
    ],
};


pub const GNU: LanguageCode = LanguageCode {
    name: "Gnau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnu",
    individual_languages: &[
    ],
};


pub const GNW: LanguageCode = LanguageCode {
    name: "Western Bolivian Guaraní",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnw",
    individual_languages: &[
    ],
};


pub const GNZ: LanguageCode = LanguageCode {
    name: "Ganzi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gnz",
    individual_languages: &[
    ],
};


pub const GOA: LanguageCode = LanguageCode {
    name: "Guro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "goa",
    individual_languages: &[
    ],
};


pub const GOB: LanguageCode = LanguageCode {
    name: "Playero",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gob",
    individual_languages: &[
    ],
};


pub const GOC: LanguageCode = LanguageCode {
    name: "Gorakor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "goc",
    individual_languages: &[
    ],
};


pub const GOD: LanguageCode = LanguageCode {
    name: "Godié",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "god",
    individual_languages: &[
    ],
};


pub const GOE: LanguageCode = LanguageCode {
    name: "Gongduk",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "goe",
    individual_languages: &[
    ],
};


pub const GOF: LanguageCode = LanguageCode {
    name: "Gofa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gof",
    individual_languages: &[
    ],
};


pub const GOG: LanguageCode = LanguageCode {
    name: "Gogo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gog",
    individual_languages: &[
    ],
};


pub const GOI: LanguageCode = LanguageCode {
    name: "Gobasi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "goi",
    individual_languages: &[
    ],
};


pub const GOJ: LanguageCode = LanguageCode {
    name: "Gowlan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "goj",
    individual_languages: &[
    ],
};


pub const GOK: LanguageCode = LanguageCode {
    name: "Gowli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gok",
    individual_languages: &[
    ],
};


pub const GOL: LanguageCode = LanguageCode {
    name: "Gola",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gol",
    individual_languages: &[
    ],
};


pub const GOM: LanguageCode = LanguageCode {
    name: "Goan Konkani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gom",
    individual_languages: &[
    ],
};


pub const GOO: LanguageCode = LanguageCode {
    name: "Gone Dau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "goo",
    individual_languages: &[
    ],
};


pub const GOP: LanguageCode = LanguageCode {
    name: "Yeretuar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gop",
    individual_languages: &[
    ],
};


pub const GOQ: LanguageCode = LanguageCode {
    name: "Gorap",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "goq",
    individual_languages: &[
    ],
};


pub const GOS: LanguageCode = LanguageCode {
    name: "Gronings",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gos",
    individual_languages: &[
    ],
};


pub const GOU: LanguageCode = LanguageCode {
    name: "Gavar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gou",
    individual_languages: &[
    ],
};


pub const GOV: LanguageCode = LanguageCode {
    name: "Goo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gov",
    individual_languages: &[
    ],
};


pub const GOW: LanguageCode = LanguageCode {
    name: "Gorowa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gow",
    individual_languages: &[
    ],
};


pub const GOX: LanguageCode = LanguageCode {
    name: "Gobu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gox",
    individual_languages: &[
    ],
};


pub const GOY: LanguageCode = LanguageCode {
    name: "Goundo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "goy",
    individual_languages: &[
    ],
};


pub const GOZ: LanguageCode = LanguageCode {
    name: "Gozarkhani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "goz",
    individual_languages: &[
    ],
};


pub const GPA: LanguageCode = LanguageCode {
    name: "Gupa-Abawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gpa",
    individual_languages: &[
    ],
};


pub const GPE: LanguageCode = LanguageCode {
    name: "Ghanaian Pidgin English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gpe",
    individual_languages: &[
    ],
};


pub const GPN: LanguageCode = LanguageCode {
    name: "Taiap",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gpn",
    individual_languages: &[
    ],
};


pub const GQA: LanguageCode = LanguageCode {
    name: "Ga'anda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gqa",
    individual_languages: &[
    ],
};


pub const GQI: LanguageCode = LanguageCode {
    name: "Guiqiong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gqi",
    individual_languages: &[
    ],
};


pub const GQN: LanguageCode = LanguageCode {
    name: "Guana (Brazil)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gqn",
    individual_languages: &[
    ],
};


pub const GQR: LanguageCode = LanguageCode {
    name: "Gor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gqr",
    individual_languages: &[
    ],
};


pub const GQU: LanguageCode = LanguageCode {
    name: "Qau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gqu",
    individual_languages: &[
    ],
};


pub const GRA: LanguageCode = LanguageCode {
    name: "Rajput Garasia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gra",
    individual_languages: &[
    ],
};


pub const GRD: LanguageCode = LanguageCode {
    name: "Guruntum-Mbaaru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grd",
    individual_languages: &[
    ],
};


pub const GRG: LanguageCode = LanguageCode {
    name: "Madi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grg",
    individual_languages: &[
    ],
};


pub const GRH: LanguageCode = LanguageCode {
    name: "Gbiri-Niragu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grh",
    individual_languages: &[
    ],
};


pub const GRI: LanguageCode = LanguageCode {
    name: "Ghari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gri",
    individual_languages: &[
    ],
};


pub const GRJ: LanguageCode = LanguageCode {
    name: "Southern Grebo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grj",
    individual_languages: &[
    ],
};


pub const GRM: LanguageCode = LanguageCode {
    name: "Kota Marudu Talantang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grm",
    individual_languages: &[
    ],
};


pub const GRO: LanguageCode = LanguageCode {
    name: "Groma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gro",
    individual_languages: &[
    ],
};


pub const GRQ: LanguageCode = LanguageCode {
    name: "Gorovu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grq",
    individual_languages: &[
    ],
};


pub const GRR: LanguageCode = LanguageCode {
    name: "Taznatit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grr",
    individual_languages: &[
    ],
};


pub const GRS: LanguageCode = LanguageCode {
    name: "Gresi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grs",
    individual_languages: &[
    ],
};


pub const GRT: LanguageCode = LanguageCode {
    name: "Garo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grt",
    individual_languages: &[
    ],
};


pub const GRU: LanguageCode = LanguageCode {
    name: "Kistane",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gru",
    individual_languages: &[
    ],
};


pub const GRV: LanguageCode = LanguageCode {
    name: "Central Grebo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grv",
    individual_languages: &[
    ],
};


pub const GRW: LanguageCode = LanguageCode {
    name: "Gweda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grw",
    individual_languages: &[
    ],
};


pub const GRX: LanguageCode = LanguageCode {
    name: "Guriaso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grx",
    individual_languages: &[
    ],
};


pub const GRY: LanguageCode = LanguageCode {
    name: "Barclayville Grebo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gry",
    individual_languages: &[
    ],
};


pub const GRZ: LanguageCode = LanguageCode {
    name: "Guramalum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "grz",
    individual_languages: &[
    ],
};


pub const GSE: LanguageCode = LanguageCode {
    name: "Ghanaian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gse",
    individual_languages: &[
    ],
};


pub const GSG: LanguageCode = LanguageCode {
    name: "German Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gsg",
    individual_languages: &[
    ],
};


pub const GSL: LanguageCode = LanguageCode {
    name: "Gusilay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gsl",
    individual_languages: &[
    ],
};


pub const GSM: LanguageCode = LanguageCode {
    name: "Guatemalan Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gsm",
    individual_languages: &[
    ],
};


pub const GSN: LanguageCode = LanguageCode {
    name: "Nema",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gsn",
    individual_languages: &[
    ],
};


pub const GSO: LanguageCode = LanguageCode {
    name: "Southwest Gbaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gso",
    individual_languages: &[
    ],
};


pub const GSP: LanguageCode = LanguageCode {
    name: "Wasembo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gsp",
    individual_languages: &[
    ],
};


pub const GSS: LanguageCode = LanguageCode {
    name: "Greek Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gss",
    individual_languages: &[
    ],
};


pub const GTA: LanguageCode = LanguageCode {
    name: "Guató",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gta",
    individual_languages: &[
    ],
};


pub const GTU: LanguageCode = LanguageCode {
    name: "Aghu-Tharnggala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gtu",
    individual_languages: &[
    ],
};


pub const GUA: LanguageCode = LanguageCode {
    name: "Shiki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gua",
    individual_languages: &[
    ],
};


pub const GUB: LanguageCode = LanguageCode {
    name: "Guajajára",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gub",
    individual_languages: &[
    ],
};


pub const GUC: LanguageCode = LanguageCode {
    name: "Wayuu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "guc",
    individual_languages: &[
    ],
};


pub const GUD: LanguageCode = LanguageCode {
    name: "Yocoboué Dida",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gud",
    individual_languages: &[
    ],
};


pub const GUE: LanguageCode = LanguageCode {
    name: "Gurindji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gue",
    individual_languages: &[
    ],
};


pub const GUF: LanguageCode = LanguageCode {
    name: "Gupapuyngu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "guf",
    individual_languages: &[
    ],
};


pub const GUG: LanguageCode = LanguageCode {
    name: "Paraguayan Guaraní",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gug",
    individual_languages: &[
    ],
};


pub const GUH: LanguageCode = LanguageCode {
    name: "Guahibo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "guh",
    individual_languages: &[
    ],
};


pub const GUI: LanguageCode = LanguageCode {
    name: "Eastern Bolivian Guaraní",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gui",
    individual_languages: &[
    ],
};


pub const GUK: LanguageCode = LanguageCode {
    name: "Gumuz",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "guk",
    individual_languages: &[
    ],
};


pub const GUL: LanguageCode = LanguageCode {
    name: "Sea Island Creole English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gul",
    individual_languages: &[
    ],
};


pub const GUM: LanguageCode = LanguageCode {
    name: "Guambiano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gum",
    individual_languages: &[
    ],
};


pub const GUN: LanguageCode = LanguageCode {
    name: "Mbyá Guaraní",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gun",
    individual_languages: &[
    ],
};


pub const GUO: LanguageCode = LanguageCode {
    name: "Guayabero",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "guo",
    individual_languages: &[
    ],
};


pub const GUP: LanguageCode = LanguageCode {
    name: "Gunwinggu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gup",
    individual_languages: &[
    ],
};


pub const GUQ: LanguageCode = LanguageCode {
    name: "Aché",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "guq",
    individual_languages: &[
    ],
};


pub const GUR: LanguageCode = LanguageCode {
    name: "Farefare",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gur",
    individual_languages: &[
    ],
};


pub const GUS: LanguageCode = LanguageCode {
    name: "Guinean Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gus",
    individual_languages: &[
    ],
};


pub const GUT: LanguageCode = LanguageCode {
    name: "Maléku Jaíka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gut",
    individual_languages: &[
    ],
};


pub const GUU: LanguageCode = LanguageCode {
    name: "Yanomamö",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "guu",
    individual_languages: &[
    ],
};


pub const GUW: LanguageCode = LanguageCode {
    name: "Gun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "guw",
    individual_languages: &[
    ],
};


pub const GUX: LanguageCode = LanguageCode {
    name: "Gourmanchéma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gux",
    individual_languages: &[
    ],
};


pub const GUZ: LanguageCode = LanguageCode {
    name: "Gusii",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "guz",
    individual_languages: &[
    ],
};


pub const GVA: LanguageCode = LanguageCode {
    name: "Guana (Paraguay)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gva",
    individual_languages: &[
    ],
};


pub const GVC: LanguageCode = LanguageCode {
    name: "Guanano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gvc",
    individual_languages: &[
    ],
};


pub const GVE: LanguageCode = LanguageCode {
    name: "Duwet",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gve",
    individual_languages: &[
    ],
};


pub const GVF: LanguageCode = LanguageCode {
    name: "Golin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gvf",
    individual_languages: &[
    ],
};


pub const GVJ: LanguageCode = LanguageCode {
    name: "Guajá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gvj",
    individual_languages: &[
    ],
};


pub const GVL: LanguageCode = LanguageCode {
    name: "Gulay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gvl",
    individual_languages: &[
    ],
};


pub const GVM: LanguageCode = LanguageCode {
    name: "Gurmana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gvm",
    individual_languages: &[
    ],
};


pub const GVN: LanguageCode = LanguageCode {
    name: "Kuku-Yalanji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gvn",
    individual_languages: &[
    ],
};


pub const GVO: LanguageCode = LanguageCode {
    name: "Gavião Do Jiparaná",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gvo",
    individual_languages: &[
    ],
};


pub const GVP: LanguageCode = LanguageCode {
    name: "Pará Gavião",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gvp",
    individual_languages: &[
    ],
};


pub const GVR: LanguageCode = LanguageCode {
    name: "Gurung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gvr",
    individual_languages: &[
    ],
};


pub const GVS: LanguageCode = LanguageCode {
    name: "Gumawana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gvs",
    individual_languages: &[
    ],
};


pub const GVY: LanguageCode = LanguageCode {
    name: "Guyani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gvy",
    individual_languages: &[
    ],
};


pub const GWA: LanguageCode = LanguageCode {
    name: "Mbato",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwa",
    individual_languages: &[
    ],
};


pub const GWB: LanguageCode = LanguageCode {
    name: "Gwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwb",
    individual_languages: &[
    ],
};


pub const GWC: LanguageCode = LanguageCode {
    name: "Gawri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwc",
    individual_languages: &[
    ],
};


pub const GWD: LanguageCode = LanguageCode {
    name: "Gawwada",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwd",
    individual_languages: &[
    ],
};


pub const GWE: LanguageCode = LanguageCode {
    name: "Gweno",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwe",
    individual_languages: &[
    ],
};


pub const GWF: LanguageCode = LanguageCode {
    name: "Gowro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwf",
    individual_languages: &[
    ],
};


pub const GWG: LanguageCode = LanguageCode {
    name: "Moo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwg",
    individual_languages: &[
    ],
};


pub const GWJ: LanguageCode = LanguageCode {
    name: "ǀGwi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwj",
    individual_languages: &[
    ],
};


pub const GWM: LanguageCode = LanguageCode {
    name: "Awngthim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwm",
    individual_languages: &[
    ],
};


pub const GWN: LanguageCode = LanguageCode {
    name: "Gwandara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwn",
    individual_languages: &[
    ],
};


pub const GWR: LanguageCode = LanguageCode {
    name: "Gwere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwr",
    individual_languages: &[
    ],
};


pub const GWT: LanguageCode = LanguageCode {
    name: "Gawar-Bati",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwt",
    individual_languages: &[
    ],
};


pub const GWU: LanguageCode = LanguageCode {
    name: "Guwamu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwu",
    individual_languages: &[
    ],
};


pub const GWW: LanguageCode = LanguageCode {
    name: "Kwini",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gww",
    individual_languages: &[
    ],
};


pub const GWX: LanguageCode = LanguageCode {
    name: "Gua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gwx",
    individual_languages: &[
    ],
};


pub const GXX: LanguageCode = LanguageCode {
    name: "Wè Southern",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gxx",
    individual_languages: &[
    ],
};


pub const GYA: LanguageCode = LanguageCode {
    name: "Northwest Gbaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gya",
    individual_languages: &[
    ],
};


pub const GYB: LanguageCode = LanguageCode {
    name: "Garus",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gyb",
    individual_languages: &[
    ],
};


pub const GYD: LanguageCode = LanguageCode {
    name: "Kayardild",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gyd",
    individual_languages: &[
    ],
};


pub const GYE: LanguageCode = LanguageCode {
    name: "Gyem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gye",
    individual_languages: &[
    ],
};


pub const GYF: LanguageCode = LanguageCode {
    name: "Gungabula",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gyf",
    individual_languages: &[
    ],
};


pub const GYG: LanguageCode = LanguageCode {
    name: "Gbayi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gyg",
    individual_languages: &[
    ],
};


pub const GYI: LanguageCode = LanguageCode {
    name: "Gyele",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gyi",
    individual_languages: &[
    ],
};


pub const GYL: LanguageCode = LanguageCode {
    name: "Gayil",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gyl",
    individual_languages: &[
    ],
};


pub const GYM: LanguageCode = LanguageCode {
    name: "Ngäbere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gym",
    individual_languages: &[
    ],
};


pub const GYN: LanguageCode = LanguageCode {
    name: "Guyanese Creole English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gyn",
    individual_languages: &[
    ],
};


pub const GYO: LanguageCode = LanguageCode {
    name: "Gyalsumdo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gyo",
    individual_languages: &[
    ],
};


pub const GYR: LanguageCode = LanguageCode {
    name: "Guarayu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gyr",
    individual_languages: &[
    ],
};


pub const GYY: LanguageCode = LanguageCode {
    name: "Gunya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gyy",
    individual_languages: &[
    ],
};


pub const GYZ: LanguageCode = LanguageCode {
    name: "Geji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gyz",
    individual_languages: &[
    ],
};


pub const GZA: LanguageCode = LanguageCode {
    name: "Ganza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gza",
    individual_languages: &[
    ],
};


pub const GZI: LanguageCode = LanguageCode {
    name: "Gazi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gzi",
    individual_languages: &[
    ],
};


pub const GZN: LanguageCode = LanguageCode {
    name: "Gane",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "gzn",
    individual_languages: &[
    ],
};


pub const HAA: LanguageCode = LanguageCode {
    name: "Han",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "haa",
    individual_languages: &[
    ],
};


pub const HAB: LanguageCode = LanguageCode {
    name: "Hanoi Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hab",
    individual_languages: &[
    ],
};


pub const HAC: LanguageCode = LanguageCode {
    name: "Gurani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hac",
    individual_languages: &[
    ],
};


pub const HAD: LanguageCode = LanguageCode {
    name: "Hatam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "had",
    individual_languages: &[
    ],
};


pub const HAE: LanguageCode = LanguageCode {
    name: "Eastern Oromo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hae",
    individual_languages: &[
    ],
};


pub const HAF: LanguageCode = LanguageCode {
    name: "Haiphong Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "haf",
    individual_languages: &[
    ],
};


pub const HAG: LanguageCode = LanguageCode {
    name: "Hanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hag",
    individual_languages: &[
    ],
};


pub const HAH: LanguageCode = LanguageCode {
    name: "Hahon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hah",
    individual_languages: &[
    ],
};


pub const HAJ: LanguageCode = LanguageCode {
    name: "Hajong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "haj",
    individual_languages: &[
    ],
};


pub const HAK: LanguageCode = LanguageCode {
    name: "Hakka Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hak",
    individual_languages: &[
    ],
};


pub const HAL: LanguageCode = LanguageCode {
    name: "Halang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hal",
    individual_languages: &[
    ],
};


pub const HAM: LanguageCode = LanguageCode {
    name: "Hewa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ham",
    individual_languages: &[
    ],
};


pub const HAN: LanguageCode = LanguageCode {
    name: "Hangaza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "han",
    individual_languages: &[
    ],
};


pub const HAO: LanguageCode = LanguageCode {
    name: "Hakö",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hao",
    individual_languages: &[
    ],
};


pub const HAP: LanguageCode = LanguageCode {
    name: "Hupla",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hap",
    individual_languages: &[
    ],
};


pub const HAQ: LanguageCode = LanguageCode {
    name: "Ha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "haq",
    individual_languages: &[
    ],
};


pub const HAR: LanguageCode = LanguageCode {
    name: "Harari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "har",
    individual_languages: &[
    ],
};


pub const HAS: LanguageCode = LanguageCode {
    name: "Haisla",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "has",
    individual_languages: &[
    ],
};


pub const HAV: LanguageCode = LanguageCode {
    name: "Havu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hav",
    individual_languages: &[
    ],
};


pub const HAX: LanguageCode = LanguageCode {
    name: "Southern Haida",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hax",
    individual_languages: &[
    ],
};


pub const HAY: LanguageCode = LanguageCode {
    name: "Haya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hay",
    individual_languages: &[
    ],
};


pub const HAZ: LanguageCode = LanguageCode {
    name: "Hazaragi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "haz",
    individual_languages: &[
    ],
};


pub const HBA: LanguageCode = LanguageCode {
    name: "Hamba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hba",
    individual_languages: &[
    ],
};


pub const HBB: LanguageCode = LanguageCode {
    name: "Huba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hbb",
    individual_languages: &[
    ],
};


pub const HBN: LanguageCode = LanguageCode {
    name: "Heiban",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hbn",
    individual_languages: &[
    ],
};


pub const HBO: LanguageCode = LanguageCode {
    name: "Ancient Hebrew",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hbo",
    individual_languages: &[
    ],
};


pub const SH: LanguageCode = LanguageCode {
    name: "Serbo-Croatian",
    code: "sh",
    code_2t: "",
    code_2b: "",
    code_3: "hbs",
    individual_languages: &[
        IndividualLanguages {
            name: "Bosnian",
            code: "bos",
        },
        IndividualLanguages {
            name: "Montenegrin",
            code: "cnr",
        },
        IndividualLanguages {
            name: "Croatian",
            code: "hrv",
        },
        IndividualLanguages {
            name: "Serbian",
            code: "srp",
        },
    ],
};


pub const HBU: LanguageCode = LanguageCode {
    name: "Habu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hbu",
    individual_languages: &[
    ],
};


pub const HCA: LanguageCode = LanguageCode {
    name: "Andaman Creole Hindi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hca",
    individual_languages: &[
    ],
};


pub const HCH: LanguageCode = LanguageCode {
    name: "Huichol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hch",
    individual_languages: &[
    ],
};


pub const HDN: LanguageCode = LanguageCode {
    name: "Northern Haida",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hdn",
    individual_languages: &[
    ],
};


pub const HDS: LanguageCode = LanguageCode {
    name: "Honduras Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hds",
    individual_languages: &[
    ],
};


pub const HDY: LanguageCode = LanguageCode {
    name: "Hadiyya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hdy",
    individual_languages: &[
    ],
};


pub const HEA: LanguageCode = LanguageCode {
    name: "Northern Qiandong Miao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hea",
    individual_languages: &[
    ],
};


pub const HED: LanguageCode = LanguageCode {
    name: "Herdé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hed",
    individual_languages: &[
    ],
};


pub const HEG: LanguageCode = LanguageCode {
    name: "Helong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "heg",
    individual_languages: &[
    ],
};


pub const HEH: LanguageCode = LanguageCode {
    name: "Hehe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "heh",
    individual_languages: &[
    ],
};


pub const HEI: LanguageCode = LanguageCode {
    name: "Heiltsuk",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hei",
    individual_languages: &[
    ],
};


pub const HEM: LanguageCode = LanguageCode {
    name: "Hemba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hem",
    individual_languages: &[
    ],
};


pub const HGM: LanguageCode = LanguageCode {
    name: "Haiǁom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hgm",
    individual_languages: &[
    ],
};


pub const HGW: LanguageCode = LanguageCode {
    name: "Haigwai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hgw",
    individual_languages: &[
    ],
};


pub const HHI: LanguageCode = LanguageCode {
    name: "Hoia Hoia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hhi",
    individual_languages: &[
    ],
};


pub const HHR: LanguageCode = LanguageCode {
    name: "Kerak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hhr",
    individual_languages: &[
    ],
};


pub const HHY: LanguageCode = LanguageCode {
    name: "Hoyahoya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hhy",
    individual_languages: &[
    ],
};


pub const HIA: LanguageCode = LanguageCode {
    name: "Lamang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hia",
    individual_languages: &[
    ],
};


pub const HIB: LanguageCode = LanguageCode {
    name: "Hibito",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hib",
    individual_languages: &[
    ],
};


pub const HID: LanguageCode = LanguageCode {
    name: "Hidatsa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hid",
    individual_languages: &[
    ],
};


pub const HIF: LanguageCode = LanguageCode {
    name: "Fiji Hindi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hif",
    individual_languages: &[
    ],
};


pub const HIG: LanguageCode = LanguageCode {
    name: "Kamwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hig",
    individual_languages: &[
    ],
};


pub const HIH: LanguageCode = LanguageCode {
    name: "Pamosu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hih",
    individual_languages: &[
    ],
};


pub const HII: LanguageCode = LanguageCode {
    name: "Hinduri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hii",
    individual_languages: &[
    ],
};


pub const HIJ: LanguageCode = LanguageCode {
    name: "Hijuk",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hij",
    individual_languages: &[
    ],
};


pub const HIK: LanguageCode = LanguageCode {
    name: "Seit-Kaitetu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hik",
    individual_languages: &[
    ],
};


pub const HIO: LanguageCode = LanguageCode {
    name: "Tsoa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hio",
    individual_languages: &[
    ],
};


pub const HIR: LanguageCode = LanguageCode {
    name: "Himarimã",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hir",
    individual_languages: &[
    ],
};


pub const HIW: LanguageCode = LanguageCode {
    name: "Hiw",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hiw",
    individual_languages: &[
    ],
};


pub const HIX: LanguageCode = LanguageCode {
    name: "Hixkaryána",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hix",
    individual_languages: &[
    ],
};


pub const HJI: LanguageCode = LanguageCode {
    name: "Haji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hji",
    individual_languages: &[
    ],
};


pub const HKA: LanguageCode = LanguageCode {
    name: "Kahe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hka",
    individual_languages: &[
    ],
};


pub const HKE: LanguageCode = LanguageCode {
    name: "Hunde",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hke",
    individual_languages: &[
    ],
};


pub const HKH: LanguageCode = LanguageCode {
    name: "Khah",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hkh",
    individual_languages: &[
    ],
};


pub const HKK: LanguageCode = LanguageCode {
    name: "Hunjara-Kaina Ke",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hkk",
    individual_languages: &[
    ],
};


pub const HKN: LanguageCode = LanguageCode {
    name: "Mel-Khaonh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hkn",
    individual_languages: &[
    ],
};


pub const HKS: LanguageCode = LanguageCode {
    name: "Hong Kong Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hks",
    individual_languages: &[
    ],
};


pub const HLA: LanguageCode = LanguageCode {
    name: "Halia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hla",
    individual_languages: &[
    ],
};


pub const HLB: LanguageCode = LanguageCode {
    name: "Halbi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hlb",
    individual_languages: &[
    ],
};


pub const HLD: LanguageCode = LanguageCode {
    name: "Halang Doan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hld",
    individual_languages: &[
    ],
};


pub const HLE: LanguageCode = LanguageCode {
    name: "Hlersu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hle",
    individual_languages: &[
    ],
};


pub const HLT: LanguageCode = LanguageCode {
    name: "Matu Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hlt",
    individual_languages: &[
    ],
};


pub const HLU: LanguageCode = LanguageCode {
    name: "Hieroglyphic Luwian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hlu",
    individual_languages: &[
    ],
};


pub const HMA: LanguageCode = LanguageCode {
    name: "Southern Mashan Hmong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hma",
    individual_languages: &[
    ],
};


pub const HMB: LanguageCode = LanguageCode {
    name: "Humburi Senni Songhay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmb",
    individual_languages: &[
    ],
};


pub const HMC: LanguageCode = LanguageCode {
    name: "Central Huishui Hmong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmc",
    individual_languages: &[
    ],
};


pub const HMD: LanguageCode = LanguageCode {
    name: "Large Flowery Miao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmd",
    individual_languages: &[
    ],
};


pub const HME: LanguageCode = LanguageCode {
    name: "Eastern Huishui Hmong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hme",
    individual_languages: &[
    ],
};


pub const HMF: LanguageCode = LanguageCode {
    name: "Hmong Don",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmf",
    individual_languages: &[
    ],
};


pub const HMG: LanguageCode = LanguageCode {
    name: "Southwestern Guiyang Hmong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmg",
    individual_languages: &[
    ],
};


pub const HMH: LanguageCode = LanguageCode {
    name: "Southwestern Huishui Hmong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmh",
    individual_languages: &[
    ],
};


pub const HMI: LanguageCode = LanguageCode {
    name: "Northern Huishui Hmong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmi",
    individual_languages: &[
    ],
};


pub const HMJ: LanguageCode = LanguageCode {
    name: "Ge",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmj",
    individual_languages: &[
    ],
};


pub const HMK: LanguageCode = LanguageCode {
    name: "Maek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmk",
    individual_languages: &[
    ],
};


pub const HML: LanguageCode = LanguageCode {
    name: "Luopohe Hmong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hml",
    individual_languages: &[
    ],
};


pub const HMM: LanguageCode = LanguageCode {
    name: "Central Mashan Hmong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmm",
    individual_languages: &[
    ],
};


pub const HMP: LanguageCode = LanguageCode {
    name: "Northern Mashan Hmong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmp",
    individual_languages: &[
    ],
};


pub const HMQ: LanguageCode = LanguageCode {
    name: "Eastern Qiandong Miao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmq",
    individual_languages: &[
    ],
};


pub const HMR: LanguageCode = LanguageCode {
    name: "Hmar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmr",
    individual_languages: &[
    ],
};


pub const HMS: LanguageCode = LanguageCode {
    name: "Southern Qiandong Miao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hms",
    individual_languages: &[
    ],
};


pub const HMT: LanguageCode = LanguageCode {
    name: "Hamtai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmt",
    individual_languages: &[
    ],
};


pub const HMU: LanguageCode = LanguageCode {
    name: "Hamap",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmu",
    individual_languages: &[
    ],
};


pub const HMV: LanguageCode = LanguageCode {
    name: "Hmong Dô",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmv",
    individual_languages: &[
    ],
};


pub const HMW: LanguageCode = LanguageCode {
    name: "Western Mashan Hmong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmw",
    individual_languages: &[
    ],
};


pub const HMY: LanguageCode = LanguageCode {
    name: "Southern Guiyang Hmong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmy",
    individual_languages: &[
    ],
};


pub const HMZ: LanguageCode = LanguageCode {
    name: "Hmong Shua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hmz",
    individual_languages: &[
    ],
};


pub const HNA: LanguageCode = LanguageCode {
    name: "Mina (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hna",
    individual_languages: &[
    ],
};


pub const HND: LanguageCode = LanguageCode {
    name: "Southern Hindko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hnd",
    individual_languages: &[
    ],
};


pub const HNE: LanguageCode = LanguageCode {
    name: "Chhattisgarhi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hne",
    individual_languages: &[
    ],
};


pub const HNG: LanguageCode = LanguageCode {
    name: "Hungu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hng",
    individual_languages: &[
    ],
};


pub const HNH: LanguageCode = LanguageCode {
    name: "ǁAni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hnh",
    individual_languages: &[
    ],
};


pub const HNI: LanguageCode = LanguageCode {
    name: "Hani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hni",
    individual_languages: &[
    ],
};


pub const HNJ: LanguageCode = LanguageCode {
    name: "Hmong Njua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hnj",
    individual_languages: &[
    ],
};


pub const HNN: LanguageCode = LanguageCode {
    name: "Hanunoo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hnn",
    individual_languages: &[
    ],
};


pub const HNO: LanguageCode = LanguageCode {
    name: "Northern Hindko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hno",
    individual_languages: &[
    ],
};


pub const HNS: LanguageCode = LanguageCode {
    name: "Caribbean Hindustani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hns",
    individual_languages: &[
    ],
};


pub const HNU: LanguageCode = LanguageCode {
    name: "Hung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hnu",
    individual_languages: &[
    ],
};


pub const HOA: LanguageCode = LanguageCode {
    name: "Hoava",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hoa",
    individual_languages: &[
    ],
};


pub const HOB: LanguageCode = LanguageCode {
    name: "Mari (Madang Province)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hob",
    individual_languages: &[
    ],
};


pub const HOC: LanguageCode = LanguageCode {
    name: "Ho",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hoc",
    individual_languages: &[
    ],
};


pub const HOD: LanguageCode = LanguageCode {
    name: "Holma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hod",
    individual_languages: &[
    ],
};


pub const HOE: LanguageCode = LanguageCode {
    name: "Horom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hoe",
    individual_languages: &[
    ],
};


pub const HOH: LanguageCode = LanguageCode {
    name: "Hobyót",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hoh",
    individual_languages: &[
    ],
};


pub const HOI: LanguageCode = LanguageCode {
    name: "Holikachuk",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hoi",
    individual_languages: &[
    ],
};


pub const HOJ: LanguageCode = LanguageCode {
    name: "Hadothi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hoj",
    individual_languages: &[
    ],
};


pub const HOL: LanguageCode = LanguageCode {
    name: "Holu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hol",
    individual_languages: &[
    ],
};


pub const HOM: LanguageCode = LanguageCode {
    name: "Homa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hom",
    individual_languages: &[
    ],
};


pub const HOO: LanguageCode = LanguageCode {
    name: "Holoholo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hoo",
    individual_languages: &[
    ],
};


pub const HOP: LanguageCode = LanguageCode {
    name: "Hopi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hop",
    individual_languages: &[
    ],
};


pub const HOR: LanguageCode = LanguageCode {
    name: "Horo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hor",
    individual_languages: &[
    ],
};


pub const HOS: LanguageCode = LanguageCode {
    name: "Ho Chi Minh City Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hos",
    individual_languages: &[
    ],
};


pub const HOT: LanguageCode = LanguageCode {
    name: "Hote",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hot",
    individual_languages: &[
    ],
};


pub const HOV: LanguageCode = LanguageCode {
    name: "Hovongan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hov",
    individual_languages: &[
    ],
};


pub const HOW: LanguageCode = LanguageCode {
    name: "Honi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "how",
    individual_languages: &[
    ],
};


pub const HOY: LanguageCode = LanguageCode {
    name: "Holiya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hoy",
    individual_languages: &[
    ],
};


pub const HOZ: LanguageCode = LanguageCode {
    name: "Hozo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hoz",
    individual_languages: &[
    ],
};


pub const HPO: LanguageCode = LanguageCode {
    name: "Hpon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hpo",
    individual_languages: &[
    ],
};


pub const HPS: LanguageCode = LanguageCode {
    name: "Hawai'i Sign Language (HSL)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hps",
    individual_languages: &[
    ],
};


pub const HRA: LanguageCode = LanguageCode {
    name: "Hrangkhol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hra",
    individual_languages: &[
    ],
};


pub const HRC: LanguageCode = LanguageCode {
    name: "Niwer Mil",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hrc",
    individual_languages: &[
    ],
};


pub const HRE: LanguageCode = LanguageCode {
    name: "Hre",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hre",
    individual_languages: &[
    ],
};


pub const HRK: LanguageCode = LanguageCode {
    name: "Haruku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hrk",
    individual_languages: &[
    ],
};


pub const HRM: LanguageCode = LanguageCode {
    name: "Horned Miao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hrm",
    individual_languages: &[
    ],
};


pub const HRO: LanguageCode = LanguageCode {
    name: "Haroi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hro",
    individual_languages: &[
    ],
};


pub const HRP: LanguageCode = LanguageCode {
    name: "Nhirrpi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hrp",
    individual_languages: &[
    ],
};


pub const HRT: LanguageCode = LanguageCode {
    name: "Hértevin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hrt",
    individual_languages: &[
    ],
};


pub const HRU: LanguageCode = LanguageCode {
    name: "Hruso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hru",
    individual_languages: &[
    ],
};


pub const HRW: LanguageCode = LanguageCode {
    name: "Warwar Feni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hrw",
    individual_languages: &[
    ],
};


pub const HRX: LanguageCode = LanguageCode {
    name: "Hunsrik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hrx",
    individual_languages: &[
    ],
};


pub const HRZ: LanguageCode = LanguageCode {
    name: "Harzani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hrz",
    individual_languages: &[
    ],
};


pub const HSH: LanguageCode = LanguageCode {
    name: "Hungarian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hsh",
    individual_languages: &[
    ],
};


pub const HSL: LanguageCode = LanguageCode {
    name: "Hausa Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hsl",
    individual_languages: &[
    ],
};


pub const HSN: LanguageCode = LanguageCode {
    name: "Xiang Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hsn",
    individual_languages: &[
    ],
};


pub const HSS: LanguageCode = LanguageCode {
    name: "Harsusi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hss",
    individual_languages: &[
    ],
};


pub const HTI: LanguageCode = LanguageCode {
    name: "Hoti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hti",
    individual_languages: &[
    ],
};


pub const HTO: LanguageCode = LanguageCode {
    name: "Minica Huitoto",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hto",
    individual_languages: &[
    ],
};


pub const HTS: LanguageCode = LanguageCode {
    name: "Hadza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hts",
    individual_languages: &[
    ],
};


pub const HTU: LanguageCode = LanguageCode {
    name: "Hitu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "htu",
    individual_languages: &[
    ],
};


pub const HTX: LanguageCode = LanguageCode {
    name: "Middle Hittite",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "htx",
    individual_languages: &[
    ],
};


pub const HUB: LanguageCode = LanguageCode {
    name: "Huambisa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hub",
    individual_languages: &[
    ],
};


pub const HUC: LanguageCode = LanguageCode {
    name: "ǂHua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "huc",
    individual_languages: &[
    ],
};


pub const HUD: LanguageCode = LanguageCode {
    name: "Huaulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hud",
    individual_languages: &[
    ],
};


pub const HUE: LanguageCode = LanguageCode {
    name: "San Francisco Del Mar Huave",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hue",
    individual_languages: &[
    ],
};


pub const HUF: LanguageCode = LanguageCode {
    name: "Humene",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "huf",
    individual_languages: &[
    ],
};


pub const HUG: LanguageCode = LanguageCode {
    name: "Huachipaeri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hug",
    individual_languages: &[
    ],
};


pub const HUH: LanguageCode = LanguageCode {
    name: "Huilliche",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "huh",
    individual_languages: &[
    ],
};


pub const HUI: LanguageCode = LanguageCode {
    name: "Huli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hui",
    individual_languages: &[
    ],
};


pub const HUJ: LanguageCode = LanguageCode {
    name: "Northern Guiyang Hmong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "huj",
    individual_languages: &[
    ],
};


pub const HUK: LanguageCode = LanguageCode {
    name: "Hulung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "huk",
    individual_languages: &[
    ],
};


pub const HUL: LanguageCode = LanguageCode {
    name: "Hula",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hul",
    individual_languages: &[
    ],
};


pub const HUM: LanguageCode = LanguageCode {
    name: "Hungana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hum",
    individual_languages: &[
    ],
};


pub const HUO: LanguageCode = LanguageCode {
    name: "Hu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "huo",
    individual_languages: &[
    ],
};


pub const HUQ: LanguageCode = LanguageCode {
    name: "Tsat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "huq",
    individual_languages: &[
    ],
};


pub const HUR: LanguageCode = LanguageCode {
    name: "Halkomelem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hur",
    individual_languages: &[
    ],
};


pub const HUS: LanguageCode = LanguageCode {
    name: "Huastec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hus",
    individual_languages: &[
    ],
};


pub const HUT: LanguageCode = LanguageCode {
    name: "Humla",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hut",
    individual_languages: &[
    ],
};


pub const HUU: LanguageCode = LanguageCode {
    name: "Murui Huitoto",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "huu",
    individual_languages: &[
    ],
};


pub const HUV: LanguageCode = LanguageCode {
    name: "San Mateo Del Mar Huave",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "huv",
    individual_languages: &[
    ],
};


pub const HUW: LanguageCode = LanguageCode {
    name: "Hukumina",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "huw",
    individual_languages: &[
    ],
};


pub const HUX: LanguageCode = LanguageCode {
    name: "Nüpode Huitoto",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hux",
    individual_languages: &[
    ],
};


pub const HUY: LanguageCode = LanguageCode {
    name: "Hulaulá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "huy",
    individual_languages: &[
    ],
};


pub const HUZ: LanguageCode = LanguageCode {
    name: "Hunzib",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "huz",
    individual_languages: &[
    ],
};


pub const HVC: LanguageCode = LanguageCode {
    name: "Haitian Vodoun Culture Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hvc",
    individual_languages: &[
    ],
};


pub const HVE: LanguageCode = LanguageCode {
    name: "San Dionisio Del Mar Huave",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hve",
    individual_languages: &[
    ],
};


pub const HVK: LanguageCode = LanguageCode {
    name: "Haveke",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hvk",
    individual_languages: &[
    ],
};


pub const HVN: LanguageCode = LanguageCode {
    name: "Sabu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hvn",
    individual_languages: &[
    ],
};


pub const HVV: LanguageCode = LanguageCode {
    name: "Santa María Del Mar Huave",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hvv",
    individual_languages: &[
    ],
};


pub const HWA: LanguageCode = LanguageCode {
    name: "Wané",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hwa",
    individual_languages: &[
    ],
};


pub const HWC: LanguageCode = LanguageCode {
    name: "Hawai'i Creole English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hwc",
    individual_languages: &[
    ],
};


pub const HWO: LanguageCode = LanguageCode {
    name: "Hwana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hwo",
    individual_languages: &[
    ],
};


pub const HYA: LanguageCode = LanguageCode {
    name: "Hya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hya",
    individual_languages: &[
    ],
};


pub const HYW: LanguageCode = LanguageCode {
    name: "Western Armenian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "hyw",
    individual_languages: &[
    ],
};


pub const IAI: LanguageCode = LanguageCode {
    name: "Iaai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iai",
    individual_languages: &[
    ],
};


pub const IAN: LanguageCode = LanguageCode {
    name: "Iatmul",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ian",
    individual_languages: &[
    ],
};


pub const IAR: LanguageCode = LanguageCode {
    name: "Purari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iar",
    individual_languages: &[
    ],
};


pub const IBB: LanguageCode = LanguageCode {
    name: "Ibibio",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ibb",
    individual_languages: &[
    ],
};


pub const IBD: LanguageCode = LanguageCode {
    name: "Iwaidja",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ibd",
    individual_languages: &[
    ],
};


pub const IBE: LanguageCode = LanguageCode {
    name: "Akpes",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ibe",
    individual_languages: &[
    ],
};


pub const IBG: LanguageCode = LanguageCode {
    name: "Ibanag",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ibg",
    individual_languages: &[
    ],
};


pub const IBH: LanguageCode = LanguageCode {
    name: "Bih",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ibh",
    individual_languages: &[
    ],
};


pub const IBL: LanguageCode = LanguageCode {
    name: "Ibaloi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ibl",
    individual_languages: &[
    ],
};


pub const IBM: LanguageCode = LanguageCode {
    name: "Agoi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ibm",
    individual_languages: &[
    ],
};


pub const IBN: LanguageCode = LanguageCode {
    name: "Ibino",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ibn",
    individual_languages: &[
    ],
};


pub const IBR: LanguageCode = LanguageCode {
    name: "Ibuoro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ibr",
    individual_languages: &[
    ],
};


pub const IBU: LanguageCode = LanguageCode {
    name: "Ibu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ibu",
    individual_languages: &[
    ],
};


pub const IBY: LanguageCode = LanguageCode {
    name: "Ibani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iby",
    individual_languages: &[
    ],
};


pub const ICA: LanguageCode = LanguageCode {
    name: "Ede Ica",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ica",
    individual_languages: &[
    ],
};


pub const ICH: LanguageCode = LanguageCode {
    name: "Etkywan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ich",
    individual_languages: &[
    ],
};


pub const ICL: LanguageCode = LanguageCode {
    name: "Icelandic Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "icl",
    individual_languages: &[
    ],
};


pub const ICR: LanguageCode = LanguageCode {
    name: "Islander Creole English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "icr",
    individual_languages: &[
    ],
};


pub const IDA: LanguageCode = LanguageCode {
    name: "Idakho-Isukha-Tiriki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ida",
    individual_languages: &[
    ],
};


pub const IDB: LanguageCode = LanguageCode {
    name: "Indo-Portuguese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "idb",
    individual_languages: &[
    ],
};


pub const IDC: LanguageCode = LanguageCode {
    name: "Idon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "idc",
    individual_languages: &[
    ],
};


pub const IDD: LanguageCode = LanguageCode {
    name: "Ede Idaca",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "idd",
    individual_languages: &[
    ],
};


pub const IDE: LanguageCode = LanguageCode {
    name: "Idere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ide",
    individual_languages: &[
    ],
};


pub const IDI: LanguageCode = LanguageCode {
    name: "Idi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "idi",
    individual_languages: &[
    ],
};


pub const IDR: LanguageCode = LanguageCode {
    name: "Indri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "idr",
    individual_languages: &[
    ],
};


pub const IDS: LanguageCode = LanguageCode {
    name: "Idesa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ids",
    individual_languages: &[
    ],
};


pub const IDT: LanguageCode = LanguageCode {
    name: "Idaté",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "idt",
    individual_languages: &[
    ],
};


pub const IDU: LanguageCode = LanguageCode {
    name: "Idoma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "idu",
    individual_languages: &[
    ],
};


pub const IFA: LanguageCode = LanguageCode {
    name: "Amganad Ifugao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ifa",
    individual_languages: &[
    ],
};


pub const IFB: LanguageCode = LanguageCode {
    name: "Batad Ifugao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ifb",
    individual_languages: &[
    ],
};


pub const IFE: LanguageCode = LanguageCode {
    name: "Ifè",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ife",
    individual_languages: &[
    ],
};


pub const IFF: LanguageCode = LanguageCode {
    name: "Ifo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iff",
    individual_languages: &[
    ],
};


pub const IFK: LanguageCode = LanguageCode {
    name: "Tuwali Ifugao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ifk",
    individual_languages: &[
    ],
};


pub const IFM: LanguageCode = LanguageCode {
    name: "Teke-Fuumu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ifm",
    individual_languages: &[
    ],
};


pub const IFU: LanguageCode = LanguageCode {
    name: "Mayoyao Ifugao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ifu",
    individual_languages: &[
    ],
};


pub const IFY: LanguageCode = LanguageCode {
    name: "Keley-I Kallahan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ify",
    individual_languages: &[
    ],
};


pub const IGB: LanguageCode = LanguageCode {
    name: "Ebira",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "igb",
    individual_languages: &[
    ],
};


pub const IGE: LanguageCode = LanguageCode {
    name: "Igede",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ige",
    individual_languages: &[
    ],
};


pub const IGG: LanguageCode = LanguageCode {
    name: "Igana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "igg",
    individual_languages: &[
    ],
};


pub const IGL: LanguageCode = LanguageCode {
    name: "Igala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "igl",
    individual_languages: &[
    ],
};


pub const IGM: LanguageCode = LanguageCode {
    name: "Kanggape",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "igm",
    individual_languages: &[
    ],
};


pub const IGN: LanguageCode = LanguageCode {
    name: "Ignaciano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ign",
    individual_languages: &[
    ],
};


pub const IGO: LanguageCode = LanguageCode {
    name: "Isebe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "igo",
    individual_languages: &[
    ],
};


pub const IGS: LanguageCode = LanguageCode {
    name: "Interglossa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "igs",
    individual_languages: &[
    ],
};


pub const IGW: LanguageCode = LanguageCode {
    name: "Igwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "igw",
    individual_languages: &[
    ],
};


pub const IHB: LanguageCode = LanguageCode {
    name: "Iha Based Pidgin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ihb",
    individual_languages: &[
    ],
};


pub const IHI: LanguageCode = LanguageCode {
    name: "Ihievbe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ihi",
    individual_languages: &[
    ],
};


pub const IHP: LanguageCode = LanguageCode {
    name: "Iha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ihp",
    individual_languages: &[
    ],
};


pub const IHW: LanguageCode = LanguageCode {
    name: "Bidhawal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ihw",
    individual_languages: &[
    ],
};


pub const IIN: LanguageCode = LanguageCode {
    name: "Thiin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iin",
    individual_languages: &[
    ],
};


pub const IJC: LanguageCode = LanguageCode {
    name: "Izon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ijc",
    individual_languages: &[
    ],
};


pub const IJE: LanguageCode = LanguageCode {
    name: "Biseni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ije",
    individual_languages: &[
    ],
};


pub const IJJ: LanguageCode = LanguageCode {
    name: "Ede Ije",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ijj",
    individual_languages: &[
    ],
};


pub const IJN: LanguageCode = LanguageCode {
    name: "Kalabari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ijn",
    individual_languages: &[
    ],
};


pub const IJS: LanguageCode = LanguageCode {
    name: "Southeast Ijo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ijs",
    individual_languages: &[
    ],
};


pub const IKE: LanguageCode = LanguageCode {
    name: "Eastern Canadian Inuktitut",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ike",
    individual_languages: &[
    ],
};


pub const IKI: LanguageCode = LanguageCode {
    name: "Iko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iki",
    individual_languages: &[
    ],
};


pub const IKK: LanguageCode = LanguageCode {
    name: "Ika",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ikk",
    individual_languages: &[
    ],
};


pub const IKL: LanguageCode = LanguageCode {
    name: "Ikulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ikl",
    individual_languages: &[
    ],
};


pub const IKO: LanguageCode = LanguageCode {
    name: "Olulumo-Ikom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iko",
    individual_languages: &[
    ],
};


pub const IKP: LanguageCode = LanguageCode {
    name: "Ikpeshi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ikp",
    individual_languages: &[
    ],
};


pub const IKR: LanguageCode = LanguageCode {
    name: "Ikaranggal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ikr",
    individual_languages: &[
    ],
};


pub const IKS: LanguageCode = LanguageCode {
    name: "Inuit Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iks",
    individual_languages: &[
    ],
};


pub const IKT: LanguageCode = LanguageCode {
    name: "Inuinnaqtun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ikt",
    individual_languages: &[
    ],
};


pub const IKV: LanguageCode = LanguageCode {
    name: "Iku-Gora-Ankwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ikv",
    individual_languages: &[
    ],
};


pub const IKW: LanguageCode = LanguageCode {
    name: "Ikwere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ikw",
    individual_languages: &[
    ],
};


pub const IKX: LanguageCode = LanguageCode {
    name: "Ik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ikx",
    individual_languages: &[
    ],
};


pub const IKZ: LanguageCode = LanguageCode {
    name: "Ikizu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ikz",
    individual_languages: &[
    ],
};


pub const ILA: LanguageCode = LanguageCode {
    name: "Ile Ape",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ila",
    individual_languages: &[
    ],
};


pub const ILB: LanguageCode = LanguageCode {
    name: "Ila",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ilb",
    individual_languages: &[
    ],
};


pub const ILG: LanguageCode = LanguageCode {
    name: "Garig-Ilgar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ilg",
    individual_languages: &[
    ],
};


pub const ILI: LanguageCode = LanguageCode {
    name: "Ili Turki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ili",
    individual_languages: &[
    ],
};


pub const ILK: LanguageCode = LanguageCode {
    name: "Ilongot",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ilk",
    individual_languages: &[
    ],
};


pub const ILM: LanguageCode = LanguageCode {
    name: "Iranun (Malaysia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ilm",
    individual_languages: &[
    ],
};


pub const ILP: LanguageCode = LanguageCode {
    name: "Iranun (Philippines)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ilp",
    individual_languages: &[
    ],
};


pub const ILS: LanguageCode = LanguageCode {
    name: "International Sign",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ils",
    individual_languages: &[
    ],
};


pub const ILU: LanguageCode = LanguageCode {
    name: "Ili'uun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ilu",
    individual_languages: &[
    ],
};


pub const ILV: LanguageCode = LanguageCode {
    name: "Ilue",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ilv",
    individual_languages: &[
    ],
};


pub const IMA: LanguageCode = LanguageCode {
    name: "Mala Malasar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ima",
    individual_languages: &[
    ],
};


pub const IMI: LanguageCode = LanguageCode {
    name: "Anamgura",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "imi",
    individual_languages: &[
    ],
};


pub const IML: LanguageCode = LanguageCode {
    name: "Miluk",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iml",
    individual_languages: &[
    ],
};


pub const IMN: LanguageCode = LanguageCode {
    name: "Imonda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "imn",
    individual_languages: &[
    ],
};


pub const IMO: LanguageCode = LanguageCode {
    name: "Imbongu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "imo",
    individual_languages: &[
    ],
};


pub const IMR: LanguageCode = LanguageCode {
    name: "Imroing",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "imr",
    individual_languages: &[
    ],
};


pub const IMS: LanguageCode = LanguageCode {
    name: "Marsian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ims",
    individual_languages: &[
    ],
};


pub const IMT: LanguageCode = LanguageCode {
    name: "Imotong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "imt",
    individual_languages: &[
    ],
};


pub const IMY: LanguageCode = LanguageCode {
    name: "Milyan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "imy",
    individual_languages: &[
    ],
};


pub const INB: LanguageCode = LanguageCode {
    name: "Inga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "inb",
    individual_languages: &[
    ],
};


pub const ING: LanguageCode = LanguageCode {
    name: "Degexit'an",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ing",
    individual_languages: &[
    ],
};


pub const INJ: LanguageCode = LanguageCode {
    name: "Jungle Inga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "inj",
    individual_languages: &[
    ],
};


pub const INL: LanguageCode = LanguageCode {
    name: "Indonesian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "inl",
    individual_languages: &[
    ],
};


pub const INM: LanguageCode = LanguageCode {
    name: "Minaean",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "inm",
    individual_languages: &[
    ],
};


pub const INN: LanguageCode = LanguageCode {
    name: "Isinai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "inn",
    individual_languages: &[
    ],
};


pub const INO: LanguageCode = LanguageCode {
    name: "Inoke-Yate",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ino",
    individual_languages: &[
    ],
};


pub const INP: LanguageCode = LanguageCode {
    name: "Iñapari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "inp",
    individual_languages: &[
    ],
};


pub const INS: LanguageCode = LanguageCode {
    name: "Indian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ins",
    individual_languages: &[
    ],
};


pub const INT: LanguageCode = LanguageCode {
    name: "Intha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "int",
    individual_languages: &[
    ],
};


pub const INZ: LanguageCode = LanguageCode {
    name: "Ineseño",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "inz",
    individual_languages: &[
    ],
};


pub const IOR: LanguageCode = LanguageCode {
    name: "Inor",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ior",
    individual_languages: &[
    ],
};


pub const IOU: LanguageCode = LanguageCode {
    name: "Tuma-Irumu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iou",
    individual_languages: &[
    ],
};


pub const IOW: LanguageCode = LanguageCode {
    name: "Iowa-Oto",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iow",
    individual_languages: &[
    ],
};


pub const IPI: LanguageCode = LanguageCode {
    name: "Ipili",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ipi",
    individual_languages: &[
    ],
};


pub const IPO: LanguageCode = LanguageCode {
    name: "Ipiko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ipo",
    individual_languages: &[
    ],
};


pub const IQU: LanguageCode = LanguageCode {
    name: "Iquito",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iqu",
    individual_languages: &[
    ],
};


pub const IQW: LanguageCode = LanguageCode {
    name: "Ikwo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iqw",
    individual_languages: &[
    ],
};


pub const IRE: LanguageCode = LanguageCode {
    name: "Iresim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ire",
    individual_languages: &[
    ],
};


pub const IRH: LanguageCode = LanguageCode {
    name: "Irarutu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "irh",
    individual_languages: &[
    ],
};


pub const IRI: LanguageCode = LanguageCode {
    name: "Rigwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iri",
    individual_languages: &[
    ],
};


pub const IRK: LanguageCode = LanguageCode {
    name: "Iraqw",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "irk",
    individual_languages: &[
    ],
};


pub const IRN: LanguageCode = LanguageCode {
    name: "Irántxe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "irn",
    individual_languages: &[
    ],
};


pub const IRR: LanguageCode = LanguageCode {
    name: "Ir",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "irr",
    individual_languages: &[
    ],
};


pub const IRU: LanguageCode = LanguageCode {
    name: "Irula",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iru",
    individual_languages: &[
    ],
};


pub const IRX: LanguageCode = LanguageCode {
    name: "Kamberau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "irx",
    individual_languages: &[
    ],
};


pub const IRY: LanguageCode = LanguageCode {
    name: "Iraya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iry",
    individual_languages: &[
    ],
};


pub const ISA: LanguageCode = LanguageCode {
    name: "Isabi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "isa",
    individual_languages: &[
    ],
};


pub const ISC: LanguageCode = LanguageCode {
    name: "Isconahua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "isc",
    individual_languages: &[
    ],
};


pub const ISD: LanguageCode = LanguageCode {
    name: "Isnag",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "isd",
    individual_languages: &[
    ],
};


pub const ISE: LanguageCode = LanguageCode {
    name: "Italian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ise",
    individual_languages: &[
    ],
};


pub const ISG: LanguageCode = LanguageCode {
    name: "Irish Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "isg",
    individual_languages: &[
    ],
};


pub const ISH: LanguageCode = LanguageCode {
    name: "Esan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ish",
    individual_languages: &[
    ],
};


pub const ISI: LanguageCode = LanguageCode {
    name: "Nkem-Nkum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "isi",
    individual_languages: &[
    ],
};


pub const ISK: LanguageCode = LanguageCode {
    name: "Ishkashimi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "isk",
    individual_languages: &[
    ],
};


pub const ISM: LanguageCode = LanguageCode {
    name: "Masimasi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ism",
    individual_languages: &[
    ],
};


pub const ISN: LanguageCode = LanguageCode {
    name: "Isanzu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "isn",
    individual_languages: &[
    ],
};


pub const ISO: LanguageCode = LanguageCode {
    name: "Isoko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iso",
    individual_languages: &[
    ],
};


pub const ISR: LanguageCode = LanguageCode {
    name: "Israeli Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "isr",
    individual_languages: &[
    ],
};


pub const IST: LanguageCode = LanguageCode {
    name: "Istriot",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ist",
    individual_languages: &[
    ],
};


pub const ISU: LanguageCode = LanguageCode {
    name: "Isu (Menchum Division)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "isu",
    individual_languages: &[
    ],
};


pub const ITB: LanguageCode = LanguageCode {
    name: "Binongan Itneg",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "itb",
    individual_languages: &[
    ],
};


pub const ITD: LanguageCode = LanguageCode {
    name: "Southern Tidung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "itd",
    individual_languages: &[
    ],
};


pub const ITE: LanguageCode = LanguageCode {
    name: "Itene",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ite",
    individual_languages: &[
    ],
};


pub const ITI: LanguageCode = LanguageCode {
    name: "Inlaod Itneg",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iti",
    individual_languages: &[
    ],
};


pub const ITK: LanguageCode = LanguageCode {
    name: "Judeo-Italian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "itk",
    individual_languages: &[
    ],
};


pub const ITL: LanguageCode = LanguageCode {
    name: "Itelmen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "itl",
    individual_languages: &[
    ],
};


pub const ITM: LanguageCode = LanguageCode {
    name: "Itu Mbon Uzo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "itm",
    individual_languages: &[
    ],
};


pub const ITO: LanguageCode = LanguageCode {
    name: "Itonama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ito",
    individual_languages: &[
    ],
};


pub const ITR: LanguageCode = LanguageCode {
    name: "Iteri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "itr",
    individual_languages: &[
    ],
};


pub const ITS: LanguageCode = LanguageCode {
    name: "Isekiri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "its",
    individual_languages: &[
    ],
};


pub const ITT: LanguageCode = LanguageCode {
    name: "Maeng Itneg",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "itt",
    individual_languages: &[
    ],
};


pub const ITV: LanguageCode = LanguageCode {
    name: "Itawit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "itv",
    individual_languages: &[
    ],
};


pub const ITW: LanguageCode = LanguageCode {
    name: "Ito",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "itw",
    individual_languages: &[
    ],
};


pub const ITX: LanguageCode = LanguageCode {
    name: "Itik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "itx",
    individual_languages: &[
    ],
};


pub const ITY: LanguageCode = LanguageCode {
    name: "Moyadan Itneg",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ity",
    individual_languages: &[
    ],
};


pub const ITZ: LanguageCode = LanguageCode {
    name: "Itzá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "itz",
    individual_languages: &[
    ],
};


pub const IUM: LanguageCode = LanguageCode {
    name: "Iu Mien",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ium",
    individual_languages: &[
    ],
};


pub const IVB: LanguageCode = LanguageCode {
    name: "Ibatan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ivb",
    individual_languages: &[
    ],
};


pub const IVV: LanguageCode = LanguageCode {
    name: "Ivatan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ivv",
    individual_languages: &[
    ],
};


pub const IWK: LanguageCode = LanguageCode {
    name: "I-Wak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iwk",
    individual_languages: &[
    ],
};


pub const IWM: LanguageCode = LanguageCode {
    name: "Iwam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iwm",
    individual_languages: &[
    ],
};


pub const IWO: LanguageCode = LanguageCode {
    name: "Iwur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iwo",
    individual_languages: &[
    ],
};


pub const IWS: LanguageCode = LanguageCode {
    name: "Sepik Iwam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iws",
    individual_languages: &[
    ],
};


pub const IXC: LanguageCode = LanguageCode {
    name: "Ixcatec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ixc",
    individual_languages: &[
    ],
};


pub const IXL: LanguageCode = LanguageCode {
    name: "Ixil",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ixl",
    individual_languages: &[
    ],
};


pub const IYA: LanguageCode = LanguageCode {
    name: "Iyayu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iya",
    individual_languages: &[
    ],
};


pub const IYO: LanguageCode = LanguageCode {
    name: "Mesaka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iyo",
    individual_languages: &[
    ],
};


pub const IYX: LanguageCode = LanguageCode {
    name: "Yaka (Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "iyx",
    individual_languages: &[
    ],
};


pub const IZH: LanguageCode = LanguageCode {
    name: "Ingrian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "izh",
    individual_languages: &[
    ],
};


pub const IZR: LanguageCode = LanguageCode {
    name: "Izere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "izr",
    individual_languages: &[
    ],
};


pub const IZZ: LanguageCode = LanguageCode {
    name: "Izii",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "izz",
    individual_languages: &[
    ],
};


pub const JAA: LanguageCode = LanguageCode {
    name: "Jamamadí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jaa",
    individual_languages: &[
    ],
};


pub const JAB: LanguageCode = LanguageCode {
    name: "Hyam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jab",
    individual_languages: &[
    ],
};


pub const JAC: LanguageCode = LanguageCode {
    name: "Popti'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jac",
    individual_languages: &[
    ],
};


pub const JAD: LanguageCode = LanguageCode {
    name: "Jahanka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jad",
    individual_languages: &[
    ],
};


pub const JAE: LanguageCode = LanguageCode {
    name: "Yabem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jae",
    individual_languages: &[
    ],
};


pub const JAF: LanguageCode = LanguageCode {
    name: "Jara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jaf",
    individual_languages: &[
    ],
};


pub const JAH: LanguageCode = LanguageCode {
    name: "Jah Hut",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jah",
    individual_languages: &[
    ],
};


pub const JAJ: LanguageCode = LanguageCode {
    name: "Zazao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jaj",
    individual_languages: &[
    ],
};


pub const JAK: LanguageCode = LanguageCode {
    name: "Jakun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jak",
    individual_languages: &[
    ],
};


pub const JAL: LanguageCode = LanguageCode {
    name: "Yalahatan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jal",
    individual_languages: &[
    ],
};


pub const JAM: LanguageCode = LanguageCode {
    name: "Jamaican Creole English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jam",
    individual_languages: &[
    ],
};


pub const JAN: LanguageCode = LanguageCode {
    name: "Jandai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jan",
    individual_languages: &[
    ],
};


pub const JAO: LanguageCode = LanguageCode {
    name: "Yanyuwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jao",
    individual_languages: &[
    ],
};


pub const JAQ: LanguageCode = LanguageCode {
    name: "Yaqay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jaq",
    individual_languages: &[
    ],
};


pub const JAS: LanguageCode = LanguageCode {
    name: "New Caledonian Javanese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jas",
    individual_languages: &[
    ],
};


pub const JAT: LanguageCode = LanguageCode {
    name: "Jakati",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jat",
    individual_languages: &[
    ],
};


pub const JAU: LanguageCode = LanguageCode {
    name: "Yaur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jau",
    individual_languages: &[
    ],
};


pub const JAX: LanguageCode = LanguageCode {
    name: "Jambi Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jax",
    individual_languages: &[
    ],
};


pub const JAY: LanguageCode = LanguageCode {
    name: "Yan-nhangu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jay",
    individual_languages: &[
    ],
};


pub const JAZ: LanguageCode = LanguageCode {
    name: "Jawe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jaz",
    individual_languages: &[
    ],
};


pub const JBE: LanguageCode = LanguageCode {
    name: "Judeo-Berber",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jbe",
    individual_languages: &[
    ],
};


pub const JBI: LanguageCode = LanguageCode {
    name: "Badjiri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jbi",
    individual_languages: &[
    ],
};


pub const JBJ: LanguageCode = LanguageCode {
    name: "Arandai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jbj",
    individual_languages: &[
    ],
};


pub const JBK: LanguageCode = LanguageCode {
    name: "Barikewa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jbk",
    individual_languages: &[
    ],
};


pub const JBM: LanguageCode = LanguageCode {
    name: "Bijim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jbm",
    individual_languages: &[
    ],
};


pub const JBN: LanguageCode = LanguageCode {
    name: "Nafusi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jbn",
    individual_languages: &[
    ],
};


pub const JBR: LanguageCode = LanguageCode {
    name: "Jofotek-Bromnya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jbr",
    individual_languages: &[
    ],
};


pub const JBT: LanguageCode = LanguageCode {
    name: "Jabutí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jbt",
    individual_languages: &[
    ],
};


pub const JBU: LanguageCode = LanguageCode {
    name: "Jukun Takum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jbu",
    individual_languages: &[
    ],
};


pub const JBW: LanguageCode = LanguageCode {
    name: "Yawijibaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jbw",
    individual_languages: &[
    ],
};


pub const JCS: LanguageCode = LanguageCode {
    name: "Jamaican Country Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jcs",
    individual_languages: &[
    ],
};


pub const JCT: LanguageCode = LanguageCode {
    name: "Krymchak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jct",
    individual_languages: &[
    ],
};


pub const JDA: LanguageCode = LanguageCode {
    name: "Jad",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jda",
    individual_languages: &[
    ],
};


pub const JDG: LanguageCode = LanguageCode {
    name: "Jadgali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jdg",
    individual_languages: &[
    ],
};


pub const JDT: LanguageCode = LanguageCode {
    name: "Judeo-Tat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jdt",
    individual_languages: &[
    ],
};


pub const JEB: LanguageCode = LanguageCode {
    name: "Jebero",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jeb",
    individual_languages: &[
    ],
};


pub const JEE: LanguageCode = LanguageCode {
    name: "Jerung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jee",
    individual_languages: &[
    ],
};


pub const JEH: LanguageCode = LanguageCode {
    name: "Jeh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jeh",
    individual_languages: &[
    ],
};


pub const JEI: LanguageCode = LanguageCode {
    name: "Yei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jei",
    individual_languages: &[
    ],
};


pub const JEK: LanguageCode = LanguageCode {
    name: "Jeri Kuo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jek",
    individual_languages: &[
    ],
};


pub const JEL: LanguageCode = LanguageCode {
    name: "Yelmek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jel",
    individual_languages: &[
    ],
};


pub const JEN: LanguageCode = LanguageCode {
    name: "Dza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jen",
    individual_languages: &[
    ],
};


pub const JER: LanguageCode = LanguageCode {
    name: "Jere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jer",
    individual_languages: &[
    ],
};


pub const JET: LanguageCode = LanguageCode {
    name: "Manem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jet",
    individual_languages: &[
    ],
};


pub const JEU: LanguageCode = LanguageCode {
    name: "Jonkor Bourmataguil",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jeu",
    individual_languages: &[
    ],
};


pub const JGB: LanguageCode = LanguageCode {
    name: "Ngbee",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jgb",
    individual_languages: &[
    ],
};


pub const JGE: LanguageCode = LanguageCode {
    name: "Judeo-Georgian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jge",
    individual_languages: &[
    ],
};


pub const JGK: LanguageCode = LanguageCode {
    name: "Gwak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jgk",
    individual_languages: &[
    ],
};


pub const JGO: LanguageCode = LanguageCode {
    name: "Ngomba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jgo",
    individual_languages: &[
    ],
};


pub const JHI: LanguageCode = LanguageCode {
    name: "Jehai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jhi",
    individual_languages: &[
    ],
};


pub const JHS: LanguageCode = LanguageCode {
    name: "Jhankot Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jhs",
    individual_languages: &[
    ],
};


pub const JIA: LanguageCode = LanguageCode {
    name: "Jina",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jia",
    individual_languages: &[
    ],
};


pub const JIB: LanguageCode = LanguageCode {
    name: "Jibu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jib",
    individual_languages: &[
    ],
};


pub const JIC: LanguageCode = LanguageCode {
    name: "Tol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jic",
    individual_languages: &[
    ],
};


pub const JID: LanguageCode = LanguageCode {
    name: "Bu (Kaduna State)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jid",
    individual_languages: &[
    ],
};


pub const JIE: LanguageCode = LanguageCode {
    name: "Jilbe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jie",
    individual_languages: &[
    ],
};


pub const JIG: LanguageCode = LanguageCode {
    name: "Jingulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jig",
    individual_languages: &[
    ],
};


pub const JIH: LanguageCode = LanguageCode {
    name: "sTodsde",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jih",
    individual_languages: &[
    ],
};


pub const JII: LanguageCode = LanguageCode {
    name: "Jiiddu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jii",
    individual_languages: &[
    ],
};


pub const JIL: LanguageCode = LanguageCode {
    name: "Jilim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jil",
    individual_languages: &[
    ],
};


pub const JIM: LanguageCode = LanguageCode {
    name: "Jimi (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jim",
    individual_languages: &[
    ],
};


pub const JIO: LanguageCode = LanguageCode {
    name: "Jiamao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jio",
    individual_languages: &[
    ],
};


pub const JIQ: LanguageCode = LanguageCode {
    name: "Guanyinqiao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jiq",
    individual_languages: &[
    ],
};


pub const JIT: LanguageCode = LanguageCode {
    name: "Jita",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jit",
    individual_languages: &[
    ],
};


pub const JIU: LanguageCode = LanguageCode {
    name: "Youle Jinuo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jiu",
    individual_languages: &[
    ],
};


pub const JIV: LanguageCode = LanguageCode {
    name: "Shuar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jiv",
    individual_languages: &[
    ],
};


pub const JIY: LanguageCode = LanguageCode {
    name: "Buyuan Jinuo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jiy",
    individual_languages: &[
    ],
};


pub const JJE: LanguageCode = LanguageCode {
    name: "Jejueo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jje",
    individual_languages: &[
    ],
};


pub const JJR: LanguageCode = LanguageCode {
    name: "Bankal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jjr",
    individual_languages: &[
    ],
};


pub const JKA: LanguageCode = LanguageCode {
    name: "Kaera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jka",
    individual_languages: &[
    ],
};


pub const JKM: LanguageCode = LanguageCode {
    name: "Mobwa Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jkm",
    individual_languages: &[
    ],
};


pub const JKO: LanguageCode = LanguageCode {
    name: "Kubo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jko",
    individual_languages: &[
    ],
};


pub const JKP: LanguageCode = LanguageCode {
    name: "Paku Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jkp",
    individual_languages: &[
    ],
};


pub const JKR: LanguageCode = LanguageCode {
    name: "Koro (India)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jkr",
    individual_languages: &[
    ],
};


pub const JKS: LanguageCode = LanguageCode {
    name: "Amami Koniya Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jks",
    individual_languages: &[
    ],
};


pub const JKU: LanguageCode = LanguageCode {
    name: "Labir",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jku",
    individual_languages: &[
    ],
};


pub const JLE: LanguageCode = LanguageCode {
    name: "Ngile",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jle",
    individual_languages: &[
    ],
};


pub const JLS: LanguageCode = LanguageCode {
    name: "Jamaican Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jls",
    individual_languages: &[
    ],
};


pub const JMA: LanguageCode = LanguageCode {
    name: "Dima",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jma",
    individual_languages: &[
    ],
};


pub const JMB: LanguageCode = LanguageCode {
    name: "Zumbun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jmb",
    individual_languages: &[
    ],
};


pub const JMC: LanguageCode = LanguageCode {
    name: "Machame",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jmc",
    individual_languages: &[
    ],
};


pub const JMD: LanguageCode = LanguageCode {
    name: "Yamdena",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jmd",
    individual_languages: &[
    ],
};


pub const JMI: LanguageCode = LanguageCode {
    name: "Jimi (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jmi",
    individual_languages: &[
    ],
};


pub const JML: LanguageCode = LanguageCode {
    name: "Jumli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jml",
    individual_languages: &[
    ],
};


pub const JMN: LanguageCode = LanguageCode {
    name: "Makuri Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jmn",
    individual_languages: &[
    ],
};


pub const JMR: LanguageCode = LanguageCode {
    name: "Kamara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jmr",
    individual_languages: &[
    ],
};


pub const JMS: LanguageCode = LanguageCode {
    name: "Mashi (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jms",
    individual_languages: &[
    ],
};


pub const JMW: LanguageCode = LanguageCode {
    name: "Mouwase",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jmw",
    individual_languages: &[
    ],
};


pub const JMX: LanguageCode = LanguageCode {
    name: "Western Juxtlahuaca Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jmx",
    individual_languages: &[
    ],
};


pub const JNA: LanguageCode = LanguageCode {
    name: "Jangshung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jna",
    individual_languages: &[
    ],
};


pub const JND: LanguageCode = LanguageCode {
    name: "Jandavra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jnd",
    individual_languages: &[
    ],
};


pub const JNG: LanguageCode = LanguageCode {
    name: "Yangman",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jng",
    individual_languages: &[
    ],
};


pub const JNI: LanguageCode = LanguageCode {
    name: "Janji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jni",
    individual_languages: &[
    ],
};


pub const JNJ: LanguageCode = LanguageCode {
    name: "Yemsa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jnj",
    individual_languages: &[
    ],
};


pub const JNL: LanguageCode = LanguageCode {
    name: "Rawat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jnl",
    individual_languages: &[
    ],
};


pub const JNS: LanguageCode = LanguageCode {
    name: "Jaunsari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jns",
    individual_languages: &[
    ],
};


pub const JOB: LanguageCode = LanguageCode {
    name: "Joba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "job",
    individual_languages: &[
    ],
};


pub const JOD: LanguageCode = LanguageCode {
    name: "Wojenaka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jod",
    individual_languages: &[
    ],
};


pub const JOG: LanguageCode = LanguageCode {
    name: "Jogi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jog",
    individual_languages: &[
    ],
};


pub const JOR: LanguageCode = LanguageCode {
    name: "Jorá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jor",
    individual_languages: &[
    ],
};


pub const JOS: LanguageCode = LanguageCode {
    name: "Jordanian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jos",
    individual_languages: &[
    ],
};


pub const JOW: LanguageCode = LanguageCode {
    name: "Jowulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jow",
    individual_languages: &[
    ],
};


pub const JPA: LanguageCode = LanguageCode {
    name: "Jewish Palestinian Aramaic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jpa",
    individual_languages: &[
    ],
};


pub const JQR: LanguageCode = LanguageCode {
    name: "Jaqaru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jqr",
    individual_languages: &[
    ],
};


pub const JRA: LanguageCode = LanguageCode {
    name: "Jarai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jra",
    individual_languages: &[
    ],
};


pub const JRR: LanguageCode = LanguageCode {
    name: "Jiru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jrr",
    individual_languages: &[
    ],
};


pub const JRT: LanguageCode = LanguageCode {
    name: "Jakattoe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jrt",
    individual_languages: &[
    ],
};


pub const JRU: LanguageCode = LanguageCode {
    name: "Japrería",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jru",
    individual_languages: &[
    ],
};


pub const JSL: LanguageCode = LanguageCode {
    name: "Japanese Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jsl",
    individual_languages: &[
    ],
};


pub const JUA: LanguageCode = LanguageCode {
    name: "Júma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jua",
    individual_languages: &[
    ],
};


pub const JUB: LanguageCode = LanguageCode {
    name: "Wannu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jub",
    individual_languages: &[
    ],
};


pub const JUC: LanguageCode = LanguageCode {
    name: "Jurchen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "juc",
    individual_languages: &[
    ],
};


pub const JUD: LanguageCode = LanguageCode {
    name: "Worodougou",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jud",
    individual_languages: &[
    ],
};


pub const JUH: LanguageCode = LanguageCode {
    name: "Hõne",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "juh",
    individual_languages: &[
    ],
};


pub const JUI: LanguageCode = LanguageCode {
    name: "Ngadjuri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jui",
    individual_languages: &[
    ],
};


pub const JUK: LanguageCode = LanguageCode {
    name: "Wapan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "juk",
    individual_languages: &[
    ],
};


pub const JUL: LanguageCode = LanguageCode {
    name: "Jirel",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jul",
    individual_languages: &[
    ],
};


pub const JUM: LanguageCode = LanguageCode {
    name: "Jumjum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jum",
    individual_languages: &[
    ],
};


pub const JUN: LanguageCode = LanguageCode {
    name: "Juang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jun",
    individual_languages: &[
    ],
};


pub const JUO: LanguageCode = LanguageCode {
    name: "Jiba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "juo",
    individual_languages: &[
    ],
};


pub const JUP: LanguageCode = LanguageCode {
    name: "Hupdë",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jup",
    individual_languages: &[
    ],
};


pub const JUR: LanguageCode = LanguageCode {
    name: "Jurúna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jur",
    individual_languages: &[
    ],
};


pub const JUS: LanguageCode = LanguageCode {
    name: "Jumla Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jus",
    individual_languages: &[
    ],
};


pub const JUT: LanguageCode = LanguageCode {
    name: "Jutish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jut",
    individual_languages: &[
    ],
};


pub const JUU: LanguageCode = LanguageCode {
    name: "Ju",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "juu",
    individual_languages: &[
    ],
};


pub const JUW: LanguageCode = LanguageCode {
    name: "Wãpha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "juw",
    individual_languages: &[
    ],
};


pub const JUY: LanguageCode = LanguageCode {
    name: "Juray",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "juy",
    individual_languages: &[
    ],
};


pub const JVD: LanguageCode = LanguageCode {
    name: "Javindo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jvd",
    individual_languages: &[
    ],
};


pub const JVN: LanguageCode = LanguageCode {
    name: "Caribbean Javanese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jvn",
    individual_languages: &[
    ],
};


pub const JWI: LanguageCode = LanguageCode {
    name: "Jwira-Pepesa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jwi",
    individual_languages: &[
    ],
};


pub const JYA: LanguageCode = LanguageCode {
    name: "Jiarong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jya",
    individual_languages: &[
    ],
};


pub const JYE: LanguageCode = LanguageCode {
    name: "Judeo-Yemeni Arabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jye",
    individual_languages: &[
    ],
};


pub const JYY: LanguageCode = LanguageCode {
    name: "Jaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "jyy",
    individual_languages: &[
    ],
};


pub const KAD: LanguageCode = LanguageCode {
    name: "Adara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kad",
    individual_languages: &[
    ],
};


pub const KAE: LanguageCode = LanguageCode {
    name: "Ketangalan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kae",
    individual_languages: &[
    ],
};


pub const KAF: LanguageCode = LanguageCode {
    name: "Katso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kaf",
    individual_languages: &[
    ],
};


pub const KAG: LanguageCode = LanguageCode {
    name: "Kajaman",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kag",
    individual_languages: &[
    ],
};


pub const KAH: LanguageCode = LanguageCode {
    name: "Kara (Central African Republic)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kah",
    individual_languages: &[
    ],
};


pub const KAI: LanguageCode = LanguageCode {
    name: "Karekare",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kai",
    individual_languages: &[
    ],
};


pub const KAJ: LanguageCode = LanguageCode {
    name: "Jju",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kaj",
    individual_languages: &[
    ],
};


pub const KAK: LanguageCode = LanguageCode {
    name: "Kalanguya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kak",
    individual_languages: &[
    ],
};


pub const KAO: LanguageCode = LanguageCode {
    name: "Xaasongaxango",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kao",
    individual_languages: &[
    ],
};


pub const KAP: LanguageCode = LanguageCode {
    name: "Bezhta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kap",
    individual_languages: &[
    ],
};


pub const KAQ: LanguageCode = LanguageCode {
    name: "Capanahua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kaq",
    individual_languages: &[
    ],
};


pub const KAV: LanguageCode = LanguageCode {
    name: "Katukína",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kav",
    individual_languages: &[
    ],
};


pub const KAX: LanguageCode = LanguageCode {
    name: "Kao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kax",
    individual_languages: &[
    ],
};


pub const KAY: LanguageCode = LanguageCode {
    name: "Kamayurá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kay",
    individual_languages: &[
    ],
};


pub const KBA: LanguageCode = LanguageCode {
    name: "Kalarko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kba",
    individual_languages: &[
    ],
};


pub const KBB: LanguageCode = LanguageCode {
    name: "Kaxuiâna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbb",
    individual_languages: &[
    ],
};


pub const KBC: LanguageCode = LanguageCode {
    name: "Kadiwéu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbc",
    individual_languages: &[
    ],
};


pub const KBE: LanguageCode = LanguageCode {
    name: "Kanju",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbe",
    individual_languages: &[
    ],
};


pub const KBG: LanguageCode = LanguageCode {
    name: "Khamba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbg",
    individual_languages: &[
    ],
};


pub const KBH: LanguageCode = LanguageCode {
    name: "Camsá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbh",
    individual_languages: &[
    ],
};


pub const KBI: LanguageCode = LanguageCode {
    name: "Kaptiau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbi",
    individual_languages: &[
    ],
};


pub const KBJ: LanguageCode = LanguageCode {
    name: "Kari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbj",
    individual_languages: &[
    ],
};


pub const KBK: LanguageCode = LanguageCode {
    name: "Grass Koiari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbk",
    individual_languages: &[
    ],
};


pub const KBL: LanguageCode = LanguageCode {
    name: "Kanembu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbl",
    individual_languages: &[
    ],
};


pub const KBM: LanguageCode = LanguageCode {
    name: "Iwal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbm",
    individual_languages: &[
    ],
};


pub const KBN: LanguageCode = LanguageCode {
    name: "Kare (Central African Republic)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbn",
    individual_languages: &[
    ],
};


pub const KBO: LanguageCode = LanguageCode {
    name: "Keliko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbo",
    individual_languages: &[
    ],
};


pub const KBP: LanguageCode = LanguageCode {
    name: "Kabiyè",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbp",
    individual_languages: &[
    ],
};


pub const KBQ: LanguageCode = LanguageCode {
    name: "Kamano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbq",
    individual_languages: &[
    ],
};


pub const KBR: LanguageCode = LanguageCode {
    name: "Kafa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbr",
    individual_languages: &[
    ],
};


pub const KBS: LanguageCode = LanguageCode {
    name: "Kande",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbs",
    individual_languages: &[
    ],
};


pub const KBT: LanguageCode = LanguageCode {
    name: "Abadi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbt",
    individual_languages: &[
    ],
};


pub const KBU: LanguageCode = LanguageCode {
    name: "Kabutra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbu",
    individual_languages: &[
    ],
};


pub const KBV: LanguageCode = LanguageCode {
    name: "Dera (Indonesia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbv",
    individual_languages: &[
    ],
};


pub const KBW: LanguageCode = LanguageCode {
    name: "Kaiep",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbw",
    individual_languages: &[
    ],
};


pub const KBX: LanguageCode = LanguageCode {
    name: "Ap Ma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbx",
    individual_languages: &[
    ],
};


pub const KBY: LanguageCode = LanguageCode {
    name: "Manga Kanuri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kby",
    individual_languages: &[
    ],
};


pub const KBZ: LanguageCode = LanguageCode {
    name: "Duhwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kbz",
    individual_languages: &[
    ],
};


pub const KCA: LanguageCode = LanguageCode {
    name: "Khanty",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kca",
    individual_languages: &[
    ],
};


pub const KCB: LanguageCode = LanguageCode {
    name: "Kawacha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcb",
    individual_languages: &[
    ],
};


pub const KCC: LanguageCode = LanguageCode {
    name: "Lubila",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcc",
    individual_languages: &[
    ],
};


pub const KCD: LanguageCode = LanguageCode {
    name: "Ngkâlmpw Kanum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcd",
    individual_languages: &[
    ],
};


pub const KCE: LanguageCode = LanguageCode {
    name: "Kaivi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kce",
    individual_languages: &[
    ],
};


pub const KCF: LanguageCode = LanguageCode {
    name: "Ukaan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcf",
    individual_languages: &[
    ],
};


pub const KCG: LanguageCode = LanguageCode {
    name: "Tyap",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcg",
    individual_languages: &[
    ],
};


pub const KCH: LanguageCode = LanguageCode {
    name: "Vono",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kch",
    individual_languages: &[
    ],
};


pub const KCI: LanguageCode = LanguageCode {
    name: "Kamantan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kci",
    individual_languages: &[
    ],
};


pub const KCJ: LanguageCode = LanguageCode {
    name: "Kobiana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcj",
    individual_languages: &[
    ],
};


pub const KCK: LanguageCode = LanguageCode {
    name: "Kalanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kck",
    individual_languages: &[
    ],
};


pub const KCL: LanguageCode = LanguageCode {
    name: "Kela (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcl",
    individual_languages: &[
    ],
};


pub const KCM: LanguageCode = LanguageCode {
    name: "Gula (Central African Republic)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcm",
    individual_languages: &[
    ],
};


pub const KCN: LanguageCode = LanguageCode {
    name: "Nubi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcn",
    individual_languages: &[
    ],
};


pub const KCO: LanguageCode = LanguageCode {
    name: "Kinalakna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kco",
    individual_languages: &[
    ],
};


pub const KCP: LanguageCode = LanguageCode {
    name: "Kanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcp",
    individual_languages: &[
    ],
};


pub const KCQ: LanguageCode = LanguageCode {
    name: "Kamo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcq",
    individual_languages: &[
    ],
};


pub const KCR: LanguageCode = LanguageCode {
    name: "Katla",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcr",
    individual_languages: &[
    ],
};


pub const KCS: LanguageCode = LanguageCode {
    name: "Koenoem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcs",
    individual_languages: &[
    ],
};


pub const KCT: LanguageCode = LanguageCode {
    name: "Kaian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kct",
    individual_languages: &[
    ],
};


pub const KCU: LanguageCode = LanguageCode {
    name: "Kami (Tanzania)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcu",
    individual_languages: &[
    ],
};


pub const KCV: LanguageCode = LanguageCode {
    name: "Kete",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcv",
    individual_languages: &[
    ],
};


pub const KCW: LanguageCode = LanguageCode {
    name: "Kabwari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcw",
    individual_languages: &[
    ],
};


pub const KCX: LanguageCode = LanguageCode {
    name: "Kachama-Ganjule",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcx",
    individual_languages: &[
    ],
};


pub const KCY: LanguageCode = LanguageCode {
    name: "Korandje",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcy",
    individual_languages: &[
    ],
};


pub const KCZ: LanguageCode = LanguageCode {
    name: "Konongo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kcz",
    individual_languages: &[
    ],
};


pub const KDA: LanguageCode = LanguageCode {
    name: "Worimi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kda",
    individual_languages: &[
    ],
};


pub const KDC: LanguageCode = LanguageCode {
    name: "Kutu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdc",
    individual_languages: &[
    ],
};


pub const KDD: LanguageCode = LanguageCode {
    name: "Yankunytjatjara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdd",
    individual_languages: &[
    ],
};


pub const KDE: LanguageCode = LanguageCode {
    name: "Makonde",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kde",
    individual_languages: &[
    ],
};


pub const KDF: LanguageCode = LanguageCode {
    name: "Mamusi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdf",
    individual_languages: &[
    ],
};


pub const KDG: LanguageCode = LanguageCode {
    name: "Seba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdg",
    individual_languages: &[
    ],
};


pub const KDH: LanguageCode = LanguageCode {
    name: "Tem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdh",
    individual_languages: &[
    ],
};


pub const KDI: LanguageCode = LanguageCode {
    name: "Kumam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdi",
    individual_languages: &[
    ],
};


pub const KDJ: LanguageCode = LanguageCode {
    name: "Karamojong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdj",
    individual_languages: &[
    ],
};


pub const KDK: LanguageCode = LanguageCode {
    name: "Numèè",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdk",
    individual_languages: &[
    ],
};


pub const KDL: LanguageCode = LanguageCode {
    name: "Tsikimba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdl",
    individual_languages: &[
    ],
};


pub const KDM: LanguageCode = LanguageCode {
    name: "Kagoma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdm",
    individual_languages: &[
    ],
};


pub const KDN: LanguageCode = LanguageCode {
    name: "Kunda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdn",
    individual_languages: &[
    ],
};


pub const KDP: LanguageCode = LanguageCode {
    name: "Kaningdon-Nindem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdp",
    individual_languages: &[
    ],
};


pub const KDQ: LanguageCode = LanguageCode {
    name: "Koch",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdq",
    individual_languages: &[
    ],
};


pub const KDR: LanguageCode = LanguageCode {
    name: "Karaim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdr",
    individual_languages: &[
    ],
};


pub const KDT: LanguageCode = LanguageCode {
    name: "Kuy",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdt",
    individual_languages: &[
    ],
};


pub const KDU: LanguageCode = LanguageCode {
    name: "Kadaru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdu",
    individual_languages: &[
    ],
};


pub const KDW: LanguageCode = LanguageCode {
    name: "Koneraw",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdw",
    individual_languages: &[
    ],
};


pub const KDX: LanguageCode = LanguageCode {
    name: "Kam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdx",
    individual_languages: &[
    ],
};


pub const KDY: LanguageCode = LanguageCode {
    name: "Keder",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdy",
    individual_languages: &[
    ],
};


pub const KDZ: LanguageCode = LanguageCode {
    name: "Kwaja",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kdz",
    individual_languages: &[
    ],
};


pub const KEA: LanguageCode = LanguageCode {
    name: "Kabuverdianu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kea",
    individual_languages: &[
    ],
};


pub const KEB: LanguageCode = LanguageCode {
    name: "Kélé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "keb",
    individual_languages: &[
    ],
};


pub const KEC: LanguageCode = LanguageCode {
    name: "Keiga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kec",
    individual_languages: &[
    ],
};


pub const KED: LanguageCode = LanguageCode {
    name: "Kerewe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ked",
    individual_languages: &[
    ],
};


pub const KEE: LanguageCode = LanguageCode {
    name: "Eastern Keres",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kee",
    individual_languages: &[
    ],
};


pub const KEF: LanguageCode = LanguageCode {
    name: "Kpessi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kef",
    individual_languages: &[
    ],
};


pub const KEG: LanguageCode = LanguageCode {
    name: "Tese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "keg",
    individual_languages: &[
    ],
};


pub const KEH: LanguageCode = LanguageCode {
    name: "Keak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "keh",
    individual_languages: &[
    ],
};


pub const KEI: LanguageCode = LanguageCode {
    name: "Kei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kei",
    individual_languages: &[
    ],
};


pub const KEJ: LanguageCode = LanguageCode {
    name: "Kadar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kej",
    individual_languages: &[
    ],
};


pub const KEK: LanguageCode = LanguageCode {
    name: "Kekchí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kek",
    individual_languages: &[
    ],
};


pub const KEL: LanguageCode = LanguageCode {
    name: "Kela (Democratic Republic of Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kel",
    individual_languages: &[
    ],
};


pub const KEM: LanguageCode = LanguageCode {
    name: "Kemak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kem",
    individual_languages: &[
    ],
};


pub const KEN: LanguageCode = LanguageCode {
    name: "Kenyang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ken",
    individual_languages: &[
    ],
};


pub const KEO: LanguageCode = LanguageCode {
    name: "Kakwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "keo",
    individual_languages: &[
    ],
};


pub const KEP: LanguageCode = LanguageCode {
    name: "Kaikadi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kep",
    individual_languages: &[
    ],
};


pub const KEQ: LanguageCode = LanguageCode {
    name: "Kamar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "keq",
    individual_languages: &[
    ],
};


pub const KER: LanguageCode = LanguageCode {
    name: "Kera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ker",
    individual_languages: &[
    ],
};


pub const KES: LanguageCode = LanguageCode {
    name: "Kugbo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kes",
    individual_languages: &[
    ],
};


pub const KET: LanguageCode = LanguageCode {
    name: "Ket",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ket",
    individual_languages: &[
    ],
};


pub const KEU: LanguageCode = LanguageCode {
    name: "Akebu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "keu",
    individual_languages: &[
    ],
};


pub const KEV: LanguageCode = LanguageCode {
    name: "Kanikkaran",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kev",
    individual_languages: &[
    ],
};


pub const KEW: LanguageCode = LanguageCode {
    name: "West Kewa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kew",
    individual_languages: &[
    ],
};


pub const KEX: LanguageCode = LanguageCode {
    name: "Kukna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kex",
    individual_languages: &[
    ],
};


pub const KEY: LanguageCode = LanguageCode {
    name: "Kupia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "key",
    individual_languages: &[
    ],
};


pub const KEZ: LanguageCode = LanguageCode {
    name: "Kukele",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kez",
    individual_languages: &[
    ],
};


pub const KFA: LanguageCode = LanguageCode {
    name: "Kodava",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfa",
    individual_languages: &[
    ],
};


pub const KFB: LanguageCode = LanguageCode {
    name: "Northwestern Kolami",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfb",
    individual_languages: &[
    ],
};


pub const KFC: LanguageCode = LanguageCode {
    name: "Konda-Dora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfc",
    individual_languages: &[
    ],
};


pub const KFD: LanguageCode = LanguageCode {
    name: "Korra Koraga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfd",
    individual_languages: &[
    ],
};


pub const KFE: LanguageCode = LanguageCode {
    name: "Kota (India)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfe",
    individual_languages: &[
    ],
};


pub const KFF: LanguageCode = LanguageCode {
    name: "Koya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kff",
    individual_languages: &[
    ],
};


pub const KFG: LanguageCode = LanguageCode {
    name: "Kudiya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfg",
    individual_languages: &[
    ],
};


pub const KFH: LanguageCode = LanguageCode {
    name: "Kurichiya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfh",
    individual_languages: &[
    ],
};


pub const KFI: LanguageCode = LanguageCode {
    name: "Kannada Kurumba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfi",
    individual_languages: &[
    ],
};


pub const KFJ: LanguageCode = LanguageCode {
    name: "Kemiehua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfj",
    individual_languages: &[
    ],
};


pub const KFK: LanguageCode = LanguageCode {
    name: "Kinnauri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfk",
    individual_languages: &[
    ],
};


pub const KFL: LanguageCode = LanguageCode {
    name: "Kung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfl",
    individual_languages: &[
    ],
};


pub const KFM: LanguageCode = LanguageCode {
    name: "Khunsari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfm",
    individual_languages: &[
    ],
};


pub const KFN: LanguageCode = LanguageCode {
    name: "Kuk",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfn",
    individual_languages: &[
    ],
};


pub const KFO: LanguageCode = LanguageCode {
    name: "Koro (Côte d'Ivoire)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfo",
    individual_languages: &[
    ],
};


pub const KFP: LanguageCode = LanguageCode {
    name: "Korwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfp",
    individual_languages: &[
    ],
};


pub const KFQ: LanguageCode = LanguageCode {
    name: "Korku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfq",
    individual_languages: &[
    ],
};


pub const KFR: LanguageCode = LanguageCode {
    name: "Kachhi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfr",
    individual_languages: &[
    ],
};


pub const KFS: LanguageCode = LanguageCode {
    name: "Bilaspuri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfs",
    individual_languages: &[
    ],
};


pub const KFT: LanguageCode = LanguageCode {
    name: "Kanjari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kft",
    individual_languages: &[
    ],
};


pub const KFU: LanguageCode = LanguageCode {
    name: "Katkari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfu",
    individual_languages: &[
    ],
};


pub const KFV: LanguageCode = LanguageCode {
    name: "Kurmukar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfv",
    individual_languages: &[
    ],
};


pub const KFW: LanguageCode = LanguageCode {
    name: "Kharam Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfw",
    individual_languages: &[
    ],
};


pub const KFX: LanguageCode = LanguageCode {
    name: "Kullu Pahari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfx",
    individual_languages: &[
    ],
};


pub const KFY: LanguageCode = LanguageCode {
    name: "Kumaoni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfy",
    individual_languages: &[
    ],
};


pub const KFZ: LanguageCode = LanguageCode {
    name: "Koromfé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kfz",
    individual_languages: &[
    ],
};


pub const KGA: LanguageCode = LanguageCode {
    name: "Koyaga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kga",
    individual_languages: &[
    ],
};


pub const KGB: LanguageCode = LanguageCode {
    name: "Kawe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgb",
    individual_languages: &[
    ],
};


pub const KGE: LanguageCode = LanguageCode {
    name: "Komering",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kge",
    individual_languages: &[
    ],
};


pub const KGF: LanguageCode = LanguageCode {
    name: "Kube",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgf",
    individual_languages: &[
    ],
};


pub const KGG: LanguageCode = LanguageCode {
    name: "Kusunda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgg",
    individual_languages: &[
    ],
};


pub const KGI: LanguageCode = LanguageCode {
    name: "Selangor Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgi",
    individual_languages: &[
    ],
};


pub const KGJ: LanguageCode = LanguageCode {
    name: "Gamale Kham",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgj",
    individual_languages: &[
    ],
};


pub const KGK: LanguageCode = LanguageCode {
    name: "Kaiwá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgk",
    individual_languages: &[
    ],
};


pub const KGL: LanguageCode = LanguageCode {
    name: "Kunggari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgl",
    individual_languages: &[
    ],
};


pub const KGM: LanguageCode = LanguageCode {
    name: "Karipúna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgm",
    individual_languages: &[
    ],
};


pub const KGN: LanguageCode = LanguageCode {
    name: "Karingani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgn",
    individual_languages: &[
    ],
};


pub const KGO: LanguageCode = LanguageCode {
    name: "Krongo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgo",
    individual_languages: &[
    ],
};


pub const KGP: LanguageCode = LanguageCode {
    name: "Kaingang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgp",
    individual_languages: &[
    ],
};


pub const KGQ: LanguageCode = LanguageCode {
    name: "Kamoro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgq",
    individual_languages: &[
    ],
};


pub const KGR: LanguageCode = LanguageCode {
    name: "Abun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgr",
    individual_languages: &[
    ],
};


pub const KGS: LanguageCode = LanguageCode {
    name: "Kumbainggar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgs",
    individual_languages: &[
    ],
};


pub const KGT: LanguageCode = LanguageCode {
    name: "Somyev",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgt",
    individual_languages: &[
    ],
};


pub const KGU: LanguageCode = LanguageCode {
    name: "Kobol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgu",
    individual_languages: &[
    ],
};


pub const KGV: LanguageCode = LanguageCode {
    name: "Karas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgv",
    individual_languages: &[
    ],
};


pub const KGW: LanguageCode = LanguageCode {
    name: "Karon Dori",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgw",
    individual_languages: &[
    ],
};


pub const KGX: LanguageCode = LanguageCode {
    name: "Kamaru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgx",
    individual_languages: &[
    ],
};


pub const KGY: LanguageCode = LanguageCode {
    name: "Kyerung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kgy",
    individual_languages: &[
    ],
};


pub const KHB: LanguageCode = LanguageCode {
    name: "Lü",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khb",
    individual_languages: &[
    ],
};


pub const KHC: LanguageCode = LanguageCode {
    name: "Tukang Besi North",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khc",
    individual_languages: &[
    ],
};


pub const KHD: LanguageCode = LanguageCode {
    name: "Bädi Kanum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khd",
    individual_languages: &[
    ],
};


pub const KHE: LanguageCode = LanguageCode {
    name: "Korowai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khe",
    individual_languages: &[
    ],
};


pub const KHF: LanguageCode = LanguageCode {
    name: "Khuen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khf",
    individual_languages: &[
    ],
};


pub const KHG: LanguageCode = LanguageCode {
    name: "Khams Tibetan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khg",
    individual_languages: &[
    ],
};


pub const KHH: LanguageCode = LanguageCode {
    name: "Kehu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khh",
    individual_languages: &[
    ],
};


pub const KHJ: LanguageCode = LanguageCode {
    name: "Kuturmi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khj",
    individual_languages: &[
    ],
};


pub const KHK: LanguageCode = LanguageCode {
    name: "Halh Mongolian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khk",
    individual_languages: &[
    ],
};


pub const KHL: LanguageCode = LanguageCode {
    name: "Lusi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khl",
    individual_languages: &[
    ],
};


pub const KHN: LanguageCode = LanguageCode {
    name: "Khandesi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khn",
    individual_languages: &[
    ],
};


pub const KHP: LanguageCode = LanguageCode {
    name: "Kapori",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khp",
    individual_languages: &[
    ],
};


pub const KHQ: LanguageCode = LanguageCode {
    name: "Koyra Chiini Songhay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khq",
    individual_languages: &[
    ],
};


pub const KHR: LanguageCode = LanguageCode {
    name: "Kharia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khr",
    individual_languages: &[
    ],
};


pub const KHS: LanguageCode = LanguageCode {
    name: "Kasua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khs",
    individual_languages: &[
    ],
};


pub const KHT: LanguageCode = LanguageCode {
    name: "Khamti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kht",
    individual_languages: &[
    ],
};


pub const KHU: LanguageCode = LanguageCode {
    name: "Nkhumbi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khu",
    individual_languages: &[
    ],
};


pub const KHV: LanguageCode = LanguageCode {
    name: "Khvarshi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khv",
    individual_languages: &[
    ],
};


pub const KHW: LanguageCode = LanguageCode {
    name: "Khowar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khw",
    individual_languages: &[
    ],
};


pub const KHX: LanguageCode = LanguageCode {
    name: "Kanu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khx",
    individual_languages: &[
    ],
};


pub const KHY: LanguageCode = LanguageCode {
    name: "Kele (Democratic Republic of Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khy",
    individual_languages: &[
    ],
};


pub const KHZ: LanguageCode = LanguageCode {
    name: "Keapara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "khz",
    individual_languages: &[
    ],
};


pub const KIA: LanguageCode = LanguageCode {
    name: "Kim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kia",
    individual_languages: &[
    ],
};


pub const KIB: LanguageCode = LanguageCode {
    name: "Koalib",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kib",
    individual_languages: &[
    ],
};


pub const KIC: LanguageCode = LanguageCode {
    name: "Kickapoo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kic",
    individual_languages: &[
    ],
};


pub const KID: LanguageCode = LanguageCode {
    name: "Koshin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kid",
    individual_languages: &[
    ],
};


pub const KIE: LanguageCode = LanguageCode {
    name: "Kibet",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kie",
    individual_languages: &[
    ],
};


pub const KIF: LanguageCode = LanguageCode {
    name: "Eastern Parbate Kham",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kif",
    individual_languages: &[
    ],
};


pub const KIG: LanguageCode = LanguageCode {
    name: "Kimaama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kig",
    individual_languages: &[
    ],
};


pub const KIH: LanguageCode = LanguageCode {
    name: "Kilmeri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kih",
    individual_languages: &[
    ],
};


pub const KII: LanguageCode = LanguageCode {
    name: "Kitsai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kii",
    individual_languages: &[
    ],
};


pub const KIJ: LanguageCode = LanguageCode {
    name: "Kilivila",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kij",
    individual_languages: &[
    ],
};


pub const KIL: LanguageCode = LanguageCode {
    name: "Kariya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kil",
    individual_languages: &[
    ],
};


pub const KIM: LanguageCode = LanguageCode {
    name: "Karagas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kim",
    individual_languages: &[
    ],
};


pub const KIO: LanguageCode = LanguageCode {
    name: "Kiowa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kio",
    individual_languages: &[
    ],
};


pub const KIP: LanguageCode = LanguageCode {
    name: "Sheshi Kham",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kip",
    individual_languages: &[
    ],
};


pub const KIQ: LanguageCode = LanguageCode {
    name: "Kosadle",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kiq",
    individual_languages: &[
    ],
};


pub const KIS: LanguageCode = LanguageCode {
    name: "Kis",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kis",
    individual_languages: &[
    ],
};


pub const KIT: LanguageCode = LanguageCode {
    name: "Agob",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kit",
    individual_languages: &[
    ],
};


pub const KIU: LanguageCode = LanguageCode {
    name: "Kirmanjki (individual language)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kiu",
    individual_languages: &[
    ],
};


pub const KIV: LanguageCode = LanguageCode {
    name: "Kimbu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kiv",
    individual_languages: &[
    ],
};


pub const KIW: LanguageCode = LanguageCode {
    name: "Northeast Kiwai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kiw",
    individual_languages: &[
    ],
};


pub const KIX: LanguageCode = LanguageCode {
    name: "Khiamniungan Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kix",
    individual_languages: &[
    ],
};


pub const KIY: LanguageCode = LanguageCode {
    name: "Kirikiri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kiy",
    individual_languages: &[
    ],
};


pub const KIZ: LanguageCode = LanguageCode {
    name: "Kisi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kiz",
    individual_languages: &[
    ],
};


pub const KJA: LanguageCode = LanguageCode {
    name: "Mlap",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kja",
    individual_languages: &[
    ],
};


pub const KJB: LanguageCode = LanguageCode {
    name: "Q'anjob'al",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjb",
    individual_languages: &[
    ],
};


pub const KJC: LanguageCode = LanguageCode {
    name: "Coastal Konjo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjc",
    individual_languages: &[
    ],
};


pub const KJD: LanguageCode = LanguageCode {
    name: "Southern Kiwai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjd",
    individual_languages: &[
    ],
};


pub const KJE: LanguageCode = LanguageCode {
    name: "Kisar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kje",
    individual_languages: &[
    ],
};


pub const KJG: LanguageCode = LanguageCode {
    name: "Khmu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjg",
    individual_languages: &[
    ],
};


pub const KJH: LanguageCode = LanguageCode {
    name: "Khakas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjh",
    individual_languages: &[
    ],
};


pub const KJI: LanguageCode = LanguageCode {
    name: "Zabana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kji",
    individual_languages: &[
    ],
};


pub const KJJ: LanguageCode = LanguageCode {
    name: "Khinalugh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjj",
    individual_languages: &[
    ],
};


pub const KJK: LanguageCode = LanguageCode {
    name: "Highland Konjo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjk",
    individual_languages: &[
    ],
};


pub const KJL: LanguageCode = LanguageCode {
    name: "Western Parbate Kham",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjl",
    individual_languages: &[
    ],
};


pub const KJM: LanguageCode = LanguageCode {
    name: "Kháng",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjm",
    individual_languages: &[
    ],
};


pub const KJN: LanguageCode = LanguageCode {
    name: "Kunjen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjn",
    individual_languages: &[
    ],
};


pub const KJO: LanguageCode = LanguageCode {
    name: "Harijan Kinnauri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjo",
    individual_languages: &[
    ],
};


pub const KJP: LanguageCode = LanguageCode {
    name: "Pwo Eastern Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjp",
    individual_languages: &[
    ],
};


pub const KJQ: LanguageCode = LanguageCode {
    name: "Western Keres",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjq",
    individual_languages: &[
    ],
};


pub const KJR: LanguageCode = LanguageCode {
    name: "Kurudu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjr",
    individual_languages: &[
    ],
};


pub const KJS: LanguageCode = LanguageCode {
    name: "East Kewa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjs",
    individual_languages: &[
    ],
};


pub const KJT: LanguageCode = LanguageCode {
    name: "Phrae Pwo Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjt",
    individual_languages: &[
    ],
};


pub const KJU: LanguageCode = LanguageCode {
    name: "Kashaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kju",
    individual_languages: &[
    ],
};


pub const KJV: LanguageCode = LanguageCode {
    name: "Kaikavian Literary Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjv",
    individual_languages: &[
    ],
};


pub const KJX: LanguageCode = LanguageCode {
    name: "Ramopa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjx",
    individual_languages: &[
    ],
};


pub const KJY: LanguageCode = LanguageCode {
    name: "Erave",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjy",
    individual_languages: &[
    ],
};


pub const KJZ: LanguageCode = LanguageCode {
    name: "Bumthangkha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kjz",
    individual_languages: &[
    ],
};


pub const KKA: LanguageCode = LanguageCode {
    name: "Kakanda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kka",
    individual_languages: &[
    ],
};


pub const KKB: LanguageCode = LanguageCode {
    name: "Kwerisa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkb",
    individual_languages: &[
    ],
};


pub const KKC: LanguageCode = LanguageCode {
    name: "Odoodee",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkc",
    individual_languages: &[
    ],
};


pub const KKD: LanguageCode = LanguageCode {
    name: "Kinuku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkd",
    individual_languages: &[
    ],
};


pub const KKE: LanguageCode = LanguageCode {
    name: "Kakabe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kke",
    individual_languages: &[
    ],
};


pub const KKF: LanguageCode = LanguageCode {
    name: "Kalaktang Monpa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkf",
    individual_languages: &[
    ],
};


pub const KKG: LanguageCode = LanguageCode {
    name: "Mabaka Valley Kalinga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkg",
    individual_languages: &[
    ],
};


pub const KKH: LanguageCode = LanguageCode {
    name: "Khün",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkh",
    individual_languages: &[
    ],
};


pub const KKI: LanguageCode = LanguageCode {
    name: "Kagulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kki",
    individual_languages: &[
    ],
};


pub const KKJ: LanguageCode = LanguageCode {
    name: "Kako",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkj",
    individual_languages: &[
    ],
};


pub const KKK: LanguageCode = LanguageCode {
    name: "Kokota",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkk",
    individual_languages: &[
    ],
};


pub const KKL: LanguageCode = LanguageCode {
    name: "Kosarek Yale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkl",
    individual_languages: &[
    ],
};


pub const KKM: LanguageCode = LanguageCode {
    name: "Kiong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkm",
    individual_languages: &[
    ],
};


pub const KKN: LanguageCode = LanguageCode {
    name: "Kon Keu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkn",
    individual_languages: &[
    ],
};


pub const KKO: LanguageCode = LanguageCode {
    name: "Karko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kko",
    individual_languages: &[
    ],
};


pub const KKP: LanguageCode = LanguageCode {
    name: "Gugubera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkp",
    individual_languages: &[
    ],
};


pub const KKQ: LanguageCode = LanguageCode {
    name: "Kaeku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkq",
    individual_languages: &[
    ],
};


pub const KKR: LanguageCode = LanguageCode {
    name: "Kir-Balar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkr",
    individual_languages: &[
    ],
};


pub const KKS: LanguageCode = LanguageCode {
    name: "Giiwo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kks",
    individual_languages: &[
    ],
};


pub const KKT: LanguageCode = LanguageCode {
    name: "Koi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkt",
    individual_languages: &[
    ],
};


pub const KKU: LanguageCode = LanguageCode {
    name: "Tumi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kku",
    individual_languages: &[
    ],
};


pub const KKV: LanguageCode = LanguageCode {
    name: "Kangean",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkv",
    individual_languages: &[
    ],
};


pub const KKW: LanguageCode = LanguageCode {
    name: "Teke-Kukuya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkw",
    individual_languages: &[
    ],
};


pub const KKX: LanguageCode = LanguageCode {
    name: "Kohin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkx",
    individual_languages: &[
    ],
};


pub const KKY: LanguageCode = LanguageCode {
    name: "Guugu Yimidhirr",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kky",
    individual_languages: &[
    ],
};


pub const KKZ: LanguageCode = LanguageCode {
    name: "Kaska",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kkz",
    individual_languages: &[
    ],
};


pub const KLA: LanguageCode = LanguageCode {
    name: "Klamath-Modoc",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kla",
    individual_languages: &[
    ],
};


pub const KLB: LanguageCode = LanguageCode {
    name: "Kiliwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klb",
    individual_languages: &[
    ],
};


pub const KLC: LanguageCode = LanguageCode {
    name: "Kolbila",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klc",
    individual_languages: &[
    ],
};


pub const KLD: LanguageCode = LanguageCode {
    name: "Gamilaraay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kld",
    individual_languages: &[
    ],
};


pub const KLE: LanguageCode = LanguageCode {
    name: "Kulung (Nepal)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kle",
    individual_languages: &[
    ],
};


pub const KLF: LanguageCode = LanguageCode {
    name: "Kendeje",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klf",
    individual_languages: &[
    ],
};


pub const KLG: LanguageCode = LanguageCode {
    name: "Tagakaulo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klg",
    individual_languages: &[
    ],
};


pub const KLH: LanguageCode = LanguageCode {
    name: "Weliki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klh",
    individual_languages: &[
    ],
};


pub const KLI: LanguageCode = LanguageCode {
    name: "Kalumpang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kli",
    individual_languages: &[
    ],
};


pub const KLJ: LanguageCode = LanguageCode {
    name: "Khalaj",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klj",
    individual_languages: &[
    ],
};


pub const KLK: LanguageCode = LanguageCode {
    name: "Kono (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klk",
    individual_languages: &[
    ],
};


pub const KLL: LanguageCode = LanguageCode {
    name: "Kagan Kalagan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kll",
    individual_languages: &[
    ],
};


pub const KLM: LanguageCode = LanguageCode {
    name: "Migum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klm",
    individual_languages: &[
    ],
};


pub const KLN: LanguageCode = LanguageCode {
    name: "Kalenjin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kln",
    individual_languages: &[
        IndividualLanguages {
            name: "Markweeta",
            code: "enb",
        },
        IndividualLanguages {
            name: "Keiyo",
            code: "eyo",
        },
        IndividualLanguages {
            name: "Nandi",
            code: "niq",
        },
        IndividualLanguages {
            name: "Okiek",
            code: "oki",
        },
        IndividualLanguages {
            name: "Pökoot",
            code: "pko",
        },
        IndividualLanguages {
            name: "Kipsigis",
            code: "sgc",
        },
        IndividualLanguages {
            name: "Sabaot",
            code: "spy",
        },
        IndividualLanguages {
            name: "Terik",
            code: "tec",
        },
        IndividualLanguages {
            name: "Tugen",
            code: "tuy",
        },
    ],
};


pub const KLO: LanguageCode = LanguageCode {
    name: "Kapya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klo",
    individual_languages: &[
    ],
};


pub const KLP: LanguageCode = LanguageCode {
    name: "Kamasa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klp",
    individual_languages: &[
    ],
};


pub const KLQ: LanguageCode = LanguageCode {
    name: "Rumu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klq",
    individual_languages: &[
    ],
};


pub const KLR: LanguageCode = LanguageCode {
    name: "Khaling",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klr",
    individual_languages: &[
    ],
};


pub const KLS: LanguageCode = LanguageCode {
    name: "Kalasha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kls",
    individual_languages: &[
    ],
};


pub const KLT: LanguageCode = LanguageCode {
    name: "Nukna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klt",
    individual_languages: &[
    ],
};


pub const KLU: LanguageCode = LanguageCode {
    name: "Klao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klu",
    individual_languages: &[
    ],
};


pub const KLV: LanguageCode = LanguageCode {
    name: "Maskelynes",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klv",
    individual_languages: &[
    ],
};


pub const KLW: LanguageCode = LanguageCode {
    name: "Tado",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klw",
    individual_languages: &[
    ],
};


pub const KLX: LanguageCode = LanguageCode {
    name: "Koluwawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klx",
    individual_languages: &[
    ],
};


pub const KLY: LanguageCode = LanguageCode {
    name: "Kalao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kly",
    individual_languages: &[
    ],
};


pub const KLZ: LanguageCode = LanguageCode {
    name: "Kabola",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "klz",
    individual_languages: &[
    ],
};


pub const KMA: LanguageCode = LanguageCode {
    name: "Konni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kma",
    individual_languages: &[
    ],
};


pub const KMC: LanguageCode = LanguageCode {
    name: "Southern Dong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmc",
    individual_languages: &[
    ],
};


pub const KMD: LanguageCode = LanguageCode {
    name: "Majukayang Kalinga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmd",
    individual_languages: &[
    ],
};


pub const KME: LanguageCode = LanguageCode {
    name: "Bakole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kme",
    individual_languages: &[
    ],
};


pub const KMF: LanguageCode = LanguageCode {
    name: "Kare (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmf",
    individual_languages: &[
    ],
};


pub const KMG: LanguageCode = LanguageCode {
    name: "Kâte",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmg",
    individual_languages: &[
    ],
};


pub const KMH: LanguageCode = LanguageCode {
    name: "Kalam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmh",
    individual_languages: &[
    ],
};


pub const KMI: LanguageCode = LanguageCode {
    name: "Kami (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmi",
    individual_languages: &[
    ],
};


pub const KMJ: LanguageCode = LanguageCode {
    name: "Kumarbhag Paharia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmj",
    individual_languages: &[
    ],
};


pub const KMK: LanguageCode = LanguageCode {
    name: "Limos Kalinga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmk",
    individual_languages: &[
    ],
};


pub const KML: LanguageCode = LanguageCode {
    name: "Tanudan Kalinga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kml",
    individual_languages: &[
    ],
};


pub const KMM: LanguageCode = LanguageCode {
    name: "Kom (India)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmm",
    individual_languages: &[
    ],
};


pub const KMN: LanguageCode = LanguageCode {
    name: "Awtuw",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmn",
    individual_languages: &[
    ],
};


pub const KMO: LanguageCode = LanguageCode {
    name: "Kwoma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmo",
    individual_languages: &[
    ],
};


pub const KMP: LanguageCode = LanguageCode {
    name: "Gimme",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmp",
    individual_languages: &[
    ],
};


pub const KMQ: LanguageCode = LanguageCode {
    name: "Kwama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmq",
    individual_languages: &[
    ],
};


pub const KMR: LanguageCode = LanguageCode {
    name: "Northern Kurdish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmr",
    individual_languages: &[
    ],
};


pub const KMS: LanguageCode = LanguageCode {
    name: "Kamasau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kms",
    individual_languages: &[
    ],
};


pub const KMT: LanguageCode = LanguageCode {
    name: "Kemtuik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmt",
    individual_languages: &[
    ],
};


pub const KMU: LanguageCode = LanguageCode {
    name: "Kanite",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmu",
    individual_languages: &[
    ],
};


pub const KMV: LanguageCode = LanguageCode {
    name: "Karipúna Creole French",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmv",
    individual_languages: &[
    ],
};


pub const KMW: LanguageCode = LanguageCode {
    name: "Komo (Democratic Republic of Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmw",
    individual_languages: &[
    ],
};


pub const KMX: LanguageCode = LanguageCode {
    name: "Waboda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmx",
    individual_languages: &[
    ],
};


pub const KMY: LanguageCode = LanguageCode {
    name: "Koma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmy",
    individual_languages: &[
    ],
};


pub const KMZ: LanguageCode = LanguageCode {
    name: "Khorasani Turkish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kmz",
    individual_languages: &[
    ],
};


pub const KNA: LanguageCode = LanguageCode {
    name: "Dera (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kna",
    individual_languages: &[
    ],
};


pub const KNB: LanguageCode = LanguageCode {
    name: "Lubuagan Kalinga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knb",
    individual_languages: &[
    ],
};


pub const KNC: LanguageCode = LanguageCode {
    name: "Central Kanuri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knc",
    individual_languages: &[
    ],
};


pub const KND: LanguageCode = LanguageCode {
    name: "Konda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knd",
    individual_languages: &[
    ],
};


pub const KNE: LanguageCode = LanguageCode {
    name: "Kankanaey",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kne",
    individual_languages: &[
    ],
};


pub const KNF: LanguageCode = LanguageCode {
    name: "Mankanya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knf",
    individual_languages: &[
    ],
};


pub const KNG: LanguageCode = LanguageCode {
    name: "Koongo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kng",
    individual_languages: &[
    ],
};


pub const KNI: LanguageCode = LanguageCode {
    name: "Kanufi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kni",
    individual_languages: &[
    ],
};


pub const KNJ: LanguageCode = LanguageCode {
    name: "Western Kanjobal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knj",
    individual_languages: &[
    ],
};


pub const KNK: LanguageCode = LanguageCode {
    name: "Kuranko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knk",
    individual_languages: &[
    ],
};


pub const KNL: LanguageCode = LanguageCode {
    name: "Keninjal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knl",
    individual_languages: &[
    ],
};


pub const KNM: LanguageCode = LanguageCode {
    name: "Kanamarí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knm",
    individual_languages: &[
    ],
};


pub const KNN: LanguageCode = LanguageCode {
    name: "Konkani (individual language)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knn",
    individual_languages: &[
    ],
};


pub const KNO: LanguageCode = LanguageCode {
    name: "Kono (Sierra Leone)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kno",
    individual_languages: &[
    ],
};


pub const KNP: LanguageCode = LanguageCode {
    name: "Kwanja",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knp",
    individual_languages: &[
    ],
};


pub const KNQ: LanguageCode = LanguageCode {
    name: "Kintaq",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knq",
    individual_languages: &[
    ],
};


pub const KNR: LanguageCode = LanguageCode {
    name: "Kaningra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knr",
    individual_languages: &[
    ],
};


pub const KNS: LanguageCode = LanguageCode {
    name: "Kensiu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kns",
    individual_languages: &[
    ],
};


pub const KNT: LanguageCode = LanguageCode {
    name: "Panoan Katukína",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knt",
    individual_languages: &[
    ],
};


pub const KNU: LanguageCode = LanguageCode {
    name: "Kono (Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knu",
    individual_languages: &[
    ],
};


pub const KNV: LanguageCode = LanguageCode {
    name: "Tabo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knv",
    individual_languages: &[
    ],
};


pub const KNW: LanguageCode = LanguageCode {
    name: "Kung-Ekoka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knw",
    individual_languages: &[
    ],
};


pub const KNX: LanguageCode = LanguageCode {
    name: "Kendayan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knx",
    individual_languages: &[
    ],
};


pub const KNY: LanguageCode = LanguageCode {
    name: "Kanyok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kny",
    individual_languages: &[
    ],
};


pub const KNZ: LanguageCode = LanguageCode {
    name: "Kalamsé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "knz",
    individual_languages: &[
    ],
};


pub const KOA: LanguageCode = LanguageCode {
    name: "Konomala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "koa",
    individual_languages: &[
    ],
};


pub const KOC: LanguageCode = LanguageCode {
    name: "Kpati",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "koc",
    individual_languages: &[
    ],
};


pub const KOD: LanguageCode = LanguageCode {
    name: "Kodi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kod",
    individual_languages: &[
    ],
};


pub const KOE: LanguageCode = LanguageCode {
    name: "Kacipo-Bale Suri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "koe",
    individual_languages: &[
    ],
};


pub const KOF: LanguageCode = LanguageCode {
    name: "Kubi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kof",
    individual_languages: &[
    ],
};


pub const KOG: LanguageCode = LanguageCode {
    name: "Cogui",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kog",
    individual_languages: &[
    ],
};


pub const KOH: LanguageCode = LanguageCode {
    name: "Koyo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "koh",
    individual_languages: &[
    ],
};


pub const KOI: LanguageCode = LanguageCode {
    name: "Komi-Permyak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "koi",
    individual_languages: &[
    ],
};


pub const KOL: LanguageCode = LanguageCode {
    name: "Kol (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kol",
    individual_languages: &[
    ],
};


pub const KOO: LanguageCode = LanguageCode {
    name: "Konzo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "koo",
    individual_languages: &[
    ],
};


pub const KOP: LanguageCode = LanguageCode {
    name: "Waube",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kop",
    individual_languages: &[
    ],
};


pub const KOQ: LanguageCode = LanguageCode {
    name: "Kota (Gabon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "koq",
    individual_languages: &[
    ],
};


pub const KOT: LanguageCode = LanguageCode {
    name: "Lagwan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kot",
    individual_languages: &[
    ],
};


pub const KOU: LanguageCode = LanguageCode {
    name: "Koke",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kou",
    individual_languages: &[
    ],
};


pub const KOV: LanguageCode = LanguageCode {
    name: "Kudu-Camo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kov",
    individual_languages: &[
    ],
};


pub const KOW: LanguageCode = LanguageCode {
    name: "Kugama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kow",
    individual_languages: &[
    ],
};


pub const KOY: LanguageCode = LanguageCode {
    name: "Koyukon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "koy",
    individual_languages: &[
    ],
};


pub const KOZ: LanguageCode = LanguageCode {
    name: "Korak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "koz",
    individual_languages: &[
    ],
};


pub const KPA: LanguageCode = LanguageCode {
    name: "Kutto",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpa",
    individual_languages: &[
    ],
};


pub const KPB: LanguageCode = LanguageCode {
    name: "Mullu Kurumba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpb",
    individual_languages: &[
    ],
};


pub const KPC: LanguageCode = LanguageCode {
    name: "Curripaco",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpc",
    individual_languages: &[
    ],
};


pub const KPD: LanguageCode = LanguageCode {
    name: "Koba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpd",
    individual_languages: &[
    ],
};


pub const KPF: LanguageCode = LanguageCode {
    name: "Komba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpf",
    individual_languages: &[
    ],
};


pub const KPG: LanguageCode = LanguageCode {
    name: "Kapingamarangi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpg",
    individual_languages: &[
    ],
};


pub const KPH: LanguageCode = LanguageCode {
    name: "Kplang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kph",
    individual_languages: &[
    ],
};


pub const KPI: LanguageCode = LanguageCode {
    name: "Kofei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpi",
    individual_languages: &[
    ],
};


pub const KPJ: LanguageCode = LanguageCode {
    name: "Karajá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpj",
    individual_languages: &[
    ],
};


pub const KPK: LanguageCode = LanguageCode {
    name: "Kpan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpk",
    individual_languages: &[
    ],
};


pub const KPL: LanguageCode = LanguageCode {
    name: "Kpala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpl",
    individual_languages: &[
    ],
};


pub const KPM: LanguageCode = LanguageCode {
    name: "Koho",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpm",
    individual_languages: &[
    ],
};


pub const KPN: LanguageCode = LanguageCode {
    name: "Kepkiriwát",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpn",
    individual_languages: &[
    ],
};


pub const KPO: LanguageCode = LanguageCode {
    name: "Ikposo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpo",
    individual_languages: &[
    ],
};


pub const KPQ: LanguageCode = LanguageCode {
    name: "Korupun-Sela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpq",
    individual_languages: &[
    ],
};


pub const KPR: LanguageCode = LanguageCode {
    name: "Korafe-Yegha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpr",
    individual_languages: &[
    ],
};


pub const KPS: LanguageCode = LanguageCode {
    name: "Tehit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kps",
    individual_languages: &[
    ],
};


pub const KPT: LanguageCode = LanguageCode {
    name: "Karata",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpt",
    individual_languages: &[
    ],
};


pub const KPU: LanguageCode = LanguageCode {
    name: "Kafoa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpu",
    individual_languages: &[
    ],
};


pub const KPV: LanguageCode = LanguageCode {
    name: "Komi-Zyrian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpv",
    individual_languages: &[
    ],
};


pub const KPW: LanguageCode = LanguageCode {
    name: "Kobon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpw",
    individual_languages: &[
    ],
};


pub const KPX: LanguageCode = LanguageCode {
    name: "Mountain Koiali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpx",
    individual_languages: &[
    ],
};


pub const KPY: LanguageCode = LanguageCode {
    name: "Koryak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpy",
    individual_languages: &[
    ],
};


pub const KPZ: LanguageCode = LanguageCode {
    name: "Kupsabiny",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kpz",
    individual_languages: &[
    ],
};


pub const KQA: LanguageCode = LanguageCode {
    name: "Mum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqa",
    individual_languages: &[
    ],
};


pub const KQB: LanguageCode = LanguageCode {
    name: "Kovai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqb",
    individual_languages: &[
    ],
};


pub const KQC: LanguageCode = LanguageCode {
    name: "Doromu-Koki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqc",
    individual_languages: &[
    ],
};


pub const KQD: LanguageCode = LanguageCode {
    name: "Koy Sanjaq Surat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqd",
    individual_languages: &[
    ],
};


pub const KQE: LanguageCode = LanguageCode {
    name: "Kalagan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqe",
    individual_languages: &[
    ],
};


pub const KQF: LanguageCode = LanguageCode {
    name: "Kakabai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqf",
    individual_languages: &[
    ],
};


pub const KQG: LanguageCode = LanguageCode {
    name: "Khe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqg",
    individual_languages: &[
    ],
};


pub const KQH: LanguageCode = LanguageCode {
    name: "Kisankasa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqh",
    individual_languages: &[
    ],
};


pub const KQI: LanguageCode = LanguageCode {
    name: "Koitabu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqi",
    individual_languages: &[
    ],
};


pub const KQJ: LanguageCode = LanguageCode {
    name: "Koromira",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqj",
    individual_languages: &[
    ],
};


pub const KQK: LanguageCode = LanguageCode {
    name: "Kotafon Gbe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqk",
    individual_languages: &[
    ],
};


pub const KQL: LanguageCode = LanguageCode {
    name: "Kyenele",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kql",
    individual_languages: &[
    ],
};


pub const KQM: LanguageCode = LanguageCode {
    name: "Khisa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqm",
    individual_languages: &[
    ],
};


pub const KQN: LanguageCode = LanguageCode {
    name: "Kaonde",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqn",
    individual_languages: &[
    ],
};


pub const KQO: LanguageCode = LanguageCode {
    name: "Eastern Krahn",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqo",
    individual_languages: &[
    ],
};


pub const KQP: LanguageCode = LanguageCode {
    name: "Kimré",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqp",
    individual_languages: &[
    ],
};


pub const KQQ: LanguageCode = LanguageCode {
    name: "Krenak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqq",
    individual_languages: &[
    ],
};


pub const KQR: LanguageCode = LanguageCode {
    name: "Kimaragang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqr",
    individual_languages: &[
    ],
};


pub const KQS: LanguageCode = LanguageCode {
    name: "Northern Kissi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqs",
    individual_languages: &[
    ],
};


pub const KQT: LanguageCode = LanguageCode {
    name: "Klias River Kadazan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqt",
    individual_languages: &[
    ],
};


pub const KQU: LanguageCode = LanguageCode {
    name: "Seroa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqu",
    individual_languages: &[
    ],
};


pub const KQV: LanguageCode = LanguageCode {
    name: "Okolod",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqv",
    individual_languages: &[
    ],
};


pub const KQW: LanguageCode = LanguageCode {
    name: "Kandas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqw",
    individual_languages: &[
    ],
};


pub const KQX: LanguageCode = LanguageCode {
    name: "Mser",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqx",
    individual_languages: &[
    ],
};


pub const KQY: LanguageCode = LanguageCode {
    name: "Koorete",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqy",
    individual_languages: &[
    ],
};


pub const KQZ: LanguageCode = LanguageCode {
    name: "Korana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kqz",
    individual_languages: &[
    ],
};


pub const KRA: LanguageCode = LanguageCode {
    name: "Kumhali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kra",
    individual_languages: &[
    ],
};


pub const KRB: LanguageCode = LanguageCode {
    name: "Karkin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krb",
    individual_languages: &[
    ],
};


pub const KRD: LanguageCode = LanguageCode {
    name: "Kairui-Midiki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krd",
    individual_languages: &[
    ],
};


pub const KRE: LanguageCode = LanguageCode {
    name: "Panará",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kre",
    individual_languages: &[
    ],
};


pub const KRF: LanguageCode = LanguageCode {
    name: "Koro (Vanuatu)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krf",
    individual_languages: &[
    ],
};


pub const KRH: LanguageCode = LanguageCode {
    name: "Kurama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krh",
    individual_languages: &[
    ],
};


pub const KRI: LanguageCode = LanguageCode {
    name: "Krio",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kri",
    individual_languages: &[
    ],
};


pub const KRJ: LanguageCode = LanguageCode {
    name: "Kinaray-A",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krj",
    individual_languages: &[
    ],
};


pub const KRK: LanguageCode = LanguageCode {
    name: "Kerek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krk",
    individual_languages: &[
    ],
};


pub const KRN: LanguageCode = LanguageCode {
    name: "Sapo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krn",
    individual_languages: &[
    ],
};


pub const KRP: LanguageCode = LanguageCode {
    name: "Korop",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krp",
    individual_languages: &[
    ],
};


pub const KRR: LanguageCode = LanguageCode {
    name: "Krung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krr",
    individual_languages: &[
    ],
};


pub const KRS: LanguageCode = LanguageCode {
    name: "Gbaya (Sudan)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krs",
    individual_languages: &[
    ],
};


pub const KRT: LanguageCode = LanguageCode {
    name: "Tumari Kanuri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krt",
    individual_languages: &[
    ],
};


pub const KRV: LanguageCode = LanguageCode {
    name: "Kavet",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krv",
    individual_languages: &[
    ],
};


pub const KRW: LanguageCode = LanguageCode {
    name: "Western Krahn",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krw",
    individual_languages: &[
    ],
};


pub const KRX: LanguageCode = LanguageCode {
    name: "Karon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krx",
    individual_languages: &[
    ],
};


pub const KRY: LanguageCode = LanguageCode {
    name: "Kryts",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kry",
    individual_languages: &[
    ],
};


pub const KRZ: LanguageCode = LanguageCode {
    name: "Sota Kanum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "krz",
    individual_languages: &[
    ],
};


pub const KSA: LanguageCode = LanguageCode {
    name: "Shuwa-Zamani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksa",
    individual_languages: &[
    ],
};


pub const KSB: LanguageCode = LanguageCode {
    name: "Shambala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksb",
    individual_languages: &[
    ],
};


pub const KSC: LanguageCode = LanguageCode {
    name: "Southern Kalinga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksc",
    individual_languages: &[
    ],
};


pub const KSD: LanguageCode = LanguageCode {
    name: "Kuanua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksd",
    individual_languages: &[
    ],
};


pub const KSE: LanguageCode = LanguageCode {
    name: "Kuni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kse",
    individual_languages: &[
    ],
};


pub const KSF: LanguageCode = LanguageCode {
    name: "Bafia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksf",
    individual_languages: &[
    ],
};


pub const KSG: LanguageCode = LanguageCode {
    name: "Kusaghe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksg",
    individual_languages: &[
    ],
};


pub const KSH: LanguageCode = LanguageCode {
    name: "Kölsch",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksh",
    individual_languages: &[
    ],
};


pub const KSI: LanguageCode = LanguageCode {
    name: "Krisa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksi",
    individual_languages: &[
    ],
};


pub const KSJ: LanguageCode = LanguageCode {
    name: "Uare",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksj",
    individual_languages: &[
    ],
};


pub const KSK: LanguageCode = LanguageCode {
    name: "Kansa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksk",
    individual_languages: &[
    ],
};


pub const KSL: LanguageCode = LanguageCode {
    name: "Kumalu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksl",
    individual_languages: &[
    ],
};


pub const KSM: LanguageCode = LanguageCode {
    name: "Kumba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksm",
    individual_languages: &[
    ],
};


pub const KSN: LanguageCode = LanguageCode {
    name: "Kasiguranin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksn",
    individual_languages: &[
    ],
};


pub const KSO: LanguageCode = LanguageCode {
    name: "Kofa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kso",
    individual_languages: &[
    ],
};


pub const KSP: LanguageCode = LanguageCode {
    name: "Kaba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksp",
    individual_languages: &[
    ],
};


pub const KSQ: LanguageCode = LanguageCode {
    name: "Kwaami",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksq",
    individual_languages: &[
    ],
};


pub const KSR: LanguageCode = LanguageCode {
    name: "Borong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksr",
    individual_languages: &[
    ],
};


pub const KSS: LanguageCode = LanguageCode {
    name: "Southern Kisi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kss",
    individual_languages: &[
    ],
};


pub const KST: LanguageCode = LanguageCode {
    name: "Winyé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kst",
    individual_languages: &[
    ],
};


pub const KSU: LanguageCode = LanguageCode {
    name: "Khamyang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksu",
    individual_languages: &[
    ],
};


pub const KSV: LanguageCode = LanguageCode {
    name: "Kusu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksv",
    individual_languages: &[
    ],
};


pub const KSW: LanguageCode = LanguageCode {
    name: "S'gaw Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksw",
    individual_languages: &[
    ],
};


pub const KSX: LanguageCode = LanguageCode {
    name: "Kedang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksx",
    individual_languages: &[
    ],
};


pub const KSY: LanguageCode = LanguageCode {
    name: "Kharia Thar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksy",
    individual_languages: &[
    ],
};


pub const KSZ: LanguageCode = LanguageCode {
    name: "Kodaku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ksz",
    individual_languages: &[
    ],
};


pub const KTA: LanguageCode = LanguageCode {
    name: "Katua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kta",
    individual_languages: &[
    ],
};


pub const KTB: LanguageCode = LanguageCode {
    name: "Kambaata",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktb",
    individual_languages: &[
    ],
};


pub const KTC: LanguageCode = LanguageCode {
    name: "Kholok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktc",
    individual_languages: &[
    ],
};


pub const KTD: LanguageCode = LanguageCode {
    name: "Kokata",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktd",
    individual_languages: &[
    ],
};


pub const KTE: LanguageCode = LanguageCode {
    name: "Nubri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kte",
    individual_languages: &[
    ],
};


pub const KTF: LanguageCode = LanguageCode {
    name: "Kwami",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktf",
    individual_languages: &[
    ],
};


pub const KTG: LanguageCode = LanguageCode {
    name: "Kalkutung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktg",
    individual_languages: &[
    ],
};


pub const KTH: LanguageCode = LanguageCode {
    name: "Karanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kth",
    individual_languages: &[
    ],
};


pub const KTI: LanguageCode = LanguageCode {
    name: "North Muyu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kti",
    individual_languages: &[
    ],
};


pub const KTJ: LanguageCode = LanguageCode {
    name: "Plapo Krumen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktj",
    individual_languages: &[
    ],
};


pub const KTK: LanguageCode = LanguageCode {
    name: "Kaniet",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktk",
    individual_languages: &[
    ],
};


pub const KTL: LanguageCode = LanguageCode {
    name: "Koroshi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktl",
    individual_languages: &[
    ],
};


pub const KTM: LanguageCode = LanguageCode {
    name: "Kurti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktm",
    individual_languages: &[
    ],
};


pub const KTN: LanguageCode = LanguageCode {
    name: "Karitiâna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktn",
    individual_languages: &[
    ],
};


pub const KTO: LanguageCode = LanguageCode {
    name: "Kuot",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kto",
    individual_languages: &[
    ],
};


pub const KTP: LanguageCode = LanguageCode {
    name: "Kaduo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktp",
    individual_languages: &[
    ],
};


pub const KTQ: LanguageCode = LanguageCode {
    name: "Katabaga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktq",
    individual_languages: &[
    ],
};


pub const KTS: LanguageCode = LanguageCode {
    name: "South Muyu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kts",
    individual_languages: &[
    ],
};


pub const KTT: LanguageCode = LanguageCode {
    name: "Ketum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktt",
    individual_languages: &[
    ],
};


pub const KTU: LanguageCode = LanguageCode {
    name: "Kituba (Democratic Republic of Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktu",
    individual_languages: &[
    ],
};


pub const KTV: LanguageCode = LanguageCode {
    name: "Eastern Katu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktv",
    individual_languages: &[
    ],
};


pub const KTW: LanguageCode = LanguageCode {
    name: "Kato",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktw",
    individual_languages: &[
    ],
};


pub const KTX: LanguageCode = LanguageCode {
    name: "Kaxararí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktx",
    individual_languages: &[
    ],
};


pub const KTY: LanguageCode = LanguageCode {
    name: "Kango (Bas-Uélé District)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kty",
    individual_languages: &[
    ],
};


pub const KTZ: LanguageCode = LanguageCode {
    name: "Juǀʼhoan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ktz",
    individual_languages: &[
    ],
};


pub const KUB: LanguageCode = LanguageCode {
    name: "Kutep",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kub",
    individual_languages: &[
    ],
};


pub const KUC: LanguageCode = LanguageCode {
    name: "Kwinsu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kuc",
    individual_languages: &[
    ],
};


pub const KUD: LanguageCode = LanguageCode {
    name: "'Auhelawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kud",
    individual_languages: &[
    ],
};


pub const KUE: LanguageCode = LanguageCode {
    name: "Kuman (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kue",
    individual_languages: &[
    ],
};


pub const KUF: LanguageCode = LanguageCode {
    name: "Western Katu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kuf",
    individual_languages: &[
    ],
};


pub const KUG: LanguageCode = LanguageCode {
    name: "Kupa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kug",
    individual_languages: &[
    ],
};


pub const KUH: LanguageCode = LanguageCode {
    name: "Kushi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kuh",
    individual_languages: &[
    ],
};


pub const KUI: LanguageCode = LanguageCode {
    name: "Kuikúro-Kalapálo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kui",
    individual_languages: &[
    ],
};


pub const KUJ: LanguageCode = LanguageCode {
    name: "Kuria",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kuj",
    individual_languages: &[
    ],
};


pub const KUK: LanguageCode = LanguageCode {
    name: "Kepo'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kuk",
    individual_languages: &[
    ],
};


pub const KUL: LanguageCode = LanguageCode {
    name: "Kulere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kul",
    individual_languages: &[
    ],
};


pub const KUN: LanguageCode = LanguageCode {
    name: "Kunama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kun",
    individual_languages: &[
    ],
};


pub const KUO: LanguageCode = LanguageCode {
    name: "Kumukio",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kuo",
    individual_languages: &[
    ],
};


pub const KUP: LanguageCode = LanguageCode {
    name: "Kunimaipa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kup",
    individual_languages: &[
    ],
};


pub const KUQ: LanguageCode = LanguageCode {
    name: "Karipuna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kuq",
    individual_languages: &[
    ],
};


pub const KUS: LanguageCode = LanguageCode {
    name: "Kusaal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kus",
    individual_languages: &[
    ],
};


pub const KUU: LanguageCode = LanguageCode {
    name: "Upper Kuskokwim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kuu",
    individual_languages: &[
    ],
};


pub const KUV: LanguageCode = LanguageCode {
    name: "Kur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kuv",
    individual_languages: &[
    ],
};


pub const KUW: LanguageCode = LanguageCode {
    name: "Kpagua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kuw",
    individual_languages: &[
    ],
};


pub const KUX: LanguageCode = LanguageCode {
    name: "Kukatja",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kux",
    individual_languages: &[
    ],
};


pub const KUY: LanguageCode = LanguageCode {
    name: "Kuuku-Ya'u",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kuy",
    individual_languages: &[
    ],
};


pub const KUZ: LanguageCode = LanguageCode {
    name: "Kunza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kuz",
    individual_languages: &[
    ],
};


pub const KVA: LanguageCode = LanguageCode {
    name: "Bagvalal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kva",
    individual_languages: &[
    ],
};


pub const KVB: LanguageCode = LanguageCode {
    name: "Kubu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvb",
    individual_languages: &[
    ],
};


pub const KVC: LanguageCode = LanguageCode {
    name: "Kove",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvc",
    individual_languages: &[
    ],
};


pub const KVD: LanguageCode = LanguageCode {
    name: "Kui (Indonesia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvd",
    individual_languages: &[
    ],
};


pub const KVE: LanguageCode = LanguageCode {
    name: "Kalabakan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kve",
    individual_languages: &[
    ],
};


pub const KVF: LanguageCode = LanguageCode {
    name: "Kabalai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvf",
    individual_languages: &[
    ],
};


pub const KVG: LanguageCode = LanguageCode {
    name: "Kuni-Boazi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvg",
    individual_languages: &[
    ],
};


pub const KVH: LanguageCode = LanguageCode {
    name: "Komodo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvh",
    individual_languages: &[
    ],
};


pub const KVI: LanguageCode = LanguageCode {
    name: "Kwang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvi",
    individual_languages: &[
    ],
};


pub const KVJ: LanguageCode = LanguageCode {
    name: "Psikye",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvj",
    individual_languages: &[
    ],
};


pub const KVK: LanguageCode = LanguageCode {
    name: "Korean Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvk",
    individual_languages: &[
    ],
};


pub const KVL: LanguageCode = LanguageCode {
    name: "Kayaw",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvl",
    individual_languages: &[
    ],
};


pub const KVM: LanguageCode = LanguageCode {
    name: "Kendem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvm",
    individual_languages: &[
    ],
};


pub const KVN: LanguageCode = LanguageCode {
    name: "Border Kuna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvn",
    individual_languages: &[
    ],
};


pub const KVO: LanguageCode = LanguageCode {
    name: "Dobel",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvo",
    individual_languages: &[
    ],
};


pub const KVP: LanguageCode = LanguageCode {
    name: "Kompane",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvp",
    individual_languages: &[
    ],
};


pub const KVQ: LanguageCode = LanguageCode {
    name: "Geba Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvq",
    individual_languages: &[
    ],
};


pub const KVR: LanguageCode = LanguageCode {
    name: "Kerinci",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvr",
    individual_languages: &[
    ],
};


pub const KVT: LanguageCode = LanguageCode {
    name: "Lahta Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvt",
    individual_languages: &[
    ],
};


pub const KVU: LanguageCode = LanguageCode {
    name: "Yinbaw Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvu",
    individual_languages: &[
    ],
};


pub const KVV: LanguageCode = LanguageCode {
    name: "Kola",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvv",
    individual_languages: &[
    ],
};


pub const KVW: LanguageCode = LanguageCode {
    name: "Wersing",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvw",
    individual_languages: &[
    ],
};


pub const KVX: LanguageCode = LanguageCode {
    name: "Parkari Koli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvx",
    individual_languages: &[
    ],
};


pub const KVY: LanguageCode = LanguageCode {
    name: "Yintale Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvy",
    individual_languages: &[
    ],
};


pub const KVZ: LanguageCode = LanguageCode {
    name: "Tsakwambo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kvz",
    individual_languages: &[
    ],
};


pub const KWA: LanguageCode = LanguageCode {
    name: "Dâw",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwa",
    individual_languages: &[
    ],
};


pub const KWB: LanguageCode = LanguageCode {
    name: "Kwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwb",
    individual_languages: &[
    ],
};


pub const KWC: LanguageCode = LanguageCode {
    name: "Likwala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwc",
    individual_languages: &[
    ],
};


pub const KWD: LanguageCode = LanguageCode {
    name: "Kwaio",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwd",
    individual_languages: &[
    ],
};


pub const KWE: LanguageCode = LanguageCode {
    name: "Kwerba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwe",
    individual_languages: &[
    ],
};


pub const KWF: LanguageCode = LanguageCode {
    name: "Kwara'ae",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwf",
    individual_languages: &[
    ],
};


pub const KWG: LanguageCode = LanguageCode {
    name: "Sara Kaba Deme",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwg",
    individual_languages: &[
    ],
};


pub const KWH: LanguageCode = LanguageCode {
    name: "Kowiai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwh",
    individual_languages: &[
    ],
};


pub const KWI: LanguageCode = LanguageCode {
    name: "Awa-Cuaiquer",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwi",
    individual_languages: &[
    ],
};


pub const KWJ: LanguageCode = LanguageCode {
    name: "Kwanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwj",
    individual_languages: &[
    ],
};


pub const KWK: LanguageCode = LanguageCode {
    name: "Kwakiutl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwk",
    individual_languages: &[
    ],
};


pub const KWL: LanguageCode = LanguageCode {
    name: "Kofyar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwl",
    individual_languages: &[
    ],
};


pub const KWM: LanguageCode = LanguageCode {
    name: "Kwambi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwm",
    individual_languages: &[
    ],
};


pub const KWN: LanguageCode = LanguageCode {
    name: "Kwangali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwn",
    individual_languages: &[
    ],
};


pub const KWO: LanguageCode = LanguageCode {
    name: "Kwomtari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwo",
    individual_languages: &[
    ],
};


pub const KWP: LanguageCode = LanguageCode {
    name: "Kodia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwp",
    individual_languages: &[
    ],
};


pub const KWR: LanguageCode = LanguageCode {
    name: "Kwer",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwr",
    individual_languages: &[
    ],
};


pub const KWS: LanguageCode = LanguageCode {
    name: "Kwese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kws",
    individual_languages: &[
    ],
};


pub const KWT: LanguageCode = LanguageCode {
    name: "Kwesten",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwt",
    individual_languages: &[
    ],
};


pub const KWU: LanguageCode = LanguageCode {
    name: "Kwakum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwu",
    individual_languages: &[
    ],
};


pub const KWV: LanguageCode = LanguageCode {
    name: "Sara Kaba Náà",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwv",
    individual_languages: &[
    ],
};


pub const KWW: LanguageCode = LanguageCode {
    name: "Kwinti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kww",
    individual_languages: &[
    ],
};


pub const KWX: LanguageCode = LanguageCode {
    name: "Khirwar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwx",
    individual_languages: &[
    ],
};


pub const KWY: LanguageCode = LanguageCode {
    name: "San Salvador Kongo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwy",
    individual_languages: &[
    ],
};


pub const KWZ: LanguageCode = LanguageCode {
    name: "Kwadi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kwz",
    individual_languages: &[
    ],
};


pub const KXA: LanguageCode = LanguageCode {
    name: "Kairiru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxa",
    individual_languages: &[
    ],
};


pub const KXB: LanguageCode = LanguageCode {
    name: "Krobu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxb",
    individual_languages: &[
    ],
};


pub const KXC: LanguageCode = LanguageCode {
    name: "Konso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxc",
    individual_languages: &[
    ],
};


pub const KXD: LanguageCode = LanguageCode {
    name: "Brunei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxd",
    individual_languages: &[
    ],
};


pub const KXF: LanguageCode = LanguageCode {
    name: "Manumanaw Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxf",
    individual_languages: &[
    ],
};


pub const KXH: LanguageCode = LanguageCode {
    name: "Karo (Ethiopia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxh",
    individual_languages: &[
    ],
};


pub const KXI: LanguageCode = LanguageCode {
    name: "Keningau Murut",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxi",
    individual_languages: &[
    ],
};


pub const KXJ: LanguageCode = LanguageCode {
    name: "Kulfa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxj",
    individual_languages: &[
    ],
};


pub const KXK: LanguageCode = LanguageCode {
    name: "Zayein Karen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxk",
    individual_languages: &[
    ],
};


pub const KXM: LanguageCode = LanguageCode {
    name: "Northern Khmer",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxm",
    individual_languages: &[
    ],
};


pub const KXN: LanguageCode = LanguageCode {
    name: "Kanowit-Tanjong Melanau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxn",
    individual_languages: &[
    ],
};


pub const KXO: LanguageCode = LanguageCode {
    name: "Kanoé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxo",
    individual_languages: &[
    ],
};


pub const KXP: LanguageCode = LanguageCode {
    name: "Wadiyara Koli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxp",
    individual_languages: &[
    ],
};


pub const KXQ: LanguageCode = LanguageCode {
    name: "Smärky Kanum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxq",
    individual_languages: &[
    ],
};


pub const KXR: LanguageCode = LanguageCode {
    name: "Koro (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxr",
    individual_languages: &[
    ],
};


pub const KXS: LanguageCode = LanguageCode {
    name: "Kangjia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxs",
    individual_languages: &[
    ],
};


pub const KXT: LanguageCode = LanguageCode {
    name: "Koiwat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxt",
    individual_languages: &[
    ],
};


pub const KXV: LanguageCode = LanguageCode {
    name: "Kuvi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxv",
    individual_languages: &[
    ],
};


pub const KXW: LanguageCode = LanguageCode {
    name: "Konai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxw",
    individual_languages: &[
    ],
};


pub const KXX: LanguageCode = LanguageCode {
    name: "Likuba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxx",
    individual_languages: &[
    ],
};


pub const KXY: LanguageCode = LanguageCode {
    name: "Kayong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxy",
    individual_languages: &[
    ],
};


pub const KXZ: LanguageCode = LanguageCode {
    name: "Kerewo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kxz",
    individual_languages: &[
    ],
};


pub const KYA: LanguageCode = LanguageCode {
    name: "Kwaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kya",
    individual_languages: &[
    ],
};


pub const KYB: LanguageCode = LanguageCode {
    name: "Butbut Kalinga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyb",
    individual_languages: &[
    ],
};


pub const KYC: LanguageCode = LanguageCode {
    name: "Kyaka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyc",
    individual_languages: &[
    ],
};


pub const KYD: LanguageCode = LanguageCode {
    name: "Karey",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyd",
    individual_languages: &[
    ],
};


pub const KYE: LanguageCode = LanguageCode {
    name: "Krache",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kye",
    individual_languages: &[
    ],
};


pub const KYF: LanguageCode = LanguageCode {
    name: "Kouya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyf",
    individual_languages: &[
    ],
};


pub const KYG: LanguageCode = LanguageCode {
    name: "Keyagana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyg",
    individual_languages: &[
    ],
};


pub const KYH: LanguageCode = LanguageCode {
    name: "Karok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyh",
    individual_languages: &[
    ],
};


pub const KYI: LanguageCode = LanguageCode {
    name: "Kiput",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyi",
    individual_languages: &[
    ],
};


pub const KYJ: LanguageCode = LanguageCode {
    name: "Karao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyj",
    individual_languages: &[
    ],
};


pub const KYK: LanguageCode = LanguageCode {
    name: "Kamayo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyk",
    individual_languages: &[
    ],
};


pub const KYL: LanguageCode = LanguageCode {
    name: "Kalapuya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyl",
    individual_languages: &[
    ],
};


pub const KYM: LanguageCode = LanguageCode {
    name: "Kpatili",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kym",
    individual_languages: &[
    ],
};


pub const KYN: LanguageCode = LanguageCode {
    name: "Northern Binukidnon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyn",
    individual_languages: &[
    ],
};


pub const KYO: LanguageCode = LanguageCode {
    name: "Kelon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyo",
    individual_languages: &[
    ],
};


pub const KYP: LanguageCode = LanguageCode {
    name: "Kang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyp",
    individual_languages: &[
    ],
};


pub const KYQ: LanguageCode = LanguageCode {
    name: "Kenga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyq",
    individual_languages: &[
    ],
};


pub const KYR: LanguageCode = LanguageCode {
    name: "Kuruáya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyr",
    individual_languages: &[
    ],
};


pub const KYS: LanguageCode = LanguageCode {
    name: "Baram Kayan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kys",
    individual_languages: &[
    ],
};


pub const KYT: LanguageCode = LanguageCode {
    name: "Kayagar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyt",
    individual_languages: &[
    ],
};


pub const KYU: LanguageCode = LanguageCode {
    name: "Western Kayah",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyu",
    individual_languages: &[
    ],
};


pub const KYV: LanguageCode = LanguageCode {
    name: "Kayort",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyv",
    individual_languages: &[
    ],
};


pub const KYW: LanguageCode = LanguageCode {
    name: "Kudmali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyw",
    individual_languages: &[
    ],
};


pub const KYX: LanguageCode = LanguageCode {
    name: "Rapoisi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyx",
    individual_languages: &[
    ],
};


pub const KYY: LanguageCode = LanguageCode {
    name: "Kambaira",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyy",
    individual_languages: &[
    ],
};


pub const KYZ: LanguageCode = LanguageCode {
    name: "Kayabí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kyz",
    individual_languages: &[
    ],
};


pub const KZA: LanguageCode = LanguageCode {
    name: "Western Karaboro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kza",
    individual_languages: &[
    ],
};


pub const KZB: LanguageCode = LanguageCode {
    name: "Kaibobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzb",
    individual_languages: &[
    ],
};


pub const KZC: LanguageCode = LanguageCode {
    name: "Bondoukou Kulango",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzc",
    individual_languages: &[
    ],
};


pub const KZD: LanguageCode = LanguageCode {
    name: "Kadai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzd",
    individual_languages: &[
    ],
};


pub const KZE: LanguageCode = LanguageCode {
    name: "Kosena",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kze",
    individual_languages: &[
    ],
};


pub const KZF: LanguageCode = LanguageCode {
    name: "Da'a Kaili",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzf",
    individual_languages: &[
    ],
};


pub const KZG: LanguageCode = LanguageCode {
    name: "Kikai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzg",
    individual_languages: &[
    ],
};


pub const KZI: LanguageCode = LanguageCode {
    name: "Kelabit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzi",
    individual_languages: &[
    ],
};


pub const KZK: LanguageCode = LanguageCode {
    name: "Kazukuru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzk",
    individual_languages: &[
    ],
};


pub const KZL: LanguageCode = LanguageCode {
    name: "Kayeli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzl",
    individual_languages: &[
    ],
};


pub const KZM: LanguageCode = LanguageCode {
    name: "Kais",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzm",
    individual_languages: &[
    ],
};


pub const KZN: LanguageCode = LanguageCode {
    name: "Kokola",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzn",
    individual_languages: &[
    ],
};


pub const KZO: LanguageCode = LanguageCode {
    name: "Kaningi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzo",
    individual_languages: &[
    ],
};


pub const KZP: LanguageCode = LanguageCode {
    name: "Kaidipang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzp",
    individual_languages: &[
    ],
};


pub const KZQ: LanguageCode = LanguageCode {
    name: "Kaike",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzq",
    individual_languages: &[
    ],
};


pub const KZR: LanguageCode = LanguageCode {
    name: "Karang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzr",
    individual_languages: &[
    ],
};


pub const KZS: LanguageCode = LanguageCode {
    name: "Sugut Dusun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzs",
    individual_languages: &[
    ],
};


pub const KZU: LanguageCode = LanguageCode {
    name: "Kayupulau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzu",
    individual_languages: &[
    ],
};


pub const KZV: LanguageCode = LanguageCode {
    name: "Komyandaret",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzv",
    individual_languages: &[
    ],
};


pub const KZW: LanguageCode = LanguageCode {
    name: "Karirí-Xocó",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzw",
    individual_languages: &[
    ],
};


pub const KZX: LanguageCode = LanguageCode {
    name: "Kamarian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzx",
    individual_languages: &[
    ],
};


pub const KZY: LanguageCode = LanguageCode {
    name: "Kango (Tshopo District)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzy",
    individual_languages: &[
    ],
};


pub const KZZ: LanguageCode = LanguageCode {
    name: "Kalabra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "kzz",
    individual_languages: &[
    ],
};


pub const LAA: LanguageCode = LanguageCode {
    name: "Southern Subanen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "laa",
    individual_languages: &[
    ],
};


pub const LAB: LanguageCode = LanguageCode {
    name: "Linear A",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lab",
    individual_languages: &[
    ],
};


pub const LAC: LanguageCode = LanguageCode {
    name: "Lacandon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lac",
    individual_languages: &[
    ],
};


pub const LAE: LanguageCode = LanguageCode {
    name: "Pattani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lae",
    individual_languages: &[
    ],
};


pub const LAF: LanguageCode = LanguageCode {
    name: "Lafofa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "laf",
    individual_languages: &[
    ],
};


pub const LAG: LanguageCode = LanguageCode {
    name: "Langi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lag",
    individual_languages: &[
    ],
};


pub const LAI: LanguageCode = LanguageCode {
    name: "Lambya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lai",
    individual_languages: &[
    ],
};


pub const LAJ: LanguageCode = LanguageCode {
    name: "Lango (Uganda)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "laj",
    individual_languages: &[
    ],
};


pub const LAL: LanguageCode = LanguageCode {
    name: "Lalia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lal",
    individual_languages: &[
    ],
};


pub const LAN: LanguageCode = LanguageCode {
    name: "Laru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lan",
    individual_languages: &[
    ],
};


pub const LAP: LanguageCode = LanguageCode {
    name: "Laka (Chad)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lap",
    individual_languages: &[
    ],
};


pub const LAQ: LanguageCode = LanguageCode {
    name: "Qabiao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "laq",
    individual_languages: &[
    ],
};


pub const LAR: LanguageCode = LanguageCode {
    name: "Larteh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lar",
    individual_languages: &[
    ],
};


pub const LAS: LanguageCode = LanguageCode {
    name: "Lama (Togo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "las",
    individual_languages: &[
    ],
};


pub const LAU: LanguageCode = LanguageCode {
    name: "Laba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lau",
    individual_languages: &[
    ],
};


pub const LAW: LanguageCode = LanguageCode {
    name: "Lauje",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "law",
    individual_languages: &[
    ],
};


pub const LAX: LanguageCode = LanguageCode {
    name: "Tiwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lax",
    individual_languages: &[
    ],
};


pub const LAY: LanguageCode = LanguageCode {
    name: "Lama Bai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lay",
    individual_languages: &[
    ],
};


pub const LAZ: LanguageCode = LanguageCode {
    name: "Aribwatsa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "laz",
    individual_languages: &[
    ],
};


pub const LBB: LanguageCode = LanguageCode {
    name: "Label",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbb",
    individual_languages: &[
    ],
};


pub const LBC: LanguageCode = LanguageCode {
    name: "Lakkia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbc",
    individual_languages: &[
    ],
};


pub const LBE: LanguageCode = LanguageCode {
    name: "Lak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbe",
    individual_languages: &[
    ],
};


pub const LBF: LanguageCode = LanguageCode {
    name: "Tinani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbf",
    individual_languages: &[
    ],
};


pub const LBG: LanguageCode = LanguageCode {
    name: "Laopang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbg",
    individual_languages: &[
    ],
};


pub const LBI: LanguageCode = LanguageCode {
    name: "La'bi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbi",
    individual_languages: &[
    ],
};


pub const LBJ: LanguageCode = LanguageCode {
    name: "Ladakhi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbj",
    individual_languages: &[
    ],
};


pub const LBK: LanguageCode = LanguageCode {
    name: "Central Bontok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbk",
    individual_languages: &[
    ],
};


pub const LBL: LanguageCode = LanguageCode {
    name: "Libon Bikol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbl",
    individual_languages: &[
    ],
};


pub const LBM: LanguageCode = LanguageCode {
    name: "Lodhi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbm",
    individual_languages: &[
    ],
};


pub const LBN: LanguageCode = LanguageCode {
    name: "Rmeet",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbn",
    individual_languages: &[
    ],
};


pub const LBO: LanguageCode = LanguageCode {
    name: "Laven",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbo",
    individual_languages: &[
    ],
};


pub const LBQ: LanguageCode = LanguageCode {
    name: "Wampar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbq",
    individual_languages: &[
    ],
};


pub const LBR: LanguageCode = LanguageCode {
    name: "Lohorung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbr",
    individual_languages: &[
    ],
};


pub const LBS: LanguageCode = LanguageCode {
    name: "Libyan Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbs",
    individual_languages: &[
    ],
};


pub const LBT: LanguageCode = LanguageCode {
    name: "Lachi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbt",
    individual_languages: &[
    ],
};


pub const LBU: LanguageCode = LanguageCode {
    name: "Labu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbu",
    individual_languages: &[
    ],
};


pub const LBV: LanguageCode = LanguageCode {
    name: "Lavatbura-Lamusong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbv",
    individual_languages: &[
    ],
};


pub const LBW: LanguageCode = LanguageCode {
    name: "Tolaki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbw",
    individual_languages: &[
    ],
};


pub const LBX: LanguageCode = LanguageCode {
    name: "Lawangan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbx",
    individual_languages: &[
    ],
};


pub const LBY: LanguageCode = LanguageCode {
    name: "Lamalama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lby",
    individual_languages: &[
    ],
};


pub const LBZ: LanguageCode = LanguageCode {
    name: "Lardil",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lbz",
    individual_languages: &[
    ],
};


pub const LCC: LanguageCode = LanguageCode {
    name: "Legenyem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lcc",
    individual_languages: &[
    ],
};


pub const LCD: LanguageCode = LanguageCode {
    name: "Lola",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lcd",
    individual_languages: &[
    ],
};


pub const LCE: LanguageCode = LanguageCode {
    name: "Loncong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lce",
    individual_languages: &[
    ],
};


pub const LCF: LanguageCode = LanguageCode {
    name: "Lubu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lcf",
    individual_languages: &[
    ],
};


pub const LCH: LanguageCode = LanguageCode {
    name: "Luchazi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lch",
    individual_languages: &[
    ],
};


pub const LCL: LanguageCode = LanguageCode {
    name: "Lisela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lcl",
    individual_languages: &[
    ],
};


pub const LCM: LanguageCode = LanguageCode {
    name: "Tungag",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lcm",
    individual_languages: &[
    ],
};


pub const LCP: LanguageCode = LanguageCode {
    name: "Western Lawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lcp",
    individual_languages: &[
    ],
};


pub const LCQ: LanguageCode = LanguageCode {
    name: "Luhu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lcq",
    individual_languages: &[
    ],
};


pub const LCS: LanguageCode = LanguageCode {
    name: "Lisabata-Nuniali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lcs",
    individual_languages: &[
    ],
};


pub const LDA: LanguageCode = LanguageCode {
    name: "Kla-Dan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lda",
    individual_languages: &[
    ],
};


pub const LDB: LanguageCode = LanguageCode {
    name: "Dũya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldb",
    individual_languages: &[
    ],
};


pub const LDD: LanguageCode = LanguageCode {
    name: "Luri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldd",
    individual_languages: &[
    ],
};


pub const LDG: LanguageCode = LanguageCode {
    name: "Lenyima",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldg",
    individual_languages: &[
    ],
};


pub const LDH: LanguageCode = LanguageCode {
    name: "Lamja-Dengsa-Tola",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldh",
    individual_languages: &[
    ],
};


pub const LDI: LanguageCode = LanguageCode {
    name: "Laari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldi",
    individual_languages: &[
    ],
};


pub const LDJ: LanguageCode = LanguageCode {
    name: "Lemoro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldj",
    individual_languages: &[
    ],
};


pub const LDK: LanguageCode = LanguageCode {
    name: "Leelau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldk",
    individual_languages: &[
    ],
};


pub const LDL: LanguageCode = LanguageCode {
    name: "Kaan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldl",
    individual_languages: &[
    ],
};


pub const LDM: LanguageCode = LanguageCode {
    name: "Landoma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldm",
    individual_languages: &[
    ],
};


pub const LDN: LanguageCode = LanguageCode {
    name: "Láadan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldn",
    individual_languages: &[
    ],
};


pub const LDO: LanguageCode = LanguageCode {
    name: "Loo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldo",
    individual_languages: &[
    ],
};


pub const LDP: LanguageCode = LanguageCode {
    name: "Tso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldp",
    individual_languages: &[
    ],
};


pub const LDQ: LanguageCode = LanguageCode {
    name: "Lufu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ldq",
    individual_languages: &[
    ],
};


pub const LEA: LanguageCode = LanguageCode {
    name: "Lega-Shabunda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lea",
    individual_languages: &[
    ],
};


pub const LEB: LanguageCode = LanguageCode {
    name: "Lala-Bisa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "leb",
    individual_languages: &[
    ],
};


pub const LEC: LanguageCode = LanguageCode {
    name: "Leco",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lec",
    individual_languages: &[
    ],
};


pub const LED: LanguageCode = LanguageCode {
    name: "Lendu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "led",
    individual_languages: &[
    ],
};


pub const LEE: LanguageCode = LanguageCode {
    name: "Lyélé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lee",
    individual_languages: &[
    ],
};


pub const LEF: LanguageCode = LanguageCode {
    name: "Lelemi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lef",
    individual_languages: &[
    ],
};


pub const LEH: LanguageCode = LanguageCode {
    name: "Lenje",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "leh",
    individual_languages: &[
    ],
};


pub const LEI: LanguageCode = LanguageCode {
    name: "Lemio",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lei",
    individual_languages: &[
    ],
};


pub const LEJ: LanguageCode = LanguageCode {
    name: "Lengola",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lej",
    individual_languages: &[
    ],
};


pub const LEK: LanguageCode = LanguageCode {
    name: "Leipon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lek",
    individual_languages: &[
    ],
};


pub const LEL: LanguageCode = LanguageCode {
    name: "Lele (Democratic Republic of Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lel",
    individual_languages: &[
    ],
};


pub const LEM: LanguageCode = LanguageCode {
    name: "Nomaande",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lem",
    individual_languages: &[
    ],
};


pub const LEN: LanguageCode = LanguageCode {
    name: "Lenca",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "len",
    individual_languages: &[
    ],
};


pub const LEO: LanguageCode = LanguageCode {
    name: "Leti (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "leo",
    individual_languages: &[
    ],
};


pub const LEP: LanguageCode = LanguageCode {
    name: "Lepcha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lep",
    individual_languages: &[
    ],
};


pub const LEQ: LanguageCode = LanguageCode {
    name: "Lembena",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "leq",
    individual_languages: &[
    ],
};


pub const LER: LanguageCode = LanguageCode {
    name: "Lenkau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ler",
    individual_languages: &[
    ],
};


pub const LES: LanguageCode = LanguageCode {
    name: "Lese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "les",
    individual_languages: &[
    ],
};


pub const LET: LanguageCode = LanguageCode {
    name: "Lesing-Gelimi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "let",
    individual_languages: &[
    ],
};


pub const LEU: LanguageCode = LanguageCode {
    name: "Kara (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "leu",
    individual_languages: &[
    ],
};


pub const LEV: LanguageCode = LanguageCode {
    name: "Lamma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lev",
    individual_languages: &[
    ],
};


pub const LEW: LanguageCode = LanguageCode {
    name: "Ledo Kaili",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lew",
    individual_languages: &[
    ],
};


pub const LEX: LanguageCode = LanguageCode {
    name: "Luang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lex",
    individual_languages: &[
    ],
};


pub const LEY: LanguageCode = LanguageCode {
    name: "Lemolang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ley",
    individual_languages: &[
    ],
};


pub const LFA: LanguageCode = LanguageCode {
    name: "Lefa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lfa",
    individual_languages: &[
    ],
};


pub const LFN: LanguageCode = LanguageCode {
    name: "Lingua Franca Nova",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lfn",
    individual_languages: &[
    ],
};


pub const LGA: LanguageCode = LanguageCode {
    name: "Lungga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lga",
    individual_languages: &[
    ],
};


pub const LGB: LanguageCode = LanguageCode {
    name: "Laghu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgb",
    individual_languages: &[
    ],
};


pub const LGG: LanguageCode = LanguageCode {
    name: "Lugbara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgg",
    individual_languages: &[
    ],
};


pub const LGH: LanguageCode = LanguageCode {
    name: "Laghuu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgh",
    individual_languages: &[
    ],
};


pub const LGI: LanguageCode = LanguageCode {
    name: "Lengilu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgi",
    individual_languages: &[
    ],
};


pub const LGK: LanguageCode = LanguageCode {
    name: "Lingarak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgk",
    individual_languages: &[
    ],
};


pub const LGL: LanguageCode = LanguageCode {
    name: "Wala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgl",
    individual_languages: &[
    ],
};


pub const LGM: LanguageCode = LanguageCode {
    name: "Lega-Mwenga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgm",
    individual_languages: &[
    ],
};


pub const LGN: LanguageCode = LanguageCode {
    name: "T'apo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgn",
    individual_languages: &[
    ],
};


pub const LGO: LanguageCode = LanguageCode {
    name: "Lango (South Sudan)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgo",
    individual_languages: &[
    ],
};


pub const LGQ: LanguageCode = LanguageCode {
    name: "Logba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgq",
    individual_languages: &[
    ],
};


pub const LGR: LanguageCode = LanguageCode {
    name: "Lengo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgr",
    individual_languages: &[
    ],
};


pub const LGT: LanguageCode = LanguageCode {
    name: "Pahi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgt",
    individual_languages: &[
    ],
};


pub const LGU: LanguageCode = LanguageCode {
    name: "Longgu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgu",
    individual_languages: &[
    ],
};


pub const LGZ: LanguageCode = LanguageCode {
    name: "Ligenza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lgz",
    individual_languages: &[
    ],
};


pub const LHA: LanguageCode = LanguageCode {
    name: "Laha (Viet Nam)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lha",
    individual_languages: &[
    ],
};


pub const LHH: LanguageCode = LanguageCode {
    name: "Laha (Indonesia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lhh",
    individual_languages: &[
    ],
};


pub const LHI: LanguageCode = LanguageCode {
    name: "Lahu Shi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lhi",
    individual_languages: &[
    ],
};


pub const LHL: LanguageCode = LanguageCode {
    name: "Lahul Lohar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lhl",
    individual_languages: &[
    ],
};


pub const LHM: LanguageCode = LanguageCode {
    name: "Lhomi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lhm",
    individual_languages: &[
    ],
};


pub const LHN: LanguageCode = LanguageCode {
    name: "Lahanan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lhn",
    individual_languages: &[
    ],
};


pub const LHP: LanguageCode = LanguageCode {
    name: "Lhokpu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lhp",
    individual_languages: &[
    ],
};


pub const LHS: LanguageCode = LanguageCode {
    name: "Mlahsö",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lhs",
    individual_languages: &[
    ],
};


pub const LHT: LanguageCode = LanguageCode {
    name: "Lo-Toga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lht",
    individual_languages: &[
    ],
};


pub const LHU: LanguageCode = LanguageCode {
    name: "Lahu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lhu",
    individual_languages: &[
    ],
};


pub const LIA: LanguageCode = LanguageCode {
    name: "West-Central Limba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lia",
    individual_languages: &[
    ],
};


pub const LIB: LanguageCode = LanguageCode {
    name: "Likum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lib",
    individual_languages: &[
    ],
};


pub const LIC: LanguageCode = LanguageCode {
    name: "Hlai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lic",
    individual_languages: &[
    ],
};


pub const LID: LanguageCode = LanguageCode {
    name: "Nyindrou",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lid",
    individual_languages: &[
    ],
};


pub const LIE: LanguageCode = LanguageCode {
    name: "Likila",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lie",
    individual_languages: &[
    ],
};


pub const LIF: LanguageCode = LanguageCode {
    name: "Limbu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lif",
    individual_languages: &[
    ],
};


pub const LIG: LanguageCode = LanguageCode {
    name: "Ligbi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lig",
    individual_languages: &[
    ],
};


pub const LIH: LanguageCode = LanguageCode {
    name: "Lihir",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lih",
    individual_languages: &[
    ],
};


pub const LIJ: LanguageCode = LanguageCode {
    name: "Ligurian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lij",
    individual_languages: &[
    ],
};


pub const LIK: LanguageCode = LanguageCode {
    name: "Lika",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lik",
    individual_languages: &[
    ],
};


pub const LIL: LanguageCode = LanguageCode {
    name: "Lillooet",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lil",
    individual_languages: &[
    ],
};


pub const LIO: LanguageCode = LanguageCode {
    name: "Liki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lio",
    individual_languages: &[
    ],
};


pub const LIP: LanguageCode = LanguageCode {
    name: "Sekpele",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lip",
    individual_languages: &[
    ],
};


pub const LIQ: LanguageCode = LanguageCode {
    name: "Libido",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "liq",
    individual_languages: &[
    ],
};


pub const LIR: LanguageCode = LanguageCode {
    name: "Liberian English",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lir",
    individual_languages: &[
    ],
};


pub const LIS: LanguageCode = LanguageCode {
    name: "Lisu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lis",
    individual_languages: &[
    ],
};


pub const LIU: LanguageCode = LanguageCode {
    name: "Logorik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "liu",
    individual_languages: &[
    ],
};


pub const LIV: LanguageCode = LanguageCode {
    name: "Liv",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "liv",
    individual_languages: &[
    ],
};


pub const LIW: LanguageCode = LanguageCode {
    name: "Col",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "liw",
    individual_languages: &[
    ],
};


pub const LIX: LanguageCode = LanguageCode {
    name: "Liabuku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lix",
    individual_languages: &[
    ],
};


pub const LIY: LanguageCode = LanguageCode {
    name: "Banda-Bambari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "liy",
    individual_languages: &[
    ],
};


pub const LIZ: LanguageCode = LanguageCode {
    name: "Libinza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "liz",
    individual_languages: &[
    ],
};


pub const LJA: LanguageCode = LanguageCode {
    name: "Golpa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lja",
    individual_languages: &[
    ],
};


pub const LJE: LanguageCode = LanguageCode {
    name: "Rampi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lje",
    individual_languages: &[
    ],
};


pub const LJI: LanguageCode = LanguageCode {
    name: "Laiyolo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lji",
    individual_languages: &[
    ],
};


pub const LJL: LanguageCode = LanguageCode {
    name: "Li'o",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ljl",
    individual_languages: &[
    ],
};


pub const LJP: LanguageCode = LanguageCode {
    name: "Lampung Api",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ljp",
    individual_languages: &[
    ],
};


pub const LJW: LanguageCode = LanguageCode {
    name: "Yirandali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ljw",
    individual_languages: &[
    ],
};


pub const LJX: LanguageCode = LanguageCode {
    name: "Yuru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ljx",
    individual_languages: &[
    ],
};


pub const LKA: LanguageCode = LanguageCode {
    name: "Lakalei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lka",
    individual_languages: &[
    ],
};


pub const LKB: LanguageCode = LanguageCode {
    name: "Kabras",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lkb",
    individual_languages: &[
    ],
};


pub const LKC: LanguageCode = LanguageCode {
    name: "Kucong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lkc",
    individual_languages: &[
    ],
};


pub const LKD: LanguageCode = LanguageCode {
    name: "Lakondê",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lkd",
    individual_languages: &[
    ],
};


pub const LKE: LanguageCode = LanguageCode {
    name: "Kenyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lke",
    individual_languages: &[
    ],
};


pub const LKH: LanguageCode = LanguageCode {
    name: "Lakha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lkh",
    individual_languages: &[
    ],
};


pub const LKI: LanguageCode = LanguageCode {
    name: "Laki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lki",
    individual_languages: &[
    ],
};


pub const LKJ: LanguageCode = LanguageCode {
    name: "Remun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lkj",
    individual_languages: &[
    ],
};


pub const LKL: LanguageCode = LanguageCode {
    name: "Laeko-Libuat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lkl",
    individual_languages: &[
    ],
};


pub const LKM: LanguageCode = LanguageCode {
    name: "Kalaamaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lkm",
    individual_languages: &[
    ],
};


pub const LKN: LanguageCode = LanguageCode {
    name: "Lakon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lkn",
    individual_languages: &[
    ],
};


pub const LKO: LanguageCode = LanguageCode {
    name: "Khayo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lko",
    individual_languages: &[
    ],
};


pub const LKR: LanguageCode = LanguageCode {
    name: "Päri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lkr",
    individual_languages: &[
    ],
};


pub const LKS: LanguageCode = LanguageCode {
    name: "Kisa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lks",
    individual_languages: &[
    ],
};


pub const LKT: LanguageCode = LanguageCode {
    name: "Lakota",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lkt",
    individual_languages: &[
    ],
};


pub const LKU: LanguageCode = LanguageCode {
    name: "Kungkari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lku",
    individual_languages: &[
    ],
};


pub const LKY: LanguageCode = LanguageCode {
    name: "Lokoya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lky",
    individual_languages: &[
    ],
};


pub const LLA: LanguageCode = LanguageCode {
    name: "Lala-Roba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lla",
    individual_languages: &[
    ],
};


pub const LLB: LanguageCode = LanguageCode {
    name: "Lolo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "llb",
    individual_languages: &[
    ],
};


pub const LLC: LanguageCode = LanguageCode {
    name: "Lele (Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "llc",
    individual_languages: &[
    ],
};


pub const LLD: LanguageCode = LanguageCode {
    name: "Ladin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lld",
    individual_languages: &[
    ],
};


pub const LLE: LanguageCode = LanguageCode {
    name: "Lele (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lle",
    individual_languages: &[
    ],
};


pub const LLF: LanguageCode = LanguageCode {
    name: "Hermit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "llf",
    individual_languages: &[
    ],
};


pub const LLG: LanguageCode = LanguageCode {
    name: "Lole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "llg",
    individual_languages: &[
    ],
};


pub const LLH: LanguageCode = LanguageCode {
    name: "Lamu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "llh",
    individual_languages: &[
    ],
};


pub const LLI: LanguageCode = LanguageCode {
    name: "Teke-Laali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lli",
    individual_languages: &[
    ],
};


pub const LLJ: LanguageCode = LanguageCode {
    name: "Ladji Ladji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "llj",
    individual_languages: &[
    ],
};


pub const LLK: LanguageCode = LanguageCode {
    name: "Lelak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "llk",
    individual_languages: &[
    ],
};


pub const LLL: LanguageCode = LanguageCode {
    name: "Lilau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lll",
    individual_languages: &[
    ],
};


pub const LLM: LanguageCode = LanguageCode {
    name: "Lasalimu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "llm",
    individual_languages: &[
    ],
};


pub const LLN: LanguageCode = LanguageCode {
    name: "Lele (Chad)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lln",
    individual_languages: &[
    ],
};


pub const LLP: LanguageCode = LanguageCode {
    name: "North Efate",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "llp",
    individual_languages: &[
    ],
};


pub const LLQ: LanguageCode = LanguageCode {
    name: "Lolak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "llq",
    individual_languages: &[
    ],
};


pub const LLS: LanguageCode = LanguageCode {
    name: "Lithuanian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lls",
    individual_languages: &[
    ],
};


pub const LLU: LanguageCode = LanguageCode {
    name: "Lau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "llu",
    individual_languages: &[
    ],
};


pub const LLX: LanguageCode = LanguageCode {
    name: "Lauan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "llx",
    individual_languages: &[
    ],
};


pub const LMA: LanguageCode = LanguageCode {
    name: "East Limba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lma",
    individual_languages: &[
    ],
};


pub const LMB: LanguageCode = LanguageCode {
    name: "Merei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmb",
    individual_languages: &[
    ],
};


pub const LMC: LanguageCode = LanguageCode {
    name: "Limilngan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmc",
    individual_languages: &[
    ],
};


pub const LMD: LanguageCode = LanguageCode {
    name: "Lumun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmd",
    individual_languages: &[
    ],
};


pub const LME: LanguageCode = LanguageCode {
    name: "Pévé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lme",
    individual_languages: &[
    ],
};


pub const LMF: LanguageCode = LanguageCode {
    name: "South Lembata",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmf",
    individual_languages: &[
    ],
};


pub const LMG: LanguageCode = LanguageCode {
    name: "Lamogai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmg",
    individual_languages: &[
    ],
};


pub const LMH: LanguageCode = LanguageCode {
    name: "Lambichhong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmh",
    individual_languages: &[
    ],
};


pub const LMI: LanguageCode = LanguageCode {
    name: "Lombi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmi",
    individual_languages: &[
    ],
};


pub const LMJ: LanguageCode = LanguageCode {
    name: "West Lembata",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmj",
    individual_languages: &[
    ],
};


pub const LMK: LanguageCode = LanguageCode {
    name: "Lamkang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmk",
    individual_languages: &[
    ],
};


pub const LML: LanguageCode = LanguageCode {
    name: "Hano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lml",
    individual_languages: &[
    ],
};


pub const LMN: LanguageCode = LanguageCode {
    name: "Lambadi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmn",
    individual_languages: &[
    ],
};


pub const LMO: LanguageCode = LanguageCode {
    name: "Lombard",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmo",
    individual_languages: &[
    ],
};


pub const LMP: LanguageCode = LanguageCode {
    name: "Limbum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmp",
    individual_languages: &[
    ],
};


pub const LMQ: LanguageCode = LanguageCode {
    name: "Lamatuka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmq",
    individual_languages: &[
    ],
};


pub const LMR: LanguageCode = LanguageCode {
    name: "Lamalera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmr",
    individual_languages: &[
    ],
};


pub const LMU: LanguageCode = LanguageCode {
    name: "Lamenu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmu",
    individual_languages: &[
    ],
};


pub const LMV: LanguageCode = LanguageCode {
    name: "Lomaiviti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmv",
    individual_languages: &[
    ],
};


pub const LMW: LanguageCode = LanguageCode {
    name: "Lake Miwok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmw",
    individual_languages: &[
    ],
};


pub const LMX: LanguageCode = LanguageCode {
    name: "Laimbue",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmx",
    individual_languages: &[
    ],
};


pub const LMY: LanguageCode = LanguageCode {
    name: "Lamboya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lmy",
    individual_languages: &[
    ],
};


pub const LNA: LanguageCode = LanguageCode {
    name: "Langbashe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lna",
    individual_languages: &[
    ],
};


pub const LNB: LanguageCode = LanguageCode {
    name: "Mbalanhu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lnb",
    individual_languages: &[
    ],
};


pub const LND: LanguageCode = LanguageCode {
    name: "Lundayeh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lnd",
    individual_languages: &[
    ],
};


pub const LNG: LanguageCode = LanguageCode {
    name: "Langobardic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lng",
    individual_languages: &[
    ],
};


pub const LNH: LanguageCode = LanguageCode {
    name: "Lanoh",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lnh",
    individual_languages: &[
    ],
};


pub const LNI: LanguageCode = LanguageCode {
    name: "Daantanai'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lni",
    individual_languages: &[
    ],
};


pub const LNJ: LanguageCode = LanguageCode {
    name: "Leningitij",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lnj",
    individual_languages: &[
    ],
};


pub const LNL: LanguageCode = LanguageCode {
    name: "South Central Banda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lnl",
    individual_languages: &[
    ],
};


pub const LNM: LanguageCode = LanguageCode {
    name: "Langam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lnm",
    individual_languages: &[
    ],
};


pub const LNN: LanguageCode = LanguageCode {
    name: "Lorediakarkar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lnn",
    individual_languages: &[
    ],
};


pub const LNS: LanguageCode = LanguageCode {
    name: "Lamnso'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lns",
    individual_languages: &[
    ],
};


pub const LNU: LanguageCode = LanguageCode {
    name: "Longuda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lnu",
    individual_languages: &[
    ],
};


pub const LNW: LanguageCode = LanguageCode {
    name: "Lanima",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lnw",
    individual_languages: &[
    ],
};


pub const LNZ: LanguageCode = LanguageCode {
    name: "Lonzo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lnz",
    individual_languages: &[
    ],
};


pub const LOA: LanguageCode = LanguageCode {
    name: "Loloda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "loa",
    individual_languages: &[
    ],
};


pub const LOB: LanguageCode = LanguageCode {
    name: "Lobi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lob",
    individual_languages: &[
    ],
};


pub const LOC: LanguageCode = LanguageCode {
    name: "Inonhan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "loc",
    individual_languages: &[
    ],
};


pub const LOE: LanguageCode = LanguageCode {
    name: "Saluan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "loe",
    individual_languages: &[
    ],
};


pub const LOF: LanguageCode = LanguageCode {
    name: "Logol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lof",
    individual_languages: &[
    ],
};


pub const LOG: LanguageCode = LanguageCode {
    name: "Logo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "log",
    individual_languages: &[
    ],
};


pub const LOH: LanguageCode = LanguageCode {
    name: "Narim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "loh",
    individual_languages: &[
    ],
};


pub const LOI: LanguageCode = LanguageCode {
    name: "Loma (Côte d'Ivoire)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "loi",
    individual_languages: &[
    ],
};


pub const LOJ: LanguageCode = LanguageCode {
    name: "Lou",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "loj",
    individual_languages: &[
    ],
};


pub const LOK: LanguageCode = LanguageCode {
    name: "Loko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lok",
    individual_languages: &[
    ],
};


pub const LOM: LanguageCode = LanguageCode {
    name: "Loma (Liberia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lom",
    individual_languages: &[
    ],
};


pub const LON: LanguageCode = LanguageCode {
    name: "Malawi Lomwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lon",
    individual_languages: &[
    ],
};


pub const LOO: LanguageCode = LanguageCode {
    name: "Lombo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "loo",
    individual_languages: &[
    ],
};


pub const LOP: LanguageCode = LanguageCode {
    name: "Lopa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lop",
    individual_languages: &[
    ],
};


pub const LOQ: LanguageCode = LanguageCode {
    name: "Lobala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "loq",
    individual_languages: &[
    ],
};


pub const LOR: LanguageCode = LanguageCode {
    name: "Téén",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lor",
    individual_languages: &[
    ],
};


pub const LOS: LanguageCode = LanguageCode {
    name: "Loniu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "los",
    individual_languages: &[
    ],
};


pub const LOT: LanguageCode = LanguageCode {
    name: "Otuho",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lot",
    individual_languages: &[
    ],
};


pub const LOU: LanguageCode = LanguageCode {
    name: "Louisiana Creole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lou",
    individual_languages: &[
    ],
};


pub const LOV: LanguageCode = LanguageCode {
    name: "Lopi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lov",
    individual_languages: &[
    ],
};


pub const LOW: LanguageCode = LanguageCode {
    name: "Tampias Lobu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "low",
    individual_languages: &[
    ],
};


pub const LOX: LanguageCode = LanguageCode {
    name: "Loun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lox",
    individual_languages: &[
    ],
};


pub const LOY: LanguageCode = LanguageCode {
    name: "Loke",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "loy",
    individual_languages: &[
    ],
};


pub const LPA: LanguageCode = LanguageCode {
    name: "Lelepa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lpa",
    individual_languages: &[
    ],
};


pub const LPE: LanguageCode = LanguageCode {
    name: "Lepki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lpe",
    individual_languages: &[
    ],
};


pub const LPN: LanguageCode = LanguageCode {
    name: "Long Phuri Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lpn",
    individual_languages: &[
    ],
};


pub const LPO: LanguageCode = LanguageCode {
    name: "Lipo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lpo",
    individual_languages: &[
    ],
};


pub const LPX: LanguageCode = LanguageCode {
    name: "Lopit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lpx",
    individual_languages: &[
    ],
};


pub const LQR: LanguageCode = LanguageCode {
    name: "Logir",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lqr",
    individual_languages: &[
    ],
};


pub const LRA: LanguageCode = LanguageCode {
    name: "Rara Bakati'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lra",
    individual_languages: &[
    ],
};


pub const LRC: LanguageCode = LanguageCode {
    name: "Northern Luri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lrc",
    individual_languages: &[
    ],
};


pub const LRE: LanguageCode = LanguageCode {
    name: "Laurentian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lre",
    individual_languages: &[
    ],
};


pub const LRG: LanguageCode = LanguageCode {
    name: "Laragia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lrg",
    individual_languages: &[
    ],
};


pub const LRI: LanguageCode = LanguageCode {
    name: "Marachi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lri",
    individual_languages: &[
    ],
};


pub const LRK: LanguageCode = LanguageCode {
    name: "Loarki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lrk",
    individual_languages: &[
    ],
};


pub const LRL: LanguageCode = LanguageCode {
    name: "Lari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lrl",
    individual_languages: &[
    ],
};


pub const LRM: LanguageCode = LanguageCode {
    name: "Marama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lrm",
    individual_languages: &[
    ],
};


pub const LRN: LanguageCode = LanguageCode {
    name: "Lorang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lrn",
    individual_languages: &[
    ],
};


pub const LRO: LanguageCode = LanguageCode {
    name: "Laro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lro",
    individual_languages: &[
    ],
};


pub const LRR: LanguageCode = LanguageCode {
    name: "Southern Yamphu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lrr",
    individual_languages: &[
    ],
};


pub const LRT: LanguageCode = LanguageCode {
    name: "Larantuka Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lrt",
    individual_languages: &[
    ],
};


pub const LRV: LanguageCode = LanguageCode {
    name: "Larevat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lrv",
    individual_languages: &[
    ],
};


pub const LRZ: LanguageCode = LanguageCode {
    name: "Lemerig",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lrz",
    individual_languages: &[
    ],
};


pub const LSA: LanguageCode = LanguageCode {
    name: "Lasgerdi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsa",
    individual_languages: &[
    ],
};


pub const LSB: LanguageCode = LanguageCode {
    name: "Burundian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsb",
    individual_languages: &[
    ],
};


pub const LSC: LanguageCode = LanguageCode {
    name: "Albarradas Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsc",
    individual_languages: &[
    ],
};


pub const LSD: LanguageCode = LanguageCode {
    name: "Lishana Deni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsd",
    individual_languages: &[
    ],
};


pub const LSE: LanguageCode = LanguageCode {
    name: "Lusengo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lse",
    individual_languages: &[
    ],
};


pub const LSH: LanguageCode = LanguageCode {
    name: "Lish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsh",
    individual_languages: &[
    ],
};


pub const LSI: LanguageCode = LanguageCode {
    name: "Lashi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsi",
    individual_languages: &[
    ],
};


pub const LSL: LanguageCode = LanguageCode {
    name: "Latvian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsl",
    individual_languages: &[
    ],
};


pub const LSM: LanguageCode = LanguageCode {
    name: "Saamia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsm",
    individual_languages: &[
    ],
};


pub const LSN: LanguageCode = LanguageCode {
    name: "Tibetan Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsn",
    individual_languages: &[
    ],
};


pub const LSO: LanguageCode = LanguageCode {
    name: "Laos Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lso",
    individual_languages: &[
    ],
};


pub const LSP: LanguageCode = LanguageCode {
    name: "Panamanian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsp",
    individual_languages: &[
    ],
};


pub const LSR: LanguageCode = LanguageCode {
    name: "Aruop",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsr",
    individual_languages: &[
    ],
};


pub const LSS: LanguageCode = LanguageCode {
    name: "Lasi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lss",
    individual_languages: &[
    ],
};


pub const LST: LanguageCode = LanguageCode {
    name: "Trinidad and Tobago Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lst",
    individual_languages: &[
    ],
};


pub const LSV: LanguageCode = LanguageCode {
    name: "Sivia Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsv",
    individual_languages: &[
    ],
};


pub const LSW: LanguageCode = LanguageCode {
    name: "Seychelles Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsw",
    individual_languages: &[
    ],
};


pub const LSY: LanguageCode = LanguageCode {
    name: "Mauritian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lsy",
    individual_languages: &[
    ],
};


pub const LTC: LanguageCode = LanguageCode {
    name: "Late Middle Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ltc",
    individual_languages: &[
    ],
};


pub const LTG: LanguageCode = LanguageCode {
    name: "Latgalian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ltg",
    individual_languages: &[
    ],
};


pub const LTH: LanguageCode = LanguageCode {
    name: "Thur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lth",
    individual_languages: &[
    ],
};


pub const LTI: LanguageCode = LanguageCode {
    name: "Leti (Indonesia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lti",
    individual_languages: &[
    ],
};


pub const LTN: LanguageCode = LanguageCode {
    name: "Latundê",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ltn",
    individual_languages: &[
    ],
};


pub const LTO: LanguageCode = LanguageCode {
    name: "Tsotso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lto",
    individual_languages: &[
    ],
};


pub const LTS: LanguageCode = LanguageCode {
    name: "Tachoni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lts",
    individual_languages: &[
    ],
};


pub const LTU: LanguageCode = LanguageCode {
    name: "Latu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ltu",
    individual_languages: &[
    ],
};


pub const LUC: LanguageCode = LanguageCode {
    name: "Aringa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "luc",
    individual_languages: &[
    ],
};


pub const LUD: LanguageCode = LanguageCode {
    name: "Ludian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lud",
    individual_languages: &[
    ],
};


pub const LUE: LanguageCode = LanguageCode {
    name: "Luvale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lue",
    individual_languages: &[
    ],
};


pub const LUF: LanguageCode = LanguageCode {
    name: "Laua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "luf",
    individual_languages: &[
    ],
};


pub const LUJ: LanguageCode = LanguageCode {
    name: "Luna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "luj",
    individual_languages: &[
    ],
};


pub const LUK: LanguageCode = LanguageCode {
    name: "Lunanakha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "luk",
    individual_languages: &[
    ],
};


pub const LUL: LanguageCode = LanguageCode {
    name: "Olu'bo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lul",
    individual_languages: &[
    ],
};


pub const LUM: LanguageCode = LanguageCode {
    name: "Luimbi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lum",
    individual_languages: &[
    ],
};


pub const LUP: LanguageCode = LanguageCode {
    name: "Lumbu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lup",
    individual_languages: &[
    ],
};


pub const LUQ: LanguageCode = LanguageCode {
    name: "Lucumi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "luq",
    individual_languages: &[
    ],
};


pub const LUR: LanguageCode = LanguageCode {
    name: "Laura",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lur",
    individual_languages: &[
    ],
};


pub const LUT: LanguageCode = LanguageCode {
    name: "Lushootseed",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lut",
    individual_languages: &[
    ],
};


pub const LUU: LanguageCode = LanguageCode {
    name: "Lumba-Yakkha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "luu",
    individual_languages: &[
    ],
};


pub const LUV: LanguageCode = LanguageCode {
    name: "Luwati",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "luv",
    individual_languages: &[
    ],
};


pub const LUW: LanguageCode = LanguageCode {
    name: "Luo (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "luw",
    individual_languages: &[
    ],
};


pub const LUY: LanguageCode = LanguageCode {
    name: "Luyia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "luy",
    individual_languages: &[
        IndividualLanguages {
            name: "Lubukusu",
            code: "bxk",
        },
        IndividualLanguages {
            name: "Luidakho-Luisukha-Lutirichi",
            code: "ida",
        },
        IndividualLanguages {
            name: "Lukabaras",
            code: "lkb",
        },
        IndividualLanguages {
            name: "Olukhayo",
            code: "lko",
        },
        IndividualLanguages {
            name: "Olushisa",
            code: "lks",
        },
        IndividualLanguages {
            name: "Olumarachi",
            code: "lri",
        },
        IndividualLanguages {
            name: "Olumarama",
            code: "lrm",
        },
        IndividualLanguages {
            name: "Saamia",
            code: "lsm",
        },
        IndividualLanguages {
            name: "Tsotso",
            code: "lto",
        },
        IndividualLanguages {
            name: "Tachoni",
            code: "lts",
        },
        IndividualLanguages {
            name: "Wanga",
            code: "lwg",
        },
        IndividualLanguages {
            name: "East Nyala",
            code: "nle",
        },
        IndividualLanguages {
            name: "Olunyole",
            code: "nyd",
        },
        IndividualLanguages {
            name: "Lulogooli",
            code: "rag",
        },
    ],
};


pub const LUZ: LanguageCode = LanguageCode {
    name: "Southern Luri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "luz",
    individual_languages: &[
    ],
};


pub const LVA: LanguageCode = LanguageCode {
    name: "Maku'a",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lva",
    individual_languages: &[
    ],
};


pub const LVI: LanguageCode = LanguageCode {
    name: "Lavi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lvi",
    individual_languages: &[
    ],
};


pub const LVK: LanguageCode = LanguageCode {
    name: "Lavukaleve",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lvk",
    individual_languages: &[
    ],
};


pub const LVS: LanguageCode = LanguageCode {
    name: "Standard Latvian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lvs",
    individual_languages: &[
    ],
};


pub const LVU: LanguageCode = LanguageCode {
    name: "Levuka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lvu",
    individual_languages: &[
    ],
};


pub const LWA: LanguageCode = LanguageCode {
    name: "Lwalu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lwa",
    individual_languages: &[
    ],
};


pub const LWE: LanguageCode = LanguageCode {
    name: "Lewo Eleng",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lwe",
    individual_languages: &[
    ],
};


pub const LWG: LanguageCode = LanguageCode {
    name: "Wanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lwg",
    individual_languages: &[
    ],
};


pub const LWH: LanguageCode = LanguageCode {
    name: "White Lachi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lwh",
    individual_languages: &[
    ],
};


pub const LWL: LanguageCode = LanguageCode {
    name: "Eastern Lawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lwl",
    individual_languages: &[
    ],
};


pub const LWM: LanguageCode = LanguageCode {
    name: "Laomian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lwm",
    individual_languages: &[
    ],
};


pub const LWO: LanguageCode = LanguageCode {
    name: "Luwo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lwo",
    individual_languages: &[
    ],
};


pub const LWS: LanguageCode = LanguageCode {
    name: "Malawian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lws",
    individual_languages: &[
    ],
};


pub const LWT: LanguageCode = LanguageCode {
    name: "Lewotobi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lwt",
    individual_languages: &[
    ],
};


pub const LWU: LanguageCode = LanguageCode {
    name: "Lawu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lwu",
    individual_languages: &[
    ],
};


pub const LWW: LanguageCode = LanguageCode {
    name: "Lewo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lww",
    individual_languages: &[
    ],
};


pub const LXM: LanguageCode = LanguageCode {
    name: "Lakurumau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lxm",
    individual_languages: &[
    ],
};


pub const LYA: LanguageCode = LanguageCode {
    name: "Layakha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lya",
    individual_languages: &[
    ],
};


pub const LYG: LanguageCode = LanguageCode {
    name: "Lyngngam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lyg",
    individual_languages: &[
    ],
};


pub const LYN: LanguageCode = LanguageCode {
    name: "Luyana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lyn",
    individual_languages: &[
    ],
};


pub const LZH: LanguageCode = LanguageCode {
    name: "Literary Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lzh",
    individual_languages: &[
    ],
};


pub const LZL: LanguageCode = LanguageCode {
    name: "Litzlitz",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lzl",
    individual_languages: &[
    ],
};


pub const LZN: LanguageCode = LanguageCode {
    name: "Leinong Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lzn",
    individual_languages: &[
    ],
};


pub const LZZ: LanguageCode = LanguageCode {
    name: "Laz",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "lzz",
    individual_languages: &[
    ],
};


pub const MAA: LanguageCode = LanguageCode {
    name: "San Jerónimo Tecóatl Mazatec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "maa",
    individual_languages: &[
    ],
};


pub const MAB: LanguageCode = LanguageCode {
    name: "Yutanduchi Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mab",
    individual_languages: &[
    ],
};


pub const MAE: LanguageCode = LanguageCode {
    name: "Bo-Rukul",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mae",
    individual_languages: &[
    ],
};


pub const MAF: LanguageCode = LanguageCode {
    name: "Mafa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "maf",
    individual_languages: &[
    ],
};


pub const MAJ: LanguageCode = LanguageCode {
    name: "Jalapa De Díaz Mazatec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "maj",
    individual_languages: &[
    ],
};


pub const MAM: LanguageCode = LanguageCode {
    name: "Mam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mam",
    individual_languages: &[
    ],
};


pub const MAQ: LanguageCode = LanguageCode {
    name: "Chiquihuitlán Mazatec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "maq",
    individual_languages: &[
    ],
};


pub const MAT: LanguageCode = LanguageCode {
    name: "San Francisco Matlatzinca",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mat",
    individual_languages: &[
    ],
};


pub const MAU: LanguageCode = LanguageCode {
    name: "Huautla Mazatec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mau",
    individual_languages: &[
    ],
};


pub const MAV: LanguageCode = LanguageCode {
    name: "Sateré-Mawé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mav",
    individual_languages: &[
    ],
};


pub const MAW: LanguageCode = LanguageCode {
    name: "Mampruli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "maw",
    individual_languages: &[
    ],
};


pub const MAX: LanguageCode = LanguageCode {
    name: "North Moluccan Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "max",
    individual_languages: &[
    ],
};


pub const MAZ: LanguageCode = LanguageCode {
    name: "Central Mazahua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "maz",
    individual_languages: &[
    ],
};


pub const MBA: LanguageCode = LanguageCode {
    name: "Higaonon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mba",
    individual_languages: &[
    ],
};


pub const MBB: LanguageCode = LanguageCode {
    name: "Western Bukidnon Manobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbb",
    individual_languages: &[
    ],
};


pub const MBC: LanguageCode = LanguageCode {
    name: "Macushi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbc",
    individual_languages: &[
    ],
};


pub const MBD: LanguageCode = LanguageCode {
    name: "Dibabawon Manobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbd",
    individual_languages: &[
    ],
};


pub const MBE: LanguageCode = LanguageCode {
    name: "Molale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbe",
    individual_languages: &[
    ],
};


pub const MBF: LanguageCode = LanguageCode {
    name: "Baba Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbf",
    individual_languages: &[
    ],
};


pub const MBH: LanguageCode = LanguageCode {
    name: "Mangseng",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbh",
    individual_languages: &[
    ],
};


pub const MBI: LanguageCode = LanguageCode {
    name: "Ilianen Manobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbi",
    individual_languages: &[
    ],
};


pub const MBJ: LanguageCode = LanguageCode {
    name: "Nadëb",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbj",
    individual_languages: &[
    ],
};


pub const MBK: LanguageCode = LanguageCode {
    name: "Malol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbk",
    individual_languages: &[
    ],
};


pub const MBL: LanguageCode = LanguageCode {
    name: "Maxakalí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbl",
    individual_languages: &[
    ],
};


pub const MBM: LanguageCode = LanguageCode {
    name: "Ombamba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbm",
    individual_languages: &[
    ],
};


pub const MBN: LanguageCode = LanguageCode {
    name: "Macaguán",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbn",
    individual_languages: &[
    ],
};


pub const MBO: LanguageCode = LanguageCode {
    name: "Mbo (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbo",
    individual_languages: &[
    ],
};


pub const MBP: LanguageCode = LanguageCode {
    name: "Malayo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbp",
    individual_languages: &[
    ],
};


pub const MBQ: LanguageCode = LanguageCode {
    name: "Maisin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbq",
    individual_languages: &[
    ],
};


pub const MBR: LanguageCode = LanguageCode {
    name: "Nukak Makú",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbr",
    individual_languages: &[
    ],
};


pub const MBS: LanguageCode = LanguageCode {
    name: "Sarangani Manobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbs",
    individual_languages: &[
    ],
};


pub const MBT: LanguageCode = LanguageCode {
    name: "Matigsalug Manobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbt",
    individual_languages: &[
    ],
};


pub const MBU: LanguageCode = LanguageCode {
    name: "Mbula-Bwazza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbu",
    individual_languages: &[
    ],
};


pub const MBV: LanguageCode = LanguageCode {
    name: "Mbulungish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbv",
    individual_languages: &[
    ],
};


pub const MBW: LanguageCode = LanguageCode {
    name: "Maring",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbw",
    individual_languages: &[
    ],
};


pub const MBX: LanguageCode = LanguageCode {
    name: "Mari (East Sepik Province)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbx",
    individual_languages: &[
    ],
};


pub const MBY: LanguageCode = LanguageCode {
    name: "Memoni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mby",
    individual_languages: &[
    ],
};


pub const MBZ: LanguageCode = LanguageCode {
    name: "Amoltepec Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mbz",
    individual_languages: &[
    ],
};


pub const MCA: LanguageCode = LanguageCode {
    name: "Maca",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mca",
    individual_languages: &[
    ],
};


pub const MCB: LanguageCode = LanguageCode {
    name: "Machiguenga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcb",
    individual_languages: &[
    ],
};


pub const MCC: LanguageCode = LanguageCode {
    name: "Bitur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcc",
    individual_languages: &[
    ],
};


pub const MCD: LanguageCode = LanguageCode {
    name: "Sharanahua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcd",
    individual_languages: &[
    ],
};


pub const MCE: LanguageCode = LanguageCode {
    name: "Itundujia Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mce",
    individual_languages: &[
    ],
};


pub const MCF: LanguageCode = LanguageCode {
    name: "Matsés",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcf",
    individual_languages: &[
    ],
};


pub const MCG: LanguageCode = LanguageCode {
    name: "Mapoyo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcg",
    individual_languages: &[
    ],
};


pub const MCH: LanguageCode = LanguageCode {
    name: "Maquiritari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mch",
    individual_languages: &[
    ],
};


pub const MCI: LanguageCode = LanguageCode {
    name: "Mese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mci",
    individual_languages: &[
    ],
};


pub const MCJ: LanguageCode = LanguageCode {
    name: "Mvanip",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcj",
    individual_languages: &[
    ],
};


pub const MCK: LanguageCode = LanguageCode {
    name: "Mbunda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mck",
    individual_languages: &[
    ],
};


pub const MCL: LanguageCode = LanguageCode {
    name: "Macaguaje",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcl",
    individual_languages: &[
    ],
};


pub const MCM: LanguageCode = LanguageCode {
    name: "Malaccan Creole Portuguese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcm",
    individual_languages: &[
    ],
};


pub const MCN: LanguageCode = LanguageCode {
    name: "Masana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcn",
    individual_languages: &[
    ],
};


pub const MCO: LanguageCode = LanguageCode {
    name: "Coatlán Mixe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mco",
    individual_languages: &[
    ],
};


pub const MCP: LanguageCode = LanguageCode {
    name: "Makaa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcp",
    individual_languages: &[
    ],
};


pub const MCQ: LanguageCode = LanguageCode {
    name: "Ese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcq",
    individual_languages: &[
    ],
};


pub const MCR: LanguageCode = LanguageCode {
    name: "Menya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcr",
    individual_languages: &[
    ],
};


pub const MCS: LanguageCode = LanguageCode {
    name: "Mambai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcs",
    individual_languages: &[
    ],
};


pub const MCT: LanguageCode = LanguageCode {
    name: "Mengisa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mct",
    individual_languages: &[
    ],
};


pub const MCU: LanguageCode = LanguageCode {
    name: "Cameroon Mambila",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcu",
    individual_languages: &[
    ],
};


pub const MCV: LanguageCode = LanguageCode {
    name: "Minanibai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcv",
    individual_languages: &[
    ],
};


pub const MCW: LanguageCode = LanguageCode {
    name: "Mawa (Chad)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcw",
    individual_languages: &[
    ],
};


pub const MCX: LanguageCode = LanguageCode {
    name: "Mpiemo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcx",
    individual_languages: &[
    ],
};


pub const MCY: LanguageCode = LanguageCode {
    name: "South Watut",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcy",
    individual_languages: &[
    ],
};


pub const MCZ: LanguageCode = LanguageCode {
    name: "Mawan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mcz",
    individual_languages: &[
    ],
};


pub const MDA: LanguageCode = LanguageCode {
    name: "Mada (Nigeria)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mda",
    individual_languages: &[
    ],
};


pub const MDB: LanguageCode = LanguageCode {
    name: "Morigi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdb",
    individual_languages: &[
    ],
};


pub const MDC: LanguageCode = LanguageCode {
    name: "Male (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdc",
    individual_languages: &[
    ],
};


pub const MDD: LanguageCode = LanguageCode {
    name: "Mbum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdd",
    individual_languages: &[
    ],
};


pub const MDE: LanguageCode = LanguageCode {
    name: "Maba (Chad)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mde",
    individual_languages: &[
    ],
};


pub const MDG: LanguageCode = LanguageCode {
    name: "Massalat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdg",
    individual_languages: &[
    ],
};


pub const MDH: LanguageCode = LanguageCode {
    name: "Maguindanaon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdh",
    individual_languages: &[
    ],
};


pub const MDI: LanguageCode = LanguageCode {
    name: "Mamvu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdi",
    individual_languages: &[
    ],
};


pub const MDJ: LanguageCode = LanguageCode {
    name: "Mangbetu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdj",
    individual_languages: &[
    ],
};


pub const MDK: LanguageCode = LanguageCode {
    name: "Mangbutu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdk",
    individual_languages: &[
    ],
};


pub const MDL: LanguageCode = LanguageCode {
    name: "Maltese Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdl",
    individual_languages: &[
    ],
};


pub const MDM: LanguageCode = LanguageCode {
    name: "Mayogo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdm",
    individual_languages: &[
    ],
};


pub const MDN: LanguageCode = LanguageCode {
    name: "Mbati",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdn",
    individual_languages: &[
    ],
};


pub const MDP: LanguageCode = LanguageCode {
    name: "Mbala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdp",
    individual_languages: &[
    ],
};


pub const MDQ: LanguageCode = LanguageCode {
    name: "Mbole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdq",
    individual_languages: &[
    ],
};


pub const MDS: LanguageCode = LanguageCode {
    name: "Maria (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mds",
    individual_languages: &[
    ],
};


pub const MDT: LanguageCode = LanguageCode {
    name: "Mbere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdt",
    individual_languages: &[
    ],
};


pub const MDU: LanguageCode = LanguageCode {
    name: "Mboko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdu",
    individual_languages: &[
    ],
};


pub const MDV: LanguageCode = LanguageCode {
    name: "Santa Lucía Monteverde Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdv",
    individual_languages: &[
    ],
};


pub const MDW: LanguageCode = LanguageCode {
    name: "Mbosi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdw",
    individual_languages: &[
    ],
};


pub const MDX: LanguageCode = LanguageCode {
    name: "Dizin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdx",
    individual_languages: &[
    ],
};


pub const MDY: LanguageCode = LanguageCode {
    name: "Male (Ethiopia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdy",
    individual_languages: &[
    ],
};


pub const MDZ: LanguageCode = LanguageCode {
    name: "Suruí Do Pará",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mdz",
    individual_languages: &[
    ],
};


pub const MEA: LanguageCode = LanguageCode {
    name: "Menka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mea",
    individual_languages: &[
    ],
};


pub const MEB: LanguageCode = LanguageCode {
    name: "Ikobi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "meb",
    individual_languages: &[
    ],
};


pub const MEC: LanguageCode = LanguageCode {
    name: "Marra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mec",
    individual_languages: &[
    ],
};


pub const MED: LanguageCode = LanguageCode {
    name: "Melpa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "med",
    individual_languages: &[
    ],
};


pub const MEE: LanguageCode = LanguageCode {
    name: "Mengen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mee",
    individual_languages: &[
    ],
};


pub const MEF: LanguageCode = LanguageCode {
    name: "Megam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mef",
    individual_languages: &[
    ],
};


pub const MEH: LanguageCode = LanguageCode {
    name: "Southwestern Tlaxiaco Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "meh",
    individual_languages: &[
    ],
};


pub const MEI: LanguageCode = LanguageCode {
    name: "Midob",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mei",
    individual_languages: &[
    ],
};


pub const MEJ: LanguageCode = LanguageCode {
    name: "Meyah",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mej",
    individual_languages: &[
    ],
};


pub const MEK: LanguageCode = LanguageCode {
    name: "Mekeo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mek",
    individual_languages: &[
    ],
};


pub const MEL: LanguageCode = LanguageCode {
    name: "Central Melanau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mel",
    individual_languages: &[
    ],
};


pub const MEM: LanguageCode = LanguageCode {
    name: "Mangala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mem",
    individual_languages: &[
    ],
};


pub const MEO: LanguageCode = LanguageCode {
    name: "Kedah Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "meo",
    individual_languages: &[
    ],
};


pub const MEP: LanguageCode = LanguageCode {
    name: "Miriwoong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mep",
    individual_languages: &[
    ],
};


pub const MEQ: LanguageCode = LanguageCode {
    name: "Merey",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "meq",
    individual_languages: &[
    ],
};


pub const MER: LanguageCode = LanguageCode {
    name: "Meru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mer",
    individual_languages: &[
    ],
};


pub const MES: LanguageCode = LanguageCode {
    name: "Masmaje",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mes",
    individual_languages: &[
    ],
};


pub const MET: LanguageCode = LanguageCode {
    name: "Mato",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "met",
    individual_languages: &[
    ],
};


pub const MEU: LanguageCode = LanguageCode {
    name: "Motu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "meu",
    individual_languages: &[
    ],
};


pub const MEV: LanguageCode = LanguageCode {
    name: "Mano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mev",
    individual_languages: &[
    ],
};


pub const MEW: LanguageCode = LanguageCode {
    name: "Maaka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mew",
    individual_languages: &[
    ],
};


pub const MEY: LanguageCode = LanguageCode {
    name: "Hassaniyya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mey",
    individual_languages: &[
    ],
};


pub const MEZ: LanguageCode = LanguageCode {
    name: "Menominee",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mez",
    individual_languages: &[
    ],
};


pub const MFA: LanguageCode = LanguageCode {
    name: "Pattani Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfa",
    individual_languages: &[
    ],
};


pub const MFB: LanguageCode = LanguageCode {
    name: "Bangka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfb",
    individual_languages: &[
    ],
};


pub const MFC: LanguageCode = LanguageCode {
    name: "Mba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfc",
    individual_languages: &[
    ],
};


pub const MFD: LanguageCode = LanguageCode {
    name: "Mendankwe-Nkwen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfd",
    individual_languages: &[
    ],
};


pub const MFE: LanguageCode = LanguageCode {
    name: "Morisyen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfe",
    individual_languages: &[
    ],
};


pub const MFF: LanguageCode = LanguageCode {
    name: "Naki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mff",
    individual_languages: &[
    ],
};


pub const MFG: LanguageCode = LanguageCode {
    name: "Mogofin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfg",
    individual_languages: &[
    ],
};


pub const MFH: LanguageCode = LanguageCode {
    name: "Matal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfh",
    individual_languages: &[
    ],
};


pub const MFI: LanguageCode = LanguageCode {
    name: "Wandala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfi",
    individual_languages: &[
    ],
};


pub const MFJ: LanguageCode = LanguageCode {
    name: "Mefele",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfj",
    individual_languages: &[
    ],
};


pub const MFK: LanguageCode = LanguageCode {
    name: "North Mofu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfk",
    individual_languages: &[
    ],
};


pub const MFL: LanguageCode = LanguageCode {
    name: "Putai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfl",
    individual_languages: &[
    ],
};


pub const MFM: LanguageCode = LanguageCode {
    name: "Marghi South",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfm",
    individual_languages: &[
    ],
};


pub const MFN: LanguageCode = LanguageCode {
    name: "Cross River Mbembe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfn",
    individual_languages: &[
    ],
};


pub const MFO: LanguageCode = LanguageCode {
    name: "Mbe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfo",
    individual_languages: &[
    ],
};


pub const MFP: LanguageCode = LanguageCode {
    name: "Makassar Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfp",
    individual_languages: &[
    ],
};


pub const MFQ: LanguageCode = LanguageCode {
    name: "Moba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfq",
    individual_languages: &[
    ],
};


pub const MFR: LanguageCode = LanguageCode {
    name: "Marrithiyel",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfr",
    individual_languages: &[
    ],
};


pub const MFS: LanguageCode = LanguageCode {
    name: "Mexican Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfs",
    individual_languages: &[
    ],
};


pub const MFT: LanguageCode = LanguageCode {
    name: "Mokerang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mft",
    individual_languages: &[
    ],
};


pub const MFU: LanguageCode = LanguageCode {
    name: "Mbwela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfu",
    individual_languages: &[
    ],
};


pub const MFV: LanguageCode = LanguageCode {
    name: "Mandjak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfv",
    individual_languages: &[
    ],
};


pub const MFW: LanguageCode = LanguageCode {
    name: "Mulaha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfw",
    individual_languages: &[
    ],
};


pub const MFX: LanguageCode = LanguageCode {
    name: "Melo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfx",
    individual_languages: &[
    ],
};


pub const MFY: LanguageCode = LanguageCode {
    name: "Mayo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfy",
    individual_languages: &[
    ],
};


pub const MFZ: LanguageCode = LanguageCode {
    name: "Mabaan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mfz",
    individual_languages: &[
    ],
};


pub const MGB: LanguageCode = LanguageCode {
    name: "Mararit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgb",
    individual_languages: &[
    ],
};


pub const MGC: LanguageCode = LanguageCode {
    name: "Morokodo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgc",
    individual_languages: &[
    ],
};


pub const MGD: LanguageCode = LanguageCode {
    name: "Moru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgd",
    individual_languages: &[
    ],
};


pub const MGE: LanguageCode = LanguageCode {
    name: "Mango",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mge",
    individual_languages: &[
    ],
};


pub const MGF: LanguageCode = LanguageCode {
    name: "Maklew",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgf",
    individual_languages: &[
    ],
};


pub const MGG: LanguageCode = LanguageCode {
    name: "Mpumpong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgg",
    individual_languages: &[
    ],
};


pub const MGH: LanguageCode = LanguageCode {
    name: "Makhuwa-Meetto",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgh",
    individual_languages: &[
    ],
};


pub const MGI: LanguageCode = LanguageCode {
    name: "Lijili",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgi",
    individual_languages: &[
    ],
};


pub const MGJ: LanguageCode = LanguageCode {
    name: "Abureni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgj",
    individual_languages: &[
    ],
};


pub const MGK: LanguageCode = LanguageCode {
    name: "Mawes",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgk",
    individual_languages: &[
    ],
};


pub const MGL: LanguageCode = LanguageCode {
    name: "Maleu-Kilenge",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgl",
    individual_languages: &[
    ],
};


pub const MGM: LanguageCode = LanguageCode {
    name: "Mambae",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgm",
    individual_languages: &[
    ],
};


pub const MGN: LanguageCode = LanguageCode {
    name: "Mbangi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgn",
    individual_languages: &[
    ],
};


pub const MGO: LanguageCode = LanguageCode {
    name: "Meta'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgo",
    individual_languages: &[
    ],
};


pub const MGP: LanguageCode = LanguageCode {
    name: "Eastern Magar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgp",
    individual_languages: &[
    ],
};


pub const MGQ: LanguageCode = LanguageCode {
    name: "Malila",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgq",
    individual_languages: &[
    ],
};


pub const MGR: LanguageCode = LanguageCode {
    name: "Mambwe-Lungu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgr",
    individual_languages: &[
    ],
};


pub const MGS: LanguageCode = LanguageCode {
    name: "Manda (Tanzania)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgs",
    individual_languages: &[
    ],
};


pub const MGT: LanguageCode = LanguageCode {
    name: "Mongol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgt",
    individual_languages: &[
    ],
};


pub const MGU: LanguageCode = LanguageCode {
    name: "Mailu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgu",
    individual_languages: &[
    ],
};


pub const MGV: LanguageCode = LanguageCode {
    name: "Matengo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgv",
    individual_languages: &[
    ],
};


pub const MGW: LanguageCode = LanguageCode {
    name: "Matumbi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgw",
    individual_languages: &[
    ],
};


pub const MGY: LanguageCode = LanguageCode {
    name: "Mbunga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgy",
    individual_languages: &[
    ],
};


pub const MGZ: LanguageCode = LanguageCode {
    name: "Mbugwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mgz",
    individual_languages: &[
    ],
};


pub const MHA: LanguageCode = LanguageCode {
    name: "Manda (India)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mha",
    individual_languages: &[
    ],
};


pub const MHB: LanguageCode = LanguageCode {
    name: "Mahongwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhb",
    individual_languages: &[
    ],
};


pub const MHC: LanguageCode = LanguageCode {
    name: "Mocho",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhc",
    individual_languages: &[
    ],
};


pub const MHD: LanguageCode = LanguageCode {
    name: "Mbugu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhd",
    individual_languages: &[
    ],
};


pub const MHE: LanguageCode = LanguageCode {
    name: "Besisi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhe",
    individual_languages: &[
    ],
};


pub const MHF: LanguageCode = LanguageCode {
    name: "Mamaa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhf",
    individual_languages: &[
    ],
};


pub const MHG: LanguageCode = LanguageCode {
    name: "Margu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhg",
    individual_languages: &[
    ],
};


pub const MHI: LanguageCode = LanguageCode {
    name: "Ma'di",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhi",
    individual_languages: &[
    ],
};


pub const MHJ: LanguageCode = LanguageCode {
    name: "Mogholi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhj",
    individual_languages: &[
    ],
};


pub const MHK: LanguageCode = LanguageCode {
    name: "Mungaka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhk",
    individual_languages: &[
    ],
};


pub const MHL: LanguageCode = LanguageCode {
    name: "Mauwake",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhl",
    individual_languages: &[
    ],
};


pub const MHM: LanguageCode = LanguageCode {
    name: "Makhuwa-Moniga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhm",
    individual_languages: &[
    ],
};


pub const MHN: LanguageCode = LanguageCode {
    name: "Mócheno",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhn",
    individual_languages: &[
    ],
};


pub const MHO: LanguageCode = LanguageCode {
    name: "Mashi (Zambia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mho",
    individual_languages: &[
    ],
};


pub const MHP: LanguageCode = LanguageCode {
    name: "Balinese Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhp",
    individual_languages: &[
    ],
};


pub const MHQ: LanguageCode = LanguageCode {
    name: "Mandan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhq",
    individual_languages: &[
    ],
};


pub const MHR: LanguageCode = LanguageCode {
    name: "Eastern Mari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhr",
    individual_languages: &[
    ],
};


pub const MHS: LanguageCode = LanguageCode {
    name: "Buru (Indonesia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhs",
    individual_languages: &[
    ],
};


pub const MHT: LanguageCode = LanguageCode {
    name: "Mandahuaca",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mht",
    individual_languages: &[
    ],
};


pub const MHU: LanguageCode = LanguageCode {
    name: "Digaro-Mishmi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhu",
    individual_languages: &[
    ],
};


pub const MHW: LanguageCode = LanguageCode {
    name: "Mbukushu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhw",
    individual_languages: &[
    ],
};


pub const MHX: LanguageCode = LanguageCode {
    name: "Maru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhx",
    individual_languages: &[
    ],
};


pub const MHY: LanguageCode = LanguageCode {
    name: "Ma'anyan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhy",
    individual_languages: &[
    ],
};


pub const MHZ: LanguageCode = LanguageCode {
    name: "Mor (Mor Islands)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mhz",
    individual_languages: &[
    ],
};


pub const MIA: LanguageCode = LanguageCode {
    name: "Miami",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mia",
    individual_languages: &[
    ],
};


pub const MIB: LanguageCode = LanguageCode {
    name: "Atatláhuca Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mib",
    individual_languages: &[
    ],
};


pub const MID: LanguageCode = LanguageCode {
    name: "Mandaic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mid",
    individual_languages: &[
    ],
};


pub const MIE: LanguageCode = LanguageCode {
    name: "Ocotepec Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mie",
    individual_languages: &[
    ],
};


pub const MIF: LanguageCode = LanguageCode {
    name: "Mofu-Gudur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mif",
    individual_languages: &[
    ],
};


pub const MIG: LanguageCode = LanguageCode {
    name: "San Miguel El Grande Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mig",
    individual_languages: &[
    ],
};


pub const MIH: LanguageCode = LanguageCode {
    name: "Chayuco Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mih",
    individual_languages: &[
    ],
};


pub const MII: LanguageCode = LanguageCode {
    name: "Chigmecatitlán Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mii",
    individual_languages: &[
    ],
};


pub const MIJ: LanguageCode = LanguageCode {
    name: "Abar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mij",
    individual_languages: &[
    ],
};


pub const MIK: LanguageCode = LanguageCode {
    name: "Mikasuki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mik",
    individual_languages: &[
    ],
};


pub const MIL: LanguageCode = LanguageCode {
    name: "Peñoles Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mil",
    individual_languages: &[
    ],
};


pub const MIM: LanguageCode = LanguageCode {
    name: "Alacatlatzala Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mim",
    individual_languages: &[
    ],
};


pub const MIO: LanguageCode = LanguageCode {
    name: "Pinotepa Nacional Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mio",
    individual_languages: &[
    ],
};


pub const MIP: LanguageCode = LanguageCode {
    name: "Apasco-Apoala Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mip",
    individual_languages: &[
    ],
};


pub const MIQ: LanguageCode = LanguageCode {
    name: "Mískito",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "miq",
    individual_languages: &[
    ],
};


pub const MIR: LanguageCode = LanguageCode {
    name: "Isthmus Mixe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mir",
    individual_languages: &[
    ],
};


pub const MIT: LanguageCode = LanguageCode {
    name: "Southern Puebla Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mit",
    individual_languages: &[
    ],
};


pub const MIU: LanguageCode = LanguageCode {
    name: "Cacaloxtepec Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "miu",
    individual_languages: &[
    ],
};


pub const MIW: LanguageCode = LanguageCode {
    name: "Akoye",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "miw",
    individual_languages: &[
    ],
};


pub const MIX: LanguageCode = LanguageCode {
    name: "Mixtepec Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mix",
    individual_languages: &[
    ],
};


pub const MIY: LanguageCode = LanguageCode {
    name: "Ayutla Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "miy",
    individual_languages: &[
    ],
};


pub const MIZ: LanguageCode = LanguageCode {
    name: "Coatzospan Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "miz",
    individual_languages: &[
    ],
};


pub const MJB: LanguageCode = LanguageCode {
    name: "Makalero",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjb",
    individual_languages: &[
    ],
};


pub const MJC: LanguageCode = LanguageCode {
    name: "San Juan Colorado Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjc",
    individual_languages: &[
    ],
};


pub const MJD: LanguageCode = LanguageCode {
    name: "Northwest Maidu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjd",
    individual_languages: &[
    ],
};


pub const MJE: LanguageCode = LanguageCode {
    name: "Muskum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mje",
    individual_languages: &[
    ],
};


pub const MJG: LanguageCode = LanguageCode {
    name: "Tu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjg",
    individual_languages: &[
    ],
};


pub const MJH: LanguageCode = LanguageCode {
    name: "Mwera (Nyasa)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjh",
    individual_languages: &[
    ],
};


pub const MJI: LanguageCode = LanguageCode {
    name: "Kim Mun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mji",
    individual_languages: &[
    ],
};


pub const MJJ: LanguageCode = LanguageCode {
    name: "Mawak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjj",
    individual_languages: &[
    ],
};


pub const MJK: LanguageCode = LanguageCode {
    name: "Matukar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjk",
    individual_languages: &[
    ],
};


pub const MJL: LanguageCode = LanguageCode {
    name: "Mandeali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjl",
    individual_languages: &[
    ],
};


pub const MJM: LanguageCode = LanguageCode {
    name: "Medebur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjm",
    individual_languages: &[
    ],
};


pub const MJN: LanguageCode = LanguageCode {
    name: "Ma (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjn",
    individual_languages: &[
    ],
};


pub const MJO: LanguageCode = LanguageCode {
    name: "Malankuravan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjo",
    individual_languages: &[
    ],
};


pub const MJP: LanguageCode = LanguageCode {
    name: "Malapandaram",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjp",
    individual_languages: &[
    ],
};


pub const MJQ: LanguageCode = LanguageCode {
    name: "Malaryan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjq",
    individual_languages: &[
    ],
};


pub const MJR: LanguageCode = LanguageCode {
    name: "Malavedan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjr",
    individual_languages: &[
    ],
};


pub const MJS: LanguageCode = LanguageCode {
    name: "Miship",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjs",
    individual_languages: &[
    ],
};


pub const MJT: LanguageCode = LanguageCode {
    name: "Sauria Paharia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjt",
    individual_languages: &[
    ],
};


pub const MJU: LanguageCode = LanguageCode {
    name: "Manna-Dora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mju",
    individual_languages: &[
    ],
};


pub const MJV: LanguageCode = LanguageCode {
    name: "Mannan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjv",
    individual_languages: &[
    ],
};


pub const MJW: LanguageCode = LanguageCode {
    name: "Karbi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjw",
    individual_languages: &[
    ],
};


pub const MJX: LanguageCode = LanguageCode {
    name: "Mahali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjx",
    individual_languages: &[
    ],
};


pub const MJY: LanguageCode = LanguageCode {
    name: "Mahican",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjy",
    individual_languages: &[
    ],
};


pub const MJZ: LanguageCode = LanguageCode {
    name: "Majhi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mjz",
    individual_languages: &[
    ],
};


pub const MKA: LanguageCode = LanguageCode {
    name: "Mbre",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mka",
    individual_languages: &[
    ],
};


pub const MKB: LanguageCode = LanguageCode {
    name: "Mal Paharia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkb",
    individual_languages: &[
    ],
};


pub const MKC: LanguageCode = LanguageCode {
    name: "Siliput",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkc",
    individual_languages: &[
    ],
};


pub const MKE: LanguageCode = LanguageCode {
    name: "Mawchi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mke",
    individual_languages: &[
    ],
};


pub const MKF: LanguageCode = LanguageCode {
    name: "Miya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkf",
    individual_languages: &[
    ],
};


pub const MKG: LanguageCode = LanguageCode {
    name: "Mak (China)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkg",
    individual_languages: &[
    ],
};


pub const MKI: LanguageCode = LanguageCode {
    name: "Dhatki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mki",
    individual_languages: &[
    ],
};


pub const MKJ: LanguageCode = LanguageCode {
    name: "Mokilese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkj",
    individual_languages: &[
    ],
};


pub const MKK: LanguageCode = LanguageCode {
    name: "Byep",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkk",
    individual_languages: &[
    ],
};


pub const MKL: LanguageCode = LanguageCode {
    name: "Mokole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkl",
    individual_languages: &[
    ],
};


pub const MKM: LanguageCode = LanguageCode {
    name: "Moklen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkm",
    individual_languages: &[
    ],
};


pub const MKN: LanguageCode = LanguageCode {
    name: "Kupang Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkn",
    individual_languages: &[
    ],
};


pub const MKO: LanguageCode = LanguageCode {
    name: "Mingang Doso",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mko",
    individual_languages: &[
    ],
};


pub const MKP: LanguageCode = LanguageCode {
    name: "Moikodi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkp",
    individual_languages: &[
    ],
};


pub const MKQ: LanguageCode = LanguageCode {
    name: "Bay Miwok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkq",
    individual_languages: &[
    ],
};


pub const MKR: LanguageCode = LanguageCode {
    name: "Malas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkr",
    individual_languages: &[
    ],
};


pub const MKS: LanguageCode = LanguageCode {
    name: "Silacayoapan Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mks",
    individual_languages: &[
    ],
};


pub const MKT: LanguageCode = LanguageCode {
    name: "Vamale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkt",
    individual_languages: &[
    ],
};


pub const MKU: LanguageCode = LanguageCode {
    name: "Konyanka Maninka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mku",
    individual_languages: &[
    ],
};


pub const MKV: LanguageCode = LanguageCode {
    name: "Mafea",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkv",
    individual_languages: &[
    ],
};


pub const MKW: LanguageCode = LanguageCode {
    name: "Kituba (Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkw",
    individual_languages: &[
    ],
};


pub const MKX: LanguageCode = LanguageCode {
    name: "Kinamiging Manobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkx",
    individual_languages: &[
    ],
};


pub const MKY: LanguageCode = LanguageCode {
    name: "East Makian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mky",
    individual_languages: &[
    ],
};


pub const MKZ: LanguageCode = LanguageCode {
    name: "Makasae",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mkz",
    individual_languages: &[
    ],
};


pub const MLA: LanguageCode = LanguageCode {
    name: "Malo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mla",
    individual_languages: &[
    ],
};


pub const MLB: LanguageCode = LanguageCode {
    name: "Mbule",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlb",
    individual_languages: &[
    ],
};


pub const MLC: LanguageCode = LanguageCode {
    name: "Cao Lan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlc",
    individual_languages: &[
    ],
};


pub const MLE: LanguageCode = LanguageCode {
    name: "Manambu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mle",
    individual_languages: &[
    ],
};


pub const MLF: LanguageCode = LanguageCode {
    name: "Mal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlf",
    individual_languages: &[
    ],
};


pub const MLH: LanguageCode = LanguageCode {
    name: "Mape",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlh",
    individual_languages: &[
    ],
};


pub const MLI: LanguageCode = LanguageCode {
    name: "Malimpung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mli",
    individual_languages: &[
    ],
};


pub const MLJ: LanguageCode = LanguageCode {
    name: "Miltu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlj",
    individual_languages: &[
    ],
};


pub const MLK: LanguageCode = LanguageCode {
    name: "Ilwana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlk",
    individual_languages: &[
    ],
};


pub const MLL: LanguageCode = LanguageCode {
    name: "Malua Bay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mll",
    individual_languages: &[
    ],
};


pub const MLM: LanguageCode = LanguageCode {
    name: "Mulam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlm",
    individual_languages: &[
    ],
};


pub const MLN: LanguageCode = LanguageCode {
    name: "Malango",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mln",
    individual_languages: &[
    ],
};


pub const MLO: LanguageCode = LanguageCode {
    name: "Mlomp",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlo",
    individual_languages: &[
    ],
};


pub const MLP: LanguageCode = LanguageCode {
    name: "Bargam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlp",
    individual_languages: &[
    ],
};


pub const MLQ: LanguageCode = LanguageCode {
    name: "Western Maninkakan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlq",
    individual_languages: &[
    ],
};


pub const MLR: LanguageCode = LanguageCode {
    name: "Vame",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlr",
    individual_languages: &[
    ],
};


pub const MLS: LanguageCode = LanguageCode {
    name: "Masalit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mls",
    individual_languages: &[
    ],
};


pub const MLU: LanguageCode = LanguageCode {
    name: "To'abaita",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlu",
    individual_languages: &[
    ],
};


pub const MLV: LanguageCode = LanguageCode {
    name: "Motlav",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlv",
    individual_languages: &[
    ],
};


pub const MLW: LanguageCode = LanguageCode {
    name: "Moloko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlw",
    individual_languages: &[
    ],
};


pub const MLX: LanguageCode = LanguageCode {
    name: "Malfaxal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlx",
    individual_languages: &[
    ],
};


pub const MLZ: LanguageCode = LanguageCode {
    name: "Malaynon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mlz",
    individual_languages: &[
    ],
};


pub const MMA: LanguageCode = LanguageCode {
    name: "Mama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mma",
    individual_languages: &[
    ],
};


pub const MMB: LanguageCode = LanguageCode {
    name: "Momina",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmb",
    individual_languages: &[
    ],
};


pub const MMC: LanguageCode = LanguageCode {
    name: "Michoacán Mazahua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmc",
    individual_languages: &[
    ],
};


pub const MMD: LanguageCode = LanguageCode {
    name: "Maonan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmd",
    individual_languages: &[
    ],
};


pub const MME: LanguageCode = LanguageCode {
    name: "Mae",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mme",
    individual_languages: &[
    ],
};


pub const MMF: LanguageCode = LanguageCode {
    name: "Mundat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmf",
    individual_languages: &[
    ],
};


pub const MMG: LanguageCode = LanguageCode {
    name: "North Ambrym",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmg",
    individual_languages: &[
    ],
};


pub const MMH: LanguageCode = LanguageCode {
    name: "Mehináku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmh",
    individual_languages: &[
    ],
};


pub const MMI: LanguageCode = LanguageCode {
    name: "Musar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmi",
    individual_languages: &[
    ],
};


pub const MMJ: LanguageCode = LanguageCode {
    name: "Majhwar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmj",
    individual_languages: &[
    ],
};


pub const MMK: LanguageCode = LanguageCode {
    name: "Mukha-Dora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmk",
    individual_languages: &[
    ],
};


pub const MML: LanguageCode = LanguageCode {
    name: "Man Met",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mml",
    individual_languages: &[
    ],
};


pub const MMM: LanguageCode = LanguageCode {
    name: "Maii",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmm",
    individual_languages: &[
    ],
};


pub const MMN: LanguageCode = LanguageCode {
    name: "Mamanwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmn",
    individual_languages: &[
    ],
};


pub const MMO: LanguageCode = LanguageCode {
    name: "Mangga Buang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmo",
    individual_languages: &[
    ],
};


pub const MMP: LanguageCode = LanguageCode {
    name: "Siawi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmp",
    individual_languages: &[
    ],
};


pub const MMQ: LanguageCode = LanguageCode {
    name: "Musak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmq",
    individual_languages: &[
    ],
};


pub const MMR: LanguageCode = LanguageCode {
    name: "Western Xiangxi Miao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmr",
    individual_languages: &[
    ],
};


pub const MMT: LanguageCode = LanguageCode {
    name: "Malalamai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmt",
    individual_languages: &[
    ],
};


pub const MMU: LanguageCode = LanguageCode {
    name: "Mmaala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmu",
    individual_languages: &[
    ],
};


pub const MMV: LanguageCode = LanguageCode {
    name: "Miriti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmv",
    individual_languages: &[
    ],
};


pub const MMW: LanguageCode = LanguageCode {
    name: "Emae",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmw",
    individual_languages: &[
    ],
};


pub const MMX: LanguageCode = LanguageCode {
    name: "Madak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmx",
    individual_languages: &[
    ],
};


pub const MMY: LanguageCode = LanguageCode {
    name: "Migaama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmy",
    individual_languages: &[
    ],
};


pub const MMZ: LanguageCode = LanguageCode {
    name: "Mabaale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mmz",
    individual_languages: &[
    ],
};


pub const MNA: LanguageCode = LanguageCode {
    name: "Mbula",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mna",
    individual_languages: &[
    ],
};


pub const MNB: LanguageCode = LanguageCode {
    name: "Muna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnb",
    individual_languages: &[
    ],
};


pub const MND: LanguageCode = LanguageCode {
    name: "Mondé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnd",
    individual_languages: &[
    ],
};


pub const MNE: LanguageCode = LanguageCode {
    name: "Naba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mne",
    individual_languages: &[
    ],
};


pub const MNF: LanguageCode = LanguageCode {
    name: "Mundani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnf",
    individual_languages: &[
    ],
};


pub const MNG: LanguageCode = LanguageCode {
    name: "Eastern Mnong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mng",
    individual_languages: &[
    ],
};


pub const MNH: LanguageCode = LanguageCode {
    name: "Mono (Democratic Republic of Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnh",
    individual_languages: &[
    ],
};


pub const MNJ: LanguageCode = LanguageCode {
    name: "Munji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnj",
    individual_languages: &[
    ],
};


pub const MNK: LanguageCode = LanguageCode {
    name: "Mandinka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnk",
    individual_languages: &[
    ],
};


pub const MNL: LanguageCode = LanguageCode {
    name: "Tiale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnl",
    individual_languages: &[
    ],
};


pub const MNM: LanguageCode = LanguageCode {
    name: "Mapena",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnm",
    individual_languages: &[
    ],
};


pub const MNN: LanguageCode = LanguageCode {
    name: "Southern Mnong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnn",
    individual_languages: &[
    ],
};


pub const MNP: LanguageCode = LanguageCode {
    name: "Min Bei Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnp",
    individual_languages: &[
    ],
};


pub const MNQ: LanguageCode = LanguageCode {
    name: "Minriq",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnq",
    individual_languages: &[
    ],
};


pub const MNR: LanguageCode = LanguageCode {
    name: "Mono (USA)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnr",
    individual_languages: &[
    ],
};


pub const MNS: LanguageCode = LanguageCode {
    name: "Mansi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mns",
    individual_languages: &[
    ],
};


pub const MNU: LanguageCode = LanguageCode {
    name: "Mer",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnu",
    individual_languages: &[
    ],
};


pub const MNV: LanguageCode = LanguageCode {
    name: "Rennell-Bellona",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnv",
    individual_languages: &[
    ],
};


pub const MNW: LanguageCode = LanguageCode {
    name: "Mon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnw",
    individual_languages: &[
    ],
};


pub const MNX: LanguageCode = LanguageCode {
    name: "Manikion",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnx",
    individual_languages: &[
    ],
};


pub const MNY: LanguageCode = LanguageCode {
    name: "Manyawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mny",
    individual_languages: &[
    ],
};


pub const MNZ: LanguageCode = LanguageCode {
    name: "Moni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mnz",
    individual_languages: &[
    ],
};


pub const MOA: LanguageCode = LanguageCode {
    name: "Mwan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "moa",
    individual_languages: &[
    ],
};


pub const MOC: LanguageCode = LanguageCode {
    name: "Mocoví",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "moc",
    individual_languages: &[
    ],
};


pub const MOD: LanguageCode = LanguageCode {
    name: "Mobilian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mod",
    individual_languages: &[
    ],
};


pub const MOE: LanguageCode = LanguageCode {
    name: "Innu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "moe",
    individual_languages: &[
    ],
};


pub const MOG: LanguageCode = LanguageCode {
    name: "Mongondow",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mog",
    individual_languages: &[
    ],
};


pub const MOI: LanguageCode = LanguageCode {
    name: "Mboi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "moi",
    individual_languages: &[
    ],
};


pub const MOJ: LanguageCode = LanguageCode {
    name: "Monzombo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "moj",
    individual_languages: &[
    ],
};


pub const MOK: LanguageCode = LanguageCode {
    name: "Morori",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mok",
    individual_languages: &[
    ],
};


pub const MOM: LanguageCode = LanguageCode {
    name: "Mangue",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mom",
    individual_languages: &[
    ],
};


pub const MOO: LanguageCode = LanguageCode {
    name: "Monom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "moo",
    individual_languages: &[
    ],
};


pub const MOP: LanguageCode = LanguageCode {
    name: "Mopán Maya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mop",
    individual_languages: &[
    ],
};


pub const MOQ: LanguageCode = LanguageCode {
    name: "Mor (Bomberai Peninsula)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "moq",
    individual_languages: &[
    ],
};


pub const MOR: LanguageCode = LanguageCode {
    name: "Moro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mor",
    individual_languages: &[
    ],
};


pub const MOT: LanguageCode = LanguageCode {
    name: "Barí",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mot",
    individual_languages: &[
    ],
};


pub const MOU: LanguageCode = LanguageCode {
    name: "Mogum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mou",
    individual_languages: &[
    ],
};


pub const MOV: LanguageCode = LanguageCode {
    name: "Mohave",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mov",
    individual_languages: &[
    ],
};


pub const MOW: LanguageCode = LanguageCode {
    name: "Moi (Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mow",
    individual_languages: &[
    ],
};


pub const MOX: LanguageCode = LanguageCode {
    name: "Molima",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mox",
    individual_languages: &[
    ],
};


pub const MOY: LanguageCode = LanguageCode {
    name: "Shekkacho",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "moy",
    individual_languages: &[
    ],
};


pub const MOZ: LanguageCode = LanguageCode {
    name: "Mukulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "moz",
    individual_languages: &[
    ],
};


pub const MPA: LanguageCode = LanguageCode {
    name: "Mpoto",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpa",
    individual_languages: &[
    ],
};


pub const MPB: LanguageCode = LanguageCode {
    name: "Malak Malak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpb",
    individual_languages: &[
    ],
};


pub const MPC: LanguageCode = LanguageCode {
    name: "Mangarrayi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpc",
    individual_languages: &[
    ],
};


pub const MPD: LanguageCode = LanguageCode {
    name: "Machinere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpd",
    individual_languages: &[
    ],
};


pub const MPE: LanguageCode = LanguageCode {
    name: "Majang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpe",
    individual_languages: &[
    ],
};


pub const MPG: LanguageCode = LanguageCode {
    name: "Marba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpg",
    individual_languages: &[
    ],
};


pub const MPH: LanguageCode = LanguageCode {
    name: "Maung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mph",
    individual_languages: &[
    ],
};


pub const MPI: LanguageCode = LanguageCode {
    name: "Mpade",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpi",
    individual_languages: &[
    ],
};


pub const MPJ: LanguageCode = LanguageCode {
    name: "Martu Wangka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpj",
    individual_languages: &[
    ],
};


pub const MPK: LanguageCode = LanguageCode {
    name: "Mbara (Chad)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpk",
    individual_languages: &[
    ],
};


pub const MPL: LanguageCode = LanguageCode {
    name: "Middle Watut",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpl",
    individual_languages: &[
    ],
};


pub const MPM: LanguageCode = LanguageCode {
    name: "Yosondúa Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpm",
    individual_languages: &[
    ],
};


pub const MPN: LanguageCode = LanguageCode {
    name: "Mindiri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpn",
    individual_languages: &[
    ],
};


pub const MPO: LanguageCode = LanguageCode {
    name: "Miu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpo",
    individual_languages: &[
    ],
};


pub const MPP: LanguageCode = LanguageCode {
    name: "Migabac",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpp",
    individual_languages: &[
    ],
};


pub const MPQ: LanguageCode = LanguageCode {
    name: "Matís",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpq",
    individual_languages: &[
    ],
};


pub const MPR: LanguageCode = LanguageCode {
    name: "Vangunu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpr",
    individual_languages: &[
    ],
};


pub const MPS: LanguageCode = LanguageCode {
    name: "Dadibi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mps",
    individual_languages: &[
    ],
};


pub const MPT: LanguageCode = LanguageCode {
    name: "Mian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpt",
    individual_languages: &[
    ],
};


pub const MPU: LanguageCode = LanguageCode {
    name: "Makuráp",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpu",
    individual_languages: &[
    ],
};


pub const MPV: LanguageCode = LanguageCode {
    name: "Mungkip",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpv",
    individual_languages: &[
    ],
};


pub const MPW: LanguageCode = LanguageCode {
    name: "Mapidian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpw",
    individual_languages: &[
    ],
};


pub const MPX: LanguageCode = LanguageCode {
    name: "Misima-Panaeati",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpx",
    individual_languages: &[
    ],
};


pub const MPY: LanguageCode = LanguageCode {
    name: "Mapia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpy",
    individual_languages: &[
    ],
};


pub const MPZ: LanguageCode = LanguageCode {
    name: "Mpi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mpz",
    individual_languages: &[
    ],
};


pub const MQA: LanguageCode = LanguageCode {
    name: "Maba (Indonesia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqa",
    individual_languages: &[
    ],
};


pub const MQB: LanguageCode = LanguageCode {
    name: "Mbuko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqb",
    individual_languages: &[
    ],
};


pub const MQC: LanguageCode = LanguageCode {
    name: "Mangole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqc",
    individual_languages: &[
    ],
};


pub const MQE: LanguageCode = LanguageCode {
    name: "Matepi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqe",
    individual_languages: &[
    ],
};


pub const MQF: LanguageCode = LanguageCode {
    name: "Momuna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqf",
    individual_languages: &[
    ],
};


pub const MQG: LanguageCode = LanguageCode {
    name: "Kota Bangun Kutai Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqg",
    individual_languages: &[
    ],
};


pub const MQH: LanguageCode = LanguageCode {
    name: "Tlazoyaltepec Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqh",
    individual_languages: &[
    ],
};


pub const MQI: LanguageCode = LanguageCode {
    name: "Mariri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqi",
    individual_languages: &[
    ],
};


pub const MQJ: LanguageCode = LanguageCode {
    name: "Mamasa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqj",
    individual_languages: &[
    ],
};


pub const MQK: LanguageCode = LanguageCode {
    name: "Rajah Kabunsuwan Manobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqk",
    individual_languages: &[
    ],
};


pub const MQL: LanguageCode = LanguageCode {
    name: "Mbelime",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mql",
    individual_languages: &[
    ],
};


pub const MQM: LanguageCode = LanguageCode {
    name: "South Marquesan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqm",
    individual_languages: &[
    ],
};


pub const MQN: LanguageCode = LanguageCode {
    name: "Moronene",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqn",
    individual_languages: &[
    ],
};


pub const MQO: LanguageCode = LanguageCode {
    name: "Modole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqo",
    individual_languages: &[
    ],
};


pub const MQP: LanguageCode = LanguageCode {
    name: "Manipa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqp",
    individual_languages: &[
    ],
};


pub const MQQ: LanguageCode = LanguageCode {
    name: "Minokok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqq",
    individual_languages: &[
    ],
};


pub const MQR: LanguageCode = LanguageCode {
    name: "Mander",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqr",
    individual_languages: &[
    ],
};


pub const MQS: LanguageCode = LanguageCode {
    name: "West Makian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqs",
    individual_languages: &[
    ],
};


pub const MQT: LanguageCode = LanguageCode {
    name: "Mok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqt",
    individual_languages: &[
    ],
};


pub const MQU: LanguageCode = LanguageCode {
    name: "Mandari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqu",
    individual_languages: &[
    ],
};


pub const MQV: LanguageCode = LanguageCode {
    name: "Mosimo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqv",
    individual_languages: &[
    ],
};


pub const MQW: LanguageCode = LanguageCode {
    name: "Murupi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqw",
    individual_languages: &[
    ],
};


pub const MQX: LanguageCode = LanguageCode {
    name: "Mamuju",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqx",
    individual_languages: &[
    ],
};


pub const MQY: LanguageCode = LanguageCode {
    name: "Manggarai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqy",
    individual_languages: &[
    ],
};


pub const MQZ: LanguageCode = LanguageCode {
    name: "Pano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mqz",
    individual_languages: &[
    ],
};


pub const MRA: LanguageCode = LanguageCode {
    name: "Mlabri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mra",
    individual_languages: &[
    ],
};


pub const MRB: LanguageCode = LanguageCode {
    name: "Marino",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrb",
    individual_languages: &[
    ],
};


pub const MRC: LanguageCode = LanguageCode {
    name: "Maricopa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrc",
    individual_languages: &[
    ],
};


pub const MRD: LanguageCode = LanguageCode {
    name: "Western Magar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrd",
    individual_languages: &[
    ],
};


pub const MRE: LanguageCode = LanguageCode {
    name: "Martha's Vineyard Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mre",
    individual_languages: &[
    ],
};


pub const MRF: LanguageCode = LanguageCode {
    name: "Elseng",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrf",
    individual_languages: &[
    ],
};


pub const MRG: LanguageCode = LanguageCode {
    name: "Mising",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrg",
    individual_languages: &[
    ],
};


pub const MRH: LanguageCode = LanguageCode {
    name: "Mara Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrh",
    individual_languages: &[
    ],
};


pub const MRJ: LanguageCode = LanguageCode {
    name: "Western Mari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrj",
    individual_languages: &[
    ],
};


pub const MRK: LanguageCode = LanguageCode {
    name: "Hmwaveke",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrk",
    individual_languages: &[
    ],
};


pub const MRL: LanguageCode = LanguageCode {
    name: "Mortlockese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrl",
    individual_languages: &[
    ],
};


pub const MRM: LanguageCode = LanguageCode {
    name: "Merlav",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrm",
    individual_languages: &[
    ],
};


pub const MRN: LanguageCode = LanguageCode {
    name: "Cheke Holo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrn",
    individual_languages: &[
    ],
};


pub const MRO: LanguageCode = LanguageCode {
    name: "Mru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mro",
    individual_languages: &[
    ],
};


pub const MRP: LanguageCode = LanguageCode {
    name: "Morouas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrp",
    individual_languages: &[
    ],
};


pub const MRQ: LanguageCode = LanguageCode {
    name: "North Marquesan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrq",
    individual_languages: &[
    ],
};


pub const MRR: LanguageCode = LanguageCode {
    name: "Maria (India)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrr",
    individual_languages: &[
    ],
};


pub const MRS: LanguageCode = LanguageCode {
    name: "Maragus",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrs",
    individual_languages: &[
    ],
};


pub const MRT: LanguageCode = LanguageCode {
    name: "Marghi Central",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrt",
    individual_languages: &[
    ],
};


pub const MRU: LanguageCode = LanguageCode {
    name: "Mono (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mru",
    individual_languages: &[
    ],
};


pub const MRV: LanguageCode = LanguageCode {
    name: "Mangareva",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrv",
    individual_languages: &[
    ],
};


pub const MRW: LanguageCode = LanguageCode {
    name: "Maranao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrw",
    individual_languages: &[
    ],
};


pub const MRX: LanguageCode = LanguageCode {
    name: "Maremgi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrx",
    individual_languages: &[
    ],
};


pub const MRY: LanguageCode = LanguageCode {
    name: "Mandaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mry",
    individual_languages: &[
    ],
};


pub const MRZ: LanguageCode = LanguageCode {
    name: "Marind",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mrz",
    individual_languages: &[
    ],
};


pub const MSB: LanguageCode = LanguageCode {
    name: "Masbatenyo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msb",
    individual_languages: &[
    ],
};


pub const MSC: LanguageCode = LanguageCode {
    name: "Sankaran Maninka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msc",
    individual_languages: &[
    ],
};


pub const MSD: LanguageCode = LanguageCode {
    name: "Yucatec Maya Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msd",
    individual_languages: &[
    ],
};


pub const MSE: LanguageCode = LanguageCode {
    name: "Musey",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mse",
    individual_languages: &[
    ],
};


pub const MSF: LanguageCode = LanguageCode {
    name: "Mekwei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msf",
    individual_languages: &[
    ],
};


pub const MSG: LanguageCode = LanguageCode {
    name: "Moraid",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msg",
    individual_languages: &[
    ],
};


pub const MSH: LanguageCode = LanguageCode {
    name: "Masikoro Malagasy",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msh",
    individual_languages: &[
    ],
};


pub const MSI: LanguageCode = LanguageCode {
    name: "Sabah Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msi",
    individual_languages: &[
    ],
};


pub const MSJ: LanguageCode = LanguageCode {
    name: "Ma (Democratic Republic of Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msj",
    individual_languages: &[
    ],
};


pub const MSK: LanguageCode = LanguageCode {
    name: "Mansaka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msk",
    individual_languages: &[
    ],
};


pub const MSL: LanguageCode = LanguageCode {
    name: "Molof",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msl",
    individual_languages: &[
    ],
};


pub const MSM: LanguageCode = LanguageCode {
    name: "Agusan Manobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msm",
    individual_languages: &[
    ],
};


pub const MSN: LanguageCode = LanguageCode {
    name: "Vurës",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msn",
    individual_languages: &[
    ],
};


pub const MSO: LanguageCode = LanguageCode {
    name: "Mombum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mso",
    individual_languages: &[
    ],
};


pub const MSP: LanguageCode = LanguageCode {
    name: "Maritsauá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msp",
    individual_languages: &[
    ],
};


pub const MSQ: LanguageCode = LanguageCode {
    name: "Caac",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msq",
    individual_languages: &[
    ],
};


pub const MSR: LanguageCode = LanguageCode {
    name: "Mongolian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msr",
    individual_languages: &[
    ],
};


pub const MSS: LanguageCode = LanguageCode {
    name: "West Masela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mss",
    individual_languages: &[
    ],
};


pub const MSU: LanguageCode = LanguageCode {
    name: "Musom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msu",
    individual_languages: &[
    ],
};


pub const MSV: LanguageCode = LanguageCode {
    name: "Maslam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msv",
    individual_languages: &[
    ],
};


pub const MSW: LanguageCode = LanguageCode {
    name: "Mansoanka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msw",
    individual_languages: &[
    ],
};


pub const MSX: LanguageCode = LanguageCode {
    name: "Moresada",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msx",
    individual_languages: &[
    ],
};


pub const MSY: LanguageCode = LanguageCode {
    name: "Aruamu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msy",
    individual_languages: &[
    ],
};


pub const MSZ: LanguageCode = LanguageCode {
    name: "Momare",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "msz",
    individual_languages: &[
    ],
};


pub const MTA: LanguageCode = LanguageCode {
    name: "Cotabato Manobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mta",
    individual_languages: &[
    ],
};


pub const MTB: LanguageCode = LanguageCode {
    name: "Anyin Morofo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtb",
    individual_languages: &[
    ],
};


pub const MTC: LanguageCode = LanguageCode {
    name: "Munit",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtc",
    individual_languages: &[
    ],
};


pub const MTD: LanguageCode = LanguageCode {
    name: "Mualang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtd",
    individual_languages: &[
    ],
};


pub const MTE: LanguageCode = LanguageCode {
    name: "Mono (Solomon Islands)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mte",
    individual_languages: &[
    ],
};


pub const MTF: LanguageCode = LanguageCode {
    name: "Murik (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtf",
    individual_languages: &[
    ],
};


pub const MTG: LanguageCode = LanguageCode {
    name: "Una",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtg",
    individual_languages: &[
    ],
};


pub const MTH: LanguageCode = LanguageCode {
    name: "Munggui",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mth",
    individual_languages: &[
    ],
};


pub const MTI: LanguageCode = LanguageCode {
    name: "Maiwa (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mti",
    individual_languages: &[
    ],
};


pub const MTJ: LanguageCode = LanguageCode {
    name: "Moskona",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtj",
    individual_languages: &[
    ],
};


pub const MTK: LanguageCode = LanguageCode {
    name: "Mbe'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtk",
    individual_languages: &[
    ],
};


pub const MTL: LanguageCode = LanguageCode {
    name: "Montol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtl",
    individual_languages: &[
    ],
};


pub const MTM: LanguageCode = LanguageCode {
    name: "Mator",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtm",
    individual_languages: &[
    ],
};


pub const MTN: LanguageCode = LanguageCode {
    name: "Matagalpa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtn",
    individual_languages: &[
    ],
};


pub const MTO: LanguageCode = LanguageCode {
    name: "Totontepec Mixe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mto",
    individual_languages: &[
    ],
};


pub const MTP: LanguageCode = LanguageCode {
    name: "Wichí Lhamtés Nocten",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtp",
    individual_languages: &[
    ],
};


pub const MTQ: LanguageCode = LanguageCode {
    name: "Muong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtq",
    individual_languages: &[
    ],
};


pub const MTR: LanguageCode = LanguageCode {
    name: "Mewari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtr",
    individual_languages: &[
    ],
};


pub const MTS: LanguageCode = LanguageCode {
    name: "Yora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mts",
    individual_languages: &[
    ],
};


pub const MTT: LanguageCode = LanguageCode {
    name: "Mota",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtt",
    individual_languages: &[
    ],
};


pub const MTU: LanguageCode = LanguageCode {
    name: "Tututepec Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtu",
    individual_languages: &[
    ],
};


pub const MTV: LanguageCode = LanguageCode {
    name: "Asaro'o",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtv",
    individual_languages: &[
    ],
};


pub const MTW: LanguageCode = LanguageCode {
    name: "Southern Binukidnon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtw",
    individual_languages: &[
    ],
};


pub const MTX: LanguageCode = LanguageCode {
    name: "Tidaá Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mtx",
    individual_languages: &[
    ],
};


pub const MTY: LanguageCode = LanguageCode {
    name: "Nabi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mty",
    individual_languages: &[
    ],
};


pub const MUA: LanguageCode = LanguageCode {
    name: "Mundang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mua",
    individual_languages: &[
    ],
};


pub const MUB: LanguageCode = LanguageCode {
    name: "Mubi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mub",
    individual_languages: &[
    ],
};


pub const MUC: LanguageCode = LanguageCode {
    name: "Ajumbu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "muc",
    individual_languages: &[
    ],
};


pub const MUD: LanguageCode = LanguageCode {
    name: "Mednyj Aleut",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mud",
    individual_languages: &[
    ],
};


pub const MUE: LanguageCode = LanguageCode {
    name: "Media Lengua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mue",
    individual_languages: &[
    ],
};


pub const MUG: LanguageCode = LanguageCode {
    name: "Musgu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mug",
    individual_languages: &[
    ],
};


pub const MUH: LanguageCode = LanguageCode {
    name: "Mündü",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "muh",
    individual_languages: &[
    ],
};


pub const MUI: LanguageCode = LanguageCode {
    name: "Musi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mui",
    individual_languages: &[
    ],
};


pub const MUJ: LanguageCode = LanguageCode {
    name: "Mabire",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "muj",
    individual_languages: &[
    ],
};


pub const MUK: LanguageCode = LanguageCode {
    name: "Mugom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "muk",
    individual_languages: &[
    ],
};


pub const MUM: LanguageCode = LanguageCode {
    name: "Maiwala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mum",
    individual_languages: &[
    ],
};


pub const MUO: LanguageCode = LanguageCode {
    name: "Nyong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "muo",
    individual_languages: &[
    ],
};


pub const MUP: LanguageCode = LanguageCode {
    name: "Malvi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mup",
    individual_languages: &[
    ],
};


pub const MUQ: LanguageCode = LanguageCode {
    name: "Eastern Xiangxi Miao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "muq",
    individual_languages: &[
    ],
};


pub const MUR: LanguageCode = LanguageCode {
    name: "Murle",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mur",
    individual_languages: &[
    ],
};


pub const MUT: LanguageCode = LanguageCode {
    name: "Western Muria",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mut",
    individual_languages: &[
    ],
};


pub const MUU: LanguageCode = LanguageCode {
    name: "Yaaku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "muu",
    individual_languages: &[
    ],
};


pub const MUV: LanguageCode = LanguageCode {
    name: "Muthuvan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "muv",
    individual_languages: &[
    ],
};


pub const MUX: LanguageCode = LanguageCode {
    name: "Bo-Ung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mux",
    individual_languages: &[
    ],
};


pub const MUY: LanguageCode = LanguageCode {
    name: "Muyang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "muy",
    individual_languages: &[
    ],
};


pub const MUZ: LanguageCode = LanguageCode {
    name: "Mursi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "muz",
    individual_languages: &[
    ],
};


pub const MVA: LanguageCode = LanguageCode {
    name: "Manam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mva",
    individual_languages: &[
    ],
};


pub const MVB: LanguageCode = LanguageCode {
    name: "Mattole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvb",
    individual_languages: &[
    ],
};


pub const MVD: LanguageCode = LanguageCode {
    name: "Mamboru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvd",
    individual_languages: &[
    ],
};


pub const MVE: LanguageCode = LanguageCode {
    name: "Marwari (Pakistan)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mve",
    individual_languages: &[
    ],
};


pub const MVF: LanguageCode = LanguageCode {
    name: "Peripheral Mongolian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvf",
    individual_languages: &[
    ],
};


pub const MVG: LanguageCode = LanguageCode {
    name: "Yucuañe Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvg",
    individual_languages: &[
    ],
};


pub const MVH: LanguageCode = LanguageCode {
    name: "Mulgi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvh",
    individual_languages: &[
    ],
};


pub const MVI: LanguageCode = LanguageCode {
    name: "Miyako",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvi",
    individual_languages: &[
    ],
};


pub const MVK: LanguageCode = LanguageCode {
    name: "Mekmek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvk",
    individual_languages: &[
    ],
};


pub const MVL: LanguageCode = LanguageCode {
    name: "Mbara (Australia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvl",
    individual_languages: &[
    ],
};


pub const MVN: LanguageCode = LanguageCode {
    name: "Minaveha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvn",
    individual_languages: &[
    ],
};


pub const MVO: LanguageCode = LanguageCode {
    name: "Marovo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvo",
    individual_languages: &[
    ],
};


pub const MVP: LanguageCode = LanguageCode {
    name: "Duri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvp",
    individual_languages: &[
    ],
};


pub const MVQ: LanguageCode = LanguageCode {
    name: "Moere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvq",
    individual_languages: &[
    ],
};


pub const MVR: LanguageCode = LanguageCode {
    name: "Marau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvr",
    individual_languages: &[
    ],
};


pub const MVS: LanguageCode = LanguageCode {
    name: "Massep",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvs",
    individual_languages: &[
    ],
};


pub const MVT: LanguageCode = LanguageCode {
    name: "Mpotovoro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvt",
    individual_languages: &[
    ],
};


pub const MVU: LanguageCode = LanguageCode {
    name: "Marfa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvu",
    individual_languages: &[
    ],
};


pub const MVV: LanguageCode = LanguageCode {
    name: "Tagal Murut",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvv",
    individual_languages: &[
    ],
};


pub const MVW: LanguageCode = LanguageCode {
    name: "Machinga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvw",
    individual_languages: &[
    ],
};


pub const MVX: LanguageCode = LanguageCode {
    name: "Meoswar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvx",
    individual_languages: &[
    ],
};


pub const MVY: LanguageCode = LanguageCode {
    name: "Indus Kohistani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvy",
    individual_languages: &[
    ],
};


pub const MVZ: LanguageCode = LanguageCode {
    name: "Mesqan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mvz",
    individual_languages: &[
    ],
};


pub const MWA: LanguageCode = LanguageCode {
    name: "Mwatebu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwa",
    individual_languages: &[
    ],
};


pub const MWB: LanguageCode = LanguageCode {
    name: "Juwal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwb",
    individual_languages: &[
    ],
};


pub const MWC: LanguageCode = LanguageCode {
    name: "Are",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwc",
    individual_languages: &[
    ],
};


pub const MWE: LanguageCode = LanguageCode {
    name: "Mwera (Chimwera)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwe",
    individual_languages: &[
    ],
};


pub const MWF: LanguageCode = LanguageCode {
    name: "Murrinh-Patha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwf",
    individual_languages: &[
    ],
};


pub const MWG: LanguageCode = LanguageCode {
    name: "Aiklep",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwg",
    individual_languages: &[
    ],
};


pub const MWH: LanguageCode = LanguageCode {
    name: "Mouk-Aria",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwh",
    individual_languages: &[
    ],
};


pub const MWI: LanguageCode = LanguageCode {
    name: "Labo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwi",
    individual_languages: &[
    ],
};


pub const MWK: LanguageCode = LanguageCode {
    name: "Kita Maninkakan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwk",
    individual_languages: &[
    ],
};


pub const MWM: LanguageCode = LanguageCode {
    name: "Sar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwm",
    individual_languages: &[
    ],
};


pub const MWN: LanguageCode = LanguageCode {
    name: "Nyamwanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwn",
    individual_languages: &[
    ],
};


pub const MWO: LanguageCode = LanguageCode {
    name: "Central Maewo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwo",
    individual_languages: &[
    ],
};


pub const MWP: LanguageCode = LanguageCode {
    name: "Kala Lagaw Ya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwp",
    individual_languages: &[
    ],
};


pub const MWQ: LanguageCode = LanguageCode {
    name: "Mün Chin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwq",
    individual_languages: &[
    ],
};


pub const MWS: LanguageCode = LanguageCode {
    name: "Mwimbi-Muthambi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mws",
    individual_languages: &[
    ],
};


pub const MWT: LanguageCode = LanguageCode {
    name: "Moken",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwt",
    individual_languages: &[
    ],
};


pub const MWU: LanguageCode = LanguageCode {
    name: "Mittu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwu",
    individual_languages: &[
    ],
};


pub const MWV: LanguageCode = LanguageCode {
    name: "Mentawai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwv",
    individual_languages: &[
    ],
};


pub const MWW: LanguageCode = LanguageCode {
    name: "Hmong Daw",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mww",
    individual_languages: &[
    ],
};


pub const MWZ: LanguageCode = LanguageCode {
    name: "Moingi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mwz",
    individual_languages: &[
    ],
};


pub const MXA: LanguageCode = LanguageCode {
    name: "Northwest Oaxaca Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxa",
    individual_languages: &[
    ],
};


pub const MXB: LanguageCode = LanguageCode {
    name: "Tezoatlán Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxb",
    individual_languages: &[
    ],
};


pub const MXC: LanguageCode = LanguageCode {
    name: "Manyika",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxc",
    individual_languages: &[
    ],
};


pub const MXD: LanguageCode = LanguageCode {
    name: "Modang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxd",
    individual_languages: &[
    ],
};


pub const MXE: LanguageCode = LanguageCode {
    name: "Mele-Fila",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxe",
    individual_languages: &[
    ],
};


pub const MXF: LanguageCode = LanguageCode {
    name: "Malgbe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxf",
    individual_languages: &[
    ],
};


pub const MXG: LanguageCode = LanguageCode {
    name: "Mbangala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxg",
    individual_languages: &[
    ],
};


pub const MXH: LanguageCode = LanguageCode {
    name: "Mvuba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxh",
    individual_languages: &[
    ],
};


pub const MXI: LanguageCode = LanguageCode {
    name: "Mozarabic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxi",
    individual_languages: &[
    ],
};


pub const MXJ: LanguageCode = LanguageCode {
    name: "Miju-Mishmi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxj",
    individual_languages: &[
    ],
};


pub const MXK: LanguageCode = LanguageCode {
    name: "Monumbo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxk",
    individual_languages: &[
    ],
};


pub const MXL: LanguageCode = LanguageCode {
    name: "Maxi Gbe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxl",
    individual_languages: &[
    ],
};


pub const MXM: LanguageCode = LanguageCode {
    name: "Meramera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxm",
    individual_languages: &[
    ],
};


pub const MXN: LanguageCode = LanguageCode {
    name: "Moi (Indonesia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxn",
    individual_languages: &[
    ],
};


pub const MXO: LanguageCode = LanguageCode {
    name: "Mbowe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxo",
    individual_languages: &[
    ],
};


pub const MXP: LanguageCode = LanguageCode {
    name: "Tlahuitoltepec Mixe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxp",
    individual_languages: &[
    ],
};


pub const MXQ: LanguageCode = LanguageCode {
    name: "Juquila Mixe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxq",
    individual_languages: &[
    ],
};


pub const MXR: LanguageCode = LanguageCode {
    name: "Murik (Malaysia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxr",
    individual_languages: &[
    ],
};


pub const MXS: LanguageCode = LanguageCode {
    name: "Huitepec Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxs",
    individual_languages: &[
    ],
};


pub const MXT: LanguageCode = LanguageCode {
    name: "Jamiltepec Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxt",
    individual_languages: &[
    ],
};


pub const MXU: LanguageCode = LanguageCode {
    name: "Mada (Cameroon)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxu",
    individual_languages: &[
    ],
};


pub const MXV: LanguageCode = LanguageCode {
    name: "Metlatónoc Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxv",
    individual_languages: &[
    ],
};


pub const MXW: LanguageCode = LanguageCode {
    name: "Namo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxw",
    individual_languages: &[
    ],
};


pub const MXX: LanguageCode = LanguageCode {
    name: "Mahou",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxx",
    individual_languages: &[
    ],
};


pub const MXY: LanguageCode = LanguageCode {
    name: "Southeastern Nochixtlán Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxy",
    individual_languages: &[
    ],
};


pub const MXZ: LanguageCode = LanguageCode {
    name: "Central Masela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mxz",
    individual_languages: &[
    ],
};


pub const MYB: LanguageCode = LanguageCode {
    name: "Mbay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myb",
    individual_languages: &[
    ],
};


pub const MYC: LanguageCode = LanguageCode {
    name: "Mayeka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myc",
    individual_languages: &[
    ],
};


pub const MYE: LanguageCode = LanguageCode {
    name: "Myene",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mye",
    individual_languages: &[
    ],
};


pub const MYF: LanguageCode = LanguageCode {
    name: "Bambassi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myf",
    individual_languages: &[
    ],
};


pub const MYG: LanguageCode = LanguageCode {
    name: "Manta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myg",
    individual_languages: &[
    ],
};


pub const MYH: LanguageCode = LanguageCode {
    name: "Makah",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myh",
    individual_languages: &[
    ],
};


pub const MYJ: LanguageCode = LanguageCode {
    name: "Mangayat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myj",
    individual_languages: &[
    ],
};


pub const MYK: LanguageCode = LanguageCode {
    name: "Mamara Senoufo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myk",
    individual_languages: &[
    ],
};


pub const MYL: LanguageCode = LanguageCode {
    name: "Moma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myl",
    individual_languages: &[
    ],
};


pub const MYM: LanguageCode = LanguageCode {
    name: "Me'en",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mym",
    individual_languages: &[
    ],
};


pub const MYO: LanguageCode = LanguageCode {
    name: "Anfillo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myo",
    individual_languages: &[
    ],
};


pub const MYP: LanguageCode = LanguageCode {
    name: "Pirahã",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myp",
    individual_languages: &[
    ],
};


pub const MYR: LanguageCode = LanguageCode {
    name: "Muniche",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myr",
    individual_languages: &[
    ],
};


pub const MYS: LanguageCode = LanguageCode {
    name: "Mesmes",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mys",
    individual_languages: &[
    ],
};


pub const MYU: LanguageCode = LanguageCode {
    name: "Mundurukú",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myu",
    individual_languages: &[
    ],
};


pub const MYW: LanguageCode = LanguageCode {
    name: "Muyuw",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myw",
    individual_languages: &[
    ],
};


pub const MYX: LanguageCode = LanguageCode {
    name: "Masaaba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myx",
    individual_languages: &[
    ],
};


pub const MYY: LanguageCode = LanguageCode {
    name: "Macuna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myy",
    individual_languages: &[
    ],
};


pub const MYZ: LanguageCode = LanguageCode {
    name: "Classical Mandaic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "myz",
    individual_languages: &[
    ],
};


pub const MZA: LanguageCode = LanguageCode {
    name: "Santa María Zacatepec Mixtec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mza",
    individual_languages: &[
    ],
};


pub const MZB: LanguageCode = LanguageCode {
    name: "Tumzabt",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzb",
    individual_languages: &[
    ],
};


pub const MZC: LanguageCode = LanguageCode {
    name: "Madagascar Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzc",
    individual_languages: &[
    ],
};


pub const MZD: LanguageCode = LanguageCode {
    name: "Malimba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzd",
    individual_languages: &[
    ],
};


pub const MZE: LanguageCode = LanguageCode {
    name: "Morawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mze",
    individual_languages: &[
    ],
};


pub const MZG: LanguageCode = LanguageCode {
    name: "Monastic Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzg",
    individual_languages: &[
    ],
};


pub const MZH: LanguageCode = LanguageCode {
    name: "Wichí Lhamtés Güisnay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzh",
    individual_languages: &[
    ],
};


pub const MZI: LanguageCode = LanguageCode {
    name: "Ixcatlán Mazatec",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzi",
    individual_languages: &[
    ],
};


pub const MZJ: LanguageCode = LanguageCode {
    name: "Manya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzj",
    individual_languages: &[
    ],
};


pub const MZK: LanguageCode = LanguageCode {
    name: "Nigeria Mambila",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzk",
    individual_languages: &[
    ],
};


pub const MZL: LanguageCode = LanguageCode {
    name: "Mazatlán Mixe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzl",
    individual_languages: &[
    ],
};


pub const MZM: LanguageCode = LanguageCode {
    name: "Mumuye",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzm",
    individual_languages: &[
    ],
};


pub const MZN: LanguageCode = LanguageCode {
    name: "Mazanderani",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzn",
    individual_languages: &[
    ],
};


pub const MZO: LanguageCode = LanguageCode {
    name: "Matipuhy",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzo",
    individual_languages: &[
    ],
};


pub const MZP: LanguageCode = LanguageCode {
    name: "Movima",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzp",
    individual_languages: &[
    ],
};


pub const MZQ: LanguageCode = LanguageCode {
    name: "Mori Atas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzq",
    individual_languages: &[
    ],
};


pub const MZR: LanguageCode = LanguageCode {
    name: "Marúbo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzr",
    individual_languages: &[
    ],
};


pub const MZS: LanguageCode = LanguageCode {
    name: "Macanese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzs",
    individual_languages: &[
    ],
};


pub const MZT: LanguageCode = LanguageCode {
    name: "Mintil",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzt",
    individual_languages: &[
    ],
};


pub const MZU: LanguageCode = LanguageCode {
    name: "Inapang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzu",
    individual_languages: &[
    ],
};


pub const MZV: LanguageCode = LanguageCode {
    name: "Manza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzv",
    individual_languages: &[
    ],
};


pub const MZW: LanguageCode = LanguageCode {
    name: "Deg",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzw",
    individual_languages: &[
    ],
};


pub const MZX: LanguageCode = LanguageCode {
    name: "Mawayana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzx",
    individual_languages: &[
    ],
};


pub const MZY: LanguageCode = LanguageCode {
    name: "Mozambican Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzy",
    individual_languages: &[
    ],
};


pub const MZZ: LanguageCode = LanguageCode {
    name: "Maiadomu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "mzz",
    individual_languages: &[
    ],
};


pub const NAA: LanguageCode = LanguageCode {
    name: "Namla",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "naa",
    individual_languages: &[
    ],
};


pub const NAB: LanguageCode = LanguageCode {
    name: "Southern Nambikuára",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nab",
    individual_languages: &[
    ],
};


pub const NAC: LanguageCode = LanguageCode {
    name: "Narak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nac",
    individual_languages: &[
    ],
};


pub const NAE: LanguageCode = LanguageCode {
    name: "Naka'ela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nae",
    individual_languages: &[
    ],
};


pub const NAF: LanguageCode = LanguageCode {
    name: "Nabak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "naf",
    individual_languages: &[
    ],
};


pub const NAG: LanguageCode = LanguageCode {
    name: "Naga Pidgin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nag",
    individual_languages: &[
    ],
};


pub const NAJ: LanguageCode = LanguageCode {
    name: "Nalu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "naj",
    individual_languages: &[
    ],
};


pub const NAK: LanguageCode = LanguageCode {
    name: "Nakanai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nak",
    individual_languages: &[
    ],
};


pub const NAL: LanguageCode = LanguageCode {
    name: "Nalik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nal",
    individual_languages: &[
    ],
};


pub const NAM: LanguageCode = LanguageCode {
    name: "Ngan'gityemerri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nam",
    individual_languages: &[
    ],
};


pub const NAN: LanguageCode = LanguageCode {
    name: "Min Nan Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nan",
    individual_languages: &[
    ],
};


pub const NAO: LanguageCode = LanguageCode {
    name: "Naaba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nao",
    individual_languages: &[
    ],
};


pub const NAQ: LanguageCode = LanguageCode {
    name: "Khoekhoe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "naq",
    individual_languages: &[
    ],
};


pub const NAR: LanguageCode = LanguageCode {
    name: "Iguta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nar",
    individual_languages: &[
    ],
};


pub const NAS: LanguageCode = LanguageCode {
    name: "Naasioi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nas",
    individual_languages: &[
    ],
};


pub const NAT: LanguageCode = LanguageCode {
    name: "Ca̱hungwa̱rya̱",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nat",
    individual_languages: &[
    ],
};


pub const NAW: LanguageCode = LanguageCode {
    name: "Nawuri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "naw",
    individual_languages: &[
    ],
};


pub const NAX: LanguageCode = LanguageCode {
    name: "Nakwi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nax",
    individual_languages: &[
    ],
};


pub const NAY: LanguageCode = LanguageCode {
    name: "Ngarrindjeri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nay",
    individual_languages: &[
    ],
};


pub const NAZ: LanguageCode = LanguageCode {
    name: "Coatepec Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "naz",
    individual_languages: &[
    ],
};


pub const NBA: LanguageCode = LanguageCode {
    name: "Nyemba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nba",
    individual_languages: &[
    ],
};


pub const NBB: LanguageCode = LanguageCode {
    name: "Ndoe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbb",
    individual_languages: &[
    ],
};


pub const NBC: LanguageCode = LanguageCode {
    name: "Chang Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbc",
    individual_languages: &[
    ],
};


pub const NBD: LanguageCode = LanguageCode {
    name: "Ngbinda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbd",
    individual_languages: &[
    ],
};


pub const NBE: LanguageCode = LanguageCode {
    name: "Konyak Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbe",
    individual_languages: &[
    ],
};


pub const NBG: LanguageCode = LanguageCode {
    name: "Nagarchal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbg",
    individual_languages: &[
    ],
};


pub const NBH: LanguageCode = LanguageCode {
    name: "Ngamo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbh",
    individual_languages: &[
    ],
};


pub const NBI: LanguageCode = LanguageCode {
    name: "Mao Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbi",
    individual_languages: &[
    ],
};


pub const NBJ: LanguageCode = LanguageCode {
    name: "Ngarinyman",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbj",
    individual_languages: &[
    ],
};


pub const NBK: LanguageCode = LanguageCode {
    name: "Nake",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbk",
    individual_languages: &[
    ],
};


pub const NBM: LanguageCode = LanguageCode {
    name: "Ngbaka Ma'bo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbm",
    individual_languages: &[
    ],
};


pub const NBN: LanguageCode = LanguageCode {
    name: "Kuri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbn",
    individual_languages: &[
    ],
};


pub const NBO: LanguageCode = LanguageCode {
    name: "Nkukoli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbo",
    individual_languages: &[
    ],
};


pub const NBP: LanguageCode = LanguageCode {
    name: "Nnam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbp",
    individual_languages: &[
    ],
};


pub const NBQ: LanguageCode = LanguageCode {
    name: "Nggem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbq",
    individual_languages: &[
    ],
};


pub const NBR: LanguageCode = LanguageCode {
    name: "Numana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbr",
    individual_languages: &[
    ],
};


pub const NBS: LanguageCode = LanguageCode {
    name: "Namibian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbs",
    individual_languages: &[
    ],
};


pub const NBT: LanguageCode = LanguageCode {
    name: "Na",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbt",
    individual_languages: &[
    ],
};


pub const NBU: LanguageCode = LanguageCode {
    name: "Rongmei Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbu",
    individual_languages: &[
    ],
};


pub const NBV: LanguageCode = LanguageCode {
    name: "Ngamambo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbv",
    individual_languages: &[
    ],
};


pub const NBW: LanguageCode = LanguageCode {
    name: "Southern Ngbandi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nbw",
    individual_languages: &[
    ],
};


pub const NBY: LanguageCode = LanguageCode {
    name: "Ningera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nby",
    individual_languages: &[
    ],
};


pub const NCA: LanguageCode = LanguageCode {
    name: "Iyo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nca",
    individual_languages: &[
    ],
};


pub const NCB: LanguageCode = LanguageCode {
    name: "Central Nicobarese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncb",
    individual_languages: &[
    ],
};


pub const NCC: LanguageCode = LanguageCode {
    name: "Ponam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncc",
    individual_languages: &[
    ],
};


pub const NCD: LanguageCode = LanguageCode {
    name: "Nachering",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncd",
    individual_languages: &[
    ],
};


pub const NCE: LanguageCode = LanguageCode {
    name: "Yale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nce",
    individual_languages: &[
    ],
};


pub const NCF: LanguageCode = LanguageCode {
    name: "Notsi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncf",
    individual_languages: &[
    ],
};


pub const NCG: LanguageCode = LanguageCode {
    name: "Nisga'a",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncg",
    individual_languages: &[
    ],
};


pub const NCH: LanguageCode = LanguageCode {
    name: "Central Huasteca Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nch",
    individual_languages: &[
    ],
};


pub const NCI: LanguageCode = LanguageCode {
    name: "Classical Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nci",
    individual_languages: &[
    ],
};


pub const NCJ: LanguageCode = LanguageCode {
    name: "Northern Puebla Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncj",
    individual_languages: &[
    ],
};


pub const NCK: LanguageCode = LanguageCode {
    name: "Na-kara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nck",
    individual_languages: &[
    ],
};


pub const NCL: LanguageCode = LanguageCode {
    name: "Michoacán Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncl",
    individual_languages: &[
    ],
};


pub const NCM: LanguageCode = LanguageCode {
    name: "Nambo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncm",
    individual_languages: &[
    ],
};


pub const NCN: LanguageCode = LanguageCode {
    name: "Nauna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncn",
    individual_languages: &[
    ],
};


pub const NCO: LanguageCode = LanguageCode {
    name: "Sibe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nco",
    individual_languages: &[
    ],
};


pub const NCQ: LanguageCode = LanguageCode {
    name: "Northern Katang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncq",
    individual_languages: &[
    ],
};


pub const NCR: LanguageCode = LanguageCode {
    name: "Ncane",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncr",
    individual_languages: &[
    ],
};


pub const NCS: LanguageCode = LanguageCode {
    name: "Nicaraguan Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncs",
    individual_languages: &[
    ],
};


pub const NCT: LanguageCode = LanguageCode {
    name: "Chothe Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nct",
    individual_languages: &[
    ],
};


pub const NCU: LanguageCode = LanguageCode {
    name: "Chumburung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncu",
    individual_languages: &[
    ],
};


pub const NCX: LanguageCode = LanguageCode {
    name: "Central Puebla Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncx",
    individual_languages: &[
    ],
};


pub const NCZ: LanguageCode = LanguageCode {
    name: "Natchez",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ncz",
    individual_languages: &[
    ],
};


pub const NDA: LanguageCode = LanguageCode {
    name: "Ndasa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nda",
    individual_languages: &[
    ],
};


pub const NDB: LanguageCode = LanguageCode {
    name: "Kenswei Nsei",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndb",
    individual_languages: &[
    ],
};


pub const NDC: LanguageCode = LanguageCode {
    name: "Ndau",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndc",
    individual_languages: &[
    ],
};


pub const NDD: LanguageCode = LanguageCode {
    name: "Nde-Nsele-Nta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndd",
    individual_languages: &[
    ],
};


pub const NDF: LanguageCode = LanguageCode {
    name: "Nadruvian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndf",
    individual_languages: &[
    ],
};


pub const NDG: LanguageCode = LanguageCode {
    name: "Ndengereko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndg",
    individual_languages: &[
    ],
};


pub const NDH: LanguageCode = LanguageCode {
    name: "Ndali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndh",
    individual_languages: &[
    ],
};


pub const NDI: LanguageCode = LanguageCode {
    name: "Samba Leko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndi",
    individual_languages: &[
    ],
};


pub const NDJ: LanguageCode = LanguageCode {
    name: "Ndamba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndj",
    individual_languages: &[
    ],
};


pub const NDK: LanguageCode = LanguageCode {
    name: "Ndaka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndk",
    individual_languages: &[
    ],
};


pub const NDL: LanguageCode = LanguageCode {
    name: "Ndolo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndl",
    individual_languages: &[
    ],
};


pub const NDM: LanguageCode = LanguageCode {
    name: "Ndam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndm",
    individual_languages: &[
    ],
};


pub const NDN: LanguageCode = LanguageCode {
    name: "Ngundi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndn",
    individual_languages: &[
    ],
};


pub const NDP: LanguageCode = LanguageCode {
    name: "Ndo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndp",
    individual_languages: &[
    ],
};


pub const NDQ: LanguageCode = LanguageCode {
    name: "Ndombe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndq",
    individual_languages: &[
    ],
};


pub const NDR: LanguageCode = LanguageCode {
    name: "Ndoola",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndr",
    individual_languages: &[
    ],
};


pub const NDT: LanguageCode = LanguageCode {
    name: "Ndunga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndt",
    individual_languages: &[
    ],
};


pub const NDU: LanguageCode = LanguageCode {
    name: "Dugun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndu",
    individual_languages: &[
    ],
};


pub const NDV: LanguageCode = LanguageCode {
    name: "Ndut",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndv",
    individual_languages: &[
    ],
};


pub const NDW: LanguageCode = LanguageCode {
    name: "Ndobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndw",
    individual_languages: &[
    ],
};


pub const NDX: LanguageCode = LanguageCode {
    name: "Nduga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndx",
    individual_languages: &[
    ],
};


pub const NDY: LanguageCode = LanguageCode {
    name: "Lutos",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndy",
    individual_languages: &[
    ],
};


pub const NDZ: LanguageCode = LanguageCode {
    name: "Ndogo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ndz",
    individual_languages: &[
    ],
};


pub const NEA: LanguageCode = LanguageCode {
    name: "Eastern Ngad'a",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nea",
    individual_languages: &[
    ],
};


pub const NEB: LanguageCode = LanguageCode {
    name: "Toura (Côte d'Ivoire)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "neb",
    individual_languages: &[
    ],
};


pub const NEC: LanguageCode = LanguageCode {
    name: "Nedebang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nec",
    individual_languages: &[
    ],
};


pub const NED: LanguageCode = LanguageCode {
    name: "Nde-Gbite",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ned",
    individual_languages: &[
    ],
};


pub const NEE: LanguageCode = LanguageCode {
    name: "Nêlêmwa-Nixumwak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nee",
    individual_languages: &[
    ],
};


pub const NEF: LanguageCode = LanguageCode {
    name: "Nefamese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nef",
    individual_languages: &[
    ],
};


pub const NEG: LanguageCode = LanguageCode {
    name: "Negidal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "neg",
    individual_languages: &[
    ],
};


pub const NEH: LanguageCode = LanguageCode {
    name: "Nyenkha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "neh",
    individual_languages: &[
    ],
};


pub const NEI: LanguageCode = LanguageCode {
    name: "Neo-Hittite",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nei",
    individual_languages: &[
    ],
};


pub const NEJ: LanguageCode = LanguageCode {
    name: "Neko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nej",
    individual_languages: &[
    ],
};


pub const NEK: LanguageCode = LanguageCode {
    name: "Neku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nek",
    individual_languages: &[
    ],
};


pub const NEM: LanguageCode = LanguageCode {
    name: "Nemi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nem",
    individual_languages: &[
    ],
};


pub const NEN: LanguageCode = LanguageCode {
    name: "Nengone",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nen",
    individual_languages: &[
    ],
};


pub const NEO: LanguageCode = LanguageCode {
    name: "Ná-Meo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "neo",
    individual_languages: &[
    ],
};


pub const NEQ: LanguageCode = LanguageCode {
    name: "North Central Mixe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "neq",
    individual_languages: &[
    ],
};


pub const NER: LanguageCode = LanguageCode {
    name: "Yahadian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ner",
    individual_languages: &[
    ],
};


pub const NES: LanguageCode = LanguageCode {
    name: "Bhoti Kinnauri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nes",
    individual_languages: &[
    ],
};


pub const NET: LanguageCode = LanguageCode {
    name: "Nete",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "net",
    individual_languages: &[
    ],
};


pub const NEU: LanguageCode = LanguageCode {
    name: "Neo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "neu",
    individual_languages: &[
    ],
};


pub const NEV: LanguageCode = LanguageCode {
    name: "Nyaheun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nev",
    individual_languages: &[
    ],
};


pub const NEX: LanguageCode = LanguageCode {
    name: "Neme",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nex",
    individual_languages: &[
    ],
};


pub const NEY: LanguageCode = LanguageCode {
    name: "Neyo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ney",
    individual_languages: &[
    ],
};


pub const NEZ: LanguageCode = LanguageCode {
    name: "Nez Perce",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nez",
    individual_languages: &[
    ],
};


pub const NFA: LanguageCode = LanguageCode {
    name: "Dhao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nfa",
    individual_languages: &[
    ],
};


pub const NFD: LanguageCode = LanguageCode {
    name: "Ahwai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nfd",
    individual_languages: &[
    ],
};


pub const NFL: LanguageCode = LanguageCode {
    name: "Ayiwo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nfl",
    individual_languages: &[
    ],
};


pub const NFR: LanguageCode = LanguageCode {
    name: "Nafaanra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nfr",
    individual_languages: &[
    ],
};


pub const NFU: LanguageCode = LanguageCode {
    name: "Mfumte",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nfu",
    individual_languages: &[
    ],
};


pub const NGA: LanguageCode = LanguageCode {
    name: "Ngbaka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nga",
    individual_languages: &[
    ],
};


pub const NGB: LanguageCode = LanguageCode {
    name: "Northern Ngbandi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngb",
    individual_languages: &[
    ],
};


pub const NGC: LanguageCode = LanguageCode {
    name: "Ngombe (Democratic Republic of Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngc",
    individual_languages: &[
    ],
};


pub const NGD: LanguageCode = LanguageCode {
    name: "Ngando (Central African Republic)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngd",
    individual_languages: &[
    ],
};


pub const NGE: LanguageCode = LanguageCode {
    name: "Ngemba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nge",
    individual_languages: &[
    ],
};


pub const NGG: LanguageCode = LanguageCode {
    name: "Ngbaka Manza",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngg",
    individual_languages: &[
    ],
};


pub const NGH: LanguageCode = LanguageCode {
    name: "Nǁng",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngh",
    individual_languages: &[
    ],
};


pub const NGI: LanguageCode = LanguageCode {
    name: "Ngizim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngi",
    individual_languages: &[
    ],
};


pub const NGJ: LanguageCode = LanguageCode {
    name: "Ngie",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngj",
    individual_languages: &[
    ],
};


pub const NGK: LanguageCode = LanguageCode {
    name: "Dalabon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngk",
    individual_languages: &[
    ],
};


pub const NGL: LanguageCode = LanguageCode {
    name: "Lomwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngl",
    individual_languages: &[
    ],
};


pub const NGM: LanguageCode = LanguageCode {
    name: "Ngatik Men's Creole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngm",
    individual_languages: &[
    ],
};


pub const NGN: LanguageCode = LanguageCode {
    name: "Ngwo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngn",
    individual_languages: &[
    ],
};


pub const NGP: LanguageCode = LanguageCode {
    name: "Ngulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngp",
    individual_languages: &[
    ],
};


pub const NGQ: LanguageCode = LanguageCode {
    name: "Ngurimi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngq",
    individual_languages: &[
    ],
};


pub const NGR: LanguageCode = LanguageCode {
    name: "Engdewu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngr",
    individual_languages: &[
    ],
};


pub const NGS: LanguageCode = LanguageCode {
    name: "Gvoko",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngs",
    individual_languages: &[
    ],
};


pub const NGT: LanguageCode = LanguageCode {
    name: "Kriang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngt",
    individual_languages: &[
    ],
};


pub const NGU: LanguageCode = LanguageCode {
    name: "Guerrero Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngu",
    individual_languages: &[
    ],
};


pub const NGV: LanguageCode = LanguageCode {
    name: "Nagumi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngv",
    individual_languages: &[
    ],
};


pub const NGW: LanguageCode = LanguageCode {
    name: "Ngwaba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngw",
    individual_languages: &[
    ],
};


pub const NGX: LanguageCode = LanguageCode {
    name: "Nggwahyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngx",
    individual_languages: &[
    ],
};


pub const NGY: LanguageCode = LanguageCode {
    name: "Tibea",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngy",
    individual_languages: &[
    ],
};


pub const NGZ: LanguageCode = LanguageCode {
    name: "Ngungwel",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ngz",
    individual_languages: &[
    ],
};


pub const NHA: LanguageCode = LanguageCode {
    name: "Nhanda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nha",
    individual_languages: &[
    ],
};


pub const NHB: LanguageCode = LanguageCode {
    name: "Beng",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhb",
    individual_languages: &[
    ],
};


pub const NHC: LanguageCode = LanguageCode {
    name: "Tabasco Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhc",
    individual_languages: &[
    ],
};


pub const NHD: LanguageCode = LanguageCode {
    name: "Chiripá",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhd",
    individual_languages: &[
    ],
};


pub const NHE: LanguageCode = LanguageCode {
    name: "Eastern Huasteca Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhe",
    individual_languages: &[
    ],
};


pub const NHF: LanguageCode = LanguageCode {
    name: "Nhuwala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhf",
    individual_languages: &[
    ],
};


pub const NHG: LanguageCode = LanguageCode {
    name: "Tetelcingo Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhg",
    individual_languages: &[
    ],
};


pub const NHH: LanguageCode = LanguageCode {
    name: "Nahari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhh",
    individual_languages: &[
    ],
};


pub const NHI: LanguageCode = LanguageCode {
    name: "Zacatlán-Ahuacatlán-Tepetzintla Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhi",
    individual_languages: &[
    ],
};


pub const NHK: LanguageCode = LanguageCode {
    name: "Isthmus-Cosoleacaque Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhk",
    individual_languages: &[
    ],
};


pub const NHM: LanguageCode = LanguageCode {
    name: "Morelos Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhm",
    individual_languages: &[
    ],
};


pub const NHN: LanguageCode = LanguageCode {
    name: "Central Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhn",
    individual_languages: &[
    ],
};


pub const NHO: LanguageCode = LanguageCode {
    name: "Takuu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nho",
    individual_languages: &[
    ],
};


pub const NHP: LanguageCode = LanguageCode {
    name: "Isthmus-Pajapan Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhp",
    individual_languages: &[
    ],
};


pub const NHQ: LanguageCode = LanguageCode {
    name: "Huaxcaleca Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhq",
    individual_languages: &[
    ],
};


pub const NHR: LanguageCode = LanguageCode {
    name: "Naro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhr",
    individual_languages: &[
    ],
};


pub const NHT: LanguageCode = LanguageCode {
    name: "Ometepec Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nht",
    individual_languages: &[
    ],
};


pub const NHU: LanguageCode = LanguageCode {
    name: "Noone",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhu",
    individual_languages: &[
    ],
};


pub const NHV: LanguageCode = LanguageCode {
    name: "Temascaltepec Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhv",
    individual_languages: &[
    ],
};


pub const NHW: LanguageCode = LanguageCode {
    name: "Western Huasteca Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhw",
    individual_languages: &[
    ],
};


pub const NHX: LanguageCode = LanguageCode {
    name: "Isthmus-Mecayapan Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhx",
    individual_languages: &[
    ],
};


pub const NHY: LanguageCode = LanguageCode {
    name: "Northern Oaxaca Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhy",
    individual_languages: &[
    ],
};


pub const NHZ: LanguageCode = LanguageCode {
    name: "Santa María La Alta Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nhz",
    individual_languages: &[
    ],
};


pub const NIB: LanguageCode = LanguageCode {
    name: "Nakame",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nib",
    individual_languages: &[
    ],
};


pub const NID: LanguageCode = LanguageCode {
    name: "Ngandi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nid",
    individual_languages: &[
    ],
};


pub const NIE: LanguageCode = LanguageCode {
    name: "Niellim",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nie",
    individual_languages: &[
    ],
};


pub const NIF: LanguageCode = LanguageCode {
    name: "Nek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nif",
    individual_languages: &[
    ],
};


pub const NIG: LanguageCode = LanguageCode {
    name: "Ngalakgan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nig",
    individual_languages: &[
    ],
};


pub const NIH: LanguageCode = LanguageCode {
    name: "Nyiha (Tanzania)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nih",
    individual_languages: &[
    ],
};


pub const NII: LanguageCode = LanguageCode {
    name: "Nii",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nii",
    individual_languages: &[
    ],
};


pub const NIJ: LanguageCode = LanguageCode {
    name: "Ngaju",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nij",
    individual_languages: &[
    ],
};


pub const NIK: LanguageCode = LanguageCode {
    name: "Southern Nicobarese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nik",
    individual_languages: &[
    ],
};


pub const NIL: LanguageCode = LanguageCode {
    name: "Nila",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nil",
    individual_languages: &[
    ],
};


pub const NIM: LanguageCode = LanguageCode {
    name: "Nilamba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nim",
    individual_languages: &[
    ],
};


pub const NIN: LanguageCode = LanguageCode {
    name: "Ninzo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nin",
    individual_languages: &[
    ],
};


pub const NIO: LanguageCode = LanguageCode {
    name: "Nganasan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nio",
    individual_languages: &[
    ],
};


pub const NIQ: LanguageCode = LanguageCode {
    name: "Nandi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "niq",
    individual_languages: &[
    ],
};


pub const NIR: LanguageCode = LanguageCode {
    name: "Nimboran",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nir",
    individual_languages: &[
    ],
};


pub const NIS: LanguageCode = LanguageCode {
    name: "Nimi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nis",
    individual_languages: &[
    ],
};


pub const NIT: LanguageCode = LanguageCode {
    name: "Southeastern Kolami",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nit",
    individual_languages: &[
    ],
};


pub const NIV: LanguageCode = LanguageCode {
    name: "Gilyak",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "niv",
    individual_languages: &[
    ],
};


pub const NIW: LanguageCode = LanguageCode {
    name: "Nimo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "niw",
    individual_languages: &[
    ],
};


pub const NIX: LanguageCode = LanguageCode {
    name: "Hema",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nix",
    individual_languages: &[
    ],
};


pub const NIY: LanguageCode = LanguageCode {
    name: "Ngiti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "niy",
    individual_languages: &[
    ],
};


pub const NIZ: LanguageCode = LanguageCode {
    name: "Ningil",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "niz",
    individual_languages: &[
    ],
};


pub const NJA: LanguageCode = LanguageCode {
    name: "Nzanyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nja",
    individual_languages: &[
    ],
};


pub const NJB: LanguageCode = LanguageCode {
    name: "Nocte Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njb",
    individual_languages: &[
    ],
};


pub const NJD: LanguageCode = LanguageCode {
    name: "Ndonde Hamba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njd",
    individual_languages: &[
    ],
};


pub const NJH: LanguageCode = LanguageCode {
    name: "Lotha Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njh",
    individual_languages: &[
    ],
};


pub const NJI: LanguageCode = LanguageCode {
    name: "Gudanji",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nji",
    individual_languages: &[
    ],
};


pub const NJJ: LanguageCode = LanguageCode {
    name: "Njen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njj",
    individual_languages: &[
    ],
};


pub const NJL: LanguageCode = LanguageCode {
    name: "Njalgulgule",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njl",
    individual_languages: &[
    ],
};


pub const NJM: LanguageCode = LanguageCode {
    name: "Angami Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njm",
    individual_languages: &[
    ],
};


pub const NJN: LanguageCode = LanguageCode {
    name: "Liangmai Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njn",
    individual_languages: &[
    ],
};


pub const NJO: LanguageCode = LanguageCode {
    name: "Ao Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njo",
    individual_languages: &[
    ],
};


pub const NJR: LanguageCode = LanguageCode {
    name: "Njerep",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njr",
    individual_languages: &[
    ],
};


pub const NJS: LanguageCode = LanguageCode {
    name: "Nisa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njs",
    individual_languages: &[
    ],
};


pub const NJT: LanguageCode = LanguageCode {
    name: "Ndyuka-Trio Pidgin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njt",
    individual_languages: &[
    ],
};


pub const NJU: LanguageCode = LanguageCode {
    name: "Ngadjunmaya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nju",
    individual_languages: &[
    ],
};


pub const NJX: LanguageCode = LanguageCode {
    name: "Kunyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njx",
    individual_languages: &[
    ],
};


pub const NJY: LanguageCode = LanguageCode {
    name: "Njyem",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njy",
    individual_languages: &[
    ],
};


pub const NJZ: LanguageCode = LanguageCode {
    name: "Nyishi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "njz",
    individual_languages: &[
    ],
};


pub const NKA: LanguageCode = LanguageCode {
    name: "Nkoya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nka",
    individual_languages: &[
    ],
};


pub const NKB: LanguageCode = LanguageCode {
    name: "Khoibu Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkb",
    individual_languages: &[
    ],
};


pub const NKC: LanguageCode = LanguageCode {
    name: "Nkongho",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkc",
    individual_languages: &[
    ],
};


pub const NKD: LanguageCode = LanguageCode {
    name: "Koireng",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkd",
    individual_languages: &[
    ],
};


pub const NKE: LanguageCode = LanguageCode {
    name: "Duke",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nke",
    individual_languages: &[
    ],
};


pub const NKF: LanguageCode = LanguageCode {
    name: "Inpui Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkf",
    individual_languages: &[
    ],
};


pub const NKG: LanguageCode = LanguageCode {
    name: "Nekgini",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkg",
    individual_languages: &[
    ],
};


pub const NKH: LanguageCode = LanguageCode {
    name: "Khezha Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkh",
    individual_languages: &[
    ],
};


pub const NKI: LanguageCode = LanguageCode {
    name: "Thangal Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nki",
    individual_languages: &[
    ],
};


pub const NKJ: LanguageCode = LanguageCode {
    name: "Nakai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkj",
    individual_languages: &[
    ],
};


pub const NKK: LanguageCode = LanguageCode {
    name: "Nokuku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkk",
    individual_languages: &[
    ],
};


pub const NKM: LanguageCode = LanguageCode {
    name: "Namat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkm",
    individual_languages: &[
    ],
};


pub const NKN: LanguageCode = LanguageCode {
    name: "Nkangala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkn",
    individual_languages: &[
    ],
};


pub const NKO: LanguageCode = LanguageCode {
    name: "Nkonya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nko",
    individual_languages: &[
    ],
};


pub const NKP: LanguageCode = LanguageCode {
    name: "Niuatoputapu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkp",
    individual_languages: &[
    ],
};


pub const NKQ: LanguageCode = LanguageCode {
    name: "Nkami",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkq",
    individual_languages: &[
    ],
};


pub const NKR: LanguageCode = LanguageCode {
    name: "Nukuoro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkr",
    individual_languages: &[
    ],
};


pub const NKS: LanguageCode = LanguageCode {
    name: "North Asmat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nks",
    individual_languages: &[
    ],
};


pub const NKT: LanguageCode = LanguageCode {
    name: "Nyika (Tanzania)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkt",
    individual_languages: &[
    ],
};


pub const NKU: LanguageCode = LanguageCode {
    name: "Bouna Kulango",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nku",
    individual_languages: &[
    ],
};


pub const NKV: LanguageCode = LanguageCode {
    name: "Nyika (Malawi and Zambia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkv",
    individual_languages: &[
    ],
};


pub const NKW: LanguageCode = LanguageCode {
    name: "Nkutu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkw",
    individual_languages: &[
    ],
};


pub const NKX: LanguageCode = LanguageCode {
    name: "Nkoroo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkx",
    individual_languages: &[
    ],
};


pub const NKZ: LanguageCode = LanguageCode {
    name: "Nkari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nkz",
    individual_languages: &[
    ],
};


pub const NLA: LanguageCode = LanguageCode {
    name: "Ngombale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nla",
    individual_languages: &[
    ],
};


pub const NLC: LanguageCode = LanguageCode {
    name: "Nalca",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nlc",
    individual_languages: &[
    ],
};


pub const NLE: LanguageCode = LanguageCode {
    name: "East Nyala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nle",
    individual_languages: &[
    ],
};


pub const NLG: LanguageCode = LanguageCode {
    name: "Gela",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nlg",
    individual_languages: &[
    ],
};


pub const NLI: LanguageCode = LanguageCode {
    name: "Grangali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nli",
    individual_languages: &[
    ],
};


pub const NLJ: LanguageCode = LanguageCode {
    name: "Nyali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nlj",
    individual_languages: &[
    ],
};


pub const NLK: LanguageCode = LanguageCode {
    name: "Ninia Yali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nlk",
    individual_languages: &[
    ],
};


pub const NLL: LanguageCode = LanguageCode {
    name: "Nihali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nll",
    individual_languages: &[
    ],
};


pub const NLM: LanguageCode = LanguageCode {
    name: "Mankiyali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nlm",
    individual_languages: &[
    ],
};


pub const NLO: LanguageCode = LanguageCode {
    name: "Ngul",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nlo",
    individual_languages: &[
    ],
};


pub const NLQ: LanguageCode = LanguageCode {
    name: "Lao Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nlq",
    individual_languages: &[
    ],
};


pub const NLU: LanguageCode = LanguageCode {
    name: "Nchumbulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nlu",
    individual_languages: &[
    ],
};


pub const NLV: LanguageCode = LanguageCode {
    name: "Orizaba Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nlv",
    individual_languages: &[
    ],
};


pub const NLW: LanguageCode = LanguageCode {
    name: "Walangama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nlw",
    individual_languages: &[
    ],
};


pub const NLX: LanguageCode = LanguageCode {
    name: "Nahali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nlx",
    individual_languages: &[
    ],
};


pub const NLY: LanguageCode = LanguageCode {
    name: "Nyamal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nly",
    individual_languages: &[
    ],
};


pub const NLZ: LanguageCode = LanguageCode {
    name: "Nalögo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nlz",
    individual_languages: &[
    ],
};


pub const NMA: LanguageCode = LanguageCode {
    name: "Maram Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nma",
    individual_languages: &[
    ],
};


pub const NMB: LanguageCode = LanguageCode {
    name: "Big Nambas",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmb",
    individual_languages: &[
    ],
};


pub const NMC: LanguageCode = LanguageCode {
    name: "Ngam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmc",
    individual_languages: &[
    ],
};


pub const NMD: LanguageCode = LanguageCode {
    name: "Ndumu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmd",
    individual_languages: &[
    ],
};


pub const NME: LanguageCode = LanguageCode {
    name: "Mzieme Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nme",
    individual_languages: &[
    ],
};


pub const NMF: LanguageCode = LanguageCode {
    name: "Tangkhul Naga (India)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmf",
    individual_languages: &[
    ],
};


pub const NMG: LanguageCode = LanguageCode {
    name: "Kwasio",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmg",
    individual_languages: &[
    ],
};


pub const NMH: LanguageCode = LanguageCode {
    name: "Monsang Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmh",
    individual_languages: &[
    ],
};


pub const NMI: LanguageCode = LanguageCode {
    name: "Nyam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmi",
    individual_languages: &[
    ],
};


pub const NMJ: LanguageCode = LanguageCode {
    name: "Ngombe (Central African Republic)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmj",
    individual_languages: &[
    ],
};


pub const NMK: LanguageCode = LanguageCode {
    name: "Namakura",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmk",
    individual_languages: &[
    ],
};


pub const NML: LanguageCode = LanguageCode {
    name: "Ndemli",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nml",
    individual_languages: &[
    ],
};


pub const NMM: LanguageCode = LanguageCode {
    name: "Manangba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmm",
    individual_languages: &[
    ],
};


pub const NMN: LanguageCode = LanguageCode {
    name: "ǃXóõ",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmn",
    individual_languages: &[
    ],
};


pub const NMO: LanguageCode = LanguageCode {
    name: "Moyon Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmo",
    individual_languages: &[
    ],
};


pub const NMP: LanguageCode = LanguageCode {
    name: "Nimanbur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmp",
    individual_languages: &[
    ],
};


pub const NMQ: LanguageCode = LanguageCode {
    name: "Nambya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmq",
    individual_languages: &[
    ],
};


pub const NMR: LanguageCode = LanguageCode {
    name: "Nimbari",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmr",
    individual_languages: &[
    ],
};


pub const NMS: LanguageCode = LanguageCode {
    name: "Letemboi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nms",
    individual_languages: &[
    ],
};


pub const NMT: LanguageCode = LanguageCode {
    name: "Namonuito",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmt",
    individual_languages: &[
    ],
};


pub const NMU: LanguageCode = LanguageCode {
    name: "Northeast Maidu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmu",
    individual_languages: &[
    ],
};


pub const NMV: LanguageCode = LanguageCode {
    name: "Ngamini",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmv",
    individual_languages: &[
    ],
};


pub const NMW: LanguageCode = LanguageCode {
    name: "Nimoa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmw",
    individual_languages: &[
    ],
};


pub const NMX: LanguageCode = LanguageCode {
    name: "Nama (Papua New Guinea)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmx",
    individual_languages: &[
    ],
};


pub const NMY: LanguageCode = LanguageCode {
    name: "Namuyi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmy",
    individual_languages: &[
    ],
};


pub const NMZ: LanguageCode = LanguageCode {
    name: "Nawdm",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nmz",
    individual_languages: &[
    ],
};


pub const NNA: LanguageCode = LanguageCode {
    name: "Nyangumarta",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nna",
    individual_languages: &[
    ],
};


pub const NNB: LanguageCode = LanguageCode {
    name: "Nande",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnb",
    individual_languages: &[
    ],
};


pub const NNC: LanguageCode = LanguageCode {
    name: "Nancere",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnc",
    individual_languages: &[
    ],
};


pub const NND: LanguageCode = LanguageCode {
    name: "West Ambae",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnd",
    individual_languages: &[
    ],
};


pub const NNE: LanguageCode = LanguageCode {
    name: "Ngandyera",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nne",
    individual_languages: &[
    ],
};


pub const NNF: LanguageCode = LanguageCode {
    name: "Ngaing",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnf",
    individual_languages: &[
    ],
};


pub const NNG: LanguageCode = LanguageCode {
    name: "Maring Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nng",
    individual_languages: &[
    ],
};


pub const NNH: LanguageCode = LanguageCode {
    name: "Ngiemboon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnh",
    individual_languages: &[
    ],
};


pub const NNI: LanguageCode = LanguageCode {
    name: "North Nuaulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nni",
    individual_languages: &[
    ],
};


pub const NNJ: LanguageCode = LanguageCode {
    name: "Nyangatom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnj",
    individual_languages: &[
    ],
};


pub const NNK: LanguageCode = LanguageCode {
    name: "Nankina",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnk",
    individual_languages: &[
    ],
};


pub const NNL: LanguageCode = LanguageCode {
    name: "Northern Rengma Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnl",
    individual_languages: &[
    ],
};


pub const NNM: LanguageCode = LanguageCode {
    name: "Namia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnm",
    individual_languages: &[
    ],
};


pub const NNN: LanguageCode = LanguageCode {
    name: "Ngete",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnn",
    individual_languages: &[
    ],
};


pub const NNP: LanguageCode = LanguageCode {
    name: "Wancho Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnp",
    individual_languages: &[
    ],
};


pub const NNQ: LanguageCode = LanguageCode {
    name: "Ngindo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnq",
    individual_languages: &[
    ],
};


pub const NNR: LanguageCode = LanguageCode {
    name: "Narungga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnr",
    individual_languages: &[
    ],
};


pub const NNT: LanguageCode = LanguageCode {
    name: "Nanticoke",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnt",
    individual_languages: &[
    ],
};


pub const NNU: LanguageCode = LanguageCode {
    name: "Dwang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnu",
    individual_languages: &[
    ],
};


pub const NNV: LanguageCode = LanguageCode {
    name: "Nugunu (Australia)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnv",
    individual_languages: &[
    ],
};


pub const NNW: LanguageCode = LanguageCode {
    name: "Southern Nuni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnw",
    individual_languages: &[
    ],
};


pub const NNY: LanguageCode = LanguageCode {
    name: "Nyangga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nny",
    individual_languages: &[
    ],
};


pub const NNZ: LanguageCode = LanguageCode {
    name: "Nda'nda'",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nnz",
    individual_languages: &[
    ],
};


pub const NOA: LanguageCode = LanguageCode {
    name: "Woun Meu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "noa",
    individual_languages: &[
    ],
};


pub const NOC: LanguageCode = LanguageCode {
    name: "Nuk",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "noc",
    individual_languages: &[
    ],
};


pub const NOD: LanguageCode = LanguageCode {
    name: "Northern Thai",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nod",
    individual_languages: &[
    ],
};


pub const NOE: LanguageCode = LanguageCode {
    name: "Nimadi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "noe",
    individual_languages: &[
    ],
};


pub const NOF: LanguageCode = LanguageCode {
    name: "Nomane",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nof",
    individual_languages: &[
    ],
};


pub const NOH: LanguageCode = LanguageCode {
    name: "Nomu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "noh",
    individual_languages: &[
    ],
};


pub const NOI: LanguageCode = LanguageCode {
    name: "Noiri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "noi",
    individual_languages: &[
    ],
};


pub const NOJ: LanguageCode = LanguageCode {
    name: "Nonuya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "noj",
    individual_languages: &[
    ],
};


pub const NOK: LanguageCode = LanguageCode {
    name: "Nooksack",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nok",
    individual_languages: &[
    ],
};


pub const NOL: LanguageCode = LanguageCode {
    name: "Nomlaki",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nol",
    individual_languages: &[
    ],
};


pub const NOM: LanguageCode = LanguageCode {
    name: "Nocamán",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nom",
    individual_languages: &[
    ],
};


pub const NOP: LanguageCode = LanguageCode {
    name: "Numanggang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nop",
    individual_languages: &[
    ],
};


pub const NOQ: LanguageCode = LanguageCode {
    name: "Ngongo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "noq",
    individual_languages: &[
    ],
};


pub const NOS: LanguageCode = LanguageCode {
    name: "Eastern Nisu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nos",
    individual_languages: &[
    ],
};


pub const NOT: LanguageCode = LanguageCode {
    name: "Nomatsiguenga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "not",
    individual_languages: &[
    ],
};


pub const NOU: LanguageCode = LanguageCode {
    name: "Ewage-Notu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nou",
    individual_languages: &[
    ],
};


pub const NOV: LanguageCode = LanguageCode {
    name: "Novial",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nov",
    individual_languages: &[
    ],
};


pub const NOW: LanguageCode = LanguageCode {
    name: "Nyambo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "now",
    individual_languages: &[
    ],
};


pub const NOY: LanguageCode = LanguageCode {
    name: "Noy",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "noy",
    individual_languages: &[
    ],
};


pub const NOZ: LanguageCode = LanguageCode {
    name: "Nayi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "noz",
    individual_languages: &[
    ],
};


pub const NPA: LanguageCode = LanguageCode {
    name: "Nar Phu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "npa",
    individual_languages: &[
    ],
};


pub const NPB: LanguageCode = LanguageCode {
    name: "Nupbikha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "npb",
    individual_languages: &[
    ],
};


pub const NPG: LanguageCode = LanguageCode {
    name: "Ponyo-Gongwang Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "npg",
    individual_languages: &[
    ],
};


pub const NPH: LanguageCode = LanguageCode {
    name: "Phom Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nph",
    individual_languages: &[
    ],
};


pub const NPI: LanguageCode = LanguageCode {
    name: "Nepali (individual language)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "npi",
    individual_languages: &[
    ],
};


pub const NPL: LanguageCode = LanguageCode {
    name: "Southeastern Puebla Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "npl",
    individual_languages: &[
    ],
};


pub const NPN: LanguageCode = LanguageCode {
    name: "Mondropolon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "npn",
    individual_languages: &[
    ],
};


pub const NPO: LanguageCode = LanguageCode {
    name: "Pochuri Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "npo",
    individual_languages: &[
    ],
};


pub const NPS: LanguageCode = LanguageCode {
    name: "Nipsan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nps",
    individual_languages: &[
    ],
};


pub const NPU: LanguageCode = LanguageCode {
    name: "Puimei Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "npu",
    individual_languages: &[
    ],
};


pub const NPX: LanguageCode = LanguageCode {
    name: "Noipx",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "npx",
    individual_languages: &[
    ],
};


pub const NPY: LanguageCode = LanguageCode {
    name: "Napu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "npy",
    individual_languages: &[
    ],
};


pub const NQG: LanguageCode = LanguageCode {
    name: "Southern Nago",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nqg",
    individual_languages: &[
    ],
};


pub const NQK: LanguageCode = LanguageCode {
    name: "Kura Ede Nago",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nqk",
    individual_languages: &[
    ],
};


pub const NQL: LanguageCode = LanguageCode {
    name: "Ngendelengo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nql",
    individual_languages: &[
    ],
};


pub const NQM: LanguageCode = LanguageCode {
    name: "Ndom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nqm",
    individual_languages: &[
    ],
};


pub const NQN: LanguageCode = LanguageCode {
    name: "Nen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nqn",
    individual_languages: &[
    ],
};


pub const NQQ: LanguageCode = LanguageCode {
    name: "Kyan-Karyaw Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nqq",
    individual_languages: &[
    ],
};


pub const NQT: LanguageCode = LanguageCode {
    name: "Nteng",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nqt",
    individual_languages: &[
    ],
};


pub const NQY: LanguageCode = LanguageCode {
    name: "Akyaung Ari Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nqy",
    individual_languages: &[
    ],
};


pub const NRA: LanguageCode = LanguageCode {
    name: "Ngom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nra",
    individual_languages: &[
    ],
};


pub const NRB: LanguageCode = LanguageCode {
    name: "Nara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrb",
    individual_languages: &[
    ],
};


pub const NRC: LanguageCode = LanguageCode {
    name: "Noric",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrc",
    individual_languages: &[
    ],
};


pub const NRE: LanguageCode = LanguageCode {
    name: "Southern Rengma Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nre",
    individual_languages: &[
    ],
};


pub const NRF: LanguageCode = LanguageCode {
    name: "Jèrriais",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrf",
    individual_languages: &[
    ],
};


pub const NRG: LanguageCode = LanguageCode {
    name: "Narango",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrg",
    individual_languages: &[
    ],
};


pub const NRI: LanguageCode = LanguageCode {
    name: "Chokri Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nri",
    individual_languages: &[
    ],
};


pub const NRK: LanguageCode = LanguageCode {
    name: "Ngarla",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrk",
    individual_languages: &[
    ],
};


pub const NRL: LanguageCode = LanguageCode {
    name: "Ngarluma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrl",
    individual_languages: &[
    ],
};


pub const NRM: LanguageCode = LanguageCode {
    name: "Narom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrm",
    individual_languages: &[
    ],
};


pub const NRN: LanguageCode = LanguageCode {
    name: "Norn",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrn",
    individual_languages: &[
    ],
};


pub const NRP: LanguageCode = LanguageCode {
    name: "North Picene",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrp",
    individual_languages: &[
    ],
};


pub const NRR: LanguageCode = LanguageCode {
    name: "Norra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrr",
    individual_languages: &[
    ],
};


pub const NRT: LanguageCode = LanguageCode {
    name: "Northern Kalapuya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrt",
    individual_languages: &[
    ],
};


pub const NRU: LanguageCode = LanguageCode {
    name: "Narua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nru",
    individual_languages: &[
    ],
};


pub const NRX: LanguageCode = LanguageCode {
    name: "Ngurmbur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrx",
    individual_languages: &[
    ],
};


pub const NRZ: LanguageCode = LanguageCode {
    name: "Lala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nrz",
    individual_languages: &[
    ],
};


pub const NSA: LanguageCode = LanguageCode {
    name: "Sangtam Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsa",
    individual_languages: &[
    ],
};


pub const NSB: LanguageCode = LanguageCode {
    name: "Lower Nossob",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsb",
    individual_languages: &[
    ],
};


pub const NSC: LanguageCode = LanguageCode {
    name: "Nshi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsc",
    individual_languages: &[
    ],
};


pub const NSD: LanguageCode = LanguageCode {
    name: "Southern Nisu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsd",
    individual_languages: &[
    ],
};


pub const NSE: LanguageCode = LanguageCode {
    name: "Nsenga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nse",
    individual_languages: &[
    ],
};


pub const NSF: LanguageCode = LanguageCode {
    name: "Northwestern Nisu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsf",
    individual_languages: &[
    ],
};


pub const NSG: LanguageCode = LanguageCode {
    name: "Ngasa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsg",
    individual_languages: &[
    ],
};


pub const NSH: LanguageCode = LanguageCode {
    name: "Ngoshie",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsh",
    individual_languages: &[
    ],
};


pub const NSI: LanguageCode = LanguageCode {
    name: "Nigerian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsi",
    individual_languages: &[
    ],
};


pub const NSK: LanguageCode = LanguageCode {
    name: "Naskapi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsk",
    individual_languages: &[
    ],
};


pub const NSL: LanguageCode = LanguageCode {
    name: "Norwegian Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsl",
    individual_languages: &[
    ],
};


pub const NSM: LanguageCode = LanguageCode {
    name: "Sumi Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsm",
    individual_languages: &[
    ],
};


pub const NSN: LanguageCode = LanguageCode {
    name: "Nehan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsn",
    individual_languages: &[
    ],
};


pub const NSP: LanguageCode = LanguageCode {
    name: "Nepalese Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsp",
    individual_languages: &[
    ],
};


pub const NSQ: LanguageCode = LanguageCode {
    name: "Northern Sierra Miwok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsq",
    individual_languages: &[
    ],
};


pub const NSR: LanguageCode = LanguageCode {
    name: "Maritime Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsr",
    individual_languages: &[
    ],
};


pub const NSS: LanguageCode = LanguageCode {
    name: "Nali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nss",
    individual_languages: &[
    ],
};


pub const NST: LanguageCode = LanguageCode {
    name: "Tase Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nst",
    individual_languages: &[
    ],
};


pub const NSU: LanguageCode = LanguageCode {
    name: "Sierra Negra Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsu",
    individual_languages: &[
    ],
};


pub const NSV: LanguageCode = LanguageCode {
    name: "Southwestern Nisu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsv",
    individual_languages: &[
    ],
};


pub const NSW: LanguageCode = LanguageCode {
    name: "Navut",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsw",
    individual_languages: &[
    ],
};


pub const NSX: LanguageCode = LanguageCode {
    name: "Nsongo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsx",
    individual_languages: &[
    ],
};


pub const NSY: LanguageCode = LanguageCode {
    name: "Nasal",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsy",
    individual_languages: &[
    ],
};


pub const NSZ: LanguageCode = LanguageCode {
    name: "Nisenan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nsz",
    individual_languages: &[
    ],
};


pub const NTD: LanguageCode = LanguageCode {
    name: "Northern Tidung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ntd",
    individual_languages: &[
    ],
};


pub const NTE: LanguageCode = LanguageCode {
    name: "Nathembo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nte",
    individual_languages: &[
    ],
};


pub const NTG: LanguageCode = LanguageCode {
    name: "Ngantangarra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ntg",
    individual_languages: &[
    ],
};


pub const NTI: LanguageCode = LanguageCode {
    name: "Natioro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nti",
    individual_languages: &[
    ],
};


pub const NTJ: LanguageCode = LanguageCode {
    name: "Ngaanyatjarra",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ntj",
    individual_languages: &[
    ],
};


pub const NTK: LanguageCode = LanguageCode {
    name: "Ikoma-Nata-Isenye",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ntk",
    individual_languages: &[
    ],
};


pub const NTM: LanguageCode = LanguageCode {
    name: "Nateni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ntm",
    individual_languages: &[
    ],
};


pub const NTO: LanguageCode = LanguageCode {
    name: "Ntomba",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nto",
    individual_languages: &[
    ],
};


pub const NTP: LanguageCode = LanguageCode {
    name: "Northern Tepehuan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ntp",
    individual_languages: &[
    ],
};


pub const NTR: LanguageCode = LanguageCode {
    name: "Delo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ntr",
    individual_languages: &[
    ],
};


pub const NTU: LanguageCode = LanguageCode {
    name: "Natügu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ntu",
    individual_languages: &[
    ],
};


pub const NTW: LanguageCode = LanguageCode {
    name: "Nottoway",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ntw",
    individual_languages: &[
    ],
};


pub const NTX: LanguageCode = LanguageCode {
    name: "Tangkhul Naga (Myanmar)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ntx",
    individual_languages: &[
    ],
};


pub const NTY: LanguageCode = LanguageCode {
    name: "Mantsi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nty",
    individual_languages: &[
    ],
};


pub const NTZ: LanguageCode = LanguageCode {
    name: "Natanzi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ntz",
    individual_languages: &[
    ],
};


pub const NUA: LanguageCode = LanguageCode {
    name: "Yuanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nua",
    individual_languages: &[
    ],
};


pub const NUC: LanguageCode = LanguageCode {
    name: "Nukuini",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nuc",
    individual_languages: &[
    ],
};


pub const NUD: LanguageCode = LanguageCode {
    name: "Ngala",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nud",
    individual_languages: &[
    ],
};


pub const NUE: LanguageCode = LanguageCode {
    name: "Ngundu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nue",
    individual_languages: &[
    ],
};


pub const NUF: LanguageCode = LanguageCode {
    name: "Nusu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nuf",
    individual_languages: &[
    ],
};


pub const NUG: LanguageCode = LanguageCode {
    name: "Nungali",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nug",
    individual_languages: &[
    ],
};


pub const NUH: LanguageCode = LanguageCode {
    name: "Ndunda",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nuh",
    individual_languages: &[
    ],
};


pub const NUI: LanguageCode = LanguageCode {
    name: "Ngumbi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nui",
    individual_languages: &[
    ],
};


pub const NUJ: LanguageCode = LanguageCode {
    name: "Nyole",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nuj",
    individual_languages: &[
    ],
};


pub const NUK: LanguageCode = LanguageCode {
    name: "Nuu-chah-nulth",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nuk",
    individual_languages: &[
    ],
};


pub const NUL: LanguageCode = LanguageCode {
    name: "Nusa Laut",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nul",
    individual_languages: &[
    ],
};


pub const NUM: LanguageCode = LanguageCode {
    name: "Niuafo'ou",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "num",
    individual_languages: &[
    ],
};


pub const NUN: LanguageCode = LanguageCode {
    name: "Anong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nun",
    individual_languages: &[
    ],
};


pub const NUO: LanguageCode = LanguageCode {
    name: "Nguôn",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nuo",
    individual_languages: &[
    ],
};


pub const NUP: LanguageCode = LanguageCode {
    name: "Nupe-Nupe-Tako",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nup",
    individual_languages: &[
    ],
};


pub const NUQ: LanguageCode = LanguageCode {
    name: "Nukumanu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nuq",
    individual_languages: &[
    ],
};


pub const NUR: LanguageCode = LanguageCode {
    name: "Nukuria",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nur",
    individual_languages: &[
    ],
};


pub const NUS: LanguageCode = LanguageCode {
    name: "Nuer",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nus",
    individual_languages: &[
    ],
};


pub const NUT: LanguageCode = LanguageCode {
    name: "Nung (Viet Nam)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nut",
    individual_languages: &[
    ],
};


pub const NUU: LanguageCode = LanguageCode {
    name: "Ngbundu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nuu",
    individual_languages: &[
    ],
};


pub const NUV: LanguageCode = LanguageCode {
    name: "Northern Nuni",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nuv",
    individual_languages: &[
    ],
};


pub const NUW: LanguageCode = LanguageCode {
    name: "Nguluwan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nuw",
    individual_languages: &[
    ],
};


pub const NUX: LanguageCode = LanguageCode {
    name: "Mehek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nux",
    individual_languages: &[
    ],
};


pub const NUY: LanguageCode = LanguageCode {
    name: "Nunggubuyu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nuy",
    individual_languages: &[
    ],
};


pub const NUZ: LanguageCode = LanguageCode {
    name: "Tlamacazapa Nahuatl",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nuz",
    individual_languages: &[
    ],
};


pub const NVH: LanguageCode = LanguageCode {
    name: "Nasarian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nvh",
    individual_languages: &[
    ],
};


pub const NVM: LanguageCode = LanguageCode {
    name: "Namiae",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nvm",
    individual_languages: &[
    ],
};


pub const NVO: LanguageCode = LanguageCode {
    name: "Nyokon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nvo",
    individual_languages: &[
    ],
};


pub const NWA: LanguageCode = LanguageCode {
    name: "Nawathinehena",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nwa",
    individual_languages: &[
    ],
};


pub const NWB: LanguageCode = LanguageCode {
    name: "Nyabwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nwb",
    individual_languages: &[
    ],
};


pub const NWE: LanguageCode = LanguageCode {
    name: "Ngwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nwe",
    individual_languages: &[
    ],
};


pub const NWG: LanguageCode = LanguageCode {
    name: "Ngayawung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nwg",
    individual_languages: &[
    ],
};


pub const NWI: LanguageCode = LanguageCode {
    name: "Southwest Tanna",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nwi",
    individual_languages: &[
    ],
};


pub const NWM: LanguageCode = LanguageCode {
    name: "Nyamusa-Molo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nwm",
    individual_languages: &[
    ],
};


pub const NWO: LanguageCode = LanguageCode {
    name: "Nauo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nwo",
    individual_languages: &[
    ],
};


pub const NWR: LanguageCode = LanguageCode {
    name: "Nawaru",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nwr",
    individual_languages: &[
    ],
};


pub const NWW: LanguageCode = LanguageCode {
    name: "Ndwewe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nww",
    individual_languages: &[
    ],
};


pub const NWX: LanguageCode = LanguageCode {
    name: "Middle Newar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nwx",
    individual_languages: &[
    ],
};


pub const NWY: LanguageCode = LanguageCode {
    name: "Nottoway-Meherrin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nwy",
    individual_languages: &[
    ],
};


pub const NXA: LanguageCode = LanguageCode {
    name: "Nauete",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxa",
    individual_languages: &[
    ],
};


pub const NXD: LanguageCode = LanguageCode {
    name: "Ngando (Democratic Republic of Congo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxd",
    individual_languages: &[
    ],
};


pub const NXE: LanguageCode = LanguageCode {
    name: "Nage",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxe",
    individual_languages: &[
    ],
};


pub const NXG: LanguageCode = LanguageCode {
    name: "Ngad'a",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxg",
    individual_languages: &[
    ],
};


pub const NXI: LanguageCode = LanguageCode {
    name: "Nindi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxi",
    individual_languages: &[
    ],
};


pub const NXK: LanguageCode = LanguageCode {
    name: "Koki Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxk",
    individual_languages: &[
    ],
};


pub const NXL: LanguageCode = LanguageCode {
    name: "South Nuaulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxl",
    individual_languages: &[
    ],
};


pub const NXM: LanguageCode = LanguageCode {
    name: "Numidian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxm",
    individual_languages: &[
    ],
};


pub const NXN: LanguageCode = LanguageCode {
    name: "Ngawun",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxn",
    individual_languages: &[
    ],
};


pub const NXO: LanguageCode = LanguageCode {
    name: "Ndambomo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxo",
    individual_languages: &[
    ],
};


pub const NXQ: LanguageCode = LanguageCode {
    name: "Naxi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxq",
    individual_languages: &[
    ],
};


pub const NXR: LanguageCode = LanguageCode {
    name: "Ninggerum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxr",
    individual_languages: &[
    ],
};


pub const NXX: LanguageCode = LanguageCode {
    name: "Nafri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nxx",
    individual_languages: &[
    ],
};


pub const NYB: LanguageCode = LanguageCode {
    name: "Nyangbo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyb",
    individual_languages: &[
    ],
};


pub const NYC: LanguageCode = LanguageCode {
    name: "Nyanga-li",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyc",
    individual_languages: &[
    ],
};


pub const NYD: LanguageCode = LanguageCode {
    name: "Nyore",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyd",
    individual_languages: &[
    ],
};


pub const NYE: LanguageCode = LanguageCode {
    name: "Nyengo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nye",
    individual_languages: &[
    ],
};


pub const NYF: LanguageCode = LanguageCode {
    name: "Giryama",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyf",
    individual_languages: &[
    ],
};


pub const NYG: LanguageCode = LanguageCode {
    name: "Nyindu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyg",
    individual_languages: &[
    ],
};


pub const NYH: LanguageCode = LanguageCode {
    name: "Nyikina",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyh",
    individual_languages: &[
    ],
};


pub const NYI: LanguageCode = LanguageCode {
    name: "Ama (Sudan)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyi",
    individual_languages: &[
    ],
};


pub const NYJ: LanguageCode = LanguageCode {
    name: "Nyanga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyj",
    individual_languages: &[
    ],
};


pub const NYK: LanguageCode = LanguageCode {
    name: "Nyaneka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyk",
    individual_languages: &[
    ],
};


pub const NYL: LanguageCode = LanguageCode {
    name: "Nyeu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyl",
    individual_languages: &[
    ],
};


pub const NYP: LanguageCode = LanguageCode {
    name: "Nyang'i",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyp",
    individual_languages: &[
    ],
};


pub const NYQ: LanguageCode = LanguageCode {
    name: "Nayini",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyq",
    individual_languages: &[
    ],
};


pub const NYR: LanguageCode = LanguageCode {
    name: "Nyiha (Malawi)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyr",
    individual_languages: &[
    ],
};


pub const NYS: LanguageCode = LanguageCode {
    name: "Nyungar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nys",
    individual_languages: &[
    ],
};


pub const NYT: LanguageCode = LanguageCode {
    name: "Nyawaygi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyt",
    individual_languages: &[
    ],
};


pub const NYU: LanguageCode = LanguageCode {
    name: "Nyungwe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyu",
    individual_languages: &[
    ],
};


pub const NYV: LanguageCode = LanguageCode {
    name: "Nyulnyul",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyv",
    individual_languages: &[
    ],
};


pub const NYW: LanguageCode = LanguageCode {
    name: "Nyaw",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyw",
    individual_languages: &[
    ],
};


pub const NYX: LanguageCode = LanguageCode {
    name: "Nganyaywana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyx",
    individual_languages: &[
    ],
};


pub const NYY: LanguageCode = LanguageCode {
    name: "Nyakyusa-Ngonde",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nyy",
    individual_languages: &[
    ],
};


pub const NZA: LanguageCode = LanguageCode {
    name: "Tigon Mbembe",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nza",
    individual_languages: &[
    ],
};


pub const NZB: LanguageCode = LanguageCode {
    name: "Njebi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nzb",
    individual_languages: &[
    ],
};


pub const NZD: LanguageCode = LanguageCode {
    name: "Nzadi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nzd",
    individual_languages: &[
    ],
};


pub const NZK: LanguageCode = LanguageCode {
    name: "Nzakara",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nzk",
    individual_languages: &[
    ],
};


pub const NZM: LanguageCode = LanguageCode {
    name: "Zeme Naga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nzm",
    individual_languages: &[
    ],
};


pub const NZS: LanguageCode = LanguageCode {
    name: "New Zealand Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nzs",
    individual_languages: &[
    ],
};


pub const NZU: LanguageCode = LanguageCode {
    name: "Teke-Nzikou",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nzu",
    individual_languages: &[
    ],
};


pub const NZY: LanguageCode = LanguageCode {
    name: "Nzakambay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nzy",
    individual_languages: &[
    ],
};


pub const NZZ: LanguageCode = LanguageCode {
    name: "Nanga Dama Dogon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "nzz",
    individual_languages: &[
    ],
};


pub const OAA: LanguageCode = LanguageCode {
    name: "Orok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oaa",
    individual_languages: &[
    ],
};


pub const OAC: LanguageCode = LanguageCode {
    name: "Oroch",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oac",
    individual_languages: &[
    ],
};


pub const OAR: LanguageCode = LanguageCode {
    name: "Old Aramaic (up to 700 BCE)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oar",
    individual_languages: &[
    ],
};


pub const OAV: LanguageCode = LanguageCode {
    name: "Old Avar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oav",
    individual_languages: &[
    ],
};


pub const OBI: LanguageCode = LanguageCode {
    name: "Obispeño",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "obi",
    individual_languages: &[
    ],
};


pub const OBK: LanguageCode = LanguageCode {
    name: "Southern Bontok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "obk",
    individual_languages: &[
    ],
};


pub const OBL: LanguageCode = LanguageCode {
    name: "Oblo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "obl",
    individual_languages: &[
    ],
};


pub const OBM: LanguageCode = LanguageCode {
    name: "Moabite",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "obm",
    individual_languages: &[
    ],
};


pub const OBO: LanguageCode = LanguageCode {
    name: "Obo Manobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "obo",
    individual_languages: &[
    ],
};


pub const OBR: LanguageCode = LanguageCode {
    name: "Old Burmese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "obr",
    individual_languages: &[
    ],
};


pub const OBT: LanguageCode = LanguageCode {
    name: "Old Breton",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "obt",
    individual_languages: &[
    ],
};


pub const OBU: LanguageCode = LanguageCode {
    name: "Obulom",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "obu",
    individual_languages: &[
    ],
};


pub const OCA: LanguageCode = LanguageCode {
    name: "Ocaina",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oca",
    individual_languages: &[
    ],
};


pub const OCH: LanguageCode = LanguageCode {
    name: "Old Chinese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "och",
    individual_languages: &[
    ],
};


pub const OCM: LanguageCode = LanguageCode {
    name: "Old Cham",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ocm",
    individual_languages: &[
    ],
};


pub const OCO: LanguageCode = LanguageCode {
    name: "Old Cornish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oco",
    individual_languages: &[
    ],
};


pub const OCU: LanguageCode = LanguageCode {
    name: "Atzingo Matlatzinca",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ocu",
    individual_languages: &[
    ],
};


pub const ODA: LanguageCode = LanguageCode {
    name: "Odut",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oda",
    individual_languages: &[
    ],
};


pub const ODK: LanguageCode = LanguageCode {
    name: "Od",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "odk",
    individual_languages: &[
    ],
};


pub const ODT: LanguageCode = LanguageCode {
    name: "Old Dutch",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "odt",
    individual_languages: &[
    ],
};


pub const ODU: LanguageCode = LanguageCode {
    name: "Odual",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "odu",
    individual_languages: &[
    ],
};


pub const OFO: LanguageCode = LanguageCode {
    name: "Ofo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ofo",
    individual_languages: &[
    ],
};


pub const OFS: LanguageCode = LanguageCode {
    name: "Old Frisian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ofs",
    individual_languages: &[
    ],
};


pub const OFU: LanguageCode = LanguageCode {
    name: "Efutop",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ofu",
    individual_languages: &[
    ],
};


pub const OGB: LanguageCode = LanguageCode {
    name: "Ogbia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ogb",
    individual_languages: &[
    ],
};


pub const OGC: LanguageCode = LanguageCode {
    name: "Ogbah",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ogc",
    individual_languages: &[
    ],
};


pub const OGE: LanguageCode = LanguageCode {
    name: "Old Georgian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oge",
    individual_languages: &[
    ],
};


pub const OGG: LanguageCode = LanguageCode {
    name: "Ogbogolo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ogg",
    individual_languages: &[
    ],
};


pub const OGO: LanguageCode = LanguageCode {
    name: "Khana",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ogo",
    individual_languages: &[
    ],
};


pub const OGU: LanguageCode = LanguageCode {
    name: "Ogbronuagum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ogu",
    individual_languages: &[
    ],
};


pub const OHT: LanguageCode = LanguageCode {
    name: "Old Hittite",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oht",
    individual_languages: &[
    ],
};


pub const OHU: LanguageCode = LanguageCode {
    name: "Old Hungarian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ohu",
    individual_languages: &[
    ],
};


pub const OIA: LanguageCode = LanguageCode {
    name: "Oirata",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oia",
    individual_languages: &[
    ],
};


pub const OIE: LanguageCode = LanguageCode {
    name: "Okolie",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oie",
    individual_languages: &[
    ],
};


pub const OIN: LanguageCode = LanguageCode {
    name: "Inebu One",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oin",
    individual_languages: &[
    ],
};


pub const OJB: LanguageCode = LanguageCode {
    name: "Northwestern Ojibwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ojb",
    individual_languages: &[
    ],
};


pub const OJC: LanguageCode = LanguageCode {
    name: "Central Ojibwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ojc",
    individual_languages: &[
    ],
};


pub const OJG: LanguageCode = LanguageCode {
    name: "Eastern Ojibwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ojg",
    individual_languages: &[
    ],
};


pub const OJP: LanguageCode = LanguageCode {
    name: "Old Japanese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ojp",
    individual_languages: &[
    ],
};


pub const OJS: LanguageCode = LanguageCode {
    name: "Severn Ojibwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ojs",
    individual_languages: &[
    ],
};


pub const OJV: LanguageCode = LanguageCode {
    name: "Ontong Java",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ojv",
    individual_languages: &[
    ],
};


pub const OJW: LanguageCode = LanguageCode {
    name: "Western Ojibwa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ojw",
    individual_languages: &[
    ],
};


pub const OKA: LanguageCode = LanguageCode {
    name: "Okanagan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oka",
    individual_languages: &[
    ],
};


pub const OKB: LanguageCode = LanguageCode {
    name: "Okobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okb",
    individual_languages: &[
    ],
};


pub const OKC: LanguageCode = LanguageCode {
    name: "Kobo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okc",
    individual_languages: &[
    ],
};


pub const OKD: LanguageCode = LanguageCode {
    name: "Okodia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okd",
    individual_languages: &[
    ],
};


pub const OKE: LanguageCode = LanguageCode {
    name: "Okpe (Southwestern Edo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oke",
    individual_languages: &[
    ],
};


pub const OKG: LanguageCode = LanguageCode {
    name: "Koko Babangk",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okg",
    individual_languages: &[
    ],
};


pub const OKH: LanguageCode = LanguageCode {
    name: "Koresh-e Rostam",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okh",
    individual_languages: &[
    ],
};


pub const OKI: LanguageCode = LanguageCode {
    name: "Okiek",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oki",
    individual_languages: &[
    ],
};


pub const OKJ: LanguageCode = LanguageCode {
    name: "Oko-Juwoi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okj",
    individual_languages: &[
    ],
};


pub const OKK: LanguageCode = LanguageCode {
    name: "Kwamtim One",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okk",
    individual_languages: &[
    ],
};


pub const OKL: LanguageCode = LanguageCode {
    name: "Old Kentish Sign Language",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okl",
    individual_languages: &[
    ],
};


pub const OKM: LanguageCode = LanguageCode {
    name: "Middle Korean (10th-16th cent.)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okm",
    individual_languages: &[
    ],
};


pub const OKN: LanguageCode = LanguageCode {
    name: "Oki-No-Erabu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okn",
    individual_languages: &[
    ],
};


pub const OKO: LanguageCode = LanguageCode {
    name: "Old Korean (3rd-9th cent.)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oko",
    individual_languages: &[
    ],
};


pub const OKR: LanguageCode = LanguageCode {
    name: "Kirike",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okr",
    individual_languages: &[
    ],
};


pub const OKS: LanguageCode = LanguageCode {
    name: "Oko-Eni-Osayen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oks",
    individual_languages: &[
    ],
};


pub const OKU: LanguageCode = LanguageCode {
    name: "Oku",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oku",
    individual_languages: &[
    ],
};


pub const OKV: LanguageCode = LanguageCode {
    name: "Orokaiva",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okv",
    individual_languages: &[
    ],
};


pub const OKX: LanguageCode = LanguageCode {
    name: "Okpe (Northwestern Edo)",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okx",
    individual_languages: &[
    ],
};


pub const OKZ: LanguageCode = LanguageCode {
    name: "Old Khmer",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "okz",
    individual_languages: &[
    ],
};


pub const OLA: LanguageCode = LanguageCode {
    name: "Walungge",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ola",
    individual_languages: &[
    ],
};


pub const OLD: LanguageCode = LanguageCode {
    name: "Mochi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "old",
    individual_languages: &[
    ],
};


pub const OLE: LanguageCode = LanguageCode {
    name: "Olekha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ole",
    individual_languages: &[
    ],
};


pub const OLK: LanguageCode = LanguageCode {
    name: "Olkol",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "olk",
    individual_languages: &[
    ],
};


pub const OLM: LanguageCode = LanguageCode {
    name: "Oloma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "olm",
    individual_languages: &[
    ],
};


pub const OLO: LanguageCode = LanguageCode {
    name: "Livvi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "olo",
    individual_languages: &[
    ],
};


pub const OLR: LanguageCode = LanguageCode {
    name: "Olrat",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "olr",
    individual_languages: &[
    ],
};


pub const OLT: LanguageCode = LanguageCode {
    name: "Old Lithuanian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "olt",
    individual_languages: &[
    ],
};


pub const OLU: LanguageCode = LanguageCode {
    name: "Kuvale",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "olu",
    individual_languages: &[
    ],
};


pub const OMA: LanguageCode = LanguageCode {
    name: "Omaha-Ponca",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oma",
    individual_languages: &[
    ],
};


pub const OMB: LanguageCode = LanguageCode {
    name: "East Ambae",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omb",
    individual_languages: &[
    ],
};


pub const OMC: LanguageCode = LanguageCode {
    name: "Mochica",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omc",
    individual_languages: &[
    ],
};


pub const OMG: LanguageCode = LanguageCode {
    name: "Omagua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omg",
    individual_languages: &[
    ],
};


pub const OMI: LanguageCode = LanguageCode {
    name: "Omi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omi",
    individual_languages: &[
    ],
};


pub const OMK: LanguageCode = LanguageCode {
    name: "Omok",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omk",
    individual_languages: &[
    ],
};


pub const OML: LanguageCode = LanguageCode {
    name: "Ombo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oml",
    individual_languages: &[
    ],
};


pub const OMN: LanguageCode = LanguageCode {
    name: "Minoan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omn",
    individual_languages: &[
    ],
};


pub const OMO: LanguageCode = LanguageCode {
    name: "Utarmbung",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omo",
    individual_languages: &[
    ],
};


pub const OMP: LanguageCode = LanguageCode {
    name: "Old Manipuri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omp",
    individual_languages: &[
    ],
};


pub const OMR: LanguageCode = LanguageCode {
    name: "Old Marathi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omr",
    individual_languages: &[
    ],
};


pub const OMT: LanguageCode = LanguageCode {
    name: "Omotik",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omt",
    individual_languages: &[
    ],
};


pub const OMU: LanguageCode = LanguageCode {
    name: "Omurano",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omu",
    individual_languages: &[
    ],
};


pub const OMW: LanguageCode = LanguageCode {
    name: "South Tairora",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omw",
    individual_languages: &[
    ],
};


pub const OMX: LanguageCode = LanguageCode {
    name: "Old Mon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omx",
    individual_languages: &[
    ],
};


pub const OMY: LanguageCode = LanguageCode {
    name: "Old Malay",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "omy",
    individual_languages: &[
    ],
};


pub const ONA: LanguageCode = LanguageCode {
    name: "Ona",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ona",
    individual_languages: &[
    ],
};


pub const ONB: LanguageCode = LanguageCode {
    name: "Lingao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "onb",
    individual_languages: &[
    ],
};


pub const ONE: LanguageCode = LanguageCode {
    name: "Oneida",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "one",
    individual_languages: &[
    ],
};


pub const ONG: LanguageCode = LanguageCode {
    name: "Olo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ong",
    individual_languages: &[
    ],
};


pub const ONI: LanguageCode = LanguageCode {
    name: "Onin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oni",
    individual_languages: &[
    ],
};


pub const ONJ: LanguageCode = LanguageCode {
    name: "Onjob",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "onj",
    individual_languages: &[
    ],
};


pub const ONK: LanguageCode = LanguageCode {
    name: "Kabore One",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "onk",
    individual_languages: &[
    ],
};


pub const ONN: LanguageCode = LanguageCode {
    name: "Onobasulu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "onn",
    individual_languages: &[
    ],
};


pub const ONO: LanguageCode = LanguageCode {
    name: "Onondaga",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ono",
    individual_languages: &[
    ],
};


pub const ONP: LanguageCode = LanguageCode {
    name: "Sartang",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "onp",
    individual_languages: &[
    ],
};


pub const ONR: LanguageCode = LanguageCode {
    name: "Northern One",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "onr",
    individual_languages: &[
    ],
};


pub const ONS: LanguageCode = LanguageCode {
    name: "Ono",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ons",
    individual_languages: &[
    ],
};


pub const ONT: LanguageCode = LanguageCode {
    name: "Ontenu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ont",
    individual_languages: &[
    ],
};


pub const ONU: LanguageCode = LanguageCode {
    name: "Unua",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "onu",
    individual_languages: &[
    ],
};


pub const ONW: LanguageCode = LanguageCode {
    name: "Old Nubian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "onw",
    individual_languages: &[
    ],
};


pub const ONX: LanguageCode = LanguageCode {
    name: "Onin Based Pidgin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "onx",
    individual_languages: &[
    ],
};


pub const OOD: LanguageCode = LanguageCode {
    name: "Tohono O'odham",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ood",
    individual_languages: &[
    ],
};


pub const OOG: LanguageCode = LanguageCode {
    name: "Ong",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oog",
    individual_languages: &[
    ],
};


pub const OON: LanguageCode = LanguageCode {
    name: "Önge",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oon",
    individual_languages: &[
    ],
};


pub const OOR: LanguageCode = LanguageCode {
    name: "Oorlams",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oor",
    individual_languages: &[
    ],
};


pub const OOS: LanguageCode = LanguageCode {
    name: "Old Ossetic",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oos",
    individual_languages: &[
    ],
};


pub const OPA: LanguageCode = LanguageCode {
    name: "Okpamheri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "opa",
    individual_languages: &[
    ],
};


pub const OPK: LanguageCode = LanguageCode {
    name: "Kopkaka",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "opk",
    individual_languages: &[
    ],
};


pub const OPM: LanguageCode = LanguageCode {
    name: "Oksapmin",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "opm",
    individual_languages: &[
    ],
};


pub const OPO: LanguageCode = LanguageCode {
    name: "Opao",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "opo",
    individual_languages: &[
    ],
};


pub const OPT: LanguageCode = LanguageCode {
    name: "Opata",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "opt",
    individual_languages: &[
    ],
};


pub const OPY: LanguageCode = LanguageCode {
    name: "Ofayé",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "opy",
    individual_languages: &[
    ],
};


pub const ORA: LanguageCode = LanguageCode {
    name: "Oroha",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ora",
    individual_languages: &[
    ],
};


pub const ORC: LanguageCode = LanguageCode {
    name: "Orma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "orc",
    individual_languages: &[
    ],
};


pub const ORE: LanguageCode = LanguageCode {
    name: "Orejón",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ore",
    individual_languages: &[
    ],
};


pub const ORG: LanguageCode = LanguageCode {
    name: "Oring",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "org",
    individual_languages: &[
    ],
};


pub const ORH: LanguageCode = LanguageCode {
    name: "Oroqen",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "orh",
    individual_languages: &[
    ],
};


pub const ORN: LanguageCode = LanguageCode {
    name: "Orang Kanaq",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "orn",
    individual_languages: &[
    ],
};


pub const ORO: LanguageCode = LanguageCode {
    name: "Orokolo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oro",
    individual_languages: &[
    ],
};


pub const ORR: LanguageCode = LanguageCode {
    name: "Oruma",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "orr",
    individual_languages: &[
    ],
};


pub const ORS: LanguageCode = LanguageCode {
    name: "Orang Seletar",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ors",
    individual_languages: &[
    ],
};


pub const ORT: LanguageCode = LanguageCode {
    name: "Adivasi Oriya",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ort",
    individual_languages: &[
    ],
};


pub const ORU: LanguageCode = LanguageCode {
    name: "Ormuri",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oru",
    individual_languages: &[
    ],
};


pub const ORV: LanguageCode = LanguageCode {
    name: "Old Russian",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "orv",
    individual_languages: &[
    ],
};


pub const ORW: LanguageCode = LanguageCode {
    name: "Oro Win",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "orw",
    individual_languages: &[
    ],
};


pub const ORX: LanguageCode = LanguageCode {
    name: "Oro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "orx",
    individual_languages: &[
    ],
};


pub const ORY: LanguageCode = LanguageCode {
    name: "Odia",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ory",
    individual_languages: &[
    ],
};


pub const ORZ: LanguageCode = LanguageCode {
    name: "Ormu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "orz",
    individual_languages: &[
    ],
};


pub const OSC: LanguageCode = LanguageCode {
    name: "Oscan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "osc",
    individual_languages: &[
    ],
};


pub const OSI: LanguageCode = LanguageCode {
    name: "Osing",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "osi",
    individual_languages: &[
    ],
};


pub const OSN: LanguageCode = LanguageCode {
    name: "Old Sundanese",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "osn",
    individual_languages: &[
    ],
};


pub const OSO: LanguageCode = LanguageCode {
    name: "Ososo",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oso",
    individual_languages: &[
    ],
};


pub const OSP: LanguageCode = LanguageCode {
    name: "Old Spanish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "osp",
    individual_languages: &[
    ],
};


pub const OST: LanguageCode = LanguageCode {
    name: "Osatu",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ost",
    individual_languages: &[
    ],
};


pub const OSU: LanguageCode = LanguageCode {
    name: "Southern One",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "osu",
    individual_languages: &[
    ],
};


pub const OSX: LanguageCode = LanguageCode {
    name: "Old Saxon",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "osx",
    individual_languages: &[
    ],
};


pub const OTB: LanguageCode = LanguageCode {
    name: "Old Tibetan",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "otb",
    individual_languages: &[
    ],
};


pub const OTD: LanguageCode = LanguageCode {
    name: "Ot Danum",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "otd",
    individual_languages: &[
    ],
};


pub const OTE: LanguageCode = LanguageCode {
    name: "Mezquital Otomi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ote",
    individual_languages: &[
    ],
};


pub const OTI: LanguageCode = LanguageCode {
    name: "Oti",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oti",
    individual_languages: &[
    ],
};


pub const OTK: LanguageCode = LanguageCode {
    name: "Old Turkish",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "otk",
    individual_languages: &[
    ],
};


pub const OTL: LanguageCode = LanguageCode {
    name: "Tilapa Otomi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "otl",
    individual_languages: &[
    ],
};


pub const OTM: LanguageCode = LanguageCode {
    name: "Eastern Highland Otomi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "otm",
    individual_languages: &[
    ],
};


pub const OTN: LanguageCode = LanguageCode {
    name: "Tenango Otomi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "otn",
    individual_languages: &[
    ],
};


pub const OTQ: LanguageCode = LanguageCode {
    name: "Querétaro Otomi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "otq",
    individual_languages: &[
    ],
};


pub const OTR: LanguageCode = LanguageCode {
    name: "Otoro",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "otr",
    individual_languages: &[
    ],
};


pub const OTS: LanguageCode = LanguageCode {
    name: "Estado de México Otomi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ots",
    individual_languages: &[
    ],
};


pub const OTT: LanguageCode = LanguageCode {
    name: "Temoaya Otomi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "ott",
    individual_languages: &[
    ],
};


pub const OTU: LanguageCode = LanguageCode {
    name: "Otuke",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "otu",
    individual_languages: &[
    ],
};


pub const OTW: LanguageCode = LanguageCode {
    name: "Ottawa",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "otw",
    individual_languages: &[
    ],
};


pub const OTX: LanguageCode = LanguageCode {
    name: "Texcatepec Otomi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "otx",
    individual_languages: &[
    ],
};


pub const OTY: LanguageCode = LanguageCode {
    name: "Old Tamil",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oty",
    individual_languages: &[
    ],
};


pub const OTZ: LanguageCode = LanguageCode {
    name: "Ixtenco Otomi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "otz",
    individual_languages: &[
    ],
};


pub const OUA: LanguageCode = LanguageCode {
    name: "Tagargrent",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oua",
    individual_languages: &[
    ],
};


pub const OUB: LanguageCode = LanguageCode {
    name: "Glio-Oubi",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oub",
    individual_languages: &[
    ],
};


pub const OUE: LanguageCode = LanguageCode {
    name: "Oune",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oue",
    individual_languages: &[
    ],
};


pub const OUI: LanguageCode = LanguageCode {
    name: "Old Uighur",
    code: "",
    code_2t: "",
    code_2b: "",
    code_3: "oui",
    individual_languages: &[
    ],
};


