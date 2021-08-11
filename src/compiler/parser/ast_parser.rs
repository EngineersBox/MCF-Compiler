use super::varname::varname;
use super::ast::*;
use crate::{eat_mcf_sep};

use nom::{self, IResult};

named!(unopexp2<(Vec<UnOp>, Exp2)>, eat_mcf_sep!(
    tuple!(
        many0!(unop),
        exp2
    )
));

named!(flatexp<FlatExp>, eat_mcf_sep!(
    do_parse!(
        head: unopexp2 >>
        binop_chain: many0!(tuple!(binop, unopexp2)) >>
        (trans::flatexp_from_components(head, binop_chain))
    ))
);

named!(pub exp<Exp>, map!(flatexp, Exp::from));


named!(statement<Statement>, eat_mcf_sep!(
    alt!(
        value!(Statement::Semicolon, mcf_tag!(";")) |
        map!(template, Statement::Template) |
        map!(varassign, Statement::Assignment) |
        map!(functioncall, Statement::FuncCall) |
        map!(foriter, Statement::ForIter) |
        map!(functiondef, Statement::FuncDef)
    )
));

named!(template<PrefixExp>, eat_mcf_sep!(
    do_parse!(
        mcf_tag!("${") >>
        var: prefixexp >>
        mcf_tag!("}") >>
        (var)
    )
));

named!(varassign<Assignment>, eat_mcf_sep!(
    do_parse!(
        mcf_tag!("let") >>
        var: prefixexp >>
        mcf_tag!("=") >>
        val: exp >>
        (Assignment { var, val })
    )
));

named!(foriter<ForIter>, eat_mcf_sep!(
    do_parse!(
        mcf_tag!("for") >>
        vars: namelist >>
        mcf_tag!("in") >>
        exps: explist >>
        mcf_tag!("do") >>
        do_blk: block >>
        mcf_tag!("end") >>
        (ForIter { vars, exps, do_blk })
    )
));

named!(functiondef<FunctionDef>, eat_mcf_sep!(
    do_parse!(
        mcf_tag!("function") >>
        name: funcname >>
        body: functionbody >>
        mcf_tag!("end") >>
        (FunctionDef { name, body })
    )
));

named!(pub funcname<FunctionName>, eat_mcf_sep!(
    do_parse!(
        path: separated_nonempty_list_complete!(mcf_tag!("."), varname) >>
        method: opt!(complete!(preceded!(mcf_tag!(":"), varname))) >>
        (FunctionName { path, method })
    )
));

named!(explist<Vec<Exp>>, eat_mcf_sep!(
    separated_nonempty_list_complete!(mcf_tag!(","), exp)
));

// Returns an empty vec if there's no explist
named!(opt_explist<Vec<Exp>>,
    map!(opt!(explist), |el| el.unwrap_or(Vec::new()))
);

named!(functionbody<FunctionBody>, eat_mcf_sep!(
    do_parse!(
       params: delimited!(mcf_tag!("("), parlist, mcf_tag!(")")) >>
       body: block >>
       (FunctionBody { params, body })
    )
));

named!(pub block<Block>, eat_mcf_sep!(
    do_parse!(
        stmts: many0!(statement) >>
        (Block { stmts })
    )
));

named!(prefixexp<PrefixExp>, eat_mcf_sep!(
    do_parse!(
        prefix: prefixexp2 >>
        suffix_chain: many0!(prefixexp3) >>
        (PrefixExp { prefix, suffix_chain })
    )
));

named!(prefixexp2<ExpOrVarName>, eat_mcf_sep!(
    alt!(
        map!(delimited!(mcf_tag!("("), exp, mcf_tag!(")")), ExpOrVarName::Exp) |
        map!(varname, ExpOrVarName::VarName)
    )
));

named!(prefixexp3<ExpSuffix>, eat_mcf_sep!(
    alt!(
        map!(preceded!(mcf_tag!("."), varname), ExpSuffix::TableDot) |
        map!(delimited!(mcf_tag!("["), exp, mcf_tag!("]")), ExpSuffix::TableIdx)
    )
));

// A functioncall is just a prefixexp that ends in a funciton call
fn functioncall(input: &[u8]) -> IResult<&[u8], PrefixExp> {
    let res: IResult<&[u8], PrefixExp> = prefixexp(input);
    // TODO: de-uglify
    let is_funccall: bool = match res {
        IResult::Ok((_, ref pe)) => match pe.suffix_chain.last() {
            Some(ref a) => match a {
                &&ExpSuffix::FuncCall(_) => true,
                _ => false,
            },
            _ => false,
        },
        _ => false,
    };

    if is_funccall { res } else {
        IResult::Err(nom::Err::Error(error_position!(input, nom::error::ErrorKind::Verify)))
    }
}

named!(varlist<Vec<PrefixExp>>, eat_mcf_sep!(
    separated_nonempty_list_complete!(mcf_tag!(","), prefixexp)
));

named!(args<Args>, eat_mcf_sep!(
    alt!(
        map!(array_lit, Args::Array) |
        map!(delimited!(mcf_tag!("("), opt_explist, mcf_tag!(")")), Args::ExpList)
    )
));

named!(array_lit<ArrayLiteral>, eat_mcf_sep!(
    do_parse!(
        mcf_tag!("[") >>
        fields: separated_list_complete!(fieldsep, field) >>
        opt!(complete!(fieldsep)) >>
        mcf_tag!("]") >>
        (fields)
    )
));

named!(field<Field>, eat_mcf_sep!(
    alt!(
        do_parse!(
            key: delimited!(mcf_tag!("["), exp, mcf_tag!("]")) >>
            mcf_tag!("=") >>
            val: exp >>
            (Field::ExpAssign(key, val))
        ) |
        do_parse!(
            name: varname >>
            mcf_tag!("=") >>
            val: exp >>
            (Field::NameAssign(name, val))
        ) |
        map!(exp, Field::PosAssign)
    )
));

// Either we get a ... or nothing, or some parameters which can be followed by ',...'
named!(pub parlist<Params>, eat_mcf_sep!(
    alt!(
        do_parse!(
            names: namelist >>
            ellip: opt!(complete!(preceded!(mcf_tag!(","), mcf_tag!("...")))) >>
            (Params { names: names, variadic: ellip.is_some() })
        ) |
        value!(Params { names: Vec::new(), variadic: true }, mcf_tag!("...")) |
        value!(Params { names: Vec::new(), variadic: false })
    )
));

named!(fieldsep, eat_mcf_sep!(alt!(mcf_tag!(",") | mcf_tag!(";"))));