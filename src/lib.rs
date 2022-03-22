
// TODO maybe
// TODO zero or more
// TODO one or more

#[derive(Debug)]
pub enum MatchResult<T> {
    Success(usize, T),  // TODO Success { start: usize, end: usize, item: T}
    Error,
    Fatal(usize), 
    FatalEndOfFile,
}

#[macro_export]
macro_rules! seq {

    // TODO predicates?
    // TODO pat has given me some problems with literals
    // TODO be able to call other parsers

    (err, $rp:ident, $input:ident, $n:ident <= $p:pat, $($rest:tt)*) => {
        let $n = match $input.next() {
            Some(y @ (i, $p)) => y,
            _ => { 
                std::mem::swap(&mut $rp, $input); 
                return MatchResult::Error; 
            },
        };
        seq!(fatal, $rp, $input, $($rest)*);
    };

    (fatal, $rp:ident, $input:ident, $n:ident <= $p:pat, $($rest:tt)*) => {
        let $n = match $input.next() {
            Some(y @ (i, $p)) => y,
            Some((i, _)) => {
                std::mem::swap(&mut $rp, $input); 
                return MatchResult::Fatal(i);  
            },
            _ => { 
                std::mem::swap(&mut $rp, $input); 
                return MatchResult::FatalEndOfFile;  
            },
        };
        seq!(fatal, $rp, $input, $($rest)*);
    };

    ($mode:ident, $rp:ident, $input:ident, $b:block) => {
        return MatchResult::Success(0, $b);
    };

    ($matcher_name:ident<$life:lifetime> : $in_t:ty => $out_t:ty = $($rest:tt)*) => {
        fn $matcher_name<$life>(input : &mut (impl Iterator<Item = (usize, $in_t)> + Clone)) -> MatchResult<$out_t> {
            let mut rp = input.clone();
            seq!(err, rp, input, $($rest)*);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::iter::Enumerate;

    // TODO test reset on failure

    #[test]
    fn it_works() {
        enum X {
            A, B
        }

        seq!(blarg<'a>: &'a X => u8 = a <= X::A, { 
            let _ = a;
            4
        });

        seq!(other<'a>: &'a X => u8 = a <= X::A, b <= X::B, { 
            let _ = a;
            4
        });

        let z = vec![X::A];
        let mut y = z.iter().enumerate();

        let w = other(&mut y);
    }
}
