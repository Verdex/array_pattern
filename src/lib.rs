
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

    // TODO pat has given me some problems with literals
    // TODO be able to call other parsers

    (err, $rp:ident, $input:ident, $start:ident, $end:ident, $n:ident <= $p:pat, $($rest:tt)*) => {
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
            let mut rp = input.clone();
            let mut _start : usize = 0;
            let mut _end : usize = 0;
            seq!(err, rp, input, _start, _end, $($rest)*);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO test fatal end of file
    // TODO test fatal index
    // TODO test success single index size
    // TODO test success multi index size

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
