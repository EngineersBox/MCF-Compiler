use std::fmt;

use crate::compiler::parser::chars::{alpha_or_underscore, is_alphanum_or_underscore};
use nom::{self, IResult};


#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VarName<'a>(pub &'a [u8]);

impl<'a> fmt::Debug for VarName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let s = String::from_utf8_lossy(self.0);
        write!(f, "'{}'", s)
    }
}

named!(pub namelist<Vec<VarName>>, separated_nonempty_list_complete!(mcf_tag!(","), varname));

named!(reserved, alt!(tag!("let") | tag!("fn") | tag!("for")));

fn varname_(buf: &[u8]) -> IResult<&[u8], VarName> {
    let name_res = recognize!(buf, preceded!(
        alpha_or_underscore,
        take_while!(is_alphanum_or_underscore)
    ));

    if let IResult::Done(_, name) = name_res {
        if let IResult::Done(rest, _) = reserved(name) {
            if rest.len() == 0 {
                // TODO: make this a real error
                return IResult::Error(error_position!(nom::ErrorKind::Custom(0), buf));
            }
        }
    }

    name_res.map(|n| (n.0, VarName(n.1)))
}

named!(pub varname<VarName>, eat_mcf_sep!(call!(varname_)));