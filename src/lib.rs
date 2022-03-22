
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

#[macro_export]
macro_rules! seq {

    // TODO predicates?
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
                return MatchResult::Error; 
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
                return MatchResult::Fatal(i);  
            },
            _ => { 
                std::mem::swap(&mut $rp, $input); 
                return MatchResult::FatalEndOfFile;  
            },
        };
        seq!(fatal, $rp, $input, $start, $end, $($rest)*);
    };

    ($mode:ident, $rp:ident, $input:ident, $start:ident, $end:ident, $b:block) => {
        let item = $b;
        return MatchResult::Success { start: $start, end: $end, item: item };
    };

    ($matcher_name:ident<$life:lifetime> : $in_t:ty => $out_t:ty = $($rest:tt)*) => {
        fn $matcher_name<$life>(input : &mut (impl Iterator<Item = (usize, $in_t)> + Clone)) -> MatchResult<$out_t> {
            let mut rp = input.clone();
            #[allow(unused)]
            let mut start : usize = 0;
            #[allow(unused)]
            let mut end : usize = 0;
            seq!(err, rp, input, start, end, $($rest)*);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO test reset on failure
    // TODO test fatal end of file
    // TODO test fatal index
    // TODO test success single index size
    // TODO test success multi index size
    // TODO test char indicies

    #[test]
    fn should_handle_single_item_match() {
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

        assert!( matches!( o, MatchResult::Success{ item: Output::A, .. } ) );
    }


    #[test]
    fn should_handle_multiple_item_match() {

    }

    #[test]
    fn should_handle_owned_item_match() {

    }

    #[test]
    fn should_handle_string_match() {

    }
}
