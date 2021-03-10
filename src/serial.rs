/* ---------------------------------------------------------------------- */
/*                       S E R I A L I Z A T I O N                        */
/* ---------------------------------------------------------------------- */

pub fn serial_bool(b: bool) -> String {
    format!("{:?}", b)
}

pub fn serial_int(i: i64) -> String {
    format!("{:?}", i)
}

pub fn serial_float(f: f64) -> String {
    format!("{:?}", f)
}

pub fn serial_str(s: &str) -> String {
    format!("{:?}", s)
}

pub fn serial_vec<T: std::fmt::Debug>(v: &[T]) -> String {
    format!("{:?}", v)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_bool() {
        assert_eq!("true", &serial_bool(true));
        assert_eq!("false", &serial_bool(false));
    }

    #[test]
    fn test_serial_int() {
        assert_eq!("-21", &serial_int(-21));
        assert_eq!("-2", &serial_int(-2));
        assert_eq!("0", &serial_int(0));
        assert_eq!("21", &serial_int(21));
    }

    #[test]
    fn test_serial_float() {
        assert_eq!("-1.28", &serial_float(-1.28));
        assert_eq!("-0.12", &serial_float(-0.12));
        assert_eq!("0.0", &serial_float(0.0));
        assert_eq!("0.12", &serial_float(0.12));
        assert_eq!("1.28", &serial_float(1.28));
    }

    #[test]
    fn test_serial_str() {
        assert_eq!("\"asdf\"", &serial_str("asdf"));
        assert_eq!("\"as\\\"df\"", &serial_str("as\"df"));
    }

    #[test]
    fn test_serial_vec() {
        assert_eq!("[1, 2, 3]", &serial_vec(&[1, 2, 3]));
        assert_eq!("[[1, 2], [3, 4], [5, 6]]", &serial_vec(&[[1, 2], [3, 4], [5, 6]]));
    }
}
