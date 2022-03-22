
// TODO maybe
// TODO zero or more
// TODO one or more

#[macro_export]
macro_rules! seq {

    // TODO error out by mentioning the index number
    // TODO be able to call other parsers

    ($input:ident, $n:ident <= $p:pat, $($rest:tt)*) => {
        let $n = match $input.next() {
            Some(y @ $p) => y,
            _ => { return None; },
        };
        seq!($input, $($rest)*);
    };

    ($input:ident, $b:block) => {
        return Some($b);
    };

    ($name:ident<$o:lifetime> : $in_t:ty => $out_t:ty = $($rest:tt)*) => {
        fn $name<$o>(input : &mut impl Iterator<Item = $in_t>) -> Option<$out_t> {
            seq!(input, $($rest)*);
        }
    };

    ($name:ident : $in_t:ty => $out_t:ty = $($rest:tt)*) => {
        fn $name(input : &mut impl Iterator<Item = $in_t>) -> Option<$out_t> {
            seq!(input, $($rest)*);
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
