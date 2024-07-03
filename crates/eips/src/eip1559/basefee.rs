use crate::{
    calc_next_block_base_fee,
    eip1559::constants::{
        BASE_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
        DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR, DEFAULT_ELASTICITY_MULTIPLIER,
        OP_MAINNET_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
        OP_MAINNET_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
        OP_SEPOLIA_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
        OP_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
    },
};

/// BaseFeeParams contains the config parameters that control block base fee computation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseFeeParams {
    /// The base_fee_max_change_denominator from EIP-1559
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity"))]
    pub max_change_denominator: u128,
    /// The elasticity multiplier from EIP-1559
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity"))]
    pub elasticity_multiplier: u128,
}

impl BaseFeeParams {
    /// Create a new BaseFeeParams
    pub const fn new(max_change_denominator: u128, elasticity_multiplier: u128) -> Self {
        Self { max_change_denominator, elasticity_multiplier }
    }

    /// Get the base fee parameters for Ethereum mainnet
    pub const fn ethereum() -> Self {
        Self {
            max_change_denominator: DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR as u128,
            elasticity_multiplier: DEFAULT_ELASTICITY_MULTIPLIER as u128,
        }
    }

    /// Get the base fee parameters for Optimism Mainnet
    pub const fn optimism() -> Self {
        Self {
            max_change_denominator: OP_MAINNET_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
            elasticity_multiplier: OP_MAINNET_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
        }
    }

    /// Get the base fee parameters for Optimism Sepolia
    pub const fn optimism_sepolia() -> Self {
        Self {
            max_change_denominator: OP_SEPOLIA_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
            elasticity_multiplier: OP_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
        }
    }

    /// Get the base fee parameters for Base Sepolia
    pub const fn base_sepolia() -> Self {
        Self {
            max_change_denominator: OP_SEPOLIA_EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR,
            elasticity_multiplier: BASE_SEPOLIA_EIP1559_DEFAULT_ELASTICITY_MULTIPLIER,
        }
    }

    /// Calculate the base fee for the next block based on the EIP-1559 specification.
    ///
    /// See also [calc_next_block_base_fee]
    #[inline]
    pub fn next_block_base_fee(self, gas_used: u128, gas_limit: u128, base_fee: u128) -> u128 {
        calc_next_block_base_fee(gas_used, gas_limit, base_fee, self)
    }
}
