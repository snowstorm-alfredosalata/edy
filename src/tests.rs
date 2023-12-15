use crate::prelude::*;

macro_rules! test_to_and_from_dynamic {
    ($($type:ident),+) => {

        #[test]
        fn test_to_and_from_dynamic() {
            $(
                {
                    let v: $type = $type::default();
                    let dyn_v = Dynamic::from(v.clone());

                    let v_recovered: $type = dyn_v.try_into().unwrap();

                    assert_eq!(v, v_recovered);
                }
            )*
        }

        #[test]
        fn test_to_and_from_dynamic_map() {
            let mut map = Map::new();

            $(
                {
                    let v: $type = $type::default();
                    let dyn_v = Dynamic::from(v.clone());

                    map.insert(stringify!($type).into(), dyn_v);

                    let v_recovered: $type = map.try_get_casted(stringify!($type)).unwrap();

                    assert_eq!(v, v_recovered);
                }

            )*
        }

    }
}

macro_rules! test_dynamic_type {
    ($($type:ident => $to:ident),+) => {

        #[test]
        fn test_dynamic_type() {
            $(
                {
                    let v: $type = $type::default();
                    let dyn_v = Dynamic::from(v.clone());

                    assert_eq!(dyn_v.get_type(), DynamicType::$to);
                }
            )*
        }
    }
}

test_to_and_from_dynamic! {
    i8, i16, i32, i64,
    u8, u16, u32,
    f32, f64,
    bool,
    String
}

test_dynamic_type! {
    i8 => Int,
    i16 => Int,
    i32 => Int,
    i64 => Int,

    u8 => Int,
    u16 => Int,
    u32 => Int,

    f32 => Float,
    f64 => Float,

    bool => Bool,

    String => Str
}

#[test]
fn test_dynamic_conversion_error() {
    let v: Dynamic = Dynamic::from(0.22f32);
    let mut map: HashMap<String, Dynamic> = HashMap::new();

    map.insert("sut".into(), v);

    let map = Map::from(map);

    let sut = map.try_get_casted::<i32>("sut");

    assert!(sut.is_err());

    let sut = map.try_get_casted::<f32>("sut");

    assert!(sut.is_ok());
}

#[test]
fn test_dynamic_default() {
    let sut = Dynamic::default();

    assert_eq!(sut.get_type(), DynamicType::Null)
}

#[cfg(feature = "serde")]
#[test]
fn test_serde() {
    let to_ser = r#"
    {
        "u8": 8, 
        "i64": -64, 
        "f32": 0.32, 
        "bool": true, 
        "String": "Hello"
    }
    "#;

    let map = serde_json::from_str::<Map>(to_ser);

    assert!(map.is_ok());

    let map = map.unwrap();

    let sut = map.try_get_casted::<u8>("u8");
    assert!(sut.is_ok());

    let sut = map.try_get_casted::<i64>("i64");
    assert!(sut.is_ok());

    let sut = map.try_get_casted::<f32>("f32");
    assert!(sut.is_ok());

    let sut = map.try_get_casted::<bool>("bool");
    assert!(sut.is_ok());

    let sut = map.try_get_casted::<String>("String");
    assert!(sut.is_ok());
}

fn double_dynamic(value: Dynamic) -> Result<Dynamic, Error> {
    match value {
        Dynamic::Str(v) => Ok(format!("{v}{v}").into()),
        Dynamic::Int(v) => Ok((v * 2).into()),
        Dynamic::Float(v) => Ok((v * 2f64).into()),
        _ => Err(UnsupportedTypeError {
            expected_types: vec![DynamicType::Str, DynamicType::Int, DynamicType::Float],
            found_type: value.get_type(),
        }
        .into()),
    }
}

#[test]
fn test_double_float() {
    let dy_float = 2f32;

    if let Ok(Dynamic::Float(v)) = double_dynamic(dy_float.into()) {
        assert!(v == 4f64)
    } else {
        unreachable!()
    }
}

#[test]
fn test_vec_conversion() {
    let vec = vec![2i32, 4, 6, 4];
    let dy_vec: Dynamic = vec.into();

    let vec: Vec<i32> = dy_vec.try_into().unwrap();

    assert_eq!(vec, vec![2i32, 4, 6, 4])
}
