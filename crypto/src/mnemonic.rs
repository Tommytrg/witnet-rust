//! # BIP39 Mnemonic and Seed generation
//!
//! Example
//!
//! ```
//! # use witnet_crypto::mnemonic::MnemonicGenerator;
//! let mnemonic = MnemonicGenerator::new().generate();
//!
//! // A Mnemonic Seed must be protected by a passphrase
//! let passphrase = "".into();
//!
//! // String of mnemonic words
//! let words = mnemonic.words();
//! // Seed that can be used to generate a master secret key
//! let seed = mnemonic.seed(&passphrase);
//! ```

use rand::RngCore;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use witnet_protected::ProtectedString;

/// BIP39 Mnemonic
pub struct Mnemonic(bip39::Mnemonic);

impl Mnemonic {
    /// Return a Mnemonic builder.
    pub fn build() -> MnemonicGenerator {
        MnemonicGenerator::default()
    }

    /// Get the list of mnemonic words
    pub fn words(&self) -> &str {
        self.0.phrase()
    }

    /// Get the binary seed used for generating a master secret key
    pub fn seed(&self, passphrase: &ProtectedString) -> Seed {
        Seed(bip39::Seed::new(&self.0, passphrase.as_ref()))
    }

    /// Get the binary seed used for generating a master secret key
    pub fn seed_ref(&self, passphrase: &str) -> Seed {
        Seed(bip39::Seed::new(&self.0, passphrase))
    }

    /// Get a mnemonic from a existing phrase in English.
    pub fn from_phrase(phrase: ProtectedString) -> Result<Mnemonic, bip39::ErrorKind> {
        Self::from_phrase_lang(phrase, Lang::English)
    }

    /// Get a mnemonic from a existing phrase in English.
    pub fn from_phrase_ref(phrase: &str) -> Result<Mnemonic, bip39::ErrorKind> {
        Self::from_phrase_lang_ref(phrase, Lang::English)
    }

    /// Get a mnemonic from a existing phrase in another language.
    pub fn from_phrase_lang(
        phrase: ProtectedString,
        language: Lang,
    ) -> Result<Mnemonic, bip39::ErrorKind> {
        bip39::Mnemonic::from_phrase(AsRef::<str>::as_ref(&phrase), language.into()).map(Mnemonic)
    }

    /// Get a mnemonic from a existing phrase in another language.
    pub fn from_phrase_lang_ref(
        phrase: &str,
        language: Lang,
    ) -> Result<Mnemonic, bip39::ErrorKind> {
        bip39::Mnemonic::from_phrase(phrase, language.into()).map(Mnemonic)
    }
}

/// BIP39 Seed generated from a Mnemonic
pub struct Seed(bip39::Seed);

impl Seed {
    /// serialize a seed
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// Number of words of the Mnemonic
///
/// The number of words of the Mnemonic is proportional to the
/// entropy:
///
/// * `128 bits` generates `12 words` mnemonic
/// * `160 bits` generates `15 words` mnemonic
/// * `192 bits` generates `18 words` mnemonic
/// * `224 bits` generates `21 words` mnemonic
/// * `256 bits` generates `24 words` mnemonic
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Length {
    /// 12 words length
    Words12,
    /// 15 words length
    Words15,
    /// 18 words length
    Words18,
    /// 21 words length
    Words21,
    /// 24 words length
    Words24,
}

/// The language in which Mnemonics are generated
#[derive(Debug, PartialEq, Eq)]
pub enum Lang {
    /// English language
    English,
}

impl From<Lang> for bip39::Language {
    fn from(lang: Lang) -> Self {
        match lang {
            Lang::English => bip39::Language::English,
        }
    }
}

/// BIP39 Mnemonic generator
pub struct MnemonicGenerator {
    len: Length,
    lang: Lang,
}
impl Default for MnemonicGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl MnemonicGenerator {
    /// Create a new BIP39 Mnemonic generator
    pub fn new() -> Self {
        MnemonicGenerator {
            len: Length::Words12,
            lang: Lang::English,
        }
    }

    /// Set how many words the Mnemonic should have
    pub fn with_len(mut self, len: Length) -> Self {
        self.len = len;
        self
    }

    /// Set which language to use in the Mnemonic words
    pub fn with_lang(mut self, lang: Lang) -> Self {
        self.lang = lang;
        self
    }

    /// Consume this generator and return the BIP39 Mnemonic
    pub fn generate(self) -> Mnemonic {
        let mnemonic_type = match self.len {
            Length::Words12 => bip39::MnemonicType::Words12,
            Length::Words15 => bip39::MnemonicType::Words15,
            Length::Words18 => bip39::MnemonicType::Words18,
            Length::Words21 => bip39::MnemonicType::Words21,
            Length::Words24 => bip39::MnemonicType::Words24,
        };
        let lang = match self.lang {
            Lang::English => bip39::Language::English,
        };

        // We handle the entropy directly here to avoid tiny-bip39's reliance on outdated rand lib
        let entropy = &mut vec![0u8; mnemonic_type.entropy_bits() / 8];
        rand::thread_rng().fill_bytes(entropy);
        let mnemonic = bip39::Mnemonic::from_entropy(entropy, lang).unwrap();

        Mnemonic(mnemonic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_default() {
        let generator = MnemonicGenerator::new();

        assert_eq!(generator.len, Length::Words12);
        assert_eq!(generator.lang, Lang::English);
    }

    #[test]
    fn test_gen_with_len() {
        let generator = MnemonicGenerator::new().with_len(Length::Words24);

        assert_eq!(generator.len, Length::Words24);
        assert_eq!(generator.lang, Lang::English);
    }

    #[test]
    fn test_generate() {
        let mnemonic = MnemonicGenerator::new().generate();
        let words = mnemonic.words().split_whitespace();

        assert_eq!(words.count(), 12);
    }

    #[test]
    fn test_seed_as_ref() {
        let mnemonic = MnemonicGenerator::new().generate();
        let seed = mnemonic.seed(&"".into());
        let bytes: &[u8] = seed.as_ref();

        assert_eq!(bytes, seed.as_bytes());
    }

    #[test]
    fn test_vectors() {
        for (phrase, expected_seed) in crate::test_vectors::TREZOR_MNEMONICS {
            let mnemonic = Mnemonic::from_phrase((*phrase).into()).unwrap();
            let seed = hex::encode(mnemonic.seed(&"TREZOR".into()));

            assert_eq!((*expected_seed).to_string(), seed);
        }
    }
}
