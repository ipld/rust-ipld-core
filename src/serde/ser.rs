// Parts of this code is based on
// https://github.com/serde-rs/json/blob/95f67a09399d546d9ecadeb747a845a77ff309b2/src/value/ser.rs
use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{convert::TryFrom, fmt::Debug, marker::PhantomData};

use cid::serde::CID_SERDE_PRIVATE_IDENTIFIER;
use cid::Cid;
use serde::ser;

use crate::{
    ipld::{IpldGeneric, Primitives},
    serde::SerdeError,
};

/// Serialize into instances of [`crate::ipld::Ipld`].
///
/// All Rust types can be serialized to [`crate::ipld::Ipld`], here is a list of how they are
/// converted:
///
///  - bool -> `Ipld::Bool`
///  - i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, usize -> `Ipld::Integer`
///  - f32, f64 -> `Ipld::Float`
///  - char, String -> `Ipld::String`
///  - slices -> `Ipld::List`
///  - struct
///    - struct -> `Ipld::Map`
///    - newtype struct -> the value the struct wraps
///    - tuple struct -> `Ipld::List`
///    - unit struct -> cannot be serialized, it errors
///  - enum:
///    - unit variant -> `Ipld::String` of the variant name
///    - newtype variant -> single element `Ipld::Map`, key: variant name, value: the one the
///      newtype wraps
///    - tuple variant -> single element `Ipld::Map`, key: variant name, value: `Ipld::List`
///    - struct variant -> single element `Ipld::Map`, key: variant name, value: `Ipld::Map`
///  - unit (`()`) -> cannot be serialized, it errors
///
/// There are also common compound types that are supported:
///
///  - [`std::option::Option`] -> eithe `Ipld::Null` or the value
///  - [`serde_bytes::ByteBuf`] -> `Ipld::Bytes`
///  - lists (like e.g. [`std::vec::Vec`]) -> `Ipld::List`
///  - maps (like e.g. [`std::collections::BTreeMap`]) -> `Ipld::Map`
///  - [`cid::Cid`] -> `Ipld::Link`
///
///
/// # Example
///
/// ```
/// use serde_derive::Serialize;
/// use ipld_core::ipld::Ipld;
/// use ipld_core::serde::to_ipld;
///
/// #[derive(Serialize)]
/// struct Person {
///     name: String,
///     age: u8,
///     hobbies: Vec<String>,
///     is_cool: bool,
/// }
///
/// let person = Person {
///     name: "Hello World!".into(),
///     age: 52,
///     hobbies: vec!["geography".into(), "programming".into()],
///     is_cool: true,
/// };
///
/// let ipld = to_ipld(person);
/// assert!(matches!(ipld, Ok(Ipld::Map(_))));
/// ```
pub fn to_ipld<T, P>(value: T) -> Result<IpldGeneric<P>, SerdeError>
where
    T: ser::Serialize,
    P: Primitives,
{
    value.serialize(Serializer)
}

impl<P> ser::Serialize for IpldGeneric<P>
where
    P: Primitives,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match &self {
            //Self::Null => serializer.serialize_none(),
            //Self::Bool(value) => serializer.serialize_bool((*value).into()),
            //Self::Integer(value) => serializer.serialize_i128((*value).into()),
            //Self::Float(value) => serializer.serialize_f64((*value).into()),
            //Self::String(value) => serializer.serialize_str(&(*value).into()),
            //Self::Bytes(value) => serializer.serialize_bytes(&(*value).into()),
            //Self::List(value) => serializer.collect_seq(value),
            //Self::Map(value) => serializer.collect_map(value.into()),
            //Self::Link(value) => (*value).into().serialize(serializer),
            _ => todo!(),
        }
    }
}

/// The IPLD serializer.
pub struct Serializer<P>(PhantomData<P>);

impl<P> serde::Serializer for Serializer<P>
where
    P: Primitives + Debug,
{
    type Ok = IpldGeneric<P>;
    type Error = SerdeError;

    type SerializeSeq = SerializeVec<P>;
    type SerializeTuple = SerializeVec<P>;
    type SerializeTupleStruct = SerializeVec<P>;
    type SerializeTupleVariant = SerializeTupleVariant<P>;
    type SerializeMap = SerializeMap<P>;
    type SerializeStruct = SerializeMap<P>;
    type SerializeStructVariant = SerializeStructVariant<P>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Self::Ok::Bool(value.into()))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(value))
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(value))
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(value))
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(i128::from(value))
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Self::Ok::Integer(value.into()))
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value.into())
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value.into())
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value.into())
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value.into())
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(value))
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Self::Ok::Float(value.into()))
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&value.to_string())
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Self::Ok::String(value.to_owned().into()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Self::Ok::Bytes(value.to_vec().into()))
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("Unit is not supported"))
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("Unit structs are not supported"))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        let ipld = value.serialize(self);
        if name == CID_SERDE_PRIVATE_IDENTIFIER {
            if let Ok(IpldGeneric::Bytes(bytes)) = ipld {
                let cid = Cid::try_from(bytes.into())
                    .map_err(|err| ser::Error::custom(format!("Invalid CID: {}", err)))?;
                return Ok(Self::Ok::Link(cid.into()));
            }
        }
        ipld
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        let values = BTreeMap::from([(variant.to_owned().into(), value.serialize(self)?)]);
        Ok(Self::Ok::Map(values))
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Self::Ok::Null)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeVec {
            vec: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant {
            name: String::from(variant),
            vec: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap {
            map: BTreeMap::new(),
            next_key: None,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariant {
            name: P::String::from(variant.into()),
            map: BTreeMap::new(),
        })
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

pub struct SerializeVec<P>
where
    P: Primitives,
{
    vec: Vec<IpldGeneric<P>>,
}

pub struct SerializeTupleVariant<P>
where
    P: Primitives,
{
    name: String,
    vec: Vec<IpldGeneric<P>>,
}

pub struct SerializeMap<P>
where
    P: Primitives,
{
    map: BTreeMap<P::String, IpldGeneric<P>>,
    next_key: Option<P::String>,
}

pub struct SerializeStructVariant<P>
where
    P: Primitives,
{
    name: P::String,
    map: BTreeMap<P::String, IpldGeneric<P>>,
}

impl<P> ser::SerializeSeq for SerializeVec<P>
where
    P: Primitives,
{
    type Ok = IpldGeneric<P>;
    type Error = SerdeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        self.vec.push(value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Self::Ok::List(self.vec))
    }
}

impl<P> ser::SerializeTuple for SerializeVec<P>
where
    P: Primitives,
{
    type Ok = IpldGeneric<P>;
    type Error = SerdeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<P> ser::SerializeTupleStruct for SerializeVec<P>
where
    P: Primitives,
{
    type Ok = IpldGeneric<P>;
    type Error = SerdeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<P> ser::SerializeTupleVariant for SerializeTupleVariant<P>
where
    P: Primitives,
{
    type Ok = IpldGeneric<P>;
    type Error = SerdeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        self.vec.push(value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let map = BTreeMap::from([(self.name.into(), Self::Ok::List(self.vec))]);
        Ok(Self::Ok::Map(map))
    }
}

impl<P> ser::SerializeMap for SerializeMap<P>
where
    P: Primitives,
{
    type Ok = IpldGeneric<P>;
    type Error = SerdeError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        match key.serialize(Serializer)? {
            IpldGeneric::String(string_key) => {
                self.next_key = Some(string_key);
                Ok(())
            }
            _ => Err(ser::Error::custom("Map keys must be strings".to_string())),
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        let key = self.next_key.take();
        // Panic because this indicates a bug in the program rather than an
        // expected failure.
        let key = key.expect("serialize_value called before serialize_key");
        self.map.insert(key, value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Self::Ok::Map(self.map))
    }
}

impl<P> ser::SerializeStruct for SerializeMap<P>
where
    P: Primitives,
{
    type Ok = IpldGeneric<P>;
    type Error = SerdeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        serde::ser::SerializeMap::serialize_key(self, key)?;
        serde::ser::SerializeMap::serialize_value(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeMap::end(self)
    }
}

impl<P> ser::SerializeStructVariant for SerializeStructVariant<P>
where
    P: Primitives,
{
    type Ok = IpldGeneric<P>;
    type Error = SerdeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize + ?Sized,
    {
        self.map
            .insert(key.to_string().into(), value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut object = BTreeMap::new();

        object.insert(self.name, Self::Ok::Map(self.map));

        Ok(Self::Ok::Map(object))
    }
}
