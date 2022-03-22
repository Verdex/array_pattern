
// TODO maybe
// TODO zero or more
// TODO one or more

#[derive(Debug)]
pub enum MatchResult<T> {
    Success { start: usize, end: usize, item: T },  
    Error,
    Fatal(usize), 
    FatalEndOfFile,
}

#[derive(Debug)]
pub enum MatchError {
    Error,
    Fatal(usize), 
    FatalEndOfFile,
}

#[derive(Debug)]
pub struct Success<T> {
    pub item : T,
    pub start : usize,
    pub end : usize,
}

#[macro_export]
macro_rules! seq {

    (err, $rp:ident, $input:ident, $start:ident, $end:ident, $n:ident <= $matcher:ident, $($rest:tt)*) => {
        let v = $matcher($input)?;
        let $n = v.item;
        seq!(fatal, $rp, $input, $start, $end, $($rest)*);
    };

    (fatal, $rp:ident, $input:ident, $start:ident, $end:ident, $n:ident <= $matcher:ident, $($rest:tt)*) => {
        let $n = match $matcher($input) {
            Ok(v) => v.item,
            Err(MatchError::Fatal(i)) => return Err(MatchError::Fatal(i)),
            Err(_) => return Err(MatchError::Fatal(0)), // TODO if Error has a usize, then we can grab it here
            // TODO need to handle end of file differently
        };
        seq!(fatal, $rp, $input, $start, $end, $($rest)*);
    };

    (err, $rp:ident, $input:ident, $start:ident, $end:ident, $n:ident <= $p:pat, $($rest:tt)*) => {
        #[allow(unreachable_patterns)]
        let $n = match $input.next() {
            Some((i, item @ $p)) => {
                $start = i;
                $end = i;
                item
            },
            _ => { 
                std::mem::swap(&mut $rp, $input); 
                return Err(MatchError::Error); 
            },
        };
        seq!(fatal, $rp, $input, $start, $end, $($rest)*);
    };

    (fatal, $rp:ident, $input:ident, $start:ident, $end:ident, $n:ident <= $p:pat, $($rest:tt)*) => {
        #[allow(unreachable_patterns)]
        let $n = match $input.next() {
            Some((i, item @ $p)) => {
                $end = i;
                item
            },
            Some((i, _)) => {
                std::mem::swap(&mut $rp, $input); 
                return Err(MatchError::Fatal(i));  
            },
            _ => { 
                std::mem::swap(&mut $rp, $input); 
                return Err(MatchError::FatalEndOfFile);  
            },
        };
        seq!(fatal, $rp, $input, $start, $end, $($rest)*);
    };

    ($mode:ident, $rp:ident, $input:ident, $start:ident, $end:ident, $b:block) => {
        let item = $b;
        return Ok( Success { start: $start, end: $end, item: item } );
    };

    ($matcher_name:ident<$life:lifetime> : $in_t:ty => $out_t:ty = $($rest:tt)*) => {
        fn $matcher_name<$life>(input : &mut (impl Iterator<Item = (usize, $in_t)> + Clone)) -> Result<Success<$out_t>, MatchError> {
            let mut _rp = input.clone();
            let mut _start : usize = 0;
            let mut _end : usize = 0;
            seq!(err, _rp, input, _start, _end, $($rest)*);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO test calling other matcher that fails resets stream from its perspective
    // TODO test calling other matcher results in correct fatal index
    // TODO test calling other matcher results in correct fatal end of file
    // TODO test calling other matcher results in correct error
    // TODO test calling other matcher results in correct start/end values

    #[test]
    fn seq_should_call_other_matcher() -> Result<(), MatchError> {
        struct A(u8, u8);
        struct Main(A, A);
        seq!(other<'a>: u8 => A = a <= _, b <= _, {
            A(a, b)
        });

        seq!(main<'a>: u8 => Main = a <= other, b <= other, {
            Main(a, b)
        });

        let v : Vec<u8> = vec![0x00, 0x11, 0x22, 0x33];
        let mut i = v.into_iter().enumerate();

        let o = main(&mut i)?;

        assert_eq!( o.item.0.0, 0x00 );
        assert_eq!( o.item.0.1, 0x11 );
        assert_eq!( o.item.1.0, 0x22 );
        assert_eq!( o.item.1.1, 0x33 );

        Ok(())
    }

    #[test]
    fn seq_should_handle_bytes() -> Result<(), MatchError> {
        struct Output {
            a : u8,
            b : u8,
        }

        seq!(p<'a>: u8 => Output = a <= 0x00, b <= 0xFF, {
            Output { a, b }
        });

        let v : Vec<u8> = vec![0x00, 0xFF];
        let mut i = v.into_iter().enumerate();

        let o = p(&mut i)?;

        assert_eq!( o.item.a, 0x00 );
        assert_eq!( o.item.b, 0xFF );

        Ok(())
    }

    #[test]
    fn seq_should_show_multiple_index_success_with_more_rules() -> Result<(), MatchError> {
        seq!(s<'a>: char => char = _a <= _, _b <= _, _c <= _, _d <= _, {
            'x'
        });

        let v = "xxxxyyyy";
        let mut i = v.char_indices();

        let _ = s(&mut i)?;
        let success = s(&mut i)?;

        assert_eq!( success.start, 4 );
        assert_eq!( success.end, 7 );

        Ok(())
    }

    #[test]
    fn seq_should_show_multiple_index_success() -> Result<(), MatchError> {
        seq!(s<'a>: char => char = _a <= _, _b <= _, {
            'x'
        });

        let v = "xxyy";
        let mut i = v.char_indices();

        let _ = s(&mut i)?;
        let success = s(&mut i)?;

        assert_eq!( success.start, 2 );
        assert_eq!( success.end, 3 );

        Ok(())
    }

    #[test]
    fn seq_should_show_single_index_success() -> Result<(), MatchError> {
        seq!(s<'a>: char => char = _a <= _, {
            'x'
        });

        let v = "x";
        let mut i = v.char_indices();

        let success = s(&mut i)?;

        assert_eq!( success.start, 0 );
        assert_eq!( success.end, 0 );

        Ok(())
    }

    #[test]
    fn seq_should_show_single_index_success_at_nonzero() -> Result<(), MatchError> {
        seq!(s<'a>: char => char = _a <= _, {
            'x'
        });

        let v = "xx";
        let mut i = v.char_indices();

        let _ = s(&mut i)?;
        let success = s(&mut i)?;

        assert_eq!( success.start, 1 );
        assert_eq!( success.end, 1 );

        Ok(())
    }

    #[test]
    fn seq_should_indicate_first_item_error_from_end_of_file() {

        seq!(f<'a>: char => char = _a <= _, _b <= _, {
            'x'
        });

        let v = "";
        let mut i = v.char_indices();

        let failure = f(&mut i);

        assert!( matches!( failure, Err(MatchError::Error ) ) );
    }

    #[test]
    fn seq_should_indicate_first_item_error_from_mismatch() {

        seq!(f<'a>: char => char = _a <= 'a', _b <= _, {
            'x'
        });

        let v = "b";
        let mut i = v.char_indices();

        let failure = f(&mut i);

        assert!( matches!( failure, Err(MatchError::Error ) ) );
    }

    #[test]
    fn seq_should_indicate_fatal_line_number() {

        seq!(f<'a>: char => char = _a <= _, _b <= 'b', {
            'x'
        });

        let v = "ac";
        let mut i = v.char_indices();

        let failure = f(&mut i);

        assert!( matches!( failure, Err(MatchError::Fatal(1) ) ) );
    }

    #[test]
    fn seq_should_indicate_fatal_end_of_file() {

        seq!(f<'a>: char => char = _a <= _, _b <= _, {
            'x'
        });

        let v = "a";
        let mut i = v.char_indices();

        let failure = f(&mut i);

        assert!( matches!( failure, Err(MatchError::FatalEndOfFile ) ) );
    }

    #[test]
    fn seq_should_reset_on_failure() -> Result<(), MatchError> {

        seq!(f<'a>: char => char = _a <= 'a', _b <= 'b', {
            'x'
        });

        seq!(s<'a>: char => char = a <= _, {
            a
        });

        let v = "aac";
        let mut i = v.char_indices();

        let failure = f(&mut i);

        assert!( matches!( failure, Err(MatchError::Fatal(_) ) ) );

        let success = s(&mut i)?;

        assert_eq!( success.item, 'a' );

        Ok(())
    }

    #[test]
    fn seq_should_handle_single_item_match() {
        enum Input {
            A, 
            #[allow(unused)]
            B,
        }

        enum Output {
            A,
            #[allow(unused)]
            B,
        }

        seq!(m<'a>: &'a Input => Output = a <= Input::A, { 
            match a {
                Input::A => Output::A,
                _ => panic!("input not A"),
            }
        });

        let v = vec![Input::A];
        let mut i = v.iter().enumerate();

        let o = m(&mut i);

        assert!( matches!( o, Ok( Success{ item: Output::A, .. } ) ) );
    }


    #[test]
    fn seq_should_handle_multiple_item_match() {
        enum Input {
            A, 
            B,
        }

        enum OutputCase {
            A,
            B,
        }

        struct Output { 
            a : OutputCase,
            b : OutputCase,
        }

        seq!(m<'a>: &'a Input => Output = a <= Input::A, b <= Input::B, { 
            let o1 = match a {
                Input::A => OutputCase::A,
                Input::B => OutputCase::B,
            };

            let o2 = match b {
                Input::A => OutputCase::A,
                Input::B => OutputCase::B,
            };

            Output { a: o1, b: o2 }
        });

        let v = vec![Input::A, Input::B];
        let mut i = v.iter().enumerate();

        let o = m(&mut i);

        assert!( matches!( o, Ok( Success{ item: Output { a: OutputCase::A, b: OutputCase::B }, .. } ) ) );
    }

    #[test]
    fn seq_should_handle_owned_item_match() {
        enum Input {
            A, 
            B,
        }

        struct Output<'a> { 
            a : &'a Input,
            b : &'a Input,
        }

        seq!(m<'a>: &'a Input => Output<'a> = a <= Input::A, b <= Input::B, { 
            Output { a: a, b: b }
        });

        let v = vec![Input::A, Input::B];
        let mut i = v.iter().enumerate();

        let o = m(&mut i);

        assert!( matches!( o, Ok( Success{ item: Output { a: Input::A, b: Input::B }, .. } ) ) );
    }

    #[test]
    fn seq_should_handle_string_match() {
        struct C(char);

        seq!(m<'a>: char => C = a <= 'a', {
            C(a)
        });

        let v = "aaa";
        let mut i = v.char_indices();

        let o = m(&mut i);

        assert!( matches!( o, Ok( Success{ item: C('a'), .. } ) ) );
    }

    #[test]
    fn seq_should_preserve_changes_from_previous_match() -> Result<(), MatchError> {

        seq!(one<'a>: char => char = a <= _, {
            a
        });

        let v = "abc";
        let mut i = v.char_indices();

        let first = one(&mut i)?;
        let second = one(&mut i)?;

        assert_eq!( first.item, 'a' );
        assert_eq!( second.item, 'b' );

        Ok(())
    }
}
