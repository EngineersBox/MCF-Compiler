named!(pub comment,
    recognize!(
        terminated!(
            tag!("#"),
            tag!("\n")
        )
    )
);