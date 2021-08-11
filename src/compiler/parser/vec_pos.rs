use nom::character::complete::digit1;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VecPosPrefix {
    Tilde,
    Carrot,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VecPosSign {
    Pos,
    Neg,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VecPosLit<'a> {
    pub prefix: Option<VecPosPrefix>,
    pub sign: Option<VecPosSign>,
    pub value: &'a str,
}

impl<'a> VecPosLit<'a> {
    fn from_lit(seperated_tuple: (Option<&[u8]>, Option<&[u8]>, &[u8], Option<&[u8]>)) -> Self {
        Self {
            prefix: seperated_tuple.0.map(|p: &[u8]| {
                if p[0] == b'~' {
                    VecPosPrefix::Tilde
                } else {
                    VecPosPrefix::Carrot
                }
            }),
            sign: seperated_tuple.1.map(|s: &[u8]| {
                if s[0] == b'+' {
                    VecPosSign::Pos
                } else {
                    VecPosSign::Neg
                }
            }),
            value: (
                String::from_utf8_lossy(seperated_tuple.2)
                    + String::from_utf8_lossy(seperated_tuple.3.unwrap_or(&[]))
            ).to_string().as_str()
        }
    }
}

named!(pub vec_pos_lit<VecPosLit>, eat_mcf_sep!(
    map!(
        tuple!(
            opt!(
                alt!(tag!("~") | tag!("^"))
            ),
            opt!(
                alt!(tag!("+") | tag!("-"))
            ),
            digit1,
            opt!(
                complete!(
                    preceded!(
                        tag!("."),
                        digit1
                    )
                )
            )
        ),
        VecPosLit::from_lit
    )
));