use nom::character::is_alphanumeric;
use super::comment::comment;
use nom::character::complete::alpha1;

pub fn is_alphanum_or_underscore(c: u8) -> bool {
    is_alphanumeric(c) || c == b'_'
}

pub fn is_eq_sign(c: u8) -> bool {
    c == b'='
}

pub fn is_whitespace(c: u8) -> bool {
    match c {
        b' ' | b'\n' | b'\t' | b'\r' => true,
        _ => false
    }
}

pub fn truncate(s: &[u8], n: usize) -> &[u8] {
    if s.len() <= n {
        s
    }
    else {
        &s[..n]
    }
}

// A tag! that can be preceded by whitespace or a comment
macro_rules! mcf_tag {
    ($i:expr, $tag:expr) => (
        eat_mcf_sep!($i, tag!($tag))
    );
}

// Eats a preceding separator, i.e., whitespace and/or a comment till the end of the line
#[macro_export]
macro_rules! eat_mcf_sep {
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        use super::chars::mcf_sep;
        complete!(
            $i,
            preceded!(mcf_sep, $submac!($($args)*))
        )
    });
}

pub fn is_ws(c: u8) -> bool {
    match c {
        b' ' | b'\t' | b'\n' | b'\r' => true,
        _ => false
    }
}

named!(pub alpha_or_underscore, recognize!(
    many1!(
        alt!(
            alpha1 |
            tag!("_")
        )
    )
));

named!(pub mcf_sep,
    recognize!(
        many0!(
            alt!(
                take_while1!(is_ws) | comment)
        )
    )
);