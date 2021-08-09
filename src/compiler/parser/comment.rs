use nom::IResult;
use nom::sequence::terminated;

named!(pub comment,
    recognize!(
        terminated!(
            tag!("#"),
            tag!("\n")
        )
    )
);