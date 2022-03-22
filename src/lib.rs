
#[macro_export]
macro_rules! array_matcher {

    ($input:ident, $n:ident <= $p:pat, $($rest:tt)*) => {
        let $n = match $input.next() {
            Some(y @ $p) => y,
            _ => { return None; },
        };
        array_matcher!($input, $($rest)*);
    };

    ($input:ident, $b:block) => {
        return Some($b);
    };

    ($name:ident<$o:lifetime> : $in_t:ty => $out_t:ty = $($rest:tt)*) => {
        fn $name<$o>(input : &mut impl Iterator<Item = $in_t>) -> Option<$out_t> {
            array_matcher!(input, $($rest)*);
        }
    };

    ($name:ident : $in_t:ty => $out_t:ty = $($rest:tt)*) => {
        fn $name(input : &mut impl Iterator<Item = $in_t>) -> Option<$out_t> {
            array_matcher!(input, $($rest)*);
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
