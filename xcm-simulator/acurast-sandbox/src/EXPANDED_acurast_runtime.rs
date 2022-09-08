mod mock_acurast {
    //! Parachain runtime mock.
    use codec::{Decode, Encode};
    use frame_support::{
        construct_runtime, parameter_types, traits::{Everything, Nothing},
        weights::{constants::WEIGHT_PER_SECOND, Weight},
    };
    use sp_core::H256;
    use sp_runtime::{
        testing::Header, traits::{Hash, IdentityLookup},
        AccountId32,
    };
    use sp_std::prelude::*;
    use pallet_xcm::XcmPassthrough;
    use polkadot_core_primitives::BlockNumber as RelayBlockNumber;
    use polkadot_parachain::primitives::{
        DmpMessageHandler, Id as ParaId, Sibling, XcmpMessageFormat, XcmpMessageHandler,
    };
    use xcm::{latest::prelude::*, VersionedXcm};
    use xcm_builder::{
        AccountId32Aliases, AllowUnpaidExecutionFrom,
        CurrencyAdapter as XcmCurrencyAdapter, EnsureXcmOrigin, FixedRateOfFungible,
        FixedWeightBounds, IsConcrete, LocationInverter, NativeAsset, ParentIsPreset,
        SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32,
        SovereignSignedViaLocation,
    };
    use xcm_executor::{Config, XcmExecutor};

    pub type AccountId = AccountId32;
    pub type Balance = u128;

    pub struct BlockHashCount;

    impl BlockHashCount {
        /// Returns the value of this parameter type.
        pub const fn get() -> u64 {
            250
        }
    }

    impl<I: From<u64>> ::frame_support::traits::Get<I> for BlockHashCount {
        fn get() -> I {
            I::from(Self::get())
        }
    }

    impl ::frame_support::traits::TypedGet for BlockHashCount {
        type Type = u64;
        fn get() -> u64 {
            Self::get()
        }
    }

    impl frame_system::Config for Runtime {
        type Origin = Origin;
        type Call = Call;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = ::sp_runtime::traits::BlakeTwo256;
        type AccountId = AccountId;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = Event;
        type BlockHashCount = BlockHashCount;
        type BlockWeights = ();
        type BlockLength = ();
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = pallet_balances::AccountData<Balance>;
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type DbWeight = ();
        type BaseCallFilter = Everything;
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = frame_support::traits::ConstU32<16>;
    }

    pub struct ExistentialDeposit;

    impl ExistentialDeposit {
        /// Returns the value of this parameter type.
        pub fn get() -> Balance {
            1
        }
    }

    impl<I: From<Balance>> ::frame_support::traits::Get<I> for ExistentialDeposit {
        fn get() -> I {
            I::from(Self::get())
        }
    }

    impl ::frame_support::traits::TypedGet for ExistentialDeposit {
        type Type = Balance;
        fn get() -> Balance {
            Self::get()
        }
    }

    pub struct MaxLocks;

    impl MaxLocks {
        /// Returns the value of this parameter type.
        pub const fn get() -> u32 {
            50
        }
    }

    impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxLocks {
        fn get() -> I {
            I::from(Self::get())
        }
    }

    impl ::frame_support::traits::TypedGet for MaxLocks {
        type Type = u32;
        fn get() -> u32 {
            Self::get()
        }
    }

    pub struct MaxReserves;

    impl MaxReserves {
        /// Returns the value of this parameter type.
        pub const fn get() -> u32 {
            50
        }
    }

    impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxReserves {
        fn get() -> I {
            I::from(Self::get())
        }
    }

    impl ::frame_support::traits::TypedGet for MaxReserves {
        type Type = u32;
        fn get() -> u32 {
            Self::get()
        }
    }

    impl pallet_balances::Config for Runtime {
        type MaxLocks = MaxLocks;
        type Balance = Balance;
        type Event = Event;
        type DustRemoval = ();
        type ExistentialDeposit = ExistentialDeposit;
        type AccountStore = System;
        type WeightInfo = ();
        type MaxReserves = MaxReserves;
        type ReserveIdentifier = [u8; 8];
    }

    pub struct ReservedXcmpWeight;

    impl ReservedXcmpWeight {
        /// Returns the value of this parameter type.
        pub const fn get() -> Weight {
            WEIGHT_PER_SECOND / 4
        }
    }

    impl<I: From<Weight>> ::frame_support::traits::Get<I> for ReservedXcmpWeight {
        fn get() -> I {
            I::from(Self::get())
        }
    }

    impl ::frame_support::traits::TypedGet for ReservedXcmpWeight {
        type Type = Weight;
        fn get() -> Weight {
            Self::get()
        }
    }

    pub struct ReservedDmpWeight;

    impl ReservedDmpWeight {
        /// Returns the value of this parameter type.
        pub const fn get() -> Weight {
            WEIGHT_PER_SECOND / 4
        }
    }

    impl<I: From<Weight>> ::frame_support::traits::Get<I> for ReservedDmpWeight {
        fn get() -> I {
            I::from(Self::get())
        }
    }

    impl ::frame_support::traits::TypedGet for ReservedDmpWeight {
        type Type = Weight;
        fn get() -> Weight {
            Self::get()
        }
    }

    pub struct KsmLocation;

    impl KsmLocation {
        /// Returns the value of this parameter type.
        pub const fn get() -> MultiLocation {
            MultiLocation::parent()
        }
    }

    impl<I: From<MultiLocation>> ::frame_support::traits::Get<I> for KsmLocation {
        fn get() -> I {
            I::from(Self::get())
        }
    }

    impl ::frame_support::traits::TypedGet for KsmLocation {
        type Type = MultiLocation;
        fn get() -> MultiLocation {
            Self::get()
        }
    }

    pub struct RelayNetwork;

    impl RelayNetwork {
        /// Returns the value of this parameter type.
        pub const fn get() -> NetworkId {
            NetworkId::Kusama
        }
    }

    impl<I: From<NetworkId>> ::frame_support::traits::Get<I> for RelayNetwork {
        fn get() -> I {
            I::from(Self::get())
        }
    }

    impl ::frame_support::traits::TypedGet for RelayNetwork {
        type Type = NetworkId;
        fn get() -> NetworkId {
            Self::get()
        }
    }

    pub struct Ancestry;

    impl Ancestry {
        /// Returns the value of this parameter type.
        pub fn get() -> MultiLocation {
            Parachain(MsgQueue::parachain_id().into()).into()
        }
    }

    impl<I: From<MultiLocation>> ::frame_support::traits::Get<I> for Ancestry {
        fn get() -> I {
            I::from(Self::get())
        }
    }

    impl ::frame_support::traits::TypedGet for Ancestry {
        type Type = MultiLocation;
        fn get() -> MultiLocation {
            Self::get()
        }
    }

    pub type LocationToAccountId = (
        ParentIsPreset<AccountId>,
        SiblingParachainConvertsVia<Sibling, AccountId>,
        AccountId32Aliases<RelayNetwork, AccountId>,
    );
    pub type XcmOriginToCallOrigin = (
        SovereignSignedViaLocation<LocationToAccountId, Origin>,
        SignedAccountId32AsNative<RelayNetwork, Origin>,
        XcmPassthrough<Origin>,
    );

    pub struct UnitWeightCost;

    impl UnitWeightCost {
        /// Returns the value of this parameter type.
        pub const fn get() -> Weight {
            1
        }
    }

    impl<I: From<Weight>> ::frame_support::traits::Get<I> for UnitWeightCost {
        fn get() -> I {
            I::from(Self::get())
        }
    }

    impl ::frame_support::traits::TypedGet for UnitWeightCost {
        type Type = Weight;
        fn get() -> Weight {
            Self::get()
        }
    }

    pub struct KsmPerSecond;

    impl KsmPerSecond {
        /// Returns the value of this parameter type.
        pub fn get() -> (AssetId, u128) {
            (Concrete(Parent.into()), 1)
        }
    }

    impl<I: From<(AssetId, u128)>> ::frame_support::traits::Get<I> for KsmPerSecond {
        fn get() -> I {
            I::from(Self::get())
        }
    }

    impl ::frame_support::traits::TypedGet for KsmPerSecond {
        type Type = (AssetId, u128);
        fn get() -> (AssetId, u128) {
            Self::get()
        }
    }

    pub struct MaxInstructions;

    impl MaxInstructions {
        /// Returns the value of this parameter type.
        pub const fn get() -> u32 {
            100
        }
    }

    impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxInstructions {
        fn get() -> I {
            I::from(Self::get())
        }
    }

    impl ::frame_support::traits::TypedGet for MaxInstructions {
        type Type = u32;
        fn get() -> u32 {
            Self::get()
        }
    }

    pub type LocalAssetTransactor = XcmCurrencyAdapter<
        Balances,
        IsConcrete<KsmLocation>,
        LocationToAccountId,
        AccountId,
        (),
    >;
    pub type XcmRouter = super::ParachainXcmRouter<MsgQueue>;
    pub type Barrier = AllowUnpaidExecutionFrom<Everything>;

    pub struct XcmConfig;

    impl Config for XcmConfig {
        type Call = Call;
        type XcmSender = XcmRouter;
        type AssetTransactor = LocalAssetTransactor;
        type OriginConverter = XcmOriginToCallOrigin;
        type IsReserve = NativeAsset;
        type IsTeleporter = ();
        type LocationInverter = LocationInverter<Ancestry>;
        type Barrier = Barrier;
        type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
        type Trader = FixedRateOfFungible<KsmPerSecond, ()>;
        type ResponseHandler = ();
        type AssetTrap = ();
        type AssetClaims = ();
        type SubscriptionService = ();
    }

    /**
    The module that hosts all the
    [FRAME](https://docs.substrate.io/v3/runtime/frame)
    types needed to add this pallet to a
    runtime.
     */
    pub mod mock_msg_queue {
        use super::*;
        use frame_support::pallet_prelude::*;

        /**
        Configuration trait of this pallet.

        Implement this type for a runtime in order to customize this pallet.
         */
        pub trait Config: frame_system::Config {
            type Event: From<Event<Self>>
            + IsType<<Self as frame_system::Config>::Event>;
            type XcmExecutor: ExecuteXcm<Self::Call>;
        }

        impl<T: Config> Pallet<T> {}

        /**
        The [pallet](https://docs.substrate.io/v3/runtime/frame#pallets) implementing
        the on-chain logic.
         */
        pub struct Pallet<T>(frame_support::sp_std::marker::PhantomData<(T)>);

        const _: () = {
            impl<T> core::clone::Clone for Pallet<T> {
                fn clone(&self) -> Self {
                    Self(core::clone::Clone::clone(&self.0))
                }
            }
        };
        const _: () = {
            impl<T> core::cmp::Eq for Pallet<T> {}
        };
        const _: () = {
            impl<T> core::cmp::PartialEq for Pallet<T> {
                fn eq(&self, other: &Self) -> bool {
                    true && self.0 == other.0
                }
            }
        };
        const _: () = {
            impl<T> core::fmt::Debug for Pallet<T> {
                fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                    fmt.debug_tuple("Pallet").field(&self.0).finish()
                }
            }
        };

        #[allow(type_alias_bounds)]
        pub(super) type ParachainId<T: Config> = StorageValue<
            _GeneratedPrefixForStorageParachainId<T>,
            ParaId,
            ValueQuery,
        >;
        /// A queue of received DMP messages
        #[allow(type_alias_bounds)]
        pub(super) type ReceivedDmp<T: Config> = StorageValue<
            _GeneratedPrefixForStorageReceivedDmp<T>,
            Vec<Xcm<T::Call>>,
            ValueQuery,
        >;

        impl<T: Config> Get<ParaId> for Pallet<T> {
            fn get() -> ParaId {
                Self::parachain_id()
            }
        }

        pub type MessageId = [u8; 32];

        /**
        The [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted
        by this pallet.
         */
        #[scale_info(skip_type_params(T), capture_docs = "always")]
        pub enum Event<T: Config> {
            /// Some XCM was executed OK.
            Success(Option<T::Hash>),
            /// Some XCM failed.
            Fail(Option<T::Hash>, XcmError),
            /// Bad XCM version used.
            BadVersion(Option<T::Hash>),
            /// Bad XCM format used.
            BadFormat(Option<T::Hash>),
            /// Downward message is invalid XCM.
            InvalidFormat(MessageId),
            /// Downward message is unsupported version of XCM.
            UnsupportedVersion(MessageId),
            /// Downward message executed with the given outcome.
            ExecutedDownward(MessageId, Outcome),
            #[doc(hidden)]
            #[codec(skip)]
            __Ignore(
                frame_support::sp_std::marker::PhantomData<(T)>,
                frame_support::Never,
            ),
        }

        const _: () = {
            impl<T: Config> core::clone::Clone for Event<T> {
                fn clone(&self) -> Self {
                    match self {
                        Self::Success(ref _0) => {
                            Self::Success(core::clone::Clone::clone(_0))
                        }
                        Self::Fail(ref _0, ref _1) => {
                            Self::Fail(
                                core::clone::Clone::clone(_0),
                                core::clone::Clone::clone(_1),
                            )
                        }
                        Self::BadVersion(ref _0) => {
                            Self::BadVersion(core::clone::Clone::clone(_0))
                        }
                        Self::BadFormat(ref _0) => {
                            Self::BadFormat(core::clone::Clone::clone(_0))
                        }
                        Self::InvalidFormat(ref _0) => {
                            Self::InvalidFormat(core::clone::Clone::clone(_0))
                        }
                        Self::UnsupportedVersion(ref _0) => {
                            Self::UnsupportedVersion(core::clone::Clone::clone(_0))
                        }
                        Self::ExecutedDownward(ref _0, ref _1) => {
                            Self::ExecutedDownward(
                                core::clone::Clone::clone(_0),
                                core::clone::Clone::clone(_1),
                            )
                        }
                        Self::__Ignore(ref _0, ref _1) => {
                            Self::__Ignore(
                                core::clone::Clone::clone(_0),
                                core::clone::Clone::clone(_1),
                            )
                        }
                    }
                }
            }
        };
        const _: () = {
            impl<T: Config> core::cmp::Eq for Event<T> {}
        };
        const _: () = {
            impl<T: Config> core::cmp::PartialEq for Event<T> {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        (Self::Success(_0), Self::Success(_0_other)) => {
                            true && _0 == _0_other
                        }
                        (Self::Fail(_0, _1), Self::Fail(_0_other, _1_other)) => {
                            true && _0 == _0_other && _1 == _1_other
                        }
                        (Self::BadVersion(_0), Self::BadVersion(_0_other)) => {
                            true && _0 == _0_other
                        }
                        (Self::BadFormat(_0), Self::BadFormat(_0_other)) => {
                            true && _0 == _0_other
                        }
                        (Self::InvalidFormat(_0), Self::InvalidFormat(_0_other)) => {
                            true && _0 == _0_other
                        }
                        (
                            Self::UnsupportedVersion(_0),
                            Self::UnsupportedVersion(_0_other),
                        ) => true && _0 == _0_other,
                        (
                            Self::ExecutedDownward(_0, _1),
                            Self::ExecutedDownward(_0_other, _1_other),
                        ) => true && _0 == _0_other && _1 == _1_other,
                        (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                            true && _0 == _0_other && _1 == _1_other
                        }
                        (Self::Success { .. }, Self::Fail { .. }) => false,
                        (Self::Success { .. }, Self::BadVersion { .. }) => false,
                        (Self::Success { .. }, Self::BadFormat { .. }) => false,
                        (Self::Success { .. }, Self::InvalidFormat { .. }) => false,
                        (Self::Success { .. }, Self::UnsupportedVersion { .. }) => false,
                        (Self::Success { .. }, Self::ExecutedDownward { .. }) => false,
                        (Self::Success { .. }, Self::__Ignore { .. }) => false,
                        (Self::Fail { .. }, Self::Success { .. }) => false,
                        (Self::Fail { .. }, Self::BadVersion { .. }) => false,
                        (Self::Fail { .. }, Self::BadFormat { .. }) => false,
                        (Self::Fail { .. }, Self::InvalidFormat { .. }) => false,
                        (Self::Fail { .. }, Self::UnsupportedVersion { .. }) => false,
                        (Self::Fail { .. }, Self::ExecutedDownward { .. }) => false,
                        (Self::Fail { .. }, Self::__Ignore { .. }) => false,
                        (Self::BadVersion { .. }, Self::Success { .. }) => false,
                        (Self::BadVersion { .. }, Self::Fail { .. }) => false,
                        (Self::BadVersion { .. }, Self::BadFormat { .. }) => false,
                        (Self::BadVersion { .. }, Self::InvalidFormat { .. }) => false,
                        (Self::BadVersion { .. }, Self::UnsupportedVersion { .. }) => {
                            false
                        }
                        (Self::BadVersion { .. }, Self::ExecutedDownward { .. }) => false,
                        (Self::BadVersion { .. }, Self::__Ignore { .. }) => false,
                        (Self::BadFormat { .. }, Self::Success { .. }) => false,
                        (Self::BadFormat { .. }, Self::Fail { .. }) => false,
                        (Self::BadFormat { .. }, Self::BadVersion { .. }) => false,
                        (Self::BadFormat { .. }, Self::InvalidFormat { .. }) => false,
                        (Self::BadFormat { .. }, Self::UnsupportedVersion { .. }) => {
                            false
                        }
                        (Self::BadFormat { .. }, Self::ExecutedDownward { .. }) => false,
                        (Self::BadFormat { .. }, Self::__Ignore { .. }) => false,
                        (Self::InvalidFormat { .. }, Self::Success { .. }) => false,
                        (Self::InvalidFormat { .. }, Self::Fail { .. }) => false,
                        (Self::InvalidFormat { .. }, Self::BadVersion { .. }) => false,
                        (Self::InvalidFormat { .. }, Self::BadFormat { .. }) => false,
                        (Self::InvalidFormat { .. }, Self::UnsupportedVersion { .. }) => {
                            false
                        }
                        (Self::InvalidFormat { .. }, Self::ExecutedDownward { .. }) => {
                            false
                        }
                        (Self::InvalidFormat { .. }, Self::__Ignore { .. }) => false,
                        (Self::UnsupportedVersion { .. }, Self::Success { .. }) => false,
                        (Self::UnsupportedVersion { .. }, Self::Fail { .. }) => false,
                        (Self::UnsupportedVersion { .. }, Self::BadVersion { .. }) => {
                            false
                        }
                        (Self::UnsupportedVersion { .. }, Self::BadFormat { .. }) => {
                            false
                        }
                        (Self::UnsupportedVersion { .. }, Self::InvalidFormat { .. }) => {
                            false
                        }
                        (
                            Self::UnsupportedVersion { .. },
                            Self::ExecutedDownward { .. },
                        ) => false,
                        (Self::UnsupportedVersion { .. }, Self::__Ignore { .. }) => false,
                        (Self::ExecutedDownward { .. }, Self::Success { .. }) => false,
                        (Self::ExecutedDownward { .. }, Self::Fail { .. }) => false,
                        (Self::ExecutedDownward { .. }, Self::BadVersion { .. }) => false,
                        (Self::ExecutedDownward { .. }, Self::BadFormat { .. }) => false,
                        (Self::ExecutedDownward { .. }, Self::InvalidFormat { .. }) => {
                            false
                        }
                        (
                            Self::ExecutedDownward { .. },
                            Self::UnsupportedVersion { .. },
                        ) => false,
                        (Self::ExecutedDownward { .. }, Self::__Ignore { .. }) => false,
                        (Self::__Ignore { .. }, Self::Success { .. }) => false,
                        (Self::__Ignore { .. }, Self::Fail { .. }) => false,
                        (Self::__Ignore { .. }, Self::BadVersion { .. }) => false,
                        (Self::__Ignore { .. }, Self::BadFormat { .. }) => false,
                        (Self::__Ignore { .. }, Self::InvalidFormat { .. }) => false,
                        (Self::__Ignore { .. }, Self::UnsupportedVersion { .. }) => false,
                        (Self::__Ignore { .. }, Self::ExecutedDownward { .. }) => false,
                    }
                }
            }
        };
        const _: () = {
            impl<T: Config> core::fmt::Debug for Event<T> {
                fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                    match *self {
                        Self::Success(ref _0) => {
                            fmt.debug_tuple("Event::Success").field(&_0).finish()
                        }
                        Self::Fail(ref _0, ref _1) => {
                            fmt.debug_tuple("Event::Fail").field(&_0).field(&_1).finish()
                        }
                        Self::BadVersion(ref _0) => {
                            fmt.debug_tuple("Event::BadVersion").field(&_0).finish()
                        }
                        Self::BadFormat(ref _0) => {
                            fmt.debug_tuple("Event::BadFormat").field(&_0).finish()
                        }
                        Self::InvalidFormat(ref _0) => {
                            fmt.debug_tuple("Event::InvalidFormat").field(&_0).finish()
                        }
                        Self::UnsupportedVersion(ref _0) => {
                            fmt
                                .debug_tuple("Event::UnsupportedVersion")
                                .field(&_0)
                                .finish()
                        }
                        Self::ExecutedDownward(ref _0, ref _1) => {
                            fmt
                                .debug_tuple("Event::ExecutedDownward")
                                .field(&_0)
                                .field(&_1)
                                .finish()
                        }
                        Self::__Ignore(ref _0, ref _1) => {
                            fmt
                                .debug_tuple("Event::__Ignore")
                                .field(&_0)
                                .field(&_1)
                                .finish()
                        }
                    }
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl<T: Config> ::codec::Encode for Event<T>
                where
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
            {
                fn encode_to<
                    __CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    match *self {
                        Event::Success(ref aa) => {
                            __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        }
                        Event::Fail(ref aa, ref ba) => {
                            __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                            ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                        }
                        Event::BadVersion(ref aa) => {
                            __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        }
                        Event::BadFormat(ref aa) => {
                            __codec_dest_edqy.push_byte(3usize as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        }
                        Event::InvalidFormat(ref aa) => {
                            __codec_dest_edqy.push_byte(4usize as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        }
                        Event::UnsupportedVersion(ref aa) => {
                            __codec_dest_edqy.push_byte(5usize as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        }
                        Event::ExecutedDownward(ref aa, ref ba) => {
                            __codec_dest_edqy.push_byte(6usize as ::core::primitive::u8);
                            ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                            ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                        }
                        _ => {}
                    }
                }
            }
            #[automatically_derived]
            impl<T: Config> ::codec::EncodeLike for Event<T>
                where
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
                    Option<T::Hash>: ::codec::Encode,
            {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl<T: Config> ::codec::Decode for Event<T>
                where
                    Option<T::Hash>: ::codec::Decode,
                    Option<T::Hash>: ::codec::Decode,
                    Option<T::Hash>: ::codec::Decode,
                    Option<T::Hash>: ::codec::Decode,
                    Option<T::Hash>: ::codec::Decode,
                    Option<T::Hash>: ::codec::Decode,
                    Option<T::Hash>: ::codec::Decode,
                    Option<T::Hash>: ::codec::Decode,
            {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    match __codec_input_edqy
                        .read_byte()
                        .map_err(|e| {
                            e
                                .chain(
                                    "Could not decode `Event`, failed to read variant byte",
                                )
                        })?
                    {
                        __codec_x_edqy if __codec_x_edqy
                            == 0usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(
                                Event::<
                                    T,
                                >::Success({
                                    let __codec_res_edqy = <Option<
                                        T::Hash,
                                    > as ::codec::Decode>::decode(__codec_input_edqy);
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Event::Success.0`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                }),
                            )
                        }
                        __codec_x_edqy if __codec_x_edqy
                            == 1usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(
                                Event::<
                                    T,
                                >::Fail(
                                    {
                                        let __codec_res_edqy = <Option<
                                            T::Hash,
                                        > as ::codec::Decode>::decode(__codec_input_edqy);
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `Event::Fail.0`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    },
                                    {
                                        let __codec_res_edqy = <XcmError as ::codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `Event::Fail.1`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    },
                                ),
                            )
                        }
                        __codec_x_edqy if __codec_x_edqy
                            == 2usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(
                                Event::<
                                    T,
                                >::BadVersion({
                                    let __codec_res_edqy = <Option<
                                        T::Hash,
                                    > as ::codec::Decode>::decode(__codec_input_edqy);
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Event::BadVersion.0`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                }),
                            )
                        }
                        __codec_x_edqy if __codec_x_edqy
                            == 3usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(
                                Event::<
                                    T,
                                >::BadFormat({
                                    let __codec_res_edqy = <Option<
                                        T::Hash,
                                    > as ::codec::Decode>::decode(__codec_input_edqy);
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Event::BadFormat.0`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                }),
                            )
                        }
                        __codec_x_edqy if __codec_x_edqy
                            == 4usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(
                                Event::<
                                    T,
                                >::InvalidFormat({
                                    let __codec_res_edqy = <MessageId as ::codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Event::InvalidFormat.0`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                }),
                            )
                        }
                        __codec_x_edqy if __codec_x_edqy
                            == 5usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(
                                Event::<
                                    T,
                                >::UnsupportedVersion({
                                    let __codec_res_edqy = <MessageId as ::codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Event::UnsupportedVersion.0`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                }),
                            )
                        }
                        __codec_x_edqy if __codec_x_edqy
                            == 6usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(
                                Event::<
                                    T,
                                >::ExecutedDownward(
                                    {
                                        let __codec_res_edqy = <MessageId as ::codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `Event::ExecutedDownward.0`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    },
                                    {
                                        let __codec_res_edqy = <Outcome as ::codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                        match __codec_res_edqy {
                                            ::core::result::Result::Err(e) => {
                                                return ::core::result::Result::Err(
                                                    e.chain("Could not decode `Event::ExecutedDownward.1`"),
                                                );
                                            }
                                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                                __codec_res_edqy
                                            }
                                        }
                                    },
                                ),
                            )
                        }
                        _ => {
                            ::core::result::Result::Err(
                                <_ as ::core::convert::Into<
                                    _,
                                >>::into("Could not decode `Event`, variant doesn't exist"),
                            )
                        }
                    }
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl<T: Config> ::scale_info::TypeInfo for Event<T>
                where
                    Option<T::Hash>: ::scale_info::TypeInfo + 'static,
                    Option<T::Hash>: ::scale_info::TypeInfo + 'static,
                    Option<T::Hash>: ::scale_info::TypeInfo + 'static,
                    Option<T::Hash>: ::scale_info::TypeInfo + 'static,
                    frame_support::sp_std::marker::PhantomData<
                        (T),
                    >: ::scale_info::TypeInfo + 'static,
                    T: Config + 'static,
            {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "Event",
                                "acurast_sandbox::mock_acurast::mock_msg_queue",
                            ),
                        )
                        .type_params(
                            <[_]>::into_vec(
                                #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                    ::scale_info::TypeParameter::new(
                                        "T",
                                        ::core::option::Option::None,
                                    ),
                                ]),
                            ),
                        )
                        .docs_always(
                            &[
                                "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t",
                            ],
                        )
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant(
                                    "Success",
                                    |v| {
                                        v
                                            .index(0usize as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| {
                                                        f
                                                            .ty::<Option<T::Hash>>()
                                                            .type_name("Option<T::Hash>")
                                                            .docs_always(&[])
                                                    }),
                                            )
                                            .docs_always(&["Some XCM was executed OK."])
                                    },
                                )
                                .variant(
                                    "Fail",
                                    |v| {
                                        v
                                            .index(1usize as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| {
                                                        f
                                                            .ty::<Option<T::Hash>>()
                                                            .type_name("Option<T::Hash>")
                                                            .docs_always(&[])
                                                    })
                                                    .field(|f| {
                                                        f.ty::<XcmError>().type_name("XcmError").docs_always(&[])
                                                    }),
                                            )
                                            .docs_always(&["Some XCM failed."])
                                    },
                                )
                                .variant(
                                    "BadVersion",
                                    |v| {
                                        v
                                            .index(2usize as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| {
                                                        f
                                                            .ty::<Option<T::Hash>>()
                                                            .type_name("Option<T::Hash>")
                                                            .docs_always(&[])
                                                    }),
                                            )
                                            .docs_always(&["Bad XCM version used."])
                                    },
                                )
                                .variant(
                                    "BadFormat",
                                    |v| {
                                        v
                                            .index(3usize as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| {
                                                        f
                                                            .ty::<Option<T::Hash>>()
                                                            .type_name("Option<T::Hash>")
                                                            .docs_always(&[])
                                                    }),
                                            )
                                            .docs_always(&["Bad XCM format used."])
                                    },
                                )
                                .variant(
                                    "InvalidFormat",
                                    |v| {
                                        v
                                            .index(4usize as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| {
                                                        f.ty::<MessageId>().type_name("MessageId").docs_always(&[])
                                                    }),
                                            )
                                            .docs_always(&["Downward message is invalid XCM."])
                                    },
                                )
                                .variant(
                                    "UnsupportedVersion",
                                    |v| {
                                        v
                                            .index(5usize as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| {
                                                        f.ty::<MessageId>().type_name("MessageId").docs_always(&[])
                                                    }),
                                            )
                                            .docs_always(
                                                &["Downward message is unsupported version of XCM."],
                                            )
                                    },
                                )
                                .variant(
                                    "ExecutedDownward",
                                    |v| {
                                        v
                                            .index(6usize as ::core::primitive::u8)
                                            .fields(
                                                ::scale_info::build::Fields::unnamed()
                                                    .field(|f| {
                                                        f.ty::<MessageId>().type_name("MessageId").docs_always(&[])
                                                    })
                                                    .field(|f| {
                                                        f.ty::<Outcome>().type_name("Outcome").docs_always(&[])
                                                    }),
                                            )
                                            .docs_always(
                                                &["Downward message executed with the given outcome."],
                                            )
                                    },
                                ),
                        )
                }
            }
        };

        impl<T: Config> Pallet<T> {
            pub fn set_para_id(para_id: ParaId) {
                ParachainId::<T>::put(para_id);
            }
            fn handle_xcmp_message(
                sender: ParaId,
                _sent_at: RelayBlockNumber,
                xcm: VersionedXcm<T::Call>,
                max_weight: Weight,
            ) -> Result<Weight, XcmError> {
                let hash = Encode::using_encoded(&xcm, T::Hashing::hash);
                let (result, event) = match Xcm::<T::Call>::try_from(xcm) {
                    Ok(xcm) => {
                        let location = (1, Parachain(sender.into()));
                        match T::XcmExecutor::execute_xcm(location, xcm, max_weight) {
                            Outcome::Error(e) => {
                                (Err(e.clone()), Event::Fail(Some(hash), e))
                            }
                            Outcome::Complete(w) => (Ok(w), Event::Success(Some(hash))),
                            Outcome::Incomplete(w, e) => {
                                (Ok(w), Event::Fail(Some(hash), e))
                            }
                        }
                    }
                    Err(()) => {
                        (
                            Err(XcmError::UnhandledXcmVersion),
                            Event::BadVersion(Some(hash)),
                        )
                    }
                };
                Self::deposit_event(event);
                result
            }
        }

        impl<T: Config> XcmpMessageHandler for Pallet<T> {
            fn handle_xcmp_messages<
                'a,
                I: Iterator<Item=(ParaId, RelayBlockNumber, &'a [u8])>,
            >(iter: I, max_weight: Weight) -> Weight {
                for (sender, sent_at, data) in iter {
                    let mut data_ref = data;
                    let _ = XcmpMessageFormat::decode(&mut data_ref)
                        .expect("Simulator encodes with versioned xcm format; qed");
                    let mut remaining_fragments = &data_ref[..];
                    while !remaining_fragments.is_empty() {
                        if let Ok(xcm)
                        = VersionedXcm::<T::Call>::decode(&mut remaining_fragments) {
                            let _ = Self::handle_xcmp_message(
                                sender,
                                sent_at,
                                xcm,
                                max_weight,
                            );
                        } else {
                            if true {
                                if !false {
                                    ::core::panicking::panic_fmt(
                                        ::core::fmt::Arguments::new_v1(
                                            &["Invalid incoming XCMP message data"],
                                            &[],
                                        ),
                                    )
                                }
                            }
                        }
                    }
                }
                max_weight
            }
        }

        impl<T: Config> DmpMessageHandler for Pallet<T> {
            fn handle_dmp_messages(
                iter: impl Iterator<Item=(RelayBlockNumber, Vec<u8>)>,
                limit: Weight,
            ) -> Weight {
                for (_i, (_sent_at, data)) in iter.enumerate() {
                    let id = sp_io::hashing::blake2_256(&data[..]);
                    let maybe_msg = VersionedXcm::<T::Call>::decode(&mut &data[..])
                        .map(Xcm::<T::Call>::try_from);
                    match maybe_msg {
                        Err(_) => {
                            Self::deposit_event(Event::InvalidFormat(id));
                        }
                        Ok(Err(())) => {
                            Self::deposit_event(Event::UnsupportedVersion(id));
                        }
                        Ok(Ok(x)) => {
                            let outcome = T::XcmExecutor::execute_xcm(
                                Parent,
                                x.clone(),
                                limit,
                            );
                            <ReceivedDmp<T>>::append(x);
                            Self::deposit_event(Event::ExecutedDownward(id, outcome));
                        }
                    }
                }
                limit
            }
        }

        impl<T: Config> Pallet<T> {
            #[doc(hidden)]
            pub fn pallet_constants_metadata() -> frame_support::sp_std::vec::Vec<
                frame_support::metadata::PalletConstantMetadata,
            > {
                ::alloc::vec::Vec::new()
            }
        }

        impl<T: Config> Pallet<T> {
            #[doc(hidden)]
            pub fn error_metadata() -> Option<
                frame_support::metadata::PalletErrorMetadata,
            > {
                None
            }
        }

        /// Type alias to `Pallet`, to be used by `construct_runtime`.
        ///
        /// Generated by `pallet` attribute macro.
        #[deprecated(note = "use `Pallet` instead")]
        #[allow(dead_code)]
        pub type Module<T> = Pallet<T>;

        impl<T: Config> frame_support::traits::GetStorageVersion for Pallet<T> {
            fn current_storage_version() -> frame_support::traits::StorageVersion {
                frame_support::traits::StorageVersion::default()
            }
            fn on_chain_storage_version() -> frame_support::traits::StorageVersion {
                frame_support::traits::StorageVersion::get::<Self>()
            }
        }

        impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
            fn on_genesis() {
                let storage_version = frame_support::traits::StorageVersion::default();
                storage_version.put::<Self>();
            }
        }

        impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
            fn index() -> usize {
                <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                    Self,
                >()
                    .expect(
                        "Pallet is part of the runtime because pallet `Config` trait is \
                        implemented by the runtime",
                    )
            }
            fn name() -> &'static str {
                <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                    Self,
                >()
                    .expect(
                        "Pallet is part of the runtime because pallet `Config` trait is \
                        implemented by the runtime",
                    )
            }
            fn module_name() -> &'static str {
                <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::module_name::<
                    Self,
                >()
                    .expect(
                        "Pallet is part of the runtime because pallet `Config` trait is \
                        implemented by the runtime",
                    )
            }
            fn crate_version() -> frame_support::traits::CrateVersion {
                frame_support::traits::CrateVersion {
                    major: 0u16,
                    minor: 9u8,
                    patch: 25u8,
                }
            }
        }

        impl<T: Config> frame_support::traits::PalletsInfoAccess for Pallet<T> {
            fn count() -> usize {
                1
            }
            fn accumulate(
                acc: &mut frame_support::sp_std::vec::Vec<
                    frame_support::traits::PalletInfoData,
                >,
            ) {
                use frame_support::traits::PalletInfoAccess;
                let item = frame_support::traits::PalletInfoData {
                    index: Self::index(),
                    name: Self::name(),
                    module_name: Self::module_name(),
                    crate_version: Self::crate_version(),
                };
                acc.push(item);
            }
        }

        impl<T: Config> frame_support::traits::StorageInfoTrait for Pallet<T> {
            fn storage_info() -> frame_support::sp_std::vec::Vec<
                frame_support::traits::StorageInfo,
            > {
                #[allow(unused_mut)]
                    let mut res = ::alloc::vec::Vec::new();
                {
                    let mut storage_info = <ParachainId<
                        T,
                    > as frame_support::traits::PartialStorageInfoTrait>::partial_storage_info();
                    res.append(&mut storage_info);
                }
                {
                    let mut storage_info = <ReceivedDmp<
                        T,
                    > as frame_support::traits::PartialStorageInfoTrait>::partial_storage_info();
                    res.append(&mut storage_info);
                }
                res
            }
        }

        #[doc(hidden)]
        pub mod __substrate_call_check {
            #[doc(hidden)]
            pub use __is_call_part_defined_0 as is_call_part_defined;
        }

        ///Contains one variant per dispatchable that can be called by an extrinsic.
        #[codec(encode_bound())]
        #[codec(decode_bound())]
        #[scale_info(skip_type_params(T), capture_docs = "always")]
        #[allow(non_camel_case_types)]
        pub enum Call<T: Config> {
            #[doc(hidden)]
            #[codec(skip)]
            __Ignore(
                frame_support::sp_std::marker::PhantomData<(T, )>,
                frame_support::Never,
            ),
        }

        const _: () = {
            impl<T: Config> core::fmt::Debug for Call<T> {
                fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                    match *self {
                        Self::__Ignore(ref _0, ref _1) => {
                            fmt
                                .debug_tuple("Call::__Ignore")
                                .field(&_0)
                                .field(&_1)
                                .finish()
                        }
                    }
                }
            }
        };
        const _: () = {
            impl<T: Config> core::clone::Clone for Call<T> {
                fn clone(&self) -> Self {
                    match self {
                        Self::__Ignore(ref _0, ref _1) => {
                            Self::__Ignore(
                                core::clone::Clone::clone(_0),
                                core::clone::Clone::clone(_1),
                            )
                        }
                    }
                }
            }
        };
        const _: () = {
            impl<T: Config> core::cmp::Eq for Call<T> {}
        };
        const _: () = {
            impl<T: Config> core::cmp::PartialEq for Call<T> {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                            true && _0 == _0_other && _1 == _1_other
                        }
                    }
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<T: Config> ::codec::Encode for Call<T> {}
            #[automatically_derived]
            impl<T: Config> ::codec::EncodeLike for Call<T> {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<T: Config> ::codec::Decode for Call<T> {
                fn decode<__CodecInputEdqy: ::codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::codec::Error> {
                    match __codec_input_edqy
                        .read_byte()
                        .map_err(|e| {
                            e
                                .chain(
                                    "Could not decode `Call`, failed to read variant byte",
                                )
                        })?
                    {
                        _ => {
                            ::core::result::Result::Err(
                                <_ as ::core::convert::Into<
                                    _,
                                >>::into("Could not decode `Call`, variant doesn't exist"),
                            )
                        }
                    }
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl<T: Config> ::scale_info::TypeInfo for Call<T>
                where
                    frame_support::sp_std::marker::PhantomData<
                        (T, ),
                    >: ::scale_info::TypeInfo + 'static,
                    T: Config + 'static,
            {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "Call",
                                "acurast_sandbox::mock_acurast::mock_msg_queue",
                            ),
                        )
                        .type_params(
                            <[_]>::into_vec(
                                #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                    ::scale_info::TypeParameter::new(
                                        "T",
                                        ::core::option::Option::None,
                                    ),
                                ]),
                            ),
                        )
                        .docs_always(
                            &[
                                "Contains one variant per dispatchable that can be called by an extrinsic.",
                            ],
                        )
                        .variant(::scale_info::build::Variants::new())
                }
            }
        };

        impl<T: Config> Call<T> {}

        impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
            fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
                match *self {
                    Self::__Ignore(_, _) => {
                        ::core::panicking::panic_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["internal error: entered unreachable code: "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(
                                        &::core::fmt::Arguments::new_v1(
                                            &["__Ignore cannot be used"],
                                            &[],
                                        ),
                                    ),
                                ],
                            ),
                        )
                    }
                }
            }
        }

        impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
            fn get_call_name(&self) -> &'static str {
                match *self {
                    Self::__Ignore(_, _) => {
                        ::core::panicking::panic_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["internal error: entered unreachable code: "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(
                                        &::core::fmt::Arguments::new_v1(
                                            &["__PhantomItem cannot be used."],
                                            &[],
                                        ),
                                    ),
                                ],
                            ),
                        )
                    }
                }
            }
            fn get_call_names() -> &'static [&'static str] {
                &[]
            }
        }

        impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
            type Origin = frame_system::pallet_prelude::OriginFor<T>;
            fn dispatch_bypass_filter(
                self,
                origin: Self::Origin,
            ) -> frame_support::dispatch::DispatchResultWithPostInfo {
                match self {
                    Self::__Ignore(_, _) => {
                        let _ = origin;
                        ::core::panicking::panic_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["internal error: entered unreachable code: "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(
                                        &::core::fmt::Arguments::new_v1(
                                            &["__PhantomItem cannot be used."],
                                            &[],
                                        ),
                                    ),
                                ],
                            ),
                        );
                    }
                }
            }
        }

        impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
            type Call = Call<T>;
        }

        impl<T: Config> Pallet<T> {
            #[doc(hidden)]
            pub fn call_functions() -> frame_support::metadata::PalletCallMetadata {
                frame_support::scale_info::meta_type::<Call<T>>().into()
            }
        }

        pub use __tt_error_token_1 as tt_error_token;

        #[doc(hidden)]
        pub mod __substrate_event_check {
            #[doc(hidden)]
            pub use __is_event_part_defined_2 as is_event_part_defined;
        }

        impl<T: Config> Pallet<T> {
            pub(super) fn deposit_event(event: Event<T>) {
                let event = <<T as Config>::Event as From<Event<T>>>::from(event);
                let event = <<T as Config>::Event as Into<
                    <T as frame_system::Config>::Event,
                >>::into(event);
                <frame_system::Pallet<T>>::deposit_event(event)
            }
        }

        impl<T: Config> From<Event<T>> for () {
            fn from(_: Event<T>) {}
        }

        impl<T: Config> Pallet<T> {
            #[doc(hidden)]
            pub fn storage_metadata() -> frame_support::metadata::PalletStorageMetadata {
                frame_support::metadata::PalletStorageMetadata {
                    prefix: <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                        Pallet<T>,
                    >()
                        .expect("Every active pallet has a name in the runtime; qed"),
                    entries: {
                        #[allow(unused_mut)]
                            let mut entries = ::alloc::vec::Vec::new();
                        {
                            <ParachainId<
                                T,
                            > as frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(
                                ::alloc::vec::Vec::new(),
                                &mut entries,
                            );
                        }
                        {
                            <ReceivedDmp<
                                T,
                            > as frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(
                                <[_]>::into_vec(
                                    #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                        " A queue of received DMP messages",
                                    ]),
                                ),
                                &mut entries,
                            );
                        }
                        entries
                    },
                }
            }
        }

        impl<T: Config> Pallet<T> {
            pub fn parachain_id() -> ParaId {
                <ParachainId<T> as frame_support::storage::StorageValue<ParaId>>::get()
            }
        }

        impl<T: Config> Pallet<T> {
            /// A queue of received DMP messages
            pub fn received_dmp() -> Vec<Xcm<T::Call>> {
                <ReceivedDmp<
                    T,
                > as frame_support::storage::StorageValue<Vec<Xcm<T::Call>>>>::get()
            }
        }

        #[doc(hidden)]
        pub(super) struct _GeneratedPrefixForStorageParachainId<T>(
            core::marker::PhantomData<(T, )>,
        );

        impl<T: Config> frame_support::traits::StorageInstance
        for _GeneratedPrefixForStorageParachainId<T> {
            fn pallet_prefix() -> &'static str {
                <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                    Pallet<T>,
                >()
                    .expect("Every active pallet has a name in the runtime; qed")
            }
            const STORAGE_PREFIX: &'static str = "ParachainId";
        }

        #[doc(hidden)]
        pub(super) struct _GeneratedPrefixForStorageReceivedDmp<T>(
            core::marker::PhantomData<(T, )>,
        );

        impl<T: Config> frame_support::traits::StorageInstance
        for _GeneratedPrefixForStorageReceivedDmp<T> {
            fn pallet_prefix() -> &'static str {
                <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                    Pallet<T>,
                >()
                    .expect("Every active pallet has a name in the runtime; qed")
            }
            const STORAGE_PREFIX: &'static str = "ReceivedDmp";
        }

        #[doc(hidden)]
        pub mod __substrate_inherent_check {
            #[doc(hidden)]
            pub use __is_inherent_part_defined_3 as is_inherent_part_defined;
        }

        /// Hidden instance generated to be internally used when module is used without
        /// instance.
        #[doc(hidden)]
        pub type __InherentHiddenInstance = ();

        pub(super) trait Store {
            type ParachainId;
            type ReceivedDmp;
        }

        impl<T: Config> Store for Pallet<T> {
            type ParachainId = ParachainId<T>;
            type ReceivedDmp = ReceivedDmp<T>;
        }

        impl<
            T: Config,
        > frame_support::traits::Hooks<<T as frame_system::Config>::BlockNumber>
        for Pallet<T> {}

        impl<
            T: Config,
        > frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T> {
            fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
                let __within_span__ = {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "on_finalize",
                                "acurast_sandbox::mock_acurast::mock_msg_queue",
                                ::tracing::Level::TRACE,
                                Some("xcm-simulator/acurast-sandbox/src/mock_acurast.rs"),
                                Some(152u32),
                                Some("acurast_sandbox::mock_acurast::mock_msg_queue"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        MacroCallsite::new(&META)
                    };
                    let mut interest = ::tracing::subscriber::Interest::never();
                    if ::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                        && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    } && CALLSITE.is_enabled(interest)
                    {
                        let meta = CALLSITE.metadata();
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        let span = CALLSITE.disabled_span();
                        {};
                        span
                    }
                };
                let __tracing_guard__ = __within_span__.enter();
                <Self as frame_support::traits::Hooks<
                    <T as frame_system::Config>::BlockNumber,
                >>::on_finalize(n)
            }
        }

        impl<
            T: Config,
        > frame_support::traits::OnIdle<<T as frame_system::Config>::BlockNumber>
        for Pallet<T> {
            fn on_idle(
                n: <T as frame_system::Config>::BlockNumber,
                remaining_weight: frame_support::weights::Weight,
            ) -> frame_support::weights::Weight {
                <Self as frame_support::traits::Hooks<
                    <T as frame_system::Config>::BlockNumber,
                >>::on_idle(n, remaining_weight)
            }
        }

        impl<
            T: Config,
        > frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T> {
            fn on_initialize(
                n: <T as frame_system::Config>::BlockNumber,
            ) -> frame_support::weights::Weight {
                let __within_span__ = {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "on_initialize",
                                "acurast_sandbox::mock_acurast::mock_msg_queue",
                                ::tracing::Level::TRACE,
                                Some("xcm-simulator/acurast-sandbox/src/mock_acurast.rs"),
                                Some(152u32),
                                Some("acurast_sandbox::mock_acurast::mock_msg_queue"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        MacroCallsite::new(&META)
                    };
                    let mut interest = ::tracing::subscriber::Interest::never();
                    if ::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                        && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    } && CALLSITE.is_enabled(interest)
                    {
                        let meta = CALLSITE.metadata();
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        let span = CALLSITE.disabled_span();
                        {};
                        span
                    }
                };
                let __tracing_guard__ = __within_span__.enter();
                <Self as frame_support::traits::Hooks<
                    <T as frame_system::Config>::BlockNumber,
                >>::on_initialize(n)
            }
        }

        impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
            fn on_runtime_upgrade() -> frame_support::weights::Weight {
                let __within_span__ = {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "on_runtime_update",
                                "acurast_sandbox::mock_acurast::mock_msg_queue",
                                ::tracing::Level::TRACE,
                                Some("xcm-simulator/acurast-sandbox/src/mock_acurast.rs"),
                                Some(152u32),
                                Some("acurast_sandbox::mock_acurast::mock_msg_queue"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        MacroCallsite::new(&META)
                    };
                    let mut interest = ::tracing::subscriber::Interest::never();
                    if ::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                        && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    } && CALLSITE.is_enabled(interest)
                    {
                        let meta = CALLSITE.metadata();
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        let span = CALLSITE.disabled_span();
                        {};
                        span
                    }
                };
                let __tracing_guard__ = __within_span__.enter();
                let pallet_name = <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                    Self,
                >()
                    .unwrap_or("<unknown pallet name>");
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["\u{2705} no migration for "],
                                &[::core::fmt::ArgumentV1::new_display(&pallet_name)],
                            ),
                            lvl,
                            &(
                                frame_support::LOG_TARGET,
                                "acurast_sandbox::mock_acurast::mock_msg_queue",
                                "xcm-simulator/acurast-sandbox/src/mock_acurast.rs",
                                152u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                <Self as frame_support::traits::Hooks<
                    <T as frame_system::Config>::BlockNumber,
                >>::on_runtime_upgrade()
            }
        }

        impl<
            T: Config,
        > frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
        for Pallet<T> {
            fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
                <Self as frame_support::traits::Hooks<
                    <T as frame_system::Config>::BlockNumber,
                >>::offchain_worker(n)
            }
        }

        impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
            fn integrity_test() {
                <Self as frame_support::traits::Hooks<
                    <T as frame_system::Config>::BlockNumber,
                >>::integrity_test()
            }
        }

        #[doc(hidden)]
        pub mod __substrate_genesis_config_check {
            #[doc(hidden)]
            pub use __is_genesis_config_defined_4 as is_genesis_config_defined;
            #[doc(hidden)]
            pub use __is_std_enabled_for_genesis_4 as is_std_enabled_for_genesis;
        }

        #[doc(hidden)]
        pub mod __substrate_origin_check {
            #[doc(hidden)]
            pub use __is_origin_part_defined_5 as is_origin_part_defined;
        }

        #[doc(hidden)]
        pub mod __substrate_validate_unsigned_check {
            #[doc(hidden)]
            pub use __is_validate_unsigned_part_defined_6 as is_validate_unsigned_part_defined;
        }

        pub use __tt_default_parts_7 as tt_default_parts;
    }

    impl mock_msg_queue::Config for Runtime {
        type Event = Event;
        type XcmExecutor = XcmExecutor<XcmConfig>;
    }

    pub type LocalOriginToLocation = SignedToAccountId32<
        Origin,
        AccountId,
        RelayNetwork,
    >;

    impl pallet_xcm::Config for Runtime {
        type Event = Event;
        type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
        type XcmRouter = XcmRouter;
        type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
        type XcmExecuteFilter = Everything;
        type XcmExecutor = XcmExecutor<XcmConfig>;
        type XcmTeleportFilter = Nothing;
        type XcmReserveTransferFilter = Everything;
        type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
        type LocationInverter = LocationInverter<Ancestry>;
        type Origin = Origin;
        type Call = Call;
        const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
        type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    }

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
    type Block = frame_system::mocking::MockBlock<Runtime>;

    #[doc(hidden)]
    mod sp_api_hidden_includes_construct_runtime {
        pub extern crate frame_support as hidden_include;
    }

    const _: () = {
        #[allow(unused)]
        type __hidden_use_of_unchecked_extrinsic = UncheckedExtrinsic;
    };

    pub struct Runtime;

    #[automatically_derived]
    impl ::core::clone::Clone for Runtime {
        #[inline]
        fn clone(&self) -> Runtime {
            *self
        }
    }

    #[automatically_derived]
    impl ::core::marker::Copy for Runtime {}

    impl ::core::marker::StructuralPartialEq for Runtime {}

    #[automatically_derived]
    impl ::core::cmp::PartialEq for Runtime {
        #[inline]
        fn eq(&self, other: &Runtime) -> bool {
            true
        }
    }

    impl ::core::marker::StructuralEq for Runtime {}

    #[automatically_derived]
    impl ::core::cmp::Eq for Runtime {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }

    impl core::fmt::Debug for Runtime {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_tuple("Runtime").finish()
        }
    }

    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Runtime {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new(
                            "Runtime",
                            "acurast_sandbox::mock_acurast",
                        ),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .docs(&[])
                    .composite(::scale_info::build::Fields::unit())
            }
        }
    };

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::GetNodeBlockType
    for Runtime {
        type NodeBlock = Block;
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::GetRuntimeBlockType
    for Runtime {
        type RuntimeBlock = Block;
    }

    #[allow(non_camel_case_types)]
    pub enum Event {
        #[codec(index = 0u8)]
        System(frame_system::Event<Runtime>),
        #[codec(index = 1u8)]
        Balances(pallet_balances::Event<Runtime>),
        #[codec(index = 2u8)]
        MsgQueue(mock_msg_queue::Event<Runtime>),
        #[codec(index = 3u8)]
        PolkadotXcm(pallet_xcm::Event<Runtime>),
        #[codec(index = 41u8)]
        TestPallet(test_pallet::Event<Runtime>),
    }

    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for Event {
        #[inline]
        fn clone(&self) -> Event {
            match self {
                Event::System(__self_0) => {
                    Event::System(::core::clone::Clone::clone(__self_0))
                }
                Event::Balances(__self_0) => {
                    Event::Balances(::core::clone::Clone::clone(__self_0))
                }
                Event::MsgQueue(__self_0) => {
                    Event::MsgQueue(::core::clone::Clone::clone(__self_0))
                }
                Event::PolkadotXcm(__self_0) => {
                    Event::PolkadotXcm(::core::clone::Clone::clone(__self_0))
                }
                Event::TestPallet(__self_0) => {
                    Event::TestPallet(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }

    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for Event {}

    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for Event {
        #[inline]
        fn eq(&self, other: &Event) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                (Event::System(__self_0), Event::System(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (Event::Balances(__self_0), Event::Balances(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (Event::MsgQueue(__self_0), Event::MsgQueue(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (Event::PolkadotXcm(__self_0), Event::PolkadotXcm(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (Event::TestPallet(__self_0), Event::TestPallet(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
        }
    }

    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralEq for Event {}

    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::Eq for Event {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<frame_system::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_balances::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<mock_msg_queue::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_xcm::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<test_pallet::Event<Runtime>>;
        }
    }

    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl ::codec::Encode for Event {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::System(ref aa) => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Event::Balances(ref aa) => {
                        __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Event::MsgQueue(ref aa) => {
                        __codec_dest_edqy.push_byte(2u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Event::PolkadotXcm(ref aa) => {
                        __codec_dest_edqy.push_byte(3u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Event::TestPallet(ref aa) => {
                        __codec_dest_edqy.push_byte(41u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl ::codec::EncodeLike for Event {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl ::codec::Decode for Event {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Event`, failed to read variant byte")
                    })?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            Event::System({
                                let __codec_res_edqy = <frame_system::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::System.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            Event::Balances({
                                let __codec_res_edqy = <pallet_balances::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::Balances.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    __codec_x_edqy if __codec_x_edqy == 2u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            Event::MsgQueue({
                                let __codec_res_edqy = <mock_msg_queue::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::MsgQueue.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    __codec_x_edqy if __codec_x_edqy == 3u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            Event::PolkadotXcm({
                                let __codec_res_edqy = <pallet_xcm::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::PolkadotXcm.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    __codec_x_edqy if __codec_x_edqy == 41u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            Event::TestPallet({
                                let __codec_res_edqy = <test_pallet::Event<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::TestPallet.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    _ => {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into("Could not decode `Event`, variant doesn't exist"),
                        )
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Event {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new("Event", "acurast_sandbox::mock_acurast"),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .docs(&[])
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "System",
                                |v| {
                                    v
                                        .index(0u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f
                                                        .ty::<frame_system::Event<Runtime>>()
                                                        .type_name("frame_system::Event<Runtime>")
                                                        .docs(&[])
                                                }),
                                        )
                                        .docs(&[])
                                },
                            )
                            .variant(
                                "Balances",
                                |v| {
                                    v
                                        .index(1u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f
                                                        .ty::<pallet_balances::Event<Runtime>>()
                                                        .type_name("pallet_balances::Event<Runtime>")
                                                        .docs(&[])
                                                }),
                                        )
                                        .docs(&[])
                                },
                            )
                            .variant(
                                "MsgQueue",
                                |v| {
                                    v
                                        .index(2u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f
                                                        .ty::<mock_msg_queue::Event<Runtime>>()
                                                        .type_name("mock_msg_queue::Event<Runtime>")
                                                        .docs(&[])
                                                }),
                                        )
                                        .docs(&[])
                                },
                            )
                            .variant(
                                "PolkadotXcm",
                                |v| {
                                    v
                                        .index(3u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f
                                                        .ty::<pallet_xcm::Event<Runtime>>()
                                                        .type_name("pallet_xcm::Event<Runtime>")
                                                        .docs(&[])
                                                }),
                                        )
                                        .docs(&[])
                                },
                            )
                            .variant(
                                "TestPallet",
                                |v| {
                                    v
                                        .index(41u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f
                                                        .ty::<test_pallet::Event<Runtime>>()
                                                        .type_name("test_pallet::Event<Runtime>")
                                                        .docs(&[])
                                                }),
                                        )
                                        .docs(&[])
                                },
                            ),
                    )
            }
        }
    };

    impl core::fmt::Debug for Event {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                Self::System(ref a0) => {
                    fmt.debug_tuple("Event::System").field(a0).finish()
                }
                Self::Balances(ref a0) => {
                    fmt.debug_tuple("Event::Balances").field(a0).finish()
                }
                Self::MsgQueue(ref a0) => {
                    fmt.debug_tuple("Event::MsgQueue").field(a0).finish()
                }
                Self::PolkadotXcm(ref a0) => {
                    fmt.debug_tuple("Event::PolkadotXcm").field(a0).finish()
                }
                Self::TestPallet(ref a0) => {
                    fmt.debug_tuple("Event::TestPallet").field(a0).finish()
                }
                _ => Ok(()),
            }
        }
    }

    impl From<frame_system::Event<Runtime>> for Event {
        fn from(x: frame_system::Event<Runtime>) -> Self {
            Event::System(x)
        }
    }

    impl TryInto<frame_system::Event<Runtime>> for Event {
        type Error = ();
        fn try_into(
            self,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
            frame_system::Event<Runtime>,
            Self::Error,
        > {
            match self {
                Self::System(evt) => Ok(evt),
                _ => Err(()),
            }
        }
    }

    impl From<pallet_balances::Event<Runtime>> for Event {
        fn from(x: pallet_balances::Event<Runtime>) -> Self {
            Event::Balances(x)
        }
    }

    impl TryInto<pallet_balances::Event<Runtime>> for Event {
        type Error = ();
        fn try_into(
            self,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
            pallet_balances::Event<Runtime>,
            Self::Error,
        > {
            match self {
                Self::Balances(evt) => Ok(evt),
                _ => Err(()),
            }
        }
    }

    impl From<mock_msg_queue::Event<Runtime>> for Event {
        fn from(x: mock_msg_queue::Event<Runtime>) -> Self {
            Event::MsgQueue(x)
        }
    }

    impl TryInto<mock_msg_queue::Event<Runtime>> for Event {
        type Error = ();
        fn try_into(
            self,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
            mock_msg_queue::Event<Runtime>,
            Self::Error,
        > {
            match self {
                Self::MsgQueue(evt) => Ok(evt),
                _ => Err(()),
            }
        }
    }

    impl From<pallet_xcm::Event<Runtime>> for Event {
        fn from(x: pallet_xcm::Event<Runtime>) -> Self {
            Event::PolkadotXcm(x)
        }
    }

    impl TryInto<pallet_xcm::Event<Runtime>> for Event {
        type Error = ();
        fn try_into(
            self,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
            pallet_xcm::Event<Runtime>,
            Self::Error,
        > {
            match self {
                Self::PolkadotXcm(evt) => Ok(evt),
                _ => Err(()),
            }
        }
    }

    impl From<test_pallet::Event<Runtime>> for Event {
        fn from(x: test_pallet::Event<Runtime>) -> Self {
            Event::TestPallet(x)
        }
    }

    impl TryInto<test_pallet::Event<Runtime>> for Event {
        type Error = ();
        fn try_into(
            self,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
            test_pallet::Event<Runtime>,
            Self::Error,
        > {
            match self {
                Self::TestPallet(evt) => Ok(evt),
                _ => Err(()),
            }
        }
    }

    /// The runtime origin type representing the origin of a call.
    ///
    /// Origin is always created with the base filter configured in [`frame_system::Config::BaseCallFilter`].
    pub struct Origin {
        caller: OriginCaller,
        filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc<
            Box<dyn Fn(&<Runtime as frame_system::Config>::Call) -> bool>,
        >,
    }

    #[automatically_derived]
    impl ::core::clone::Clone for Origin {
        #[inline]
        fn clone(&self) -> Origin {
            Origin {
                caller: ::core::clone::Clone::clone(&self.caller),
                filter: ::core::clone::Clone::clone(&self.filter),
            }
        }
    }

    #[cfg(not(feature = "std"))]
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Debug
    for Origin {
        fn fmt(
            &self,
            fmt: &mut self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Formatter,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
            (),
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Error,
        > {
            fmt.write_str("<wasm:stripped>")
        }
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait
    for Origin {
        type Call = <Runtime as frame_system::Config>::Call;
        type PalletsOrigin = OriginCaller;
        type AccountId = <Runtime as frame_system::Config>::AccountId;
        fn add_filter(&mut self, filter: impl Fn(&Self::Call) -> bool + 'static) {
            let f = self.filter.clone();
            self
                .filter = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                Box::new(move |call| { f(call) && filter(call) }),
            );
        }
        fn reset_filter(&mut self) {
            let filter = <<Runtime as frame_system::Config>::BaseCallFilter as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Contains<
                <Runtime as frame_system::Config>::Call,
            >>::contains;
            self
                .filter = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                Box::new(filter),
            );
        }
        fn set_caller_from(&mut self, other: impl Into<Self>) {
            self.caller = other.into().caller;
        }
        fn filter_call(&self, call: &Self::Call) -> bool {
            match self.caller {
                OriginCaller::system(frame_system::Origin::<Runtime>::Root) => true,
                _ => (self.filter)(call),
            }
        }
        fn caller(&self) -> &Self::PalletsOrigin {
            &self.caller
        }
        fn try_with_caller<R>(
            mut self,
            f: impl FnOnce(Self::PalletsOrigin) -> Result<R, Self::PalletsOrigin>,
        ) -> Result<R, Self> {
            match f(self.caller) {
                Ok(r) => Ok(r),
                Err(caller) => {
                    self.caller = caller;
                    Err(self)
                }
            }
        }
        fn none() -> Self {
            frame_system::RawOrigin::None.into()
        }
        fn root() -> Self {
            frame_system::RawOrigin::Root.into()
        }
        fn signed(by: Self::AccountId) -> Self {
            frame_system::RawOrigin::Signed(by).into()
        }
        fn as_signed(self) -> Option<Self::AccountId> {
            match self.caller {
                OriginCaller::system(frame_system::RawOrigin::Signed(by)) => Some(by),
                _ => None,
            }
        }
    }

    #[allow(non_camel_case_types)]
    pub enum OriginCaller {
        #[codec(index = 0u8)]
        system(frame_system::Origin<Runtime>),
        #[codec(index = 3u8)]
        PolkadotXcm(pallet_xcm::Origin),
        #[allow(dead_code)]
        Void(self::sp_api_hidden_includes_construct_runtime::hidden_include::Void),
    }

    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for OriginCaller {
        #[inline]
        fn clone(&self) -> OriginCaller {
            match self {
                OriginCaller::system(__self_0) => {
                    OriginCaller::system(::core::clone::Clone::clone(__self_0))
                }
                OriginCaller::PolkadotXcm(__self_0) => {
                    OriginCaller::PolkadotXcm(::core::clone::Clone::clone(__self_0))
                }
                OriginCaller::Void(__self_0) => {
                    OriginCaller::Void(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }

    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for OriginCaller {}

    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for OriginCaller {
        #[inline]
        fn eq(&self, other: &OriginCaller) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                (OriginCaller::system(__self_0), OriginCaller::system(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (
                    OriginCaller::PolkadotXcm(__self_0),
                    OriginCaller::PolkadotXcm(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                (OriginCaller::Void(__self_0), OriginCaller::Void(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
        }
    }

    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralEq for OriginCaller {}

    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::Eq for OriginCaller {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<frame_system::Origin<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_xcm::Origin>;
            let _: ::core::cmp::AssertParamIsEq<
                self::sp_api_hidden_includes_construct_runtime::hidden_include::Void,
            >;
        }
    }

    impl core::fmt::Debug for OriginCaller {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                Self::system(ref a0) => {
                    fmt.debug_tuple("OriginCaller::system").field(a0).finish()
                }
                Self::PolkadotXcm(ref a0) => {
                    fmt.debug_tuple("OriginCaller::PolkadotXcm").field(a0).finish()
                }
                Self::Void(ref a0) => {
                    fmt.debug_tuple("OriginCaller::Void").field(a0).finish()
                }
                _ => Ok(()),
            }
        }
    }

    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl ::codec::Encode for OriginCaller {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    OriginCaller::system(ref aa) => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    OriginCaller::PolkadotXcm(ref aa) => {
                        __codec_dest_edqy.push_byte(3u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    OriginCaller::Void(ref aa) => {
                        __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl ::codec::EncodeLike for OriginCaller {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl ::codec::Decode for OriginCaller {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e
                            .chain(
                                "Could not decode `OriginCaller`, failed to read variant byte",
                            )
                    })?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            OriginCaller::system({
                                let __codec_res_edqy = <frame_system::Origin<
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `OriginCaller::system.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    __codec_x_edqy if __codec_x_edqy == 3u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            OriginCaller::PolkadotXcm({
                                let __codec_res_edqy = <pallet_xcm::Origin as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `OriginCaller::PolkadotXcm.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    __codec_x_edqy if __codec_x_edqy
                        == 2usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            OriginCaller::Void({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::Void as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `OriginCaller::Void.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    _ => {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `OriginCaller`, variant doesn't exist",
                            ),
                        )
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for OriginCaller {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new(
                            "OriginCaller",
                            "acurast_sandbox::mock_acurast",
                        ),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .docs(&[])
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "system",
                                |v| {
                                    v
                                        .index(0u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f
                                                        .ty::<frame_system::Origin<Runtime>>()
                                                        .type_name("frame_system::Origin<Runtime>")
                                                        .docs(&[])
                                                }),
                                        )
                                        .docs(&[])
                                },
                            )
                            .variant(
                                "PolkadotXcm",
                                |v| {
                                    v
                                        .index(3u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f
                                                        .ty::<pallet_xcm::Origin>()
                                                        .type_name("pallet_xcm::Origin")
                                                        .docs(&[])
                                                }),
                                        )
                                        .docs(&[])
                                },
                            )
                            .variant(
                                "Void",
                                |v| {
                                    v
                                        .index(2usize as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f
                                                        .ty::<
                                                            self::sp_api_hidden_includes_construct_runtime::hidden_include::Void,
                                                        >()
                                                        .type_name(
                                                            "self::sp_api_hidden_includes_construct_runtime::hidden_include::Void",
                                                        )
                                                        .docs(&[])
                                                }),
                                        )
                                        .docs(&[])
                                },
                            ),
                    )
            }
        }
    };

    #[allow(dead_code)]
    impl Origin {
        /// Create with system none origin and [`frame_system::Config::BaseCallFilter`].
        pub fn none() -> Self {
            <Origin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::none()
        }
        /// Create with system root origin and [`frame_system::Config::BaseCallFilter`].
        pub fn root() -> Self {
            <Origin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::root()
        }
        /// Create with system signed origin and [`frame_system::Config::BaseCallFilter`].
        pub fn signed(by: <Runtime as frame_system::Config>::AccountId) -> Self {
            <Origin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::signed(
                by,
            )
        }
    }

    impl From<frame_system::Origin<Runtime>> for OriginCaller {
        fn from(x: frame_system::Origin<Runtime>) -> Self {
            OriginCaller::system(x)
        }
    }

    impl TryFrom<OriginCaller> for frame_system::Origin<Runtime> {
        type Error = OriginCaller;
        fn try_from(
            x: OriginCaller,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
            frame_system::Origin<Runtime>,
            OriginCaller,
        > {
            if let OriginCaller::system(l) = x { Ok(l) } else { Err(x) }
        }
    }

    impl From<frame_system::Origin<Runtime>> for Origin {
        /// Convert to runtime origin, using as filter: [`frame_system::Config::BaseCallFilter`].
        fn from(x: frame_system::Origin<Runtime>) -> Self {
            let o: OriginCaller = x.into();
            o.into()
        }
    }

    impl From<OriginCaller> for Origin {
        fn from(x: OriginCaller) -> Self {
            let mut o = Origin {
                caller: x,
                filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                    Box::new(|_| true),
                ),
            };
            self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait::reset_filter(
                &mut o,
            );
            o
        }
    }

    impl From<Origin>
    for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        frame_system::Origin<Runtime>,
        Origin,
    > {
        /// NOTE: converting to pallet origin loses the origin filter information.
        fn from(val: Origin) -> Self {
            if let OriginCaller::system(l) = val.caller { Ok(l) } else { Err(val) }
        }
    }

    impl From<Option<<Runtime as frame_system::Config>::AccountId>> for Origin {
        /// Convert to runtime origin with caller being system signed or none and use filter [`frame_system::Config::BaseCallFilter`].
        fn from(x: Option<<Runtime as frame_system::Config>::AccountId>) -> Self {
            <frame_system::Origin<Runtime>>::from(x).into()
        }
    }

    impl From<pallet_xcm::Origin> for OriginCaller {
        fn from(x: pallet_xcm::Origin) -> Self {
            OriginCaller::PolkadotXcm(x)
        }
    }

    impl From<pallet_xcm::Origin> for Origin {
        ///  Convert to runtime origin using [`pallet_xcm::Config::BaseCallFilter`].
        fn from(x: pallet_xcm::Origin) -> Self {
            let x: OriginCaller = x.into();
            x.into()
        }
    }

    impl From<Origin>
    for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_xcm::Origin,
        Origin,
    > {
        /// NOTE: converting to pallet origin loses the origin filter information.
        fn from(val: Origin) -> Self {
            if let OriginCaller::PolkadotXcm(l) = val.caller { Ok(l) } else { Err(val) }
        }
    }

    impl TryFrom<OriginCaller> for pallet_xcm::Origin {
        type Error = OriginCaller;
        fn try_from(
            x: OriginCaller,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
            pallet_xcm::Origin,
            OriginCaller,
        > {
            if let OriginCaller::PolkadotXcm(l) = x { Ok(l) } else { Err(x) }
        }
    }

    pub type System = frame_system::Pallet<Runtime>;
    pub type Balances = pallet_balances::Pallet<Runtime>;
    pub type MsgQueue = mock_msg_queue::Pallet<Runtime>;
    pub type PolkadotXcm = pallet_xcm::Pallet<Runtime>;
    pub type TestPallet = test_pallet::Pallet<Runtime>;
    /// All pallets included in the runtime as a nested tuple of types.
    #[deprecated(
    note = "The type definition has changed from representing all pallets \
            excluding system, in reversed order to become the representation of all pallets \
            including system pallet in regular order. For this reason it is encouraged to use \
            explicitly one of `AllPalletsWithSystem`, `AllPalletsWithoutSystem`, \
            `AllPalletsWithSystemReversed`, `AllPalletsWithoutSystemReversed`. \
            Note that the type `frame_executive::Executive` expects one of `AllPalletsWithSystem` \
            , `AllPalletsWithSystemReversed`, `AllPalletsReversedWithSystemFirst`. More details in \
            https://github.com/paritytech/substrate/pull/10043"
    )]
    pub type AllPallets = AllPalletsWithSystem;
    /// All pallets included in the runtime as a nested tuple of types.
    pub type AllPalletsWithSystem = ((
        System,
        (Balances, (MsgQueue, (PolkadotXcm, (TestPallet, )))),
    ));
    /// All pallets included in the runtime as a nested tuple of types.
    /// Excludes the System pallet.
    pub type AllPalletsWithoutSystem = ((
        Balances,
        (MsgQueue, (PolkadotXcm, (TestPallet, ))),
    ));
    /// All pallets included in the runtime as a nested tuple of types in reversed order.
    /// Excludes the System pallet.
    pub type AllPalletsWithoutSystemReversed = ((
        TestPallet,
        (PolkadotXcm, (MsgQueue, (Balances, ))),
    ));
    /// All pallets included in the runtime as a nested tuple of types in reversed order.
    pub type AllPalletsWithSystemReversed = ((
        TestPallet,
        (PolkadotXcm, (MsgQueue, (Balances, (System, )))),
    ));
    /// All pallets included in the runtime as a nested tuple of types in reversed order.
    /// With the system pallet first.
    pub type AllPalletsReversedWithSystemFirst = (
        System,
        AllPalletsWithoutSystemReversed,
    );

    /// Provides an implementation of `PalletInfo` to provide information
    /// about the pallet setup in the runtime.
    pub struct PalletInfo;

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfo
    for PalletInfo {
        fn index<P: 'static>() -> Option<usize> {
            let type_id = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                System,
            >()
            {
                return Some(0usize);
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Balances,
            >()
            {
                return Some(1usize);
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                MsgQueue,
            >()
            {
                return Some(2usize);
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                PolkadotXcm,
            >()
            {
                return Some(3usize);
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                TestPallet,
            >()
            {
                return Some(41usize);
            }
            None
        }
        fn name<P: 'static>() -> Option<&'static str> {
            let type_id = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                System,
            >()
            {
                return Some("System");
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Balances,
            >()
            {
                return Some("Balances");
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                MsgQueue,
            >()
            {
                return Some("MsgQueue");
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                PolkadotXcm,
            >()
            {
                return Some("PolkadotXcm");
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                TestPallet,
            >()
            {
                return Some("TestPallet");
            }
            None
        }
        fn module_name<P: 'static>() -> Option<&'static str> {
            let type_id = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                System,
            >()
            {
                return Some("frame_system");
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Balances,
            >()
            {
                return Some("pallet_balances");
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                MsgQueue,
            >()
            {
                return Some("mock_msg_queue");
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                PolkadotXcm,
            >()
            {
                return Some("pallet_xcm");
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                TestPallet,
            >()
            {
                return Some("test_pallet");
            }
            None
        }
        fn crate_version<P: 'static>() -> Option<
            self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::CrateVersion,
        > {
            let type_id = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                System,
            >()
            {
                return Some(
                    <frame_system::Pallet<
                        Runtime,
                    > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
                );
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                Balances,
            >()
            {
                return Some(
                    <pallet_balances::Pallet<
                        Runtime,
                    > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
                );
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                MsgQueue,
            >()
            {
                return Some(
                    <mock_msg_queue::Pallet<
                        Runtime,
                    > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
                );
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                PolkadotXcm,
            >()
            {
                return Some(
                    <pallet_xcm::Pallet<
                        Runtime,
                    > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
                );
            }
            if type_id
                == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                TestPallet,
            >()
            {
                return Some(
                    <test_pallet::Pallet<
                        Runtime,
                    > as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version(),
                );
            }
            None
        }
    }

    pub enum Call {
        #[codec(index = 0u8)]
        System(
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                System,
                Runtime,
            >,
        ),
        #[codec(index = 1u8)]
        Balances(
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                Balances,
                Runtime,
            >,
        ),
        #[codec(index = 3u8)]
        PolkadotXcm(
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                PolkadotXcm,
                Runtime,
            >,
        ),
        #[codec(index = 41u8)]
        TestPallet(
            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                TestPallet,
                Runtime,
            >,
        ),
    }

    #[automatically_derived]
    impl ::core::clone::Clone for Call {
        #[inline]
        fn clone(&self) -> Call {
            match self {
                Call::System(__self_0) => {
                    Call::System(::core::clone::Clone::clone(__self_0))
                }
                Call::Balances(__self_0) => {
                    Call::Balances(::core::clone::Clone::clone(__self_0))
                }
                Call::PolkadotXcm(__self_0) => {
                    Call::PolkadotXcm(::core::clone::Clone::clone(__self_0))
                }
                Call::TestPallet(__self_0) => {
                    Call::TestPallet(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }

    impl ::core::marker::StructuralPartialEq for Call {}

    #[automatically_derived]
    impl ::core::cmp::PartialEq for Call {
        #[inline]
        fn eq(&self, other: &Call) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                (Call::System(__self_0), Call::System(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (Call::Balances(__self_0), Call::Balances(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (Call::PolkadotXcm(__self_0), Call::PolkadotXcm(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (Call::TestPallet(__self_0), Call::TestPallet(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
        }
    }

    impl ::core::marker::StructuralEq for Call {}

    #[automatically_derived]
    impl ::core::cmp::Eq for Call {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                    System,
                    Runtime,
                >,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                    Balances,
                    Runtime,
                >,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                    PolkadotXcm,
                    Runtime,
                >,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                    TestPallet,
                    Runtime,
                >,
            >;
        }
    }

    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::codec::Encode for Call {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::System(ref aa) => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Call::Balances(ref aa) => {
                        __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Call::PolkadotXcm(ref aa) => {
                        __codec_dest_edqy.push_byte(3u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Call::TestPallet(ref aa) => {
                        __codec_dest_edqy.push_byte(41u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl ::codec::EncodeLike for Call {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::codec::Decode for Call {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Call`, failed to read variant byte")
                    })?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            Call::System({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    System,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::System.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            Call::Balances({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    Balances,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::Balances.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    __codec_x_edqy if __codec_x_edqy == 3u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            Call::PolkadotXcm({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    PolkadotXcm,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::PolkadotXcm.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    __codec_x_edqy if __codec_x_edqy == 41u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            Call::TestPallet({
                                let __codec_res_edqy = <self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                    TestPallet,
                                    Runtime,
                                > as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::TestPallet.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    _ => {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into("Could not decode `Call`, variant doesn't exist"),
                        )
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Call {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new("Call", "acurast_sandbox::mock_acurast"),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .docs(&[])
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "System",
                                |v| {
                                    v
                                        .index(0u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f
                                                        .ty::<
                                                            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                                System,
                                                                Runtime,
                                                            >,
                                                        >()
                                                        .type_name(
                                                            "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<System, Runtime>",
                                                        )
                                                        .docs(&[])
                                                }),
                                        )
                                        .docs(&[])
                                },
                            )
                            .variant(
                                "Balances",
                                |v| {
                                    v
                                        .index(1u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f
                                                        .ty::<
                                                            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                                Balances,
                                                                Runtime,
                                                            >,
                                                        >()
                                                        .type_name(
                                                            "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Balances, Runtime>",
                                                        )
                                                        .docs(&[])
                                                }),
                                        )
                                        .docs(&[])
                                },
                            )
                            .variant(
                                "PolkadotXcm",
                                |v| {
                                    v
                                        .index(3u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f
                                                        .ty::<
                                                            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                                PolkadotXcm,
                                                                Runtime,
                                                            >,
                                                        >()
                                                        .type_name(
                                                            "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<PolkadotXcm, Runtime>",
                                                        )
                                                        .docs(&[])
                                                }),
                                        )
                                        .docs(&[])
                                },
                            )
                            .variant(
                                "TestPallet",
                                |v| {
                                    v
                                        .index(41u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::unnamed()
                                                .field(|f| {
                                                    f
                                                        .ty::<
                                                            self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                                                                TestPallet,
                                                                Runtime,
                                                            >,
                                                        >()
                                                        .type_name(
                                                            "self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<TestPallet, Runtime>",
                                                        )
                                                        .docs(&[])
                                                }),
                                        )
                                        .docs(&[])
                                },
                            ),
                    )
            }
        }
    };

    impl core::fmt::Debug for Call {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                Self::System(ref a0) => {
                    fmt.debug_tuple("Call::System").field(a0).finish()
                }
                Self::Balances(ref a0) => {
                    fmt.debug_tuple("Call::Balances").field(a0).finish()
                }
                Self::PolkadotXcm(ref a0) => {
                    fmt.debug_tuple("Call::PolkadotXcm").field(a0).finish()
                }
                Self::TestPallet(ref a0) => {
                    fmt.debug_tuple("Call::TestPallet").field(a0).finish()
                }
                _ => Ok(()),
            }
        }
    }

    #[cfg(test)]
    impl Call {
        /// Return a list of the module names together with their size in memory.
        pub const fn sizes() -> &'static [(&'static str, usize)] {
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::Callable;
            use core::mem::size_of;
            &[
                ("System", size_of::<<System as Callable<Runtime>>::Call>()),
                ("Balances", size_of::<<Balances as Callable<Runtime>>::Call>()),
                ("PolkadotXcm", size_of::<<PolkadotXcm as Callable<Runtime>>::Call>()),
                ("TestPallet", size_of::<<TestPallet as Callable<Runtime>>::Call>()),
            ]
        }
        /// Panics with diagnostic information if the size is greater than the given `limit`.
        pub fn assert_size_under(limit: usize) {
            let size = core::mem::size_of::<Self>();
            let call_oversize = size > limit;
            if call_oversize {
                {
                    ::std::io::_print(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "Size of `Call` is ",
                                " bytes (provided limit is ",
                                " bytes)\n",
                            ],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&size),
                                ::core::fmt::ArgumentV1::new_display(&limit),
                            ],
                        ),
                    );
                };
                let mut sizes = Self::sizes().to_vec();
                sizes.sort_by_key(|x| -(x.1 as isize));
                for (i, &(name, size)) in sizes.iter().enumerate().take(5) {
                    {
                        ::std::io::_print(
                            ::core::fmt::Arguments::new_v1(
                                &["Offender #", ": ", " at ", " bytes\n"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&(i + 1)),
                                    ::core::fmt::ArgumentV1::new_display(&name),
                                    ::core::fmt::ArgumentV1::new_display(&size),
                                ],
                            ),
                        );
                    };
                }
                if let Some((_, next_size)) = sizes.get(5) {
                    {
                        ::std::io::_print(
                            ::core::fmt::Arguments::new_v1(
                                &["", " others of size ", " bytes or less\n"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&(sizes.len() - 5)),
                                    ::core::fmt::ArgumentV1::new_display(&next_size),
                                ],
                            ),
                        );
                    };
                }
                ::core::panicking::panic_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[
                            "Size of `Call` is more than limit; use `Box` on complex parameter types to reduce the\n\t\t\t\t\t\tsize of `Call`.\n\t\t\t\t\t\tIf the limit is too strong, maybe consider providing a higher limit.",
                        ],
                        &[],
                    ),
                );
            }
        }
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetDispatchInfo
    for Call {
        fn get_dispatch_info(
            &self,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchInfo {
            match self {
                Call::System(call) => call.get_dispatch_info(),
                Call::Balances(call) => call.get_dispatch_info(),
                Call::PolkadotXcm(call) => call.get_dispatch_info(),
                Call::TestPallet(call) => call.get_dispatch_info(),
            }
        }
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallMetadata
    for Call {
        fn get_call_metadata(
            &self,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallName;
            match self {
                Call::System(call) => {
                    let function_name = call.get_call_name();
                    let pallet_name = "System";
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
                        function_name,
                        pallet_name,
                    }
                }
                Call::Balances(call) => {
                    let function_name = call.get_call_name();
                    let pallet_name = "Balances";
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
                        function_name,
                        pallet_name,
                    }
                }
                Call::PolkadotXcm(call) => {
                    let function_name = call.get_call_name();
                    let pallet_name = "PolkadotXcm";
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
                        function_name,
                        pallet_name,
                    }
                }
                Call::TestPallet(call) => {
                    let function_name = call.get_call_name();
                    let pallet_name = "TestPallet";
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
                        function_name,
                        pallet_name,
                    }
                }
            }
        }
        fn get_module_names() -> &'static [&'static str] {
            &["System", "Balances", "PolkadotXcm", "TestPallet"]
        }
        fn get_call_names(module: &str) -> &'static [&'static str] {
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::{
                Callable, GetCallName,
            };
            match module {
                "System" => {
                    <<System as Callable<
                        Runtime,
                    >>::Call as GetCallName>::get_call_names()
                }
                "Balances" => {
                    <<Balances as Callable<
                        Runtime,
                    >>::Call as GetCallName>::get_call_names()
                }
                "PolkadotXcm" => {
                    <<PolkadotXcm as Callable<
                        Runtime,
                    >>::Call as GetCallName>::get_call_names()
                }
                "TestPallet" => {
                    <<TestPallet as Callable<
                        Runtime,
                    >>::Call as GetCallName>::get_call_names()
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::Dispatchable
    for Call {
        type Origin = Origin;
        type Config = Call;
        type Info = self::sp_api_hidden_includes_construct_runtime::hidden_include::weights::DispatchInfo;
        type PostInfo = self::sp_api_hidden_includes_construct_runtime::hidden_include::weights::PostDispatchInfo;
        fn dispatch(
            self,
            origin: Origin,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchResultWithPostInfo {
            if !<Self::Origin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::filter_call(
                &origin,
                &self,
            ) {
                return self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result::Err(
                    frame_system::Error::<Runtime>::CallFiltered.into(),
                );
            }
            self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                self,
                origin,
            )
        }
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable
    for Call {
        type Origin = Origin;
        fn dispatch_bypass_filter(
            self,
            origin: Origin,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchResultWithPostInfo {
            match self {
                Call::System(call) => {
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                        call,
                        origin,
                    )
                }
                Call::Balances(call) => {
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                        call,
                        origin,
                    )
                }
                Call::PolkadotXcm(call) => {
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                        call,
                        origin,
                    )
                }
                Call::TestPallet(call) => {
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                        call,
                        origin,
                    )
                }
            }
        }
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::DispatchableWithStorageLayer
    for Call {
        type Origin = Origin;
        fn dispatch_with_storage_layer(
            self,
            origin: Origin,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchResultWithPostInfo {
            self::sp_api_hidden_includes_construct_runtime::hidden_include::storage::with_storage_layer(||
                {
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::Dispatchable::dispatch(
                        self,
                        origin,
                    )
                })
        }
        fn dispatch_bypass_filter_with_storage_layer(
            self,
            origin: Origin,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchResultWithPostInfo {
            self::sp_api_hidden_includes_construct_runtime::hidden_include::storage::with_storage_layer(||
                {
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                        self,
                        origin,
                    )
                })
        }
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    > for Call {
        #[allow(unreachable_patterns)]
        fn is_sub_type(
            &self,
        ) -> Option<
            &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                System,
                Runtime,
            >,
        > {
            match self {
                Call::System(call) => Some(call),
                _ => None,
            }
        }
    }

    impl From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    > for Call {
        fn from(
            call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                System,
                Runtime,
            >,
        ) -> Self {
            Call::System(call)
        }
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    > for Call {
        #[allow(unreachable_patterns)]
        fn is_sub_type(
            &self,
        ) -> Option<
            &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                Balances,
                Runtime,
            >,
        > {
            match self {
                Call::Balances(call) => Some(call),
                _ => None,
            }
        }
    }

    impl From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    > for Call {
        fn from(
            call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                Balances,
                Runtime,
            >,
        ) -> Self {
            Call::Balances(call)
        }
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            PolkadotXcm,
            Runtime,
        >,
    > for Call {
        #[allow(unreachable_patterns)]
        fn is_sub_type(
            &self,
        ) -> Option<
            &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                PolkadotXcm,
                Runtime,
            >,
        > {
            match self {
                Call::PolkadotXcm(call) => Some(call),
                _ => None,
            }
        }
    }

    impl From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            PolkadotXcm,
            Runtime,
        >,
    > for Call {
        fn from(
            call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                PolkadotXcm,
                Runtime,
            >,
        ) -> Self {
            Call::PolkadotXcm(call)
        }
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TestPallet,
            Runtime,
        >,
    > for Call {
        #[allow(unreachable_patterns)]
        fn is_sub_type(
            &self,
        ) -> Option<
            &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                TestPallet,
                Runtime,
            >,
        > {
            match self {
                Call::TestPallet(call) => Some(call),
                _ => None,
            }
        }
    }

    impl From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TestPallet,
            Runtime,
        >,
    > for Call {
        fn from(
            call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
                TestPallet,
                Runtime,
            >,
        ) -> Self {
            Call::TestPallet(call)
        }
    }

    impl Runtime {
        pub fn metadata() -> self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::RuntimeMetadataPrefixed {
            self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::RuntimeMetadataLastVersion::new(
                <[_]>::into_vec(
                    #[rustc_box]
                        ::alloc::boxed::Box::new([
                        self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata {
                            name: "System",
                            index: 0u8,
                            storage: Some(
                                frame_system::Pallet::<Runtime>::storage_metadata(),
                            ),
                            calls: Some(
                                frame_system::Pallet::<Runtime>::call_functions(),
                            ),
                            event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata {
                                ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                                    frame_system::Event<Runtime>,
                                >(),
                            }),
                            constants: frame_system::Pallet::<
                                Runtime,
                            >::pallet_constants_metadata(),
                            error: frame_system::Pallet::<Runtime>::error_metadata(),
                        },
                        self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata {
                            name: "Balances",
                            index: 1u8,
                            storage: Some(
                                pallet_balances::Pallet::<Runtime>::storage_metadata(),
                            ),
                            calls: Some(
                                pallet_balances::Pallet::<Runtime>::call_functions(),
                            ),
                            event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata {
                                ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                                    pallet_balances::Event<Runtime>,
                                >(),
                            }),
                            constants: pallet_balances::Pallet::<
                                Runtime,
                            >::pallet_constants_metadata(),
                            error: pallet_balances::Pallet::<Runtime>::error_metadata(),
                        },
                        self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata {
                            name: "MsgQueue",
                            index: 2u8,
                            storage: Some(
                                mock_msg_queue::Pallet::<Runtime>::storage_metadata(),
                            ),
                            calls: None,
                            event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata {
                                ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                                    mock_msg_queue::Event<Runtime>,
                                >(),
                            }),
                            constants: mock_msg_queue::Pallet::<
                                Runtime,
                            >::pallet_constants_metadata(),
                            error: mock_msg_queue::Pallet::<Runtime>::error_metadata(),
                        },
                        self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata {
                            name: "PolkadotXcm",
                            index: 3u8,
                            storage: None,
                            calls: Some(
                                pallet_xcm::Pallet::<Runtime>::call_functions(),
                            ),
                            event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata {
                                ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                                    pallet_xcm::Event<Runtime>,
                                >(),
                            }),
                            constants: pallet_xcm::Pallet::<
                                Runtime,
                            >::pallet_constants_metadata(),
                            error: pallet_xcm::Pallet::<Runtime>::error_metadata(),
                        },
                        self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata {
                            name: "TestPallet",
                            index: 41u8,
                            storage: Some(
                                test_pallet::Pallet::<Runtime>::storage_metadata(),
                            ),
                            calls: Some(
                                test_pallet::Pallet::<Runtime>::call_functions(),
                            ),
                            event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata {
                                ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                                    test_pallet::Event<Runtime>,
                                >(),
                            }),
                            constants: test_pallet::Pallet::<
                                Runtime,
                            >::pallet_constants_metadata(),
                            error: test_pallet::Pallet::<Runtime>::error_metadata(),
                        },
                    ]),
                ),
                self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::ExtrinsicMetadata {
                    ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                        UncheckedExtrinsic,
                    >(),
                    version: <UncheckedExtrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::ExtrinsicMetadata>::VERSION,
                    signed_extensions: <<UncheckedExtrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::ExtrinsicMetadata>::SignedExtensions as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::SignedExtension>::metadata()
                        .into_iter()
                        .map(|meta| self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::SignedExtensionMetadata {
                            identifier: meta.identifier,
                            ty: meta.ty,
                            additional_signed: meta.additional_signed,
                        })
                        .collect(),
                },
                self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<
                    Runtime,
                >(),
            )
                .into()
        }
    }

    #[cfg(any(feature = "std", test))]
    pub type SystemConfig = frame_system::GenesisConfig;
    #[cfg(any(feature = "std", test))]
    pub type BalancesConfig = pallet_balances::GenesisConfig<Runtime>;

    #[cfg(any(feature = "std", test))]
    use self::sp_api_hidden_includes_construct_runtime::hidden_include::serde as __genesis_config_serde_import__;

    #[cfg(any(feature = "std", test))]
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    #[serde(crate = "__genesis_config_serde_import__")]
    pub struct GenesisConfig {
        pub system: SystemConfig,
        pub balances: BalancesConfig,
    }

    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use __genesis_config_serde_import__ as _serde;
        #[automatically_derived]
        impl __genesis_config_serde_import__::Serialize for GenesisConfig {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> __genesis_config_serde_import__::__private::Result<__S::Ok, __S::Error>
                where
                    __S: __genesis_config_serde_import__::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "GenesisConfig",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "system",
                    &self.system,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "balances",
                    &self.balances,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use __genesis_config_serde_import__ as _serde;
        #[automatically_derived]
        impl<'de> __genesis_config_serde_import__::Deserialize<'de> for GenesisConfig {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> __genesis_config_serde_import__::__private::Result<Self, __D::Error>
                where
                    __D: __genesis_config_serde_import__::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"field index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                    {
                        match __value {
                            "system" => _serde::__private::Ok(__Field::__field0),
                            "balances" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                    {
                        match __value {
                            b"system" => _serde::__private::Ok(__Field::__field0),
                            b"balances" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<GenesisConfig>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = GenesisConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct GenesisConfig",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            SystemConfig,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct GenesisConfig with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            BalancesConfig,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct GenesisConfig with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(GenesisConfig {
                            system: __field0,
                            balances: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<SystemConfig> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<BalancesConfig> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                        = match _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("system"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            SystemConfig,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "balances",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            BalancesConfig,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("system") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("balances") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(GenesisConfig {
                            system: __field0,
                            balances: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["system", "balances"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GenesisConfig",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<GenesisConfig>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };

    #[automatically_derived]
    impl ::core::default::Default for GenesisConfig {
        #[inline]
        fn default() -> GenesisConfig {
            GenesisConfig {
                system: ::core::default::Default::default(),
                balances: ::core::default::Default::default(),
            }
        }
    }

    #[cfg(any(feature = "std", test))]
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildStorage
    for GenesisConfig {
        fn assimilate_storage(
            &self,
            storage: &mut self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::Storage,
        ) -> std::result::Result<(), String> {
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<
                Runtime,
                frame_system::__InherentHiddenInstance,
            >::build_module_genesis_storage(&self.system, storage)?;
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<
                Runtime,
                pallet_balances::__InherentHiddenInstance,
            >::build_module_genesis_storage(&self.balances, storage)?;
            self::sp_api_hidden_includes_construct_runtime::hidden_include::BasicExternalities::execute_with_storage(
                storage,
                || {
                    <AllPalletsWithSystem as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OnGenesis>::on_genesis();
                },
            );
            Ok(())
        }
    }

    trait InherentDataExt {
        fn create_extrinsics(
            &self,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Vec<
            <Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::BlockT>::Extrinsic,
        >;
        fn check_extrinsics(
            &self,
            block: &Block,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::CheckInherentsResult;
    }

    impl InherentDataExt
    for self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::InherentData {
        fn create_extrinsics(
            &self,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Vec<
            <Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::BlockT>::Extrinsic,
        > {
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
            let mut inherents = Vec::new();
            inherents
        }
        fn check_extrinsics(
            &self,
            block: &Block,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::CheckInherentsResult {
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::{
                ProvideInherent, IsFatalError,
            };
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{
                IsSubType, ExtrinsicCall,
            };
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block as _;
            let mut result = self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::CheckInherentsResult::new();
            for xt in block.extrinsics() {
                if self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Extrinsic::is_signed(
                    xt,
                )
                    .unwrap_or(false)
                {
                    break;
                }
                let mut is_inherent = false;
                if !is_inherent {
                    break;
                }
            }
            result
        }
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::EnsureInherentsAreFirst<
        Block,
    > for Runtime {
        fn ensure_inherents_are_first(block: &Block) -> Result<(), u32> {
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{
                IsSubType, ExtrinsicCall,
            };
            use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block as _;
            let mut first_signed_observed = false;
            for (i, xt) in block.extrinsics().iter().enumerate() {
                let is_signed = self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Extrinsic::is_signed(
                    xt,
                )
                    .unwrap_or(false);
                let is_inherent = if is_signed {
                    false
                } else {
                    let mut is_inherent = false;
                    is_inherent
                };
                if !is_inherent {
                    first_signed_observed = true;
                }
                if first_signed_observed && is_inherent {
                    return Err(i as u32);
                }
            }
            Ok(())
        }
    }

    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::ValidateUnsigned
    for Runtime {
        type Call = Call;
        fn pre_dispatch(
            call: &Self::Call,
        ) -> Result<
            (),
            self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionValidityError,
        > {
            #[allow(unreachable_patterns)]
            match call {
                _ => Ok(()),
            }
        }
        fn validate_unsigned(
            #[allow(unused_variables)]
            source: self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionSource,
            call: &Self::Call,
        ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionValidity {
            #[allow(unreachable_patterns)]
            match call {
                _ => {
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::UnknownTransaction::NoUnsignedValidator
                        .into()
                }
            }
        }
    }

    #[cfg(test)]
    mod __construct_runtime_integrity_test {
        use super::*;

        extern crate test;

        #[cfg(test)]
        #[rustc_test_marker]
        pub const runtime_integrity_tests: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "mock_acurast::__construct_runtime_integrity_test::runtime_integrity_tests",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(
                runtime_integrity_tests(),
            )),
        };

        pub fn runtime_integrity_tests() {
            <AllPalletsWithSystem as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IntegrityTest>::integrity_test();
        }
    }

    const _: () = if !(<frame_system::Error<
        Runtime,
    > as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
        <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
    {
        ::core::panicking::panic_fmt(
            ::core::fmt::Arguments::new_v1(
                &[
                    "The maximum encoded size of the error type in the `System` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`",
                ],
                &[],
            ),
        )
    };
    const _: () = if !(<pallet_balances::Error<
        Runtime,
    > as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
        <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
    {
        ::core::panicking::panic_fmt(
            ::core::fmt::Arguments::new_v1(
                &[
                    "The maximum encoded size of the error type in the `Balances` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`",
                ],
                &[],
            ),
        )
    };
    const _: () = if !(<pallet_xcm::Error<
        Runtime,
    > as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
        <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
    {
        ::core::panicking::panic_fmt(
            ::core::fmt::Arguments::new_v1(
                &[
                    "The maximum encoded size of the error type in the `PolkadotXcm` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`",
                ],
                &[],
            ),
        )
    };
    const _: () = if !(<test_pallet::Error<
        Runtime,
    > as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
        <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
    {
        ::core::panicking::panic_fmt(
            ::core::fmt::Arguments::new_v1(
                &[
                    "The maximum encoded size of the error type in the `TestPallet` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`",
                ],
                &[],
            ),
        )
    };

    impl test_pallet::Config for Runtime {
        type Event = Event;
        type Origin = Origin;
        type XcmSender = XcmRouter;
    }
}

