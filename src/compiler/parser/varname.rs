use std::fmt;

use crate::compiler::parser::chars::{alpha_or_underscore, is_alphanum_or_underscore};
use nom::{self, IResult};
use nom::error::ErrorKind;
use std::borrow::Cow;
use crate::eat_mcf_sep;


#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VarName<'a>(pub &'a [u8]);

impl<'a> fmt::Debug for VarName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let s: Cow<str> = String::from_utf8_lossy(self.0);
        write!(f, "'{}'", s)
    }
}

named!(reserved, alt!(tag!("let") | tag!("fn") | tag!("for")));

fn varname_(buf: &[u8]) -> IResult<&[u8], VarName> {
    let name_res: IResult<&[u8], &[u8]> = recognize!(buf, preceded!(
        alpha_or_underscore,
        take_while!(is_alphanum_or_underscore)
    ));

    if let IResult::Ok((_, name)) = name_res {
        if let IResult::Ok((rest, _)) = reserved(name) {
            if rest.len() == 0 {
                // TODO: make this a real error
                return IResult::Err(nom::Err::Error(error_position!(buf, ErrorKind::LengthValue)));
            }
        }
    }

    name_res.map(|n| (n.0, VarName(n.1)))
}

named!(pub varname<VarName>, eat_mcf_sep!(call!(varname_)));