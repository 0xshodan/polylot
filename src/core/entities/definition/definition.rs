use anyhow::{bail, Result};

pub struct Definition {
    pub translate: Option<String>,
    pub picture: Option<String>, // TODO: URL Type
    pub meaning: Option<String>,
    pub meaning_native: Option<String>,
    pub use_cases: Option<Vec<String>>,
}

impl Definition {
    pub fn new(
        self,
        translate: Option<String>,
        picture: Option<String>,
        meaning: Option<String>,
        meaning_native: Option<String>,
        use_cases: Option<Vec<String>>,
    ) -> Result<Self> {
        if translate.is_none()
            && picture.is_none()
            && meaning.is_none()
            && meaning_native.is_none()
            && use_cases.is_none()
        {
            bail!("No fields are specified");
        }
        Ok(Definition {
            translate: translate,
            picture: picture,
            meaning: meaning,
            meaning_native: meaning_native,
            use_cases: use_cases,
        })
    }
}
