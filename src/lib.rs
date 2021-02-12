use std::{cmp, fmt::Display, hint::unreachable_unchecked};

use regex::{Regex, RegexBuilder, RegexSet, RegexSetBuilder};

static WORD_PATTERN: &str = r"\b(\p{L}+(?:[-']\p{L}+)?)\b";
static SENTENCE_PATTERN: &str = r"[.?!]+";
static VOWEL_GROUP_PATTERN: &str = r"[aeiou]+";

#[derive(Clone, Debug)]
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

    pub fn scorer(&self) -> Scorer {
        Scorer::new(self)
    }

    pub fn reading_ease(&self, text: &str) -> ReadingEase {
        let mut score = self.scorer();
        score.add(text);
        score.reading_ease()
    }

    pub fn grade_level(&self, text: &str) -> GradeLevel {
        let mut score = self.scorer();
        score.add(text);
        score.grade_level()
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

impl Default for Kincaid {
    fn default() -> Self {
        Kincaid::new()
    }
}

/// Allows scoring of multi-part text.
pub struct Scorer<'a> {
    kincaid: &'a Kincaid,
    words: usize,
    syllables: usize,
    sentences: usize,
}

impl<'a> Scorer<'a> {
    pub fn new(kincaid: &'a Kincaid) -> Self {
        Self {
            kincaid,
            words: 0,
            syllables: 0,
            sentences: 0,
        }
    }

    /// Add a text.
    pub fn add(&mut self, text: &str) {
        self.words += self.kincaid.word_count(text);
        self.syllables += self.kincaid.syllable_count(text);
        self.sentences += self.kincaid.sentence_count(text);
    }

    /// Calculate grade level.
    pub fn grade_level(&self) -> GradeLevel {
        let grade_level = 0.39 * (self.words as f64 / self.sentences as f64)
            + 11.8 * (self.syllables as f64 / self.words as f64)
            - 15.9;

        // Ord is not implemented for f64, so cmp::max does not work.
        if grade_level < 1.0 {
            GradeLevel(1.0)
        } else {
            GradeLevel(grade_level)
        }
    }

    /// Calculate reading ease.
    pub fn reading_ease(&self) -> ReadingEase {
        // I have chosen to clamp this value because the score isn't
        // particularly meaningful outside this range. Also, clamp
        // is kind of a new feature. /shrug
        ReadingEase(
            (206.835
                - 1.015 * self.words as f64 / self.sentences as f64
                - 84.6 * self.syllables as f64 / self.words as f64)
                .clamp(0.0, 100.0),
        )
    }
}

pub struct ReadingEase(f64);

impl ReadingEase {
    /// Provide descriptors for this reading level.
    ///
    /// The first descriptor is short; the second is more detailed.
    pub fn description(self) -> (&'static str, &'static str) {
        if (0.0..10.0).contains(&self.0) {
            return (
                "Professional",
                "Extremely difficult to read. Best understood by university graduates.",
            );
        }

        if (10.0..30.0).contains(&self.0) {
            return (
                "College graduate",
                "Very difficult to read. Best understood by university graduates.",
            );
        }

        if (30.0..50.0).contains(&self.0) {
            return ("College", "Difficult to read.");
        }

        if (50.0..60.0).contains(&self.0) {
            return ("10th to 12th grade", "Fairly difficult to read.");
        }

        if (60.0..70.0).contains(&self.0) {
            return (
                "8th & 9th grade",
                "Plain English. Easily understood by 13- to 15-year-old students.",
            );
        }

        if (70.0..80.0).contains(&self.0) {
            return ("7th grade", "Fairly easy to read.");
        }

        if (80.0..90.0).contains(&self.0) {
            return (
                "6th grade",
                "Easy to read. Conversational English for consumers.",
            );
        }

        if (90.00..=100.00).contains(&self.0) {
            return (
                "5th grade",
                "Very easy to read. Easily understood by an average 11-year-old student.",
            );
        }

        // Based on my in-depth (mis)understanding of floating point values,
        // I conclude that this point can only be reached on Halloween, and
        // only during a thunderstorm.
        unsafe {
            unreachable_unchecked();
        }
    }
}

impl Display for ReadingEase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.01}", self.0)
    }
}

pub struct GradeLevel(f64);

impl GradeLevel {
    pub fn description(&self) -> String {
        match self.0.trunc() as i32 {
            1 => String::from("1st grade"),
            2 => String::from("2nd grade"),
            3 => String::from("3rd grade"),
            n => format!("{}th grade", n),
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
