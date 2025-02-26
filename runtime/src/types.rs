use crate::{metadata, Config, InterBtcRuntime, SS58_PREFIX};
pub use metadata_aliases::*;
use subxt::sp_core::{crypto::Ss58Codec, sr25519::Pair as KeyPair};

pub use primitives::{
    CurrencyId,
    CurrencyId::Token,
    TokenSymbol::{DOT, IBTC, INTR, KBTC, KINT, KSM},
};

pub use currency_id::CurrencyIdExt;
pub use h256_le::RichH256Le;
pub use module_btc_relay::{RichBlockHeader, MAIN_CHAIN_ID};

pub type AccountId = subxt::sp_runtime::AccountId32;
pub type Balance = primitives::Balance;
pub type Index = u32;
pub type BlockNumber = u32;
pub type H160 = subxt::sp_core::H160;
pub type H256 = subxt::sp_core::H256;
pub type U256 = subxt::sp_core::U256;

pub type InterBtcSigner = subxt::PairSigner<InterBtcRuntime, KeyPair>;

pub type BtcAddress = module_btc_relay::BtcAddress;

pub type FixedU128 = sp_arithmetic::FixedU128;

mod metadata_aliases {
    use super::*;

    pub type BtcPublicKey = metadata::runtime_types::bitcoin::address::PublicKey;

    pub type OracleKey = metadata::runtime_types::interbtc_primitives::oracle::Key;

    pub type StatusCode = metadata::runtime_types::security::types::StatusCode;
    pub type ErrorCode = metadata::runtime_types::security::types::ErrorCode;
    pub type RawBlockHeader = metadata::runtime_types::bitcoin::types::RawBlockHeader;
    pub type VaultStatus = metadata::runtime_types::vault_registry::types::VaultStatus;
    pub type InterBtcVault =
        metadata::runtime_types::vault_registry::types::Vault<AccountId, BlockNumber, Balance, CurrencyId>;
    pub type Wallet = metadata::runtime_types::vault_registry::types::Wallet;
    pub type InterBtcRichBlockHeader = metadata::runtime_types::btc_relay::types::RichBlockHeader<BlockNumber>;
    pub type BitcoinBlockHeight = u32;

    pub type FeedValuesEvent = metadata::oracle::events::FeedValues;

    pub type CancelIssueEvent = metadata::issue::events::CancelIssue;
    pub type ExecuteIssueEvent = metadata::issue::events::ExecuteIssue;
    pub type RequestIssueEvent = metadata::issue::events::RequestIssue;

    pub type AcceptReplaceEvent = metadata::replace::events::AcceptReplace;
    pub type ExecuteReplaceEvent = metadata::replace::events::ExecuteReplace;
    pub type RequestReplaceEvent = metadata::replace::events::RequestReplace;
    pub type WithdrawReplaceEvent = metadata::replace::events::WithdrawReplace;
    pub type CancelReplaceEvent = metadata::replace::events::CancelReplace;

    pub type RequestRefundEvent = metadata::refund::events::RequestRefund;
    pub type ExecuteRefundEvent = metadata::refund::events::ExecuteRefund;

    pub type RequestRedeemEvent = metadata::redeem::events::RequestRedeem;
    pub type ExecuteRedeemEvent = metadata::redeem::events::ExecuteRedeem;

    pub type UpdateActiveBlockEvent = metadata::security::events::UpdateActiveBlock;

    pub type RegisterVaultEvent = metadata::vault_registry::events::RegisterVault;
    pub type RegisterAddressEvent = metadata::vault_registry::events::RegisterAddress;
    pub type DepositCollateralEvent = metadata::vault_registry::events::DepositCollateral;
    pub type LiquidateVaultEvent = metadata::vault_registry::events::LiquidateVault;

    pub type StoreMainChainHeaderEvent = metadata::btc_relay::events::StoreMainChainHeader;

    pub type VaultTheftEvent = metadata::relay::events::VaultTheft;
    pub type VaultDoublePaymentEvent = metadata::relay::events::VaultDoublePayment;

    pub type EndowedEvent = metadata::tokens::events::Endowed;

    pub type BtcRelayPalletError = metadata::runtime_types::btc_relay::pallet::Error;
    pub type IssuePalletError = metadata::runtime_types::issue::pallet::Error;
    pub type RedeemPalletError = metadata::runtime_types::redeem::pallet::Error;
    pub type RelayPalletError = metadata::runtime_types::relay::pallet::Error;
    pub type SecurityPalletError = metadata::runtime_types::security::pallet::Error;
    pub type SystemPalletError = metadata::runtime_types::frame_system::pallet::Error;

    pub type H256Le = metadata::runtime_types::bitcoin::types::H256Le;

    pub type InterBtcHeader = <InterBtcRuntime as Config>::Header;

    pub type InterBtcIssueRequest =
        metadata::runtime_types::interbtc_primitives::issue::IssueRequest<AccountId, BlockNumber, Balance, CurrencyId>;
    pub type IssueRequestStatus = metadata::runtime_types::interbtc_primitives::issue::IssueRequestStatus;
    pub type InterBtcRedeemRequest = metadata::runtime_types::interbtc_primitives::redeem::RedeemRequest<
        AccountId,
        BlockNumber,
        Balance,
        CurrencyId,
    >;
    pub type RedeemRequestStatus = metadata::runtime_types::interbtc_primitives::redeem::RedeemRequestStatus;
    pub type ReplaceRequestStatus = metadata::runtime_types::interbtc_primitives::replace::ReplaceRequestStatus;
    pub type InterBtcRefundRequest =
        metadata::runtime_types::interbtc_primitives::refund::RefundRequest<AccountId, Balance, CurrencyId>;
    pub type InterBtcReplaceRequest = metadata::runtime_types::interbtc_primitives::replace::ReplaceRequest<
        AccountId,
        BlockNumber,
        Balance,
        CurrencyId,
    >;
    pub type VaultId = metadata::runtime_types::interbtc_primitives::VaultId<AccountId, CurrencyId>;
    pub type VaultCurrencyPair = metadata::runtime_types::interbtc_primitives::VaultCurrencyPair<CurrencyId>;

    #[cfg(feature = "parachain-metadata-interlay")]
    pub type EncodedCall = metadata::runtime_types::interlay_runtime_parachain::Call;
    #[cfg(feature = "parachain-metadata-kintsugi")]
    pub type EncodedCall = metadata::runtime_types::kintsugi_runtime_parachain::Call;
    #[cfg(feature = "parachain-metadata-testnet")]
    pub type EncodedCall = metadata::runtime_types::testnet_runtime_parachain::Call;
    #[cfg(feature = "standalone-metadata")]
    pub type EncodedCall = metadata::runtime_types::interbtc_runtime_standalone::Call;

    pub type SecurityCall = metadata::runtime_types::security::pallet::Call;
}

impl crate::RawBlockHeader {
    pub fn hash(&self) -> crate::H256Le {
        module_bitcoin::utils::sha256d_le(&self.0).into()
    }
}

impl From<[u8; 33]> for crate::BtcPublicKey {
    fn from(input: [u8; 33]) -> Self {
        crate::BtcPublicKey { 0: input }
    }
}

mod currency_id {
    use super::*;

    pub trait CurrencyIdExt {
        fn inner(&self) -> primitives::TokenSymbol;
    }

    impl CurrencyIdExt for CurrencyId {
        fn inner(&self) -> primitives::TokenSymbol {
            match self {
                Token(x) => *x,
            }
        }
    }
}

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

mod account_id {
    use super::*;

    impl PrettyPrint for AccountId {
        fn pretty_print(&self) -> String {
            self.to_ss58check_with_version(SS58_PREFIX.into())
        }
    }
}

mod vault_id {
    use super::*;
    use primitives::CurrencyInfo;

    type RichVaultId = primitives::VaultId<AccountId, primitives::CurrencyId>;

    impl crate::VaultId {
        pub fn new(account_id: AccountId, collateral_currency: CurrencyId, wrapped_currency: CurrencyId) -> Self {
            Self {
                account_id,
                currencies: VaultCurrencyPair {
                    collateral: collateral_currency,
                    wrapped: wrapped_currency,
                },
            }
        }

        pub fn collateral_currency(&self) -> CurrencyId {
            self.currencies.collateral
        }

        pub fn wrapped_currency(&self) -> CurrencyId {
            self.currencies.wrapped
        }
    }

    impl PrettyPrint for VaultId {
        fn pretty_print(&self) -> String {
            let collateral_currency: CurrencyId = self.collateral_currency();
            let wrapped_currency: CurrencyId = self.wrapped_currency();
            format!(
                "{}[{}->{}]",
                self.account_id.pretty_print(),
                collateral_currency.inner().symbol(),
                wrapped_currency.inner().symbol()
            )
        }
    }

    impl From<crate::VaultId> for RichVaultId {
        fn from(value: crate::VaultId) -> Self {
            Self {
                account_id: value.account_id,
                currencies: primitives::VaultCurrencyPair {
                    collateral: value.currencies.collateral,
                    wrapped: value.currencies.wrapped,
                },
            }
        }
    }

    impl From<RichVaultId> for crate::VaultId {
        fn from(value: RichVaultId) -> Self {
            Self {
                account_id: value.account_id,
                currencies: crate::VaultCurrencyPair {
                    collateral: value.currencies.collateral,
                    wrapped: value.currencies.wrapped,
                },
            }
        }
    }

    impl serde::Serialize for crate::VaultId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let value: RichVaultId = self.clone().into();
            value.serialize(serializer)
        }
    }

    impl<'de> serde::Deserialize<'de> for crate::VaultId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let value = RichVaultId::deserialize(deserializer)?;
            Ok(value.into())
        }
    }

    impl std::hash::Hash for crate::VaultId {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            let vault: RichVaultId = self.clone().into();
            vault.hash(state)
        }
    }
}

mod h256_le {
    use super::*;

    pub type RichH256Le = module_bitcoin::types::H256Le;

    impl From<RichH256Le> for crate::H256Le {
        fn from(value: RichH256Le) -> Self {
            Self {
                content: value.to_bytes_le(),
            }
        }
    }

    impl From<crate::H256Le> for RichH256Le {
        fn from(value: crate::H256Le) -> Self {
            Self::from_bytes_le(&value.content)
        }
    }

    impl crate::H256Le {
        pub fn from_bytes_le(bytes: &[u8]) -> H256Le {
            RichH256Le::from_bytes_le(bytes).into()
        }
        pub fn to_bytes_le(&self) -> [u8; 32] {
            RichH256Le::to_bytes_le(&self.clone().into())
        }
        pub fn is_zero(&self) -> bool {
            RichH256Le::is_zero(&self.clone().into())
        }
        pub fn to_hex_le(&self) -> String {
            RichH256Le::to_hex_le(&self.clone().into())
        }
    }
}

mod dispatch_error {
    use crate::metadata::{
        runtime_types::sp_runtime::{ArithmeticError, ModuleError, TokenError},
        DispatchError,
    };

    type RichTokenError = sp_runtime::TokenError;
    type RichArithmeticError = sp_runtime::ArithmeticError;
    type RichDispatchError = sp_runtime::DispatchError;
    type RichModuleError = sp_runtime::ModuleError;

    macro_rules! convert_enum{($src: ident, $dst: ident, $($variant: ident,)*)=> {
        impl From<$src> for $dst {
            fn from(src: $src) -> Self {
                match src {
                    $($src::$variant => Self::$variant,)*
                }
            }
        }
    }}

    convert_enum!(
        RichTokenError,
        TokenError,
        NoFunds,
        WouldDie,
        BelowMinimum,
        CannotCreate,
        UnknownAsset,
        Frozen,
        Unsupported,
    );

    convert_enum!(
        RichArithmeticError,
        ArithmeticError,
        Underflow,
        Overflow,
        DivisionByZero,
    );

    impl From<RichDispatchError> for DispatchError {
        fn from(value: RichDispatchError) -> Self {
            match value {
                RichDispatchError::Other(_) => DispatchError::Other,
                RichDispatchError::CannotLookup => DispatchError::CannotLookup,
                RichDispatchError::BadOrigin => DispatchError::BadOrigin,
                RichDispatchError::Module(RichModuleError { index, error, .. }) => {
                    DispatchError::Module(ModuleError { index, error })
                }
                RichDispatchError::ConsumerRemaining => DispatchError::ConsumerRemaining,
                RichDispatchError::NoProviders => DispatchError::NoProviders,
                RichDispatchError::TooManyConsumers => DispatchError::TooManyConsumers,
                RichDispatchError::Token(token_error) => DispatchError::Token(token_error.into()),
                RichDispatchError::Arithmetic(arithmetic_error) => DispatchError::Arithmetic(arithmetic_error.into()),
            }
        }
    }

    impl<'de> serde::Deserialize<'de> for DispatchError {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let value = RichDispatchError::deserialize(deserializer)?;
            Ok(value.into())
        }
    }
}
