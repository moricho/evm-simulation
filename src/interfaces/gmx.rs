use anyhow::Result;
use ethers::prelude::BaseContract;
use ethers::utils::keccak256;
use ethers_contract::{abigen, EthAbiType};
use serde::{Deserialize, Serialize};
use ethers::types::{Address, H256, U256, H160,Bytes};
use bytes::Bytes as OutputBytes;
use ethers_core::abi::{Tokenizable, Detokenize, Tokenize};
// use ethers_core::abi::Tokenizable;

// abigen!(GmxV2Reader, "./src/interfaces/abi/gmx_v2/reader.json");
#[derive(Clone)]
pub struct GmxV2ABI {
    pub abi: BaseContract,
}

// NOTE: Consider using Abigen to generate abi bindings
// Also, create enum for u8 values like order_type, decrease_position_swap_type

// For createOrder
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize, EthAbiType)]
pub struct CreateOrderParamsAddresses {
    pub receiver: Address,
    pub callback_contract: Address,
    pub ui_fee_receiver: Address,
    pub market: Address,
    pub initial_collateral_token: Address,
    pub swap_path: Vec<Address>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize, EthAbiType)]
pub struct CreateOrderParamsNumbers {
    pub size_delta_usd: U256,
    pub initial_collateral_delta_amount: U256,
    pub trigger_price: U256,
    pub acceptable_price: U256,
    pub execution_fee: U256,
    pub callback_gas_limit: U256,
    pub min_output_amount: U256,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize, EthAbiType)]
pub struct CreateOrderParams {
    pub addresses: CreateOrderParamsAddresses,
    pub numbers: CreateOrderParamsNumbers,
    pub order_type: u8, // Enumの値
    pub decrease_position_swap_type: u8, // Enumの値
    pub is_long: bool,
    pub should_unwrap_native_token: bool,
    pub referral_code: H256,
}

// For getAccountPositions
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Addresses {
    pub account: Address,
    pub market: Address,
    pub collateral_token: Address,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Numbers {
    pub size_in_usd: U256,
    pub size_in_tokens: U256,
    pub collateral_amount: U256,
    pub borrowing_factor: U256,
    pub funding_fee_amount_per_size: U256,
    pub long_token_claimable_funding_amount_per_size: U256,
    pub short_token_claimable_funding_amount_per_size: U256,
    pub increased_at_block: U256,
    pub decreased_at_block: U256,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Flags {
    pub is_long: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PositionProps {
    pub addresses: Addresses,
    pub numbers: Numbers,
    pub flags: Flags,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize, EthAbiType)]
pub struct PriceProps {
    pub min: U256,
    pub max: U256,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize, EthAbiType)]
pub struct MarketPrices {
    pub index_token_price: PriceProps,
    pub long_token_price: PriceProps,
    pub short_token_price: PriceProps,

}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize, EthAbiType)]
pub struct PositionInfo {
    pub position_props: bool, // TODO: Change to PositionProps
}

pub struct TokenInfo {
    pub name: &'static str,
    pub address: &'static str,
    pub decimals: u8,
}

pub enum Token {
    ETH,
    BTC
}

impl Token {
    pub fn info(&self) -> TokenInfo {
        match self {
            Token::ETH => TokenInfo { name: "ETH", address: "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1", decimals: 18 },
            Token::BTC => TokenInfo { name: "BTC", address: "0x47904963fc8b2340414262125aF798B9655E58Cd", decimals: 8 },
        }
    }

    pub fn from_name(name: &str) -> Option<Token> {
        match name {
            "ETH" => Some(Token::ETH),
            "BTC" => Some(Token::BTC),
            _ => None
        }
    }

    pub fn address_from_name(name: &str) -> Option<String> {
        Token::from_name(name)
            .map(|token| token.info().address.to_string())
    }
}

// impl GmxV2ABI {
//     pub fn new() -> Self {
//         println!("GmxV2ABI::new");
//         let abi = BaseContract::from(
//             parse_abi(&[
//                 // ExchangeRounter Contract
//                 "function multicall(bytes[] calldata data) external payable virtual returns (bytes[] memory results)",
//                 // "function createOrder(IBaseOrderUtils.CreateOrderParams calldata params) external payable returns (bytes32)",
//                 // "function createOrder(((address,address,address,address,address,address[]),(uint256,uint256,uint256,uint256,uint256,uint256,uint256),uint8,uint8,bool,bool,bytes32)) external payable returns (bytes32)",
//                 "function sendWnt(address receiver, uint256 amount) external payable",

//                 // reader contract: 0x22199a49A999c351eF7927602CFB187ec3cae489
//                 "function getPositionInfo(DataStore dataStore,IReferralStorage referralStorage,bytes32 positionKey,MarketUtils.MarketPrices memory prices,uint256 sizeDeltaUsd,address uiFeeReceiver,bool usePositionSizeAsSizeDeltaUsd,) public view returns (ReaderUtils.PositionInfo memory)",
//             ])
//             .unwrap(),
//         );
//         println!("GmxV2ABI::new end");
//         Self {  abi}
//     }

//     pub fn multicall_input(&self, data: Vec<Bytes>) -> Result<Bytes> {
//         let calldata = self.abi.encode("multicall", data)?;
//         Ok(calldata)
//     }

//     pub fn multicall_output(&self, output: OutputBytes) -> Result<Vec<Bytes>> {
//         let results: Vec<Bytes> = self.abi.decode("multicall", output)?;
//         Ok(results)
//     }

//     pub fn create_order_input(&self, params: CreateOrderParams) -> Result<Bytes> {
//         let calldata = self.abi.encode("createOrder", params)?;
//         Ok(calldata)
//     }

//     pub fn send_wnt_input(&self, receiver: Address, amount: U256) -> Result<Bytes> {
//         let calldata = self.abi.encode("sendWnt", (receiver, amount))?;
//         Ok(calldata)
//     }

//     pub fn get_position_info_input(&self, data_store: Address, referral_storage: Address, position_key: H256, prices: MarketPrices, size_delta_usd: U256, ui_fee_receiver: Address, use_position_size_as_size_delta_usd: bool) -> Result<Bytes> {
//         let calldata = self.abi.encode("getPositionInfo", (data_store, referral_storage, position_key, prices, size_delta_usd, ui_fee_receiver, use_position_size_as_size_delta_usd))?;
//         Ok(calldata)
//     }

//     pub fn get_position_info_output(&self, output: OutputBytes) -> Result<PositionInfo> {
//         let position_info: PositionInfo = self.abi.decode("getPositionInfo", output)?;
//         Ok(position_info)
//     }

// }


use ethers::abi::Token as AbiToken;

pub fn get_position_key(account: H160, market: H160, collateral_token: H160, is_long: bool) -> H256 {
    let data_values: Vec<AbiToken> = vec![
        AbiToken::Address(account),
        AbiToken::Address(market),
        AbiToken::Address(collateral_token),
        AbiToken::Bool(is_long),
    ];

    let hash_hex = hash_data(data_values);
    // Convert hex string to H256
    H256::from_slice(&hex::decode(hash_hex).expect("Invalid hex string"))
}

pub fn hash_data(data_values: Vec<AbiToken>) -> String {
    let encoded_bytes = ethers::abi::encode(&data_values);
    let hash = keccak256(encoded_bytes);
    hex::encode(hash)
}
