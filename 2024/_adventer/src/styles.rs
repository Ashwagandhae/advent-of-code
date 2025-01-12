use console::{style, StyledObject};

pub fn error<T>(s: T) -> StyledObject<String>
where
    T: Into<String>,
{
    style(s.into()).red()
}

pub fn warning<T>(s: T) -> StyledObject<String>
where
    T: Into<String>,
{
    style(s.into()).yellow()
}

pub fn success<T>(s: T) -> StyledObject<String>
where
    T: Into<String>,
{
    style(s.into()).green()
}

pub fn accent<T>(s: T) -> StyledObject<String>
where
    T: Into<String>,
{
    style(s.into()).cyan()
}

pub fn example<T>(s: T) -> StyledObject<String>
where
    T: Into<String>,
{
    style(s.into()).magenta()
}

pub fn real<T>(s: T) -> StyledObject<String>
where
    T: Into<String>,
{
    style(s.into()).blue()
}
