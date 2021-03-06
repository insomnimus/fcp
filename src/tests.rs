use super::*;

#[test]
fn parse_get_request() {
    use GetRequest::*;
    let cases = [
        ("cfg", Config),
        ("temp", Temperature),
        ("volt", Voltage),
        ("%", Percentage),
    ];

    for (s, e) in cases {
        assert_eq!(GetRequest::parse(s.as_bytes()), Ok(e));
    }
}

#[test]
fn parse_set_request() {
    use SetRequest::*;
    let must_ok = [
        ("v50", Voltage(50u16)),
        ("a", Auto),
        ("%93", Percentage(93u8)),
    ];

    let must_err = [
        ("", MissingValue),
        ("v5;", InvalidValue),
        ("%%", InvalidValue),
    ];

    for (s, e) in must_ok {
        assert_eq!(SetRequest::parse(s.as_bytes()), Ok(e));
    }
    for (s, e) in must_err {
        assert_eq!(SetRequest::parse(s.as_bytes()), Err(e));
    }
}

#[test]
fn parse_adj_request() {
    use AdjRequest::*;
    let must_ok = [
        ("v5", Voltage(5i16)),
        ("v-11", Voltage(-11i16)),
        ("%23", Percentage(23i8)),
        ("%-55", Percentage(-55i8)),
    ];

    let must_err = [
        ("%22.15", InvalidValue),
        (" v2", InvalidValue),
        ("", MissingValue),
    ];

    for (s, e) in must_ok {
        assert_eq!(AdjRequest::parse(s.as_bytes()), Ok(e));
    }
    for (s, e) in must_err {
        assert_eq!(AdjRequest::parse(s.as_bytes()), Err(e));
    }
}

#[test]
fn parse_request() {
    use Request::*;
    {
        use GetRequest::*;
        let must_ok = [
            ("GET cfg", Config),
            ("GET %", Percentage),
            ("GET temp", Temperature),
            ("GET volt", Voltage),
        ];

        for (s, e) in must_ok {
            assert_eq!(Request::parse(s), Ok(Get(e)));
        }
    }
    {
        use SetRequest::*;
        let must_ok = [
            ("SET v5", Voltage(5u16)),
            ("SET %55", Percentage(55u8)),
            ("SET a", Auto),
            ("SET v1", Voltage(1u16)),
        ];

        for (s, e) in must_ok {
            assert_eq!(Request::parse(s), Ok(Set(e)));
        }
    }
    {
        use AdjRequest::*;
        let must_ok = [
            ("ADJ v-1", Voltage(-1i16)),
            ("ADJ %+55", Percentage(55i8)),
            ("ADJ v+02", Voltage(2i16)),
        ];

        for (s, e) in must_ok {
            assert_eq!(Request::parse(s), Ok(Adj(e)));
        }
    }

    let must_err = [
        ("LOL", UnknownRequestType),
        ("", Empty),
        ("SET", MissingValue),
        ("GET 55", InvalidValue),
        ("ADJ -", InvalidValue),
    ];

    for (s, e) in must_err {
        assert_eq!(Request::parse(s), Err(e));
    }
}

/// Test to see if marshaling and unmarshaling is reflexive.
#[test]
fn marshal_unmarshal() {
    use Request::*;
    let tests = &[
        Get(GetRequest::All),
        Get(GetRequest::Config),
        Get(GetRequest::Voltage),
        Get(GetRequest::Temperature),
        Get(GetRequest::Percentage),
        Set(SetRequest::Auto),
        Set(SetRequest::Voltage(255u16)),
        Set(SetRequest::Percentage(25u8)),
        Adj(AdjRequest::Percentage(-25i8)),
        Adj(AdjRequest::Voltage(-150i16)),
    ];

    for r in tests {
        assert_eq!(Ok(r), Request::parse(&format!("{}", r)).as_ref(),)
    }
}
