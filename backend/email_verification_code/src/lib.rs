use std::sync::LazyLock;

use rand::Rng;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

static VERIFY_EMAIL_CODE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Z0-9]{6}$").unwrap()
});

const ALPHANUMERIC_UPPERCASE: [char; 36] = [
    'A',
    'B',
    'C',
    'D',
    'E',
    'F',
    'G',
    'H',
    'I',
    'J',
    'K',
    'L',
    'M',
    'N',
    'O',
    'P',
    'Q',
    'R',
    'S',
    'T',
    'U',
    'V',
    'W',
    'X',
    'Y',
    'Z',
    '0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
];

fn random_6_chars() -> String {
    let mut rng = rand::rng();
    (0..6)
        .map(|_| ALPHANUMERIC_UPPERCASE[rng.random_range(0..ALPHANUMERIC_UPPERCASE.len())])
        .collect()
}

#[derive(Debug, PartialEq, Validate, Deserialize, Clone)]
pub struct EmailVerificationCode {
    #[validate(regex(path = *VERIFY_EMAIL_CODE))]
    pub code: String,
}

impl Default for EmailVerificationCode {
    fn default() -> Self {
        Self {
            code: random_6_chars(),
        }
    }
}

impl EmailVerificationCode {
    pub fn from_str(code: String) -> Result<Self, String> {
        let s = Self {
            code,
        };

        if s.validate().is_err() {
            return Err("Invalid code".into());
        }

        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::EmailVerificationCode;

    #[test]
    fn should_be_able_to_generate_random_6_char_code() {
        let email_verification_code_1 = EmailVerificationCode::default();
        assert_eq!(email_verification_code_1.code.len(), 6);

        let email_verification_code_2 = EmailVerificationCode::default();
        assert_eq!(email_verification_code_2.code.len(), 6);

        // Can be flaky, but low chance.
        // 36^6 possibilities, which comes out to about 2.17 billion unique combinations.
        assert_ne!(email_verification_code_1, email_verification_code_2);
    }

    #[test]
    fn should_succeed_from_str_code() {
        assert!(EmailVerificationCode::from_str("AAAAAA".into()).is_ok());
        assert!(EmailVerificationCode::from_str("ZZZZZZ".into()).is_ok());
        assert!(EmailVerificationCode::from_str("123ABC".into()).is_ok());
        assert!(EmailVerificationCode::from_str("ABC123".into()).is_ok());
        assert!(EmailVerificationCode::from_str("123456".into()).is_ok());
    }

    #[test]
    fn should_fail_from_str_code() {
        assert!(EmailVerificationCode::from_str("A".into()).is_err());
        assert!(EmailVerificationCode::from_str("AAAAA".into()).is_err());
        assert!(EmailVerificationCode::from_str("AAAAAAA".into()).is_err());
        assert!(EmailVerificationCode::from_str("aaaaaa".into()).is_err());
        assert!(EmailVerificationCode::from_str("zzzzzz".into()).is_err());
        assert!(EmailVerificationCode::from_str("!!!!!!".into()).is_err());
    }
}
