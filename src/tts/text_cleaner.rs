use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref SYMBOL_TO_ID: HashMap<char, i64> = {
        let pad = "$";
        let punctuation = ";:,.!?¡¿—…\"«»\"\" ";
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let letters_ipa = "ɑɐɒæɓʙβɔɕçɗɖðʤəɘɚɛɜɝɞɟʄɡɠɢʛɦɧħɥʜɨɪʝɭɬɫɮʟɱɯɰŋɳɲɴøɵɸθœɶʘɹɺɾɻʀʁɽʂʃʈʧʉʊʋⱱʌɣɤʍχʎʏʑʐʒʔʡʕʢǀǁǂǃˈˌːˑʼʴʰʱʲʷˠˤ˞↓↑→↗↘'̩'ᵻ";

        let mut symbols = Vec::new();
        symbols.push(pad.chars().next().unwrap());
        symbols.extend(punctuation.chars());
        symbols.extend(letters.chars());
        symbols.extend(letters_ipa.chars());

        let mut map = HashMap::new();
        for (i, symbol) in symbols.iter().enumerate() {
            map.insert(*symbol, i as i64);
        }
        map
    };
}

pub struct TextCleaner;

impl TextCleaner {
    pub fn new() -> Self {
        Self
    }

    pub fn clean(&self, text: &str) -> Vec<i64> {
        let mut tokens = Vec::new();
        for ch in text.chars() {
            if let Some(&token_id) = SYMBOL_TO_ID.get(&ch) {
                tokens.push(token_id);
            }
        }
        tokens
    }
}

impl Default for TextCleaner {
    fn default() -> Self {
        Self::new()
    }
}