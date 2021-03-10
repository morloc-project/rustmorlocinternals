/* ---------------------------------------------------------------------- */
/*                       S E R I A L I Z A T I O N                        */
/* ---------------------------------------------------------------------- */

pub fn serialize<T: std::fmt::Debug>(t: T) -> String {
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

#[derive(Debug)]
pub enum DeSerialResult {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Vec(Vec<DeSerialResult>)
}

impl From<DeSerialResult> for bool {
    fn from(v: DeSerialResult) -> bool {
        match v {
            DeSerialResult::Bool(b) => b,
            _ => panic!("{:?} cannot be converted into a bool!", v)
        }
    }
}

impl From<DeSerialResult> for i64 {
    fn from(v: DeSerialResult) -> i64 {
        match v {
            DeSerialResult::Int(i) => i,
            _ => panic!("{:?} cannot be converted into an i64!", v)
        }
    }
}

impl From<DeSerialResult> for f64 {
    fn from(v: DeSerialResult) -> f64 {
        match v {
            DeSerialResult::Float(f) => f,
            _ => panic!("{:?} cannot be converted into a f64!", v)
        }
    }
}

impl From<DeSerialResult> for String {
    fn from(v: DeSerialResult) -> String {
        match v {
            DeSerialResult::String(s) => s,
            _ => panic!("{:?} cannot be converted into a String!", v)
        }
    }
}

impl<T: From<DeSerialResult>> Into<Vec<T>> for DeSerialResult {
    fn into(self) -> Vec<T> {
        match self {
            DeSerialResult::Vec(v) => v.into_iter().map(|v| v.into()).collect(),
            _ => panic!("{:?} cannot be converted into a bool!", self)
        }
    }
}

pub fn deserialize(serial: &str) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_bool() {
        assert_eq!("true", &serialize(true));
        assert_eq!("false", &serialize(false));
    }

    #[test]
    fn test_serialize_int() {
        assert_eq!("-21", &serialize(-21));
        assert_eq!("-2", &serialize(-2));
        assert_eq!("0", &serialize(0));
        assert_eq!("21", &serialize(21));
    }

    #[test]
    fn test_serialize_float() {
        assert_eq!("-1.28", &serialize(-1.28));
        assert_eq!("-0.12", &serialize(-0.12));
        assert_eq!("0.0", &serialize(0.0));
        assert_eq!("0.12", &serialize(0.12));
        assert_eq!("1.28", &serialize(1.28));
    }

    #[test]
    fn test_serialize_str() {
        assert_eq!("\"asdf\"", &serialize("asdf"));
        assert_eq!("\"as\\\"df\"", &serialize("as\"df"));
    }

    #[test]
    fn test_serialize_vec() {
        assert_eq!("[1, 2, 3]", &serialize(&[1, 2, 3]));
        assert_eq!("[[1, 2], [3, 4], [5, 6]]", &serialize(&[[1, 2], [3, 4], [5, 6]]));
    }

    #[test]
    fn test_serialize_tuple() {
        assert_eq!("[1, 2.48]", &serialize((1, 2.48)));
        assert_eq!("[1, 2.48, \"asdf\"]", &serialize((1, 2.48, "asdf")));
        assert_eq!("[1, 2.48, \"as\\\"()df\"]", &serialize((1, 2.48, "as\"()df")));
    }

    fn _testy() {
        let _: i64 = DeSerialResult::Int(0).into();
        let _: Vec<i64> = DeSerialResult::Vec(vec![DeSerialResult::Int(1)]).into();
    }
}
