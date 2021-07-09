use bstr::{BStr, BString};
use itertools::Itertools;
use num_enum::FromPrimitive;
use paste::paste;
use serde::Deserialize;

use std::fmt;

use crate::Timestamp;

#[derive(Debug, Clone, Deserialize)]
pub struct ColumnMeta {
    pub name: String,
    pub type_: TaosDataType,
    pub bytes: i16,
}
#[derive(Debug)]
pub struct TaosQueryData {
    pub column_meta: Vec<ColumnMeta>,
    pub rows: Vec<Vec<Field>>,
}

#[derive(Debug)]
pub struct TaosDescribe(TaosQueryData);

impl TaosDescribe {
    pub fn names(&self) -> Vec<String> {
        self.0
            .rows
            .iter()
            .map(|row| {
                row.first()
                    .expect("first column must exists in describe")
                    .to_string()
            })
            .collect_vec()
    }
    pub fn col_names(&self) -> Vec<String> {
        self.0
            .rows
            .iter()
            .filter(|row| {
                row[3] != Field::Binary("TAG".into())
            })
            .map(|row| {
                row.first()
                    .expect("first column must exists in describe")
                    .to_string()
            })
            .collect_vec()
    }
    pub fn tag_names(&self) -> Vec<String> {
        self.0
            .rows
            .iter()
            .filter(|row| {
                row[3] == Field::Binary("TAG".into())
            })
            .map(|row| {
                row.first()
                    .expect("first column must exists in describe")
                    .to_string()
            })
            .collect_vec()
    }
}

impl From<TaosQueryData> for TaosDescribe {
    fn from(rhs: TaosQueryData) -> Self {
        Self(rhs)
    }
}
impl TaosQueryData {
    /// Total rows count of query result
    pub fn rows(&self) -> usize {
        self.rows.len()
    }
}

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum TaosDataType {
    Null = 0,
    Bool,      // 1
    TinyInt,   // 2
    SmallInt,  // 3
    Int,       // 4
    BigInt,    // 5
    Float,     // 6
    Double,    // 7
    Binary,    // 8
    Timestamp, // 9
    NChar,     // 10
    UTinyInt,  // 11
    USmallInt, // 12
    UInt,      // 13
    UBigInt,   // 14
    #[num_enum(default)]
    NonZero = 255,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Field {
    Null,        // 0
    Bool(bool),  // 1
    TinyInt(i8), // 2
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    Float(f32),
    Double(f64),
    Binary(BString),
    Timestamp(Timestamp),
    NChar(String),
    UTinyInt(u8),
    USmallInt(u16),
    UInt(u32),
    UBigInt(u64), // 14
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Field::Null => write!(f, "NULL"),
            Field::Bool(v) => write!(f, "{}", v),
            Field::TinyInt(v) => write!(f, "{}", v),
            Field::SmallInt(v) => write!(f, "{}", v),
            Field::Int(v) => write!(f, "{}", v),
            Field::BigInt(v) => write!(f, "{}", v),
            Field::Float(v) => write!(f, "{}", v),
            Field::Double(v) => write!(f, "{}", v),
            Field::Binary(v) => write!(f, "{}", v),
            Field::NChar(v) => write!(f, "{}", v),
            Field::Timestamp(v) => write!(f, "{}", v),
            Field::UTinyInt(v) => write!(f, "{}", v),
            Field::USmallInt(v) => write!(f, "{}", v),
            Field::UInt(v) => write!(f, "{}", v),
            Field::UBigInt(v) => write!(f, "{}", v),
        }
    }
}

impl Field {
    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Field::Bool(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_tiny_int(&self) -> Option<&i8> {
        match self {
            Field::TinyInt(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_small_int(&self) -> Option<&i16> {
        match self {
            Field::SmallInt(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<&i32> {
        match self {
            Field::Int(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_big_int(&self) -> Option<&i64> {
        match self {
            Field::BigInt(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<&f32> {
        match self {
            Field::Float(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_double(&self) -> Option<&f64> {
        match self {
            Field::Double(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_binary(&self) -> Option<&BStr> {
        match self {
            Field::Binary(v) => Some(v.as_ref()),
            _ => None,
        }
    }
    pub fn as_nchar(&self) -> Option<&str> {
        match self {
            Field::NChar(v) => Some(v),
            _ => None,
        }
    }

    /// BINARY or NCHAR typed string reference
    pub fn as_string(&self) -> Option<String> {
        match self {
            Field::Binary(v) => Some(v.to_string()),
            Field::NChar(v) => Some(v.to_string()),
            _ => None,
        }
    }
    pub fn as_timestamp(&self) -> Option<&Timestamp> {
        match self {
            Field::Timestamp(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_raw_timestamp(&self) -> Option<i64> {
        match self {
            Field::Timestamp(v) => Some(v.as_raw_timestamp()),
            _ => None,
        }
    }
    pub fn as_unsigned_tiny_int(&self) -> Option<&u8> {
        match self {
            Field::UTinyInt(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_unsigned_samll_int(&self) -> Option<&u16> {
        match self {
            Field::USmallInt(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_unsigned_int(&self) -> Option<&u32> {
        match self {
            Field::UInt(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_unsigned_big_int(&self) -> Option<&u64> {
        match self {
            Field::UBigInt(v) => Some(v),
            _ => None,
        }
    }

    pub fn data_type(&self) -> TaosDataType {
        match self {
            Field::Null => TaosDataType::Null,
            Field::Bool(_v) => TaosDataType::Bool,
            Field::TinyInt(_v) => TaosDataType::TinyInt,
            Field::SmallInt(_v) => TaosDataType::SmallInt,
            Field::Int(_v) => TaosDataType::Int,
            Field::BigInt(_v) => TaosDataType::BigInt,
            Field::Float(_v) => TaosDataType::Float,
            Field::Double(_v) => TaosDataType::Double,
            Field::Binary(_v) => TaosDataType::Binary,
            Field::NChar(_v) => TaosDataType::NChar,
            Field::Timestamp(_v) => TaosDataType::Timestamp,
            Field::UTinyInt(_v) => TaosDataType::UTinyInt,
            Field::USmallInt(_v) => TaosDataType::USmallInt,
            Field::UInt(_v) => TaosDataType::UInt,
            Field::UBigInt(_v) => TaosDataType::UBigInt,
        }
    }
}

pub trait IntoField {
    fn into_field(self) -> Field;
}

macro_rules! _impl_primitive_type {
    ($ty:ty, $target:ident, $v:expr) => {
        impl IntoField for $ty {
            fn into_field(self) -> Field {
                Field::$target(self)
            }
        }
        paste! {
            #[test]
            fn [<test_ $ty:snake>]() {
                let v: $ty = $v;
                assert_eq!(v.clone().into_field(), Field::$target(v));
            }
        }
    };
}

_impl_primitive_type!(bool, Bool, true);
_impl_primitive_type!(i8, TinyInt, 0);
_impl_primitive_type!(i16, SmallInt, 0);
_impl_primitive_type!(i32, Int, 0);
_impl_primitive_type!(i64, BigInt, 0);
_impl_primitive_type!(u8, UTinyInt, 0);
_impl_primitive_type!(u16, USmallInt, 0);
_impl_primitive_type!(u32, UInt, 0);
_impl_primitive_type!(u64, UBigInt, 0);
_impl_primitive_type!(f32, Float, 0.);
_impl_primitive_type!(f64, Double, 0.);
_impl_primitive_type!(BString, Binary, "A".into());
_impl_primitive_type!(String, NChar, "A".into());

impl IntoField for &BStr {
    fn into_field(self) -> Field {
        self.to_owned().into_field()
    }
}
impl IntoField for &str {
    fn into_field(self) -> Field {
        self.to_owned().into_field()
    }
}
