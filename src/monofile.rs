use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf, str::FromStr};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Monofile {
    pub sync_files: Vec<SyncFiles>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SyncFiles {
    pub from: Glob,
    pub to: InterpolatedString,
}

#[derive(Deserialize, Debug)]
pub struct Glob(pub PathBuf);

#[derive(Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct InterpolatedString {
    segments: Vec<Segment>,
}

impl TryFrom<String> for InterpolatedString {
    type Error = InterpolatedStringError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl FromStr for InterpolatedString {
    type Err = InterpolatedStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rest = s;
        let mut segments = Vec::new();

        while let Some((pre, post)) = rest.split_once("${") {
            segments.push(Segment::String(pre.to_string()));
            let (expr, new_rest) = post
                .split_once("}")
                .ok_or(InterpolatedStringError::MissingClosingBrace)?;
            segments.push(Segment::Expr(expr.trim().to_string()));
            rest = new_rest;
        }
        segments.push(Segment::String(rest.to_string()));

        Ok(InterpolatedString { segments })
    }
}

impl InterpolatedString {
    pub fn interpolate(
        &self,
        map: &HashMap<String, String>,
    ) -> Result<String, InterpolatedStringError> {
        Ok(self
            .segments
            .iter()
            .map(|seg| {
                Ok(match seg {
                    Segment::String(s) => s.to_string(),
                    Segment::Expr(expr) => map
                        .get(expr)
                        .ok_or_else(|| InterpolatedStringError::MissingVar(expr.to_string()))?
                        .to_string(),
                })
            })
            .collect::<Result<String, _>>()?)
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum InterpolatedStringError {
    #[error("unknown interpolation variable {0}")]
    MissingVar(String),

    #[error("missing closing brace")]
    MissingClosingBrace,
}

#[derive(Debug)]
enum Segment {
    String(String),
    Expr(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::*;

    #[cfg(test)]
    mod interpolated_path {
        use super::*;

        #[test]
        fn just_string() {
            assert_eq!(
                &"just_string"
                    .parse::<InterpolatedString>()
                    .unwrap()
                    .interpolate(hashmap! {})
                    .unwrap(),
                "just_string"
            );
        }

        #[test]
        fn single_var() {
            assert_eq!(
                &"a_string_${ var }"
                    .parse::<InterpolatedString>()
                    .unwrap()
                    .interpolate(hashmap! {
                        "var".to_string() => "with_var".to_string(),
                    })
                    .unwrap(),
                "a_string_with_var"
            );
        }

        #[test]
        fn missing_var() {
            assert_eq!(
                "a_string_${ var }"
                    .parse::<InterpolatedString>()
                    .unwrap()
                    .interpolate(hashmap! {}),
                Err(InterpolatedStringError::MissingVar("var".to_string())),
            );
        }

        #[test]
        fn no_closing_brace() {
            assert_eq!(
                "a_string_${ var".parse::<InterpolatedString>().unwrap_err(),
                InterpolatedStringError::MissingClosingBrace
            );
        }
    }
}
