use clap::Parser;
use move_core_types::language_storage::TypeTag;
use move_core_types::parser::parse_type_tag;
use std::{fmt, str::FromStr};

/// TypeTag vector container.
#[derive(Debug, Parser, Clone)]
pub struct TypeArgVec {
    /// TypeTag arguments separated by spaces.
    ///
    /// Example: `u8 u16 u32 u64 u128 u256 bool address vector signer`.
    #[clap(long, multiple_values(true))]
    pub(super) type_args: Vec<MoveType>,
}

// This function cannot handle the full range of types that MoveType can
// represent. Internally, it uses parse_type_tag, which cannot handle references
// or generic type parameters. This function adds nominal support for references
// on top of parse_type_tag, but it still does not work for generic type params.
// For that, we have the Unparsable variant of MoveType, so the deserialization
// doesn't fail when dealing with these values.
impl FromStr for MoveType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s;
        let mut is_ref = false;
        let mut is_mut = false;

        if s.starts_with('&') {
            s = &s[1..];
            is_ref = true;
        }
        if is_ref && s.starts_with("mut ") {
            s = &s[4..];
            is_mut = true;
        }
        // Previously this would just crap out, but this meant the API could
        // return a serialized version of an object and not be able to
        // deserialize it using that same object.
        let inner = match parse_type_tag(s) {
            Ok(inner) => inner.into(),
            Err(_e) => MoveType::Unparsable(s.to_string()),
        };
        if is_ref {
            Ok(MoveType::Reference {
                mutable: is_mut,
                to: Box::new(inner),
            })
        } else {
            Ok(inner)
        }
    }
}

impl From<TypeTag> for MoveType {
    fn from(tag: TypeTag) -> Self {
        match tag {
            TypeTag::Bool => MoveType::Bool,
            TypeTag::U8 => MoveType::U8,
            TypeTag::U16 => MoveType::U16,
            TypeTag::U32 => MoveType::U32,
            TypeTag::U64 => MoveType::U64,
            TypeTag::U256 => MoveType::U256,
            TypeTag::U128 => MoveType::U128,
            TypeTag::Address => MoveType::Address,
            TypeTag::Signer => MoveType::Signer,
            TypeTag::Vector(v) => MoveType::Vector {
                items: Box::new(MoveType::from(*v)),
            },
            TypeTag::Struct(_) => unreachable!("not supported by setheum-move"),
        }
    }
}

impl From<&TypeTag> for MoveType {
    fn from(tag: &TypeTag) -> Self {
        match tag {
            TypeTag::Bool => MoveType::Bool,
            TypeTag::U8 => MoveType::U8,
            TypeTag::U16 => MoveType::U16,
            TypeTag::U32 => MoveType::U32,
            TypeTag::U64 => MoveType::U64,
            TypeTag::U128 => MoveType::U128,
            TypeTag::U256 => MoveType::U256,
            TypeTag::Address => MoveType::Address,
            TypeTag::Signer => MoveType::Signer,
            TypeTag::Vector(v) => MoveType::Vector {
                items: Box::new(MoveType::from(v.as_ref())),
            },
            TypeTag::Struct(_) => unreachable!("not supported by setheum-move"),
        }
    }
}

impl TryFrom<&MoveType> for TypeTag {
    type Error = anyhow::Error;

    fn try_from(tag: &MoveType) -> anyhow::Result<Self> {
        let ret = match tag {
            MoveType::Bool => TypeTag::Bool,
            MoveType::U8 => TypeTag::U8,
            MoveType::U16 => TypeTag::U16,
            MoveType::U32 => TypeTag::U32,
            MoveType::U64 => TypeTag::U64,
            MoveType::U128 => TypeTag::U128,
            MoveType::U256 => TypeTag::U256,
            MoveType::Address => TypeTag::Address,
            MoveType::Signer => TypeTag::Signer,
            MoveType::Vector { items } => TypeTag::Vector(Box::new(items.as_ref().try_into()?)),
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid move type for converting into `TypeTag`: {tag:?}",
                ))
            }
        };
        Ok(ret)
    }
}

/// An enum of Move's possible types on-chain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MoveType {
    /// A bool type.
    Bool,
    /// An 8-bit unsigned int.
    U8,
    /// A 16-bit unsigned int.
    U16,
    /// A 32-bit unsigned int.
    U32,
    /// A 64-bit unsigned int.
    U64,
    /// A 128-bit unsigned int.
    U128,
    /// A 256-bit unsigned int.
    U256,
    /// A 32-byte account address.
    Address,
    /// An account signer.
    Signer,
    /// A Vector of [`MoveType`].
    Vector { items: Box<MoveType> },
    /// A reference
    Reference { mutable: bool, to: Box<MoveType> },
    /// A move type that couldn't be parsed.
    ///
    /// This prevents the parser from just throwing an error because one field
    /// was unparsable, and gives the value in it.
    Unparsable(String),
}

impl fmt::Display for MoveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveType::U8 => write!(f, "u8"),
            MoveType::U16 => write!(f, "u16"),
            MoveType::U32 => write!(f, "u32"),
            MoveType::U64 => write!(f, "u64"),
            MoveType::U128 => write!(f, "u128"),
            MoveType::U256 => write!(f, "u256"),
            MoveType::Address => write!(f, "address"),
            MoveType::Signer => write!(f, "signer"),
            MoveType::Bool => write!(f, "bool"),
            MoveType::Vector { items } => write!(f, "vector<{}>", items),
            MoveType::Reference { mutable, to } => {
                if *mutable {
                    write!(f, "&mut {}", to)
                } else {
                    write!(f, "&{}", to)
                }
            }
            MoveType::Unparsable(string) => write!(f, "unparsable<{}>", string),
        }
    }
}
