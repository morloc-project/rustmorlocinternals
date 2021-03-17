use std::collections::HashMap;

/* ---------------------------------------------------------------------- */
/*                       S E R I A L I Z A T I O N                        */
/* ---------------------------------------------------------------------- */

pub trait Serialize {
    fn serialize(&self) -> String;
}

impl Serialize for bool {
    fn serialize(&self) -> String {
        format!("{}", self)
    }
}

impl Serialize for i64 {
    fn serialize(&self) -> String {
        format!("{}", self)
    }
}

impl Serialize for f64 {
    fn serialize(&self) -> String {
        format!("{:?}", self)
    }
}

impl Serialize for String {
    fn serialize(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self) -> String {
        let mut built = String::from("[");
        let mut comma = false;

        for i in self {
            if comma {
                built.push_str(", ");
            } else {
                comma = true;
            }

            built.push_str(&i.serialize());
        }

        built.push(']');
        built
    }
}

macro_rules! impl_serialize_tuple {
    () => { };

    ($A: ident => $a: literal, $($I: ident => $i: tt,)*) => {
        // Create previous implementation
        impl_serialize_tuple!($($I => $i,)*);

        // Current implementation
        impl<$A: Serialize, $($I: Serialize),*> Serialize for ($A,$($I,)*) {
            #[allow(unused_mut)]
            fn serialize(&self) -> String {
                let mut built = String::from("[");
                built.push_str(&self.0.serialize());
                let mut parts: Vec<String> = Vec::with_capacity(23);

                $(parts.push(self.$i.serialize());)*

                for p in parts.into_iter().rev() {
                    built.push_str(", ");
                    built.push_str(&p);
                }
                built.push(']');
                built
            }
        }
    }
}

impl_serialize_tuple!(
    A => 23,
    B => 22,
    C => 21,
    D => 20,
    E => 19,
    F => 18,
    G => 17,
    H => 16,
    I => 15,
    J => 14,
    K => 13,
    L => 12,
    M => 11,
    N => 10,
    O => 9,
    P => 8,
    Q => 7,
    R => 6,
    S => 5,
    T => 4,
    U => 3,
    V => 2,
    W => 1,
);

pub fn serialize<T: Serialize>(t: &T) -> String {
    t.serialize()
}

/* ---------------------------------------------------------------------- */
/*                      D E S E R I A L I Z A T I O N                     */
/* ---------------------------------------------------------------------- */

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum DeSerialResult {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Vec(Vec<DeSerialResult>),
    Struct(HashMap<String, DeSerialResult>)
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
            DeSerialResult::Int(i) => i as f64,
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

impl<T: From<DeSerialResult>> From<DeSerialResult> for Vec<T> {
    fn from(v: DeSerialResult) -> Vec<T> {
        match v {
            DeSerialResult::Vec(v) => v.into_iter().map(|v| v.into()).collect(),
            _ => panic!("{:?} cannot be converted into a Vec!", v)
        }
    }
}

// Don't you just love recursive macros :>
macro_rules! impl_from_tuple {
    () => { };

    ($A: ident, $($I: ident,)*) => {
        // Create previous implementation
        impl_from_tuple!($($I,)*);

        // Current implementation
        impl<$A: From<DeSerialResult>, $($I: From<DeSerialResult>),*> From<DeSerialResult> for ($A, $($I),*) {
            fn from(v: DeSerialResult) -> ($A, $($I),*) {
                match v {
                    // Create tuple from iterator
                    DeSerialResult::Vec(v) => {
                        let mut v = v.into_iter();
                        (v.next().unwrap().into(), $(Into::<$I>::into(v.next().unwrap())),*)
                    }

                    _ => panic!("{:?} cannot be converted into a tuple!", v)
                }
            }
        }
    }
}

impl_from_tuple!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,);

pub fn deserialize(serial: &str) -> DeSerialResult {
    serde_json::from_str(serial).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_bool() {
        assert_eq!("true", &serialize(&true));
        assert_eq!("false", &serialize(&false));
    }

    #[test]
    fn test_deserialize_bool() {
        assert_eq!(true, Into::<bool>::into(deserialize("true")));
        assert_eq!(false, Into::<bool>::into(deserialize("false")));
    }

    #[test]
    fn test_serialize_int() {
        assert_eq!("-21", &serialize(&-21));
        assert_eq!("-2", &serialize(&-2));
        assert_eq!("0", &serialize(&0));
        assert_eq!("21", &serialize(&21));
    }

    #[test]
    fn test_deserialize_int() {
        assert_eq!(-21, Into::<i64>::into(deserialize("-21")));
        assert_eq!(-2, Into::<i64>::into(deserialize("-2")));
        assert_eq!(0, Into::<i64>::into(deserialize("0")));
        assert_eq!(21, Into::<i64>::into(deserialize("21")));
    }

    #[test]
    fn test_serialize_float() {
        assert_eq!("-1.28", &serialize(&-1.28));
        assert_eq!("-0.12", &serialize(&-0.12));
        assert_eq!("0.0", &serialize(&0.0));
        assert_eq!("0.12", &serialize(&0.12));
        assert_eq!("1.28", &serialize(&1.28));
    }

    #[test]
    fn test_deserialize_float() {
        assert_eq!(-1.28, Into::<f64>::into(deserialize("-1.28")));
        assert_eq!(-0.12, Into::<f64>::into(deserialize("-0.12")));
        assert_eq!(0.0, Into::<f64>::into(deserialize("0.0")));
        assert_eq!(0.12, Into::<f64>::into(deserialize("0.12")));
        assert_eq!(1.28, Into::<f64>::into(deserialize("1.28")));
    }


    #[test]
    fn test_serialize_str() {
        assert_eq!("\"asdf\"", &serialize(&String::from("asdf")));
        assert_eq!("\"as\\\"df\"", &serialize(&String::from("as\"df")));
    }

    #[test]
    fn test_deserialize_str() {
        assert_eq!("asdf", &Into::<String>::into(deserialize("\"asdf\"")));
        assert_eq!("as\"df", &Into::<String>::into(deserialize("\"as\\\"df\"")));
    }

    #[test]
    fn test_serialize_vec() {
        assert_eq!("[1, 2, 3]", &serialize(&vec![1, 2, 3]));
        assert_eq!("[[1, 2], [3, 4], [5, 6]]", &serialize(&vec![vec![1, 2], vec![3, 4], vec![5, 6]]));
    }

    #[test]
    fn test_deserialize_vec() {
        let v: Vec<i64> = deserialize("[1, 2, 3]").into();
        assert_eq!(vec![1, 2, 3], v);
        let v: Vec<Vec<i64>> = deserialize("[[1, 2], [3, 4], [5, 6]]").into();
        assert_eq!(vec![vec![1, 2], vec![3, 4], vec![5, 6]], v);
    }

    #[test]
    fn test_serialize_tuple() {
        assert_eq!("[1, 2.48]", &serialize(&(1, 2.48)));
        assert_eq!("[1, 2.48, \"asdf\"]", &serialize(&(1, 2.48, String::from("asdf"))));
    }

    #[test]
    fn test_deserialize_tuple() {
        let v: (i64, f64) = deserialize("[1, 2.48]").into();
        assert_eq!((1, 2.48), v);
        let v: (i64, f64, String) = deserialize("[1, 2.48, \"asdf\"]").into();
        assert_eq!((1, 2.48, String::from("asdf")), v);
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Person {
        name: String,
        age: i64
    }

    impl Serialize for Person {
        fn serialize(&self) -> String {
            format!("{{\"name\": {}, \"age\": {}}}", self.name.serialize(), self.age.serialize())
        }
    }

    impl From<DeSerialResult> for Person {
        fn from(s: DeSerialResult) -> Person {
            match s {
                DeSerialResult::Struct(mut map) => Person {
                    name: map.remove("name").unwrap().into(),
                    age: map.remove("age").unwrap().into()
                },

                _ => panic!("cannot convert {:?} into Person", s)
            }
        }
    }

    #[test]
    fn test_serialize_struct() {
        let v = Person { name: String::from("test name"), age: -1 };
        assert_eq!("{\"name\": \"test name\", \"age\": -1}", &v.serialize());
    }

    #[test]
    fn test_deserialize_struct() {
        let v: Person = deserialize("{\"name\": \"test name\", \"age\": -1}").into();
        assert_eq!(Person { name: String::from("test name"), age: -1 }, v);
    }
}
