/* ---------------------------------------------------------------------- */
/*                       S E R I A L I Z A T I O N                        */
/* ---------------------------------------------------------------------- */

pub fn serial<T: std::fmt::Debug>(t: T) -> String {
    format!("{:?}", t)
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
}
