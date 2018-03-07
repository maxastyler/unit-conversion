use nom::{digit, be_i32, double, alpha, rest};
use std::str;
use std::str::{from_utf8, FromStr};
use unit::Unit;

/// Get a number from an array of u8
named!(pub get_digits<i32>,
    map_res!(
        map_res!(digit,
            from_utf8
        ),
        FromStr::from_str
    )
);

named!(pub match_line<&[u8]>, do_parse!(
        text: alt_complete!(take_until!("\n") | rest) >>
        opt!(complete!(tag!("\n"))) >>
        (text)
));

/// Match a set of lines and return a vector of tuples containing the indentation and the text
named!(pub match_lines<Vec<&[u8]> >, alt_complete!(many1!(match_line) | value!(vec!(), ws!(eof!()))));


named!(pub get_int<i32>,
    do_parse!(
        neg: opt!(tag!(b"-")) >>
        num: get_digits >>
        (match neg {
            Some(_) => -num,
            None => num,
        })
    )
);

named!(pub parse_unit<(&str, Unit)>, do_parse!(
    name: ws!(alpha) >> 
    tag!(b":") >>
    value: ws!(double) >>
    tag!(b":") >>
    dimensions: ws!(get_dimensions) >>
    (from_utf8(name).unwrap(), Unit::new(value, &dimensions))
        ));

named!(pub get_dimensions<[i32; 9]>, map!(many_m_n!(9, 9, delimited!(opt!(tag!(b",")), ws!(get_int), opt!(complete!(tag!(b","))))),
|v| [v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8]]
)
);

pub fn get_units_from_lines(lines: &[u8]) -> Vec<(&str, Unit)> {
    use nom::IResult::{Done, Error};
    let split_lines = match_lines(lines);
    let mut split_units = vec!();
    match split_lines {
        Done(_, tup) => {
            for u in tup.into_iter().map(|x| parse_unit(x)){
                match u {
                    Done(_, s) => {
                        split_units.push(s);
                    },
                    _ => ()
                }
            }
        },
        _ => ()
    }
    split_units
}

#[cfg(test)]
mod tests {
        use super::*;
        use nom::IResult::{Done, Error};
        
        #[test]
        fn test_get_int_pos() {
            let n = get_int(b"123");
            assert_eq!(n, Done(&b""[..], 123));
        }

        #[test]
        fn test_get_int_neg() {
            let n = get_int(b"-1233");
            assert_eq!(n, Done(&b""[..], -1233));
        }

        #[test]
        fn test_get_double() {
            assert_eq!(double(b"12.32e-12"), Done(&b""[..], 12.32e-12));
        }

        #[test]
        fn test_get_unit() {
            let r = parse_unit(b"hello : -1.02e-10 : 1 2 3 4 5 6 7 8 9");
            let d = [1, 2, 3, 4, 5, 6, 7, 8, 9];
            assert_eq!(r, Done(&b""[..], ("hello", Unit::new(-1.02e-10, &d))));
        }

        #[test]
        fn test_get_dimensions() {
            let n = get_dimensions(b",1 2 3 4 5 -6 7 0 0");
            assert_eq!(n, Done(&b""[..], [1, 2, 3, 4, 5, -6, 7, 0, 0]));
        }
        
        #[test]
        fn test_get_units() {
            let s = &b"hello : -1.0e-2 : 1 0 0 0 0 0 0 2 0\nhi : 1.23e2 : 1 0 0 2 0 0 0 2 0"[..];
            let p = get_units_from_lines(s);
            assert_eq!(p, vec!());
        }
}
