use anyhow::{format_err, Error, Result};
use clap::Parser;
use move_core_types::account_address::AccountAddress;
use move_core_types::u256::U256;
use serde::{Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Parser)]
pub(super) struct ArgWithTypeVec {
    /// Arguments combined with their type separated by spaces.
    ///
    /// Supported types [address, bool, hex, string, u8, u16, u32, u64, u128, u256, raw].
    ///
    /// Vectors may be specified using JSON array literal syntax (you may need to escape this with
    /// quotes based on your shell interpreter).
    ///
    /// Example: `address:0x1 bool:true u8:0 u256:1234 "bool:[true, false]" 'address:[["0xace", "0xbee"], []]'`
    #[clap(long, multiple_values(true))]
    pub(super) args: Vec<ArgWithType>,
}

/// A parseable arg with a type separated by a colon.
#[derive(Clone, Debug)]
pub(super) struct ArgWithType {
    vector_depth: u8,
    pub(super) arg: Vec<u8>,
}

/// Does not support string arguments that contain the following characters:
///
/// * `,`
/// * `[`
/// * `]`
impl FromStr for ArgWithType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Splits on the first colon, returning at most two elements.
        // This is required to support args that contain a colon.
        let parts: Vec<_> = s.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(Error::msg(
                "Arguments must be pairs of <type>:<arg> e.g. bool:true".to_string(),
            ));
        }

        // These unwraps can't fail.
        let ty = FunctionArgType::from_str(parts.first().unwrap())?;
        let mut arg = String::from(*parts.last().unwrap());
        // May need to surround with quotes if not an array, so arg can be parsed into JSON.
        if !arg.starts_with('[') {
            if let FunctionArgType::Address
            | FunctionArgType::Signer
            | FunctionArgType::Hex
            | FunctionArgType::String
            | FunctionArgType::Raw = ty
            {
                arg = format!("\"{arg}\"");
            }
        }

        let json = serde_json::from_str::<serde_json::Value>(arg.as_str()).map_err(Error::msg)?;
        ty.parse_arg_json(&json)
    }
}

/// Type of the function argument.
#[derive(Clone, Debug, PartialEq, Eq)]
enum FunctionArgType {
    Signer,
    Address,
    Bool,
    Hex,
    String,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Raw,
}

impl fmt::Display for FunctionArgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionArgType::Signer => write!(f, "signer"),
            FunctionArgType::Address => write!(f, "address"),
            FunctionArgType::Bool => write!(f, "bool"),
            FunctionArgType::Hex => write!(f, "hex"),
            FunctionArgType::String => write!(f, "string"),
            FunctionArgType::U8 => write!(f, "u8"),
            FunctionArgType::U16 => write!(f, "u16"),
            FunctionArgType::U32 => write!(f, "u32"),
            FunctionArgType::U64 => write!(f, "u64"),
            FunctionArgType::U128 => write!(f, "u128"),
            FunctionArgType::U256 => write!(f, "u256"),
            FunctionArgType::Raw => write!(f, "raw"),
        }
    }
}

/// Parse the address which can have multiple formats.
///
/// - Move address format
///   - Strict raw 32-hex format (e.g. 1234abce5678ef901234abce5678ef901234abce5678ef901234abce5678ef90)
///   - Flexible prefixed hex format (e.g. 0x5)
///
/// - SS58 address format
///   - e.g. 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694t
fn parse_address(addr: &str) -> Result<AccountAddress> {
    if let Ok(addr) = move_vm_support::ss58_address::ss58_to_move_address(addr) {
        // TODO: distant future - if an error is in ss58 address, the user won't get any ss58-related error
        Ok(addr)
    } else {
        AccountAddress::from_str(addr).map_err(Error::msg)
    }
}

impl FunctionArgType {
    /// Parse a standalone argument (not a vector) from string slice into BCS representation.
    fn parse_arg_str(&self, arg: &str) -> Result<Vec<u8>> {
        match self {
            FunctionArgType::Signer => bcs::to_bytes(&parse_address(arg)?).map_err(Error::msg),
            FunctionArgType::Address => bcs::to_bytes(&parse_address(arg)?).map_err(Error::msg),
            FunctionArgType::Bool => {
                bcs::to_bytes(&bool::from_str(arg).map_err(Error::msg)?).map_err(Error::msg)
            }
            FunctionArgType::Hex => {
                bcs::to_bytes(HexEncodedBytes::from_str(arg).map_err(Error::msg)?.inner())
                    .map_err(Error::msg)
            }
            FunctionArgType::String => bcs::to_bytes(arg).map_err(Error::msg),
            FunctionArgType::U8 => {
                bcs::to_bytes(&u8::from_str(arg).map_err(Error::msg)?).map_err(Error::msg)
            }
            FunctionArgType::U16 => {
                bcs::to_bytes(&u16::from_str(arg).map_err(Error::msg)?).map_err(Error::msg)
            }
            FunctionArgType::U32 => bcs::to_bytes(&u32::from_str(arg)?).map_err(Error::msg),
            FunctionArgType::U64 => bcs::to_bytes(&u64::from_str(arg)?).map_err(Error::msg),
            FunctionArgType::U128 => {
                bcs::to_bytes(&u128::from_str(arg).map_err(Error::msg)?).map_err(Error::msg)
            }
            FunctionArgType::U256 => {
                bcs::to_bytes(&U256::from_str(arg).map_err(Error::msg)?).map_err(Error::msg)
            }
            FunctionArgType::Raw => Ok(HexEncodedBytes::from_str(arg)
                .map_err(Error::msg)?
                .inner()
                .to_vec()),
        }
    }

    /// Recursively parse argument JSON into BCS representation.
    fn parse_arg_json(&self, arg: &serde_json::Value) -> Result<ArgWithType> {
        match arg {
            serde_json::Value::Bool(value) => Ok(ArgWithType {
                vector_depth: 0,
                arg: self.parse_arg_str(value.to_string().as_str())?,
            }),
            serde_json::Value::Number(value) => Ok(ArgWithType {
                vector_depth: 0,
                arg: self.parse_arg_str(value.to_string().as_str())?,
            }),
            serde_json::Value::String(value) => Ok(ArgWithType {
                vector_depth: 0,
                arg: self.parse_arg_str(value.as_str())?,
            }),
            serde_json::Value::Array(_) => {
                let mut bcs: Vec<u8> = vec![]; // BCS representation of argument.
                let mut common_sub_arg_depth = None;
                // Prepend argument sequence length to BCS bytes vector.
                write_u64_as_uleb128(&mut bcs, arg.as_array().unwrap().len());
                // Loop over all of the vector's sub-arguments, which may also be vectors:
                for sub_arg in arg.as_array().unwrap() {
                    let ArgWithType {
                        vector_depth: sub_arg_depth,
                        arg: mut sub_arg_bcs,
                    } = self.parse_arg_json(sub_arg)?;
                    // Verify all sub-arguments have same depth.
                    if let Some(check_depth) = common_sub_arg_depth {
                        if check_depth != sub_arg_depth {
                            return Err(Error::msg("Variable vector depth".to_string()));
                        }
                    };
                    common_sub_arg_depth = Some(sub_arg_depth);
                    bcs.append(&mut sub_arg_bcs); // Append sub-argument BCS.
                }

                // Default sub-argument depth is 0 for when no sub-arguments were looped over.
                Ok(ArgWithType {
                    vector_depth: common_sub_arg_depth.unwrap_or(0) + 1,
                    arg: bcs,
                })
            }
            serde_json::Value::Null => Err(Error::msg("Null argument".to_string())),
            serde_json::Value::Object(_) => Err(Error::msg("JSON object argument".to_string())),
        }
    }
}

// A copied function from: substrate-move/language/move-binary-format/src/file_format_common.rs
// since that function is still not a public function.
fn write_u64_as_uleb128(binary: &mut Vec<u8>, mut val: usize) {
    loop {
        let cur = val & 0x7F;
        if cur != val {
            binary.push((cur | 0x80) as u8);
            val >>= 7;
        } else {
            binary.push(cur as u8);
            break;
        }
    }
}

impl FromStr for FunctionArgType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "signer" => Ok(FunctionArgType::Signer),
            "address" => Ok(FunctionArgType::Address),
            "bool" => Ok(FunctionArgType::Bool),
            "hex" => Ok(FunctionArgType::Hex),
            "string" => Ok(FunctionArgType::String),
            "u8" => Ok(FunctionArgType::U8),
            "u16" => Ok(FunctionArgType::U16),
            "u32" => Ok(FunctionArgType::U32),
            "u64" => Ok(FunctionArgType::U64),
            "u128" => Ok(FunctionArgType::U128),
            "u256" => Ok(FunctionArgType::U256),
            "raw" => Ok(FunctionArgType::Raw),
            wrong_arg => {Err(Error::msg(format!(
                "Invalid arg type '{wrong_arg}'.  Must be one of: ['{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}']",
                FunctionArgType::Signer,
                FunctionArgType::Address,
                FunctionArgType::Bool,
                FunctionArgType::Hex,
                FunctionArgType::String,
                FunctionArgType::U8,
                FunctionArgType::U16,
                FunctionArgType::U32,
                FunctionArgType::U64,
                FunctionArgType::U128,
                FunctionArgType::U256,
                FunctionArgType::Raw)))
            }
        }
    }
}

/// Hex encoded bytes to allow for having bytes represented in JSON.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct HexEncodedBytes(Vec<u8>);

impl FromStr for HexEncodedBytes {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self, anyhow::Error> {
        let hex_str = if let Some(hex) = s.strip_prefix("0x") {
            hex
        } else {
            s
        };
        Ok(Self(hex::decode(hex_str).map_err(|e| {
            format_err!("decode hex-encoded string({s:?}) failed, caused by error: {e}",)
        })?))
    }
}

impl From<[u8; 32]> for HexEncodedBytes {
    fn from(arr: [u8; 32]) -> Self {
        HexEncodedBytes(arr.to_vec())
    }
}

impl fmt::Display for HexEncodedBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(&self.0))
    }
}

impl Serialize for HexEncodedBytes {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl HexEncodedBytes {
    fn inner(&self) -> &[u8] {
        &self.0
    }
}
