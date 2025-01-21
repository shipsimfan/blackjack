use crate::model::Ratio;
use argparse::{ArgumentSource, DefaultDisplay, Error, Flag, FlagInfo, Result};
use std::num::ParseIntError;

/// An error that can occur while parsing a ratio
#[derive(Debug)]
enum RatioParseError {
    MissingColon,
    MissingDenominator,
    UnexpectedCharacters,
    InvalidNumerator(ParseIntError),
    InvalidDenominator(ParseIntError),
}

impl Flag for Ratio {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let string = String::parse(source, &info.drop_default(), long)?;

        let mut parts = string.split(':');
        let numerator = parts
            .next()
            .ok_or(Error::invalid_flag_value(
                info,
                long,
                RatioParseError::MissingColon,
            ))?
            .parse()
            .map_err(|error| {
                Error::invalid_flag_value(info, long, RatioParseError::InvalidNumerator(error))
            })?;
        let denominator = parts
            .next()
            .ok_or(Error::invalid_flag_value(
                info,
                long,
                RatioParseError::MissingDenominator,
            ))?
            .parse()
            .map_err(|error| {
                Error::invalid_flag_value(info, long, RatioParseError::InvalidDenominator(error))
            })?;

        if parts.next().is_some() {
            return Err(Error::invalid_flag_value(
                info,
                long,
                RatioParseError::UnexpectedCharacters,
            ));
        }

        Ok(Ratio::new(numerator, denominator))
    }
}

impl DefaultDisplay for Ratio {
    type Display<'a> = &'a Self;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self
    }
}

impl std::error::Error for RatioParseError {}

impl std::fmt::Display for RatioParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RatioParseError::MissingColon => f.write_str("missing ':' in ratio"),
            RatioParseError::MissingDenominator => f.write_str("missing denominator"),
            RatioParseError::UnexpectedCharacters => {
                f.write_str("unexpected extra characters at end of ratio")
            }
            RatioParseError::InvalidNumerator(error) => write!(f, "invalid numerator - {}", error),
            RatioParseError::InvalidDenominator(error) => {
                write!(f, "invalid denominator - {}", error)
            }
        }
    }
}
