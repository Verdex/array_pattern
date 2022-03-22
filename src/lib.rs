
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

    // TODO predicates?
    // TODO pat has given me some problems with literals
    // TODO be able to call other parsers
    // TODO reset on failure

    (err, $rp:ident, $input:ident, $n:ident <= $p:pat, $($rest:tt)*) => {
        let $n = match $input.next() {
            Some(y @ (i, $p)) => y,
            _ => { return MatchResult::Error; },
        };
        seq!(fatal, $rp, $input, $($rest)*);
    };

    (fatal, $rp:ident, $input:ident, $n:ident <= $p:pat, $($rest:tt)*) => {
        let $n = match $input.next() {
            Some(y @ (i, $p)) => y,
            _ => { return MatchResult::Fatal(0); },
        };
        seq!(fatal, $rp, $input, $($rest)*);
    };

    ($mode:ident, $rp:ident, $input:ident, $b:block) => {
        return MatchResult::Success(0, $b);
    };

    ($name:ident<$o:lifetime> : $in_t:ty => $out_t:ty = $($rest:tt)*) => {
        fn $name<$o>(input : &mut (impl Iterator<Item = (usize, $in_t)> + Clone)) -> MatchResult<$out_t> {
            let rp = input.clone();
            seq!(err, rp, input, $($rest)*);
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
