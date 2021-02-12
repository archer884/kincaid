use std::cmp;

use regex::{Regex, RegexBuilder, RegexSet, RegexSetBuilder};

static WORD_PATTERN: &str = r"\b(\p{L}+(?:[-']\p{L}+)?)\b";
static SENTENCE_PATTERN: &str = r"[.?!]+";
static VOWEL_GROUP_PATTERN: &str = r"[aeiou]+";

pub struct Kincaid {
    word: Regex,
    sentence: Regex,
    vowel_group: Regex,
    add: RegexSet,
    sub: RegexSet,
}

impl Kincaid {
    pub fn new() -> Self {
        static SUB_PATTERNS: &[&str] = &[
            r"e\b",
            r"ey\b",
            r"ed\b",
            r"ay\b",
            r"[kmrpbdtnvrw]es\b",
            r"ely\b",
            r"oy\b",
            r"cia",
            r"[aeilouy]le\b",
            r"tia[nl]?\b",
            r"tia([nl]s)?\b",
            r"[aeo]ym",
            r"eness\b",
            r"\bfore",
            r"ay[bclntrw]",
            r"ement",
            r"iles\b",
            r"[ao]les\b",
            r"eman\b",
            r"aying\b",
            r"oy[cln]",
            r"eful",
            r"\bey",
            r"geon",
            r"\bhome",
            r"eyn",
            r"ically",
            r"eless",
            r"sian\b",
            r"yles",
            r"\bwhite",
            r"eway",
            r"georg",
            r"lles\b",
            r"busine",
            r"illia",
            r"ules\b",
            r"\bhym",
            r"ryst",
            r"eyl",
            r"ehou",
            r"eyw",
            r"ekeep",
            r"people",
            r"every",
            r"\blife",
            r"giu",
            r"eyin",
            r"eout",
            r"oying\b",
            r"gues\b",
            r"\breine",
            r"geou",
            r"ques\b",
            r"vior",
            r"sewo",
            r"oseb",
            r"eyc",
            r"\bspace",
            r"\bstone",
            r"eover",
            r"ehol",
            r"iliar",
            r"estone",
            r"eyb",
            r"oyk",
            r"velan",
            r"piet",
            r"\bgia",
            r"somet",
            r"esvil",
            r"lyst",
            r"arriag",
            r"gior",
        ];

        static ADD_PATTERNS: &[&str] = &[
            r"y\b",
            r"ia",
            r"\bmc",
            r"[il]e\b",
            r"ted\b",
            r"ee\b",
            r"io\b",
            r"ded\b",
            r"[io]er\b",
            r"y[bckglmnrstwxv]",
            r"sms?\b",
            r"eo",
            r"[eior]ed\b",
            r"iol",
            r"\bhy",
            r"iu",
            r"s'",
            r"oe\b",
            r"iot",
            r"tua",
            r"aue",
            r"ea\b",
            r"iest\b",
            r"ios",
            r"yst",
            r"nte\b",
            r"ce's",
            r"ying\b",
            r"[bcdfgkopt]led\b",
            r"ciat",
            r"lement",
            r"typ",
            r"ly[dehops]",
            r"[drv]ious",
            r"z's\b",
            r"ae\b",
            r"io[mpr]",
            r"tre\b",
            r"ione\b",
            r"[cdehlorn]ue\b",
            r"se's",
            r"nua",
            r"x'",
            r"oing",
            r"yz",
            r"creat",
            r"lua",
            r"iod",
            r"\breass",
            r"eing\b",
            r"dua",
            r"[bdprz]ion",
            r"iello\b",
            r"oa\b",
            r"ge's",
            r"phys",
            r"eact",
            r"ioc",
            r"iog",
            r"scien",
            r"dys",
            r"uou",
            r"\brein",
            r"ienn",
            r"rya",
            r"bre\b",
            r"tke\b",
            r"ryd",
            r"sh's\b",
            r"rua",
            r"ryp",
            r"rient",
            r"uing",
            r"xual",
            r"eely\b",
            r"leman\b",
            r"fluen",
            r"he'",
            r"dre\b",
            r"iet",
            r"loui",
            r"dl\b",
            r"\bio",
            r"rys",
            r"tui",
            r"rye",
            r"\bcoe",
            r"\breali",
            r"ntes\b",
            r"ch'",
            r"mye",
            r"eeman\b",
            r"ryo",
            r"linea",
            r"theat",
            r"reapp",
            r"oers\b",
            r"tys",
            r"\bcyp",
            r"eemp",
            r"nys",
            r"aic\b",
            r"cua",
            r"tl\b",
            r"tres\b",
            r"ciano",
            r"lione",
            r"eand",
            r"\bdya",
            r"gyp",
            r"croat",
            r"heroi",
            r"rearr",
            r"eex",
            r"cre\b",
            r"oniou",
            r"eum\b",
            r"fred\b",
            r"dien",
            r"oua",
            r"oincid",
            r"coordi",
            r"nucle",
            r"nyd",
            r"\breen",
            r"\breun",
            r"bys",
            r"iale\b",
            r"ifiers",
            r"rean",
            r"pre\b",
            r"iore\b",
            r"-in\b",
        ];

        Self {
            word: build(WORD_PATTERN),
            sentence: build(SENTENCE_PATTERN),
            vowel_group: build(VOWEL_GROUP_PATTERN),
            add: build_set(ADD_PATTERNS),
            sub: build_set(SUB_PATTERNS),
        }
    }

    pub fn word_count(&self, text: &str) -> usize {
        self.word.find_iter(text).count()
    }

    pub fn sentence_count(&self, text: &str) -> usize {
        cmp::max(1, self.sentence.find_iter(text).count())
    }

    pub fn syllable_count(&self, text: &str) -> usize {
        self.word
            .find_iter(text)
            .map(|x| self.syllables_in_word(x.as_str()))
            .sum()
    }

    pub fn reading_ease_score(&self, text: &str) -> f64 {
        let words = self.word_count(text) as f64;
        let syllables = self.syllable_count(text) as f64;
        let sentences = self.sentence_count(text) as f64;

        206.835 - 1.015 * words / sentences - 84.6 * syllables / words
    }

    fn syllables_in_word(&self, text: &str) -> usize {
        let count = self.vowel_group.find_iter(text).count();
        let add = self.add.matches(text).iter().count();
        let sub = self.sub.matches(text).iter().count();

        // BLACK MAGIC.
        if count + add < sub + 1 {
            1
        } else {
            count + add - sub
        }
    }
}

fn build(pattern: &str) -> Regex {
    RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .unwrap()
}

fn build_set(patterns: &[&str]) -> RegexSet {
    RegexSetBuilder::new(patterns)
        .case_insensitive(true)
        .build()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_syllables_in_text() {
        let kincaid = Kincaid::new();
        assert_eq!(kincaid.syllable_count(""), 0);
        assert_eq!(kincaid.syllable_count("Hello"), 2);
        assert_eq!(kincaid.syllable_count("Hello World"), 3);
        assert_eq!(kincaid.syllable_count("Test-case"), 2);
        assert_eq!(kincaid.syllable_count("Hello, World! This is a test"), 7);
        assert_eq!(kincaid.syllable_count("Zylka"), 2);
    }

    #[test]
    fn test_syllables_in_word() {
        let kincaid = Kincaid::new();
        assert_eq!(kincaid.syllables_in_word("unaware"), 3);
        assert_eq!(kincaid.syllables_in_word("sum"), 1);
        assert_eq!(kincaid.syllables_in_word("some"), 1);
        assert_eq!(kincaid.syllables_in_word("pernicious"), 3);
        assert_eq!(kincaid.syllables_in_word("egregious"), 3);
    }

    #[test]
    fn test_word_count() {
        let kincaid = Kincaid::new();
        assert_eq!(kincaid.word_count("sample text"), 2);
        assert_eq!(kincaid.word_count("$5 only"), 1);
        assert_eq!(kincaid.word_count("This is noted in the book(1)"), 6);
        assert_eq!(
            kincaid.word_count(
                "A sentence that's long—very-long, and contains some real world stuff;funny!"
            ),
            12
        );
    }
}
