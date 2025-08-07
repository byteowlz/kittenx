use anyhow::Result;
use espeak_rs::text_to_phonemes;
use whatlang::{detect, Lang};

pub fn detect_language(text: &str) -> Option<String> {
    if let Some(info) = detect(text) {
        let lang_code = match info.lang() {
            Lang::Eng => "en-us",
            Lang::Spa => "es",
            Lang::Fra => "fr",
            Lang::Deu => "de",
            Lang::Ita => "it",
            Lang::Por => "pt",
            Lang::Rus => "ru",
            Lang::Jpn => "ja",
            Lang::Kor => "ko",
            Lang::Cmn => "zh",
            _ => "en-us",
        };
        Some(lang_code.to_string())
    } else {
        None
    }
}

pub fn text_to_phonemes_simple(text: &str, language: &str) -> Result<String> {
    // Use espeak for phonemization with preserve_punctuation=true and with_stress=true
    // to match the Python implementation
    match text_to_phonemes(text, language, None, true, true) {
        Ok(phonemes) => {
            let phonemes_str = phonemes.join("");
            // Apply the same tokenization as Python's basic_english_tokenize
            let tokens = basic_english_tokenize(&phonemes_str);
            Ok(tokens.join(" "))
        },
        Err(_) => {
            // Fallback: use basic tokenization on original text
            let tokens = basic_english_tokenize(text);
            Ok(tokens.join(" "))
        }
    }
}

pub fn basic_english_tokenize(text: &str) -> Vec<String> {
    use regex::Regex;
    // Match Python's basic_english_tokenize: r"\w+|[^\w\s]"
    let re = Regex::new(r"\w+|[^\w\s]").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

pub fn basic_tokenize(text: &str) -> Vec<String> {
    basic_english_tokenize(text)
}