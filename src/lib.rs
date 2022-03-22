
// TODO maybe
// TODO zero or more
// TODO one or more

#[derive(Debug)]
pub enum MatchResult<T> {
    Success(usize, T),
    Error,
    Fatal(usize)
}

#[macro_export]
macro_rules! seq {

    // TODO be able to call other parsers
    // TODO reset on failure

    (err, $input:ident, $n:ident <= $p:pat, $($rest:tt)*) => {
        let $n = match $input.next() {
            Some(y @ (i, $p)) => y,
            _ => { return MatchResult::Error; },
        };
        seq!(fatal, $input, $($rest)*);
    };

    (fatal, $input:ident, $n:ident <= $p:pat, $($rest:tt)*) => {
        let $n = match $input.next() {
            Some(y @ (i, $p)) => y,
            _ => { return MatchResult::Fatal(0); },
        };
        seq!(fatal, $input, $($rest)*);
    };

    ($mode:ident, $input:ident, $b:block) => {
        return MatchResult::Success(0, $b);
    };

    ($name:ident<$o:lifetime> : $in_t:ty => $out_t:ty = $($rest:tt)*) => {
        fn $name<$o>(input : &mut impl Iterator<Item = (usize, $in_t)>) -> MatchResult<$out_t> {
            seq!(err, input, $($rest)*);
        }
    };

    ($name:ident : $in_t:ty => $out_t:ty = $($rest:tt)*) => {
        fn $name(input : &mut impl Iterator<Item = (usize, $in_t)>) -> MatchResult<$out_t> {
            seq!(err, input, $($rest)*);
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
