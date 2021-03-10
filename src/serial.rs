/* ---------------------------------------------------------------------- */
/*                       S E R I A L I Z A T I O N                        */
/* ---------------------------------------------------------------------- */

pub fn serial<T: std::fmt::Debug>(t: T) -> String {
    let formatted = format!("{:?}", t);
    let mut built = String::new();
    let mut in_str = false;
    let mut escaped = false;

    for c in formatted.chars() {
        built.push({
            let last_escape = escaped;
            let c = match c {
                '\"' if !escaped => {
                    in_str ^= true;
                    '\"'
                }

                '\\' if in_str => {
                    escaped = true;
                    '\\'
                }

                '(' if !in_str => '[',
                ')' if !in_str => ']',

                c => c
            };
            if last_escape {
                escaped = false;
            }
            c
        });
    }
    built
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_bool() {
        assert_eq!("true", &serial(true));
        assert_eq!("false", &serial(false));
    }

    #[test]
    fn test_serial_int() {
        assert_eq!("-21", &serial(-21));
        assert_eq!("-2", &serial(-2));
        assert_eq!("0", &serial(0));
        assert_eq!("21", &serial(21));
    }

    #[test]
    fn test_serial_float() {
        assert_eq!("-1.28", &serial(-1.28));
        assert_eq!("-0.12", &serial(-0.12));
        assert_eq!("0.0", &serial(0.0));
        assert_eq!("0.12", &serial(0.12));
        assert_eq!("1.28", &serial(1.28));
    }

    #[test]
    fn test_serial_str() {
        assert_eq!("\"asdf\"", &serial("asdf"));
        assert_eq!("\"as\\\"df\"", &serial("as\"df"));
    }

    #[test]
    fn test_serial_vec() {
        assert_eq!("[1, 2, 3]", &serial(&[1, 2, 3]));
        assert_eq!("[[1, 2], [3, 4], [5, 6]]", &serial(&[[1, 2], [3, 4], [5, 6]]));
    }

    #[test]
    fn test_serial_tuple() {
        assert_eq!("[1, 2.48]", &serial((1, 2.48)));
        assert_eq!("[1, 2.48, \"asdf\"]", &serial((1, 2.48, "asdf")));
        assert_eq!("[1, 2.48, \"as\\\"()df\"]", &serial((1, 2.48, "as\"()df")));
    }
}
