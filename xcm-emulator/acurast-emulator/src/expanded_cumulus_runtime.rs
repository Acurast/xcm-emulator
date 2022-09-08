#[doc(hidden)]
mod sp_api_hidden_includes_construct_runtime {
    pub extern crate frame_support as hidden_include;
}
const _: () = {
    #[allow(unused)]
    type __hidden_use_of_unchecked_extrinsic = UncheckedExtrinsic;
};
#[derive(Clone, Copy, PartialEq, Eq, self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::RuntimeDebug, self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::TypeInfo)]
pub struct Runtime;
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::GetNodeBlockType for Runtime { type NodeBlock = opaque::Block; }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::GetRuntimeBlockType for Runtime { type RuntimeBlock = Block; }
#[derive(Clone, PartialEq, Eq, self::sp_api_hidden_includes_construct_runtime::hidden_include::codec::Encode, self::sp_api_hidden_includes_construct_runtime::hidden_include::codec::Decode, self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::TypeInfo, self::sp_api_hidden_includes_construct_runtime::hidden_include::RuntimeDebug, )]
#[allow(non_camel_case_types)]
pub enum Event {
    #[codec(index = 0u8)] System(frame_system::Event<Runtime>),
    #[codec(index = 1u8)] ParachainSystem(cumulus_pallet_parachain_system::Event<Runtime>),
    #[codec(index = 10u8)] Balances(pallet_balances::Event<Runtime>),
    #[codec(index = 21u8)] CollatorSelection(pallet_collator_selection::Event<Runtime>),
    #[codec(index = 22u8)] Session(pallet_session::Event),
    #[codec(index = 30u8)] XcmpQueue(cumulus_pallet_xcmp_queue::Event<Runtime>),
    #[codec(index = 31u8)] PolkadotXcm(pallet_xcm::Event<Runtime>),
    #[codec(index = 32u8)] CumulusXcm(cumulus_pallet_xcm::Event<Runtime>),
    #[codec(index = 33u8)] DmpQueue(cumulus_pallet_dmp_queue::Event<Runtime>),
    #[codec(index = 40u8)] TemplatePallet(pallet_template::Event<Runtime>),
    #[codec(index = 41u8)] ProxyPallet(proxy_pallet::Event<Runtime>),
}
impl From<frame_system::Event::<Runtime>> for Event { fn from(x: frame_system::Event::<Runtime>) -> Self { Event::System(x) } }
impl TryInto<frame_system::Event::<Runtime>> for Event {
    type Error = ();
    fn try_into(self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<frame_system::Event::<Runtime>, Self::Error> {
        match self {
            Self::System(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<cumulus_pallet_parachain_system::Event::<Runtime>> for Event { fn from(x: cumulus_pallet_parachain_system::Event::<Runtime>) -> Self { Event::ParachainSystem(x) } }
impl TryInto<cumulus_pallet_parachain_system::Event::<Runtime>> for Event {
    type Error = ();
    fn try_into(self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<cumulus_pallet_parachain_system::Event::<Runtime>, Self::Error> {
        match self {
            Self::ParachainSystem(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_balances::Event::<Runtime>> for Event { fn from(x: pallet_balances::Event::<Runtime>) -> Self { Event::Balances(x) } }
impl TryInto<pallet_balances::Event::<Runtime>> for Event {
    type Error = ();
    fn try_into(self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<pallet_balances::Event::<Runtime>, Self::Error> {
        match self {
            Self::Balances(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_collator_selection::Event::<Runtime>> for Event { fn from(x: pallet_collator_selection::Event::<Runtime>) -> Self { Event::CollatorSelection(x) } }
impl TryInto<pallet_collator_selection::Event::<Runtime>> for Event {
    type Error = ();
    fn try_into(self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<pallet_collator_selection::Event::<Runtime>, Self::Error> {
        match self {
            Self::CollatorSelection(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_session::Event> for Event { fn from(x: pallet_session::Event) -> Self { Event::Session(x) } }
impl TryInto<pallet_session::Event> for Event {
    type Error = ();
    fn try_into(self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<pallet_session::Event, Self::Error> {
        match self {
            Self::Session(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<cumulus_pallet_xcmp_queue::Event::<Runtime>> for Event { fn from(x: cumulus_pallet_xcmp_queue::Event::<Runtime>) -> Self { Event::XcmpQueue(x) } }
impl TryInto<cumulus_pallet_xcmp_queue::Event::<Runtime>> for Event {
    type Error = ();
    fn try_into(self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<cumulus_pallet_xcmp_queue::Event::<Runtime>, Self::Error> {
        match self {
            Self::XcmpQueue(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_xcm::Event::<Runtime>> for Event { fn from(x: pallet_xcm::Event::<Runtime>) -> Self { Event::PolkadotXcm(x) } }
impl TryInto<pallet_xcm::Event::<Runtime>> for Event {
    type Error = ();
    fn try_into(self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<pallet_xcm::Event::<Runtime>, Self::Error> {
        match self {
            Self::PolkadotXcm(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<cumulus_pallet_xcm::Event::<Runtime>> for Event { fn from(x: cumulus_pallet_xcm::Event::<Runtime>) -> Self { Event::CumulusXcm(x) } }
impl TryInto<cumulus_pallet_xcm::Event::<Runtime>> for Event {
    type Error = ();
    fn try_into(self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<cumulus_pallet_xcm::Event::<Runtime>, Self::Error> {
        match self {
            Self::CumulusXcm(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<cumulus_pallet_dmp_queue::Event::<Runtime>> for Event { fn from(x: cumulus_pallet_dmp_queue::Event::<Runtime>) -> Self { Event::DmpQueue(x) } }
impl TryInto<cumulus_pallet_dmp_queue::Event::<Runtime>> for Event {
    type Error = ();
    fn try_into(self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<cumulus_pallet_dmp_queue::Event::<Runtime>, Self::Error> {
        match self {
            Self::DmpQueue(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_template::Event::<Runtime>> for Event { fn from(x: pallet_template::Event::<Runtime>) -> Self { Event::TemplatePallet(x) } }
impl TryInto<pallet_template::Event::<Runtime>> for Event {
    type Error = ();
    fn try_into(self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<pallet_template::Event::<Runtime>, Self::Error> {
        match self {
            Self::TemplatePallet(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<proxy_pallet::Event::<Runtime>> for Event { fn from(x: proxy_pallet::Event::<Runtime>) -> Self { Event::ProxyPallet(x) } }
impl TryInto<proxy_pallet::Event::<Runtime>> for Event {
    type Error = ();
    fn try_into(self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<proxy_pallet::Event::<Runtime>, Self::Error> {
        match self {
            Self::ProxyPallet(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
#[doc = r" The runtime origin type representing the origin of a call."]
#[doc = r""]
#[doc = " Origin is always created with the base filter configured in [`frame_system::Config::BaseCallFilter`]."]
#[derive(Clone)]
pub struct Origin {
    caller: OriginCaller,
    filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc<Box<dyn Fn(&<Runtime as frame_system::Config>::Call) -> bool>>,
}
#[cfg(not(feature = "std"))]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Debug for Origin { fn fmt(&self, fmt: &mut self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Formatter ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<(), self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Error> { fmt.write_str("<wasm:stripped>") } }
#[cfg(feature = "std")]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Debug for Origin { fn fmt(&self, fmt: &mut self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Formatter ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<(), self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Error> { fmt.debug_struct("Origin").field("caller", &self.caller).field("filter", &"[function ptr]").finish() } }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait for Origin {
    type Call = <Runtime as frame_system::Config>::Call;
    type PalletsOrigin = OriginCaller;
    type AccountId = <Runtime as frame_system::Config>::AccountId;
    fn add_filter(&mut self, filter: impl Fn(&Self::Call) -> bool + 'static) {
        let f = self.filter.clone();
        self.filter = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(Box::new(move |call| { f(call) && filter(call) }));
    }
    fn reset_filter(&mut self) {
        let filter = <<Runtime as frame_system::Config>::BaseCallFilter as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Contains<<Runtime as frame_system::Config>::Call>>::contains;
        self.filter = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(Box::new(filter));
    }
    fn set_caller_from(&mut self, other: impl Into<Self>) { self.caller = other.into().caller; }
    fn filter_call(&self, call: &Self::Call) -> bool {
        match self.caller {
            OriginCaller::system(frame_system::Origin::<Runtime>::Root) => true,
            _ => (self.filter)(call),
        }
    }
    fn caller(&self) -> &Self::PalletsOrigin { &self.caller }
    fn try_with_caller<R>(mut self, f: impl FnOnce(Self::PalletsOrigin) -> Result<R, Self::PalletsOrigin> ) -> Result<R, Self> {
        match f(self.caller) {
            Ok(r) => Ok(r),
            Err(caller) => {
                self.caller = caller;
                Err(self)
            }
        }
    }
    fn none() -> Self { frame_system::RawOrigin::None.into() }
    fn root() -> Self { frame_system::RawOrigin::Root.into() }
    fn signed(by: Self::AccountId) -> Self { frame_system::RawOrigin::Signed(by).into() }
    fn as_signed(self) -> Option<Self::AccountId> {
        match self.caller {
            OriginCaller::system(frame_system::RawOrigin::Signed(by)) => Some(by),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, self::sp_api_hidden_includes_construct_runtime::hidden_include::RuntimeDebug, self::sp_api_hidden_includes_construct_runtime::hidden_include::codec::Encode, self::sp_api_hidden_includes_construct_runtime::hidden_include::codec::Decode, self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::TypeInfo, )]
#[allow(non_camel_case_types)]
pub enum OriginCaller { #[codec(index = 0u8)] system(frame_system::Origin<Runtime>), #[codec(index = 31u8)] PolkadotXcm(pallet_xcm::Origin), #[codec(index = 32u8)] CumulusXcm(cumulus_pallet_xcm::Origin), #[allow(dead_code)] Void(self::sp_api_hidden_includes_construct_runtime::hidden_include::Void) }
#[allow(dead_code)]
impl Origin {
    #[doc = " Create with system none origin and [`frame_system::Config::BaseCallFilter`]."]
    pub fn none() -> Self { <Origin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::none() }
    #[doc = " Create with system root origin and [`frame_system::Config::BaseCallFilter`]."]
    pub fn root() -> Self { <Origin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::root() }
    #[doc = " Create with system signed origin and [`frame_system::Config::BaseCallFilter`]."]
    pub fn signed(by: <Runtime as frame_system::Config>::AccountId) -> Self { <Origin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::signed(by) }
}
impl From<frame_system::Origin<Runtime>> for OriginCaller { fn from(x: frame_system::Origin<Runtime>) -> Self { OriginCaller::system(x) } }
impl TryFrom<OriginCaller> for frame_system::Origin<Runtime> {
    type Error = OriginCaller;
    fn try_from(x: OriginCaller) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<frame_system::Origin<Runtime>, OriginCaller> { if let OriginCaller::system(l) = x { Ok(l) } else { Err(x) } }
}
impl From<frame_system::Origin<Runtime>> for Origin {
    #[doc = " Convert to runtime origin, using as filter: [`frame_system::Config::BaseCallFilter`]."]
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        let o: OriginCaller = x.into();
        o.into()
    }
}
impl From<OriginCaller> for Origin {
    fn from(x: OriginCaller) -> Self {
        let mut o = Origin { caller: x, filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(Box::new(|_| true)) };
        self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait::reset_filter(&mut o);
        o
    }
}
impl From<Origin> for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<frame_system::Origin<Runtime>, Origin> {
    #[doc = r" NOTE: converting to pallet origin loses the origin filter information."]
    fn from(val: Origin) -> Self { if let OriginCaller::system(l) = val.caller { Ok(l) } else { Err(val) } }
}
impl From<Option<<Runtime as frame_system::Config>::AccountId>> for Origin {
    #[doc = " Convert to runtime origin with caller being system signed or none and use filter [`frame_system::Config::BaseCallFilter`]."]
    fn from(x: Option<<Runtime as frame_system::Config>::AccountId>) -> Self { <frame_system::Origin<Runtime>>::from(x).into() }
}
impl From<pallet_xcm::Origin> for OriginCaller { fn from(x: pallet_xcm::Origin) -> Self { OriginCaller::PolkadotXcm(x) } }
impl From<pallet_xcm::Origin> for Origin {
    #[doc = "  Convert to runtime origin using [`pallet_xcm::Config::BaseCallFilter`]."]
    fn from(x: pallet_xcm::Origin) -> Self {
        let x: OriginCaller = x.into();
        x.into()
    }
}
impl From<Origin> for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<pallet_xcm::Origin, Origin> {
    #[doc = r" NOTE: converting to pallet origin loses the origin filter information."]
    fn from(val: Origin) -> Self { if let OriginCaller::PolkadotXcm(l) = val.caller { Ok(l) } else { Err(val) } }
}
impl TryFrom<OriginCaller> for pallet_xcm::Origin {
    type Error = OriginCaller;
    fn try_from(x: OriginCaller ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<pallet_xcm::Origin, OriginCaller> { if let OriginCaller::PolkadotXcm(l) = x { Ok(l) } else { Err(x) } }
}
impl From<cumulus_pallet_xcm::Origin> for OriginCaller { fn from(x: cumulus_pallet_xcm::Origin) -> Self { OriginCaller::CumulusXcm(x) } }
impl From<cumulus_pallet_xcm::Origin> for Origin {
    #[doc = "  Convert to runtime origin using [`cumulus_pallet_xcm::Config::BaseCallFilter`]."]
    fn from(x: cumulus_pallet_xcm::Origin) -> Self {
        let x: OriginCaller = x.into();
        x.into()
    }
}
impl From<Origin> for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<cumulus_pallet_xcm::Origin, Origin> {
    #[doc = r" NOTE: converting to pallet origin loses the origin filter information."]
    fn from(val: Origin) -> Self { if let OriginCaller::CumulusXcm(l) = val.caller { Ok(l) } else { Err(val) } }
}
impl TryFrom<OriginCaller> for cumulus_pallet_xcm::Origin {
    type Error = OriginCaller;
    fn try_from(x: OriginCaller ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<cumulus_pallet_xcm::Origin, OriginCaller> { if let OriginCaller::CumulusXcm(l) = x { Ok(l) } else { Err(x) } }
}
pub type System = frame_system::Pallet<Runtime>;
pub type ParachainSystem = cumulus_pallet_parachain_system::Pallet<Runtime>;
pub type Timestamp = pallet_timestamp::Pallet<Runtime>;
pub type ParachainInfo = parachain_info::Pallet<Runtime>;
pub type Balances = pallet_balances::Pallet<Runtime>;
pub type TransactionPayment = pallet_transaction_payment::Pallet<Runtime>;
pub type Authorship = pallet_authorship::Pallet<Runtime>;
pub type CollatorSelection = pallet_collator_selection::Pallet<Runtime>;
pub type Session = pallet_session::Pallet<Runtime>;
pub type Aura = pallet_aura::Pallet<Runtime>;
pub type AuraExt = cumulus_pallet_aura_ext::Pallet<Runtime>;
pub type XcmpQueue = cumulus_pallet_xcmp_queue::Pallet<Runtime>;
pub type PolkadotXcm = pallet_xcm::Pallet<Runtime>;
pub type CumulusXcm = cumulus_pallet_xcm::Pallet<Runtime>;
pub type DmpQueue = cumulus_pallet_dmp_queue::Pallet<Runtime>;
pub type TemplatePallet = pallet_template::Pallet<Runtime>;
pub type ProxyPallet = proxy_pallet::Pallet<Runtime>;
#[doc = r" All pallets included in the runtime as a nested tuple of types."]
#[deprecated(note = "The type definition has changed from representing all pallets \
			excluding system, in reversed order to become the representation of all pallets \
			including system pallet in regular order. For this reason it is encouraged to use \
			explicitly one of `AllPalletsWithSystem`, `AllPalletsWithoutSystem`, \
			`AllPalletsWithSystemReversed`, `AllPalletsWithoutSystemReversed`. \
			Note that the type `frame_executive::Executive` expects one of `AllPalletsWithSystem` \
			, `AllPalletsWithSystemReversed`, `AllPalletsReversedWithSystemFirst`. More details in \
			https://github.com/paritytech/substrate/pull/10043")]
pub type AllPallets = AllPalletsWithSystem;
#[doc = r" All pallets included in the runtime as a nested tuple of types."]
pub type AllPalletsWithSystem = ( (System, (ParachainSystem, (Timestamp, (ParachainInfo, (Balances, (TransactionPayment, (Authorship, (CollatorSelection, (Session, (Aura, (AuraExt, (XcmpQueue, (PolkadotXcm, (CumulusXcm, (DmpQueue, (TemplatePallet, (ProxyPallet, ))))))))))))))))) );
#[doc = r" All pallets included in the runtime as a nested tuple of types."]
#[doc = r" Excludes the System pallet."]
pub type AllPalletsWithoutSystem = ( (ParachainSystem, (Timestamp, (ParachainInfo, (Balances, (TransactionPayment, (Authorship, (CollatorSelection, (Session, (Aura, (AuraExt, (XcmpQueue, (PolkadotXcm, (CumulusXcm, (DmpQueue, (TemplatePallet, (ProxyPallet, )))))))))))))))) );
#[doc = r" All pallets included in the runtime as a nested tuple of types in reversed order."]
#[doc = r" Excludes the System pallet."]
pub type AllPalletsWithoutSystemReversed = ( (ProxyPallet, (TemplatePallet, (DmpQueue, (CumulusXcm, (PolkadotXcm, (XcmpQueue, (AuraExt, (Aura, (Session, (CollatorSelection, (Authorship, (TransactionPayment, (Balances, (ParachainInfo, (Timestamp, (ParachainSystem, )))))))))))))))) );
#[doc = r" All pallets included in the runtime as a nested tuple of types in reversed order."]
pub type AllPalletsWithSystemReversed = ( (ProxyPallet, (TemplatePallet, (DmpQueue, (CumulusXcm, (PolkadotXcm, (XcmpQueue, (AuraExt, (Aura, (Session, (CollatorSelection, (Authorship, (TransactionPayment, (Balances, (ParachainInfo, (Timestamp, (ParachainSystem, (System, ))))))))))))))))) );
#[doc = r" All pallets included in the runtime as a nested tuple of types in reversed order."]
#[doc = r" With the system pallet first."]
pub type AllPalletsReversedWithSystemFirst = (System, AllPalletsWithoutSystemReversed);
#[doc = r" Provides an implementation of `PalletInfo` to provide information"]
#[doc = r" about the pallet setup in the runtime."]
pub struct PalletInfo;
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfo for PalletInfo {
    fn index<P: 'static>() -> Option<usize> {
        let type_id = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<P>();
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<System>() { return Some(0usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<ParachainSystem>() { return Some(1usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Timestamp>() { return Some(2usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<ParachainInfo>() { return Some(3usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Balances>() { return Some(10usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<TransactionPayment>() { return Some(11usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Authorship>() { return Some(20usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<CollatorSelection>() { return Some(21usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Session>() { return Some(22usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Aura>() { return Some(23usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<AuraExt>() { return Some(24usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<XcmpQueue>() { return Some(30usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<PolkadotXcm>() { return Some(31usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<CumulusXcm>() { return Some(32usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<DmpQueue>() { return Some(33usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<TemplatePallet>() { return Some(40usize); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<ProxyPallet>() { return Some(41usize); }
        None
    }
    fn name<P: 'static>() -> Option<&'static str> {
        let type_id = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<P>();
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<System>() { return Some("System"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<ParachainSystem>() { return Some("ParachainSystem"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Timestamp>() { return Some("Timestamp"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<ParachainInfo>() { return Some("ParachainInfo"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Balances>() { return Some("Balances"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<TransactionPayment>() { return Some("TransactionPayment"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Authorship>() { return Some("Authorship"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<CollatorSelection>() { return Some("CollatorSelection"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Session>() { return Some("Session"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Aura>() { return Some("Aura"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<AuraExt>() { return Some("AuraExt"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<XcmpQueue>() { return Some("XcmpQueue"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<PolkadotXcm>() { return Some("PolkadotXcm"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<CumulusXcm>() { return Some("CumulusXcm"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<DmpQueue>() { return Some("DmpQueue"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<TemplatePallet>() { return Some("TemplatePallet"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<ProxyPallet>() { return Some("ProxyPallet"); }
        None
    }
    fn module_name<P: 'static>() -> Option<&'static str> {
        let type_id = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<P>();
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<System>() { return Some("frame_system"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<ParachainSystem>() { return Some("cumulus_pallet_parachain_system"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Timestamp>() { return Some("pallet_timestamp"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<ParachainInfo>() { return Some("parachain_info"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Balances>() { return Some("pallet_balances"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<TransactionPayment>() { return Some("pallet_transaction_payment"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Authorship>() { return Some("pallet_authorship"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<CollatorSelection>() { return Some("pallet_collator_selection"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Session>() { return Some("pallet_session"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Aura>() { return Some("pallet_aura"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<AuraExt>() { return Some("cumulus_pallet_aura_ext"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<XcmpQueue>() { return Some("cumulus_pallet_xcmp_queue"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<PolkadotXcm>() { return Some("pallet_xcm"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<CumulusXcm>() { return Some("cumulus_pallet_xcm"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<DmpQueue>() { return Some("cumulus_pallet_dmp_queue"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<TemplatePallet>() { return Some("pallet_template"); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<ProxyPallet>() { return Some("proxy_pallet"); }
        None
    }
    fn crate_version<P: 'static>() -> Option<self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::CrateVersion> {
        let type_id = self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<P>();
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<System>() { return Some(<frame_system::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<ParachainSystem>() { return Some(<cumulus_pallet_parachain_system::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Timestamp>() { return Some(<pallet_timestamp::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<ParachainInfo>() { return Some(<parachain_info::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Balances>() { return Some(<pallet_balances::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<TransactionPayment>() { return Some(<pallet_transaction_payment::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Authorship>() { return Some(<pallet_authorship::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<CollatorSelection>() { return Some(<pallet_collator_selection::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Session>() { return Some(<pallet_session::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<Aura>() { return Some(<pallet_aura::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<AuraExt>() { return Some(<cumulus_pallet_aura_ext::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<XcmpQueue>() { return Some(<cumulus_pallet_xcmp_queue::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<PolkadotXcm>() { return Some(<pallet_xcm::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<CumulusXcm>() { return Some(<cumulus_pallet_xcm::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<DmpQueue>() { return Some(<cumulus_pallet_dmp_queue::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<TemplatePallet>() { return Some(<pallet_template::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        if type_id == self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<ProxyPallet>() { return Some(<proxy_pallet::Pallet<Runtime> as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfoAccess>::crate_version()); }
        None
    }
}
#[derive(Clone, PartialEq, Eq, self::sp_api_hidden_includes_construct_runtime::hidden_include::codec::Encode, self::sp_api_hidden_includes_construct_runtime::hidden_include::codec::Decode, self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::TypeInfo, self::sp_api_hidden_includes_construct_runtime::hidden_include::RuntimeDebug, )]
pub enum Call {
    #[codec(index = 0u8)] System(self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<System, Runtime>),
    #[codec(index = 1u8)] ParachainSystem(self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<ParachainSystem, Runtime>),
    #[codec(index = 2u8)] Timestamp(self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Timestamp, Runtime>),
    #[codec(index = 10u8)] Balances(self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Balances, Runtime>),
    #[codec(index = 20u8)] Authorship(self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Authorship, Runtime>),
    #[codec(index = 21u8)] CollatorSelection(self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<CollatorSelection, Runtime>),
    #[codec(index = 22u8)] Session(self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Session, Runtime>),
    #[codec(index = 30u8)] XcmpQueue(self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<XcmpQueue, Runtime>),
    #[codec(index = 31u8)] PolkadotXcm(self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<PolkadotXcm, Runtime>),
    #[codec(index = 33u8)] DmpQueue(self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<DmpQueue, Runtime>),
    #[codec(index = 40u8)] TemplatePallet(self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<TemplatePallet, Runtime>),
    #[codec(index = 41u8)] ProxyPallet(self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<ProxyPallet, Runtime>),
}
#[cfg(test)]
impl Call {
    #[doc = r" Return a list of the module names together with their size in memory."]
    pub const fn sizes() -> &'static [(&'static str, usize)] {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::Callable;
        use core::mem::size_of;
        &[(stringify!(System ), size_of::<<System as Callable<Runtime>>::Call>(), ), (stringify!(ParachainSystem ), size_of::<<ParachainSystem as Callable<Runtime>>::Call>(), ), (stringify!(Timestamp ), size_of::<<Timestamp as Callable<Runtime>>::Call>(), ), (stringify!(Balances ), size_of::<<Balances as Callable<Runtime>>::Call>(), ), (stringify!(Authorship ), size_of::<<Authorship as Callable<Runtime>>::Call>(), ), (stringify!(CollatorSelection ), size_of::<<CollatorSelection as Callable<Runtime>>::Call>(), ), (stringify!(Session ), size_of::<<Session as Callable<Runtime>>::Call>(), ), (stringify!(XcmpQueue ), size_of::<<XcmpQueue as Callable<Runtime>>::Call>(), ), (stringify!(PolkadotXcm ), size_of::<<PolkadotXcm as Callable<Runtime>>::Call>(), ), (stringify!(DmpQueue ), size_of::<<DmpQueue as Callable<Runtime>>::Call>(), ), (stringify!(TemplatePallet ), size_of::<<TemplatePallet as Callable<Runtime>>::Call>(), ), (stringify!(ProxyPallet ), size_of::<<ProxyPallet as Callable<Runtime>>::Call>(), ), ]
    }
    #[doc = r" Panics with diagnostic information if the size is greater than the given `limit`."]
    pub fn assert_size_under(limit: usize) {
        let size = core::mem::size_of::<Self>();
        let call_oversize = size > limit;
        if call_oversize {
            {
                ::std::io::_print(IntellijRustDollarCrate::format_args_nl!("Size of `Call` is {} bytes (provided limit is {} bytes)", size, limit));
            };
            let mut sizes = Self::sizes().to_vec();
            sizes.sort_by_key(|x| -(x.1 as isize));
            for (i, &(name, size)) in sizes.iter().enumerate().take(5) {
                {
                    ::std::io::_print(IntellijRustDollarCrate::format_args_nl!("Offender #{}: {} at {} bytes", i + 1, name, size));
                };
            }
            if let Some((_, next_size)) = sizes.get(5) {
                {
                    ::std::io::_print(IntellijRustDollarCrate::format_args_nl!("{} others of size {} bytes or less", sizes.len() - 5, next_size));
                };
            }
            panic!("Size of `Call` is more than limit; use `Box` on complex parameter types to reduce the
						size of `Call`.
						If the limit is too strong, maybe consider providing a higher limit.");
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetDispatchInfo for Call {
    fn get_dispatch_info(&self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchInfo {
        match self {
            Call::System(call) => call.get_dispatch_info(),
            Call::ParachainSystem(call) => call.get_dispatch_info(),
            Call::Timestamp(call) => call.get_dispatch_info(),
            Call::Balances(call) => call.get_dispatch_info(),
            Call::Authorship(call) => call.get_dispatch_info(),
            Call::CollatorSelection(call) => call.get_dispatch_info(),
            Call::Session(call) => call.get_dispatch_info(),
            Call::XcmpQueue(call) => call.get_dispatch_info(),
            Call::PolkadotXcm(call) => call.get_dispatch_info(),
            Call::DmpQueue(call) => call.get_dispatch_info(),
            Call::TemplatePallet(call) => call.get_dispatch_info(),
            Call::ProxyPallet(call) => call.get_dispatch_info(),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallMetadata for Call {
    fn get_call_metadata(&self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallName;
        match self {
            Call::System(call) => {
                let function_name = call.get_call_name();
                let pallet_name = stringify!(System );
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata { function_name, pallet_name }
            }
            Call::ParachainSystem(call) => {
                let function_name = call.get_call_name();
                let pallet_name = stringify!(ParachainSystem );
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata { function_name, pallet_name }
            }
            Call::Timestamp(call) => {
                let function_name = call.get_call_name();
                let pallet_name = stringify!(Timestamp );
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata { function_name, pallet_name }
            }
            Call::Balances(call) => {
                let function_name = call.get_call_name();
                let pallet_name = stringify!(Balances );
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata { function_name, pallet_name }
            }
            Call::Authorship(call) => {
                let function_name = call.get_call_name();
                let pallet_name = stringify!(Authorship );
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata { function_name, pallet_name }
            }
            Call::CollatorSelection(call) => {
                let function_name = call.get_call_name();
                let pallet_name = stringify!(CollatorSelection );
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata { function_name, pallet_name }
            }
            Call::Session(call) => {
                let function_name = call.get_call_name();
                let pallet_name = stringify!(Session );
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata { function_name, pallet_name }
            }
            Call::XcmpQueue(call) => {
                let function_name = call.get_call_name();
                let pallet_name = stringify!(XcmpQueue );
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata { function_name, pallet_name }
            }
            Call::PolkadotXcm(call) => {
                let function_name = call.get_call_name();
                let pallet_name = stringify!(PolkadotXcm );
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata { function_name, pallet_name }
            }
            Call::DmpQueue(call) => {
                let function_name = call.get_call_name();
                let pallet_name = stringify!(DmpQueue );
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata { function_name, pallet_name }
            }
            Call::TemplatePallet(call) => {
                let function_name = call.get_call_name();
                let pallet_name = stringify!(TemplatePallet );
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata { function_name, pallet_name }
            }
            Call::ProxyPallet(call) => {
                let function_name = call.get_call_name();
                let pallet_name = stringify!(ProxyPallet );
                self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata { function_name, pallet_name }
            }
        }
    }
    fn get_module_names() -> &'static [&'static str] { &[stringify!(System ), stringify!(ParachainSystem ), stringify!(Timestamp ), stringify!(Balances ), stringify!(Authorship ), stringify!(CollatorSelection ), stringify!(Session ), stringify!(XcmpQueue ), stringify!(PolkadotXcm ), stringify!(DmpQueue ), stringify!(TemplatePallet ), stringify!(ProxyPallet ), ] }
    fn get_call_names(module: &str) -> &'static [&'static str] {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::{Callable, GetCallName};
        match module {
            stringify!(System ) => <<System as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            stringify!(ParachainSystem ) => <<ParachainSystem as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            stringify!(Timestamp ) => <<Timestamp as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            stringify!(Balances ) => <<Balances as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            stringify!(Authorship ) => <<Authorship as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            stringify!(CollatorSelection ) => <<CollatorSelection as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            stringify!(Session ) => <<Session as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            stringify!(XcmpQueue ) => <<XcmpQueue as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            stringify!(PolkadotXcm ) => <<PolkadotXcm as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            stringify!(DmpQueue ) => <<DmpQueue as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            stringify!(TemplatePallet ) => <<TemplatePallet as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            stringify!(ProxyPallet ) => <<ProxyPallet as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            _ => unreachable!(),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::Dispatchable for Call {
    type Origin = Origin;
    type Config = Call;
    type Info = self::sp_api_hidden_includes_construct_runtime::hidden_include::weights::DispatchInfo;
    type PostInfo = self::sp_api_hidden_includes_construct_runtime::hidden_include::weights::PostDispatchInfo;
    fn dispatch(self, origin: Origin) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchResultWithPostInfo {
        if !<Self::Origin as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait>::filter_call(&origin, &self) { return self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result::Err(frame_system::Error::<Runtime>::CallFiltered.into()); }
        self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(self, origin)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable for Call {
    type Origin = Origin;
    fn dispatch_bypass_filter(self, origin: Origin) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchResultWithPostInfo {
        match self {
            Call::System(call) => self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(call, origin),
            Call::ParachainSystem(call) => self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(call, origin),
            Call::Timestamp(call) => self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(call, origin),
            Call::Balances(call) => self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(call, origin),
            Call::Authorship(call) => self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(call, origin),
            Call::CollatorSelection(call) => self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(call, origin),
            Call::Session(call) => self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(call, origin),
            Call::XcmpQueue(call) => self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(call, origin),
            Call::PolkadotXcm(call) => self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(call, origin),
            Call::DmpQueue(call) => self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(call, origin),
            Call::TemplatePallet(call) => self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(call, origin),
            Call::ProxyPallet(call) => self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(call, origin),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::DispatchableWithStorageLayer for Call {
    type Origin = Origin;
    fn dispatch_with_storage_layer(self, origin: Origin) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchResultWithPostInfo { self::sp_api_hidden_includes_construct_runtime::hidden_include::storage::with_storage_layer(|| { self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::Dispatchable::dispatch(self, origin) }) }
    fn dispatch_bypass_filter_with_storage_layer(self, origin: Origin) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchResultWithPostInfo { self::sp_api_hidden_includes_construct_runtime::hidden_include::storage::with_storage_layer(|| { self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable::dispatch_bypass_filter(self, origin) }) }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<System, Runtime>> for Call {
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<System, Runtime>> {
        match self {
            Call::System(call) => Some(call),
            _ => None,
        }
    }
}
impl From<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<System, Runtime>> for Call { fn from(call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<System, Runtime>) -> Self { Call::System(call) } }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<ParachainSystem, Runtime>> for Call {
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<ParachainSystem, Runtime>> {
        match self {
            Call::ParachainSystem(call) => Some(call),
            _ => None,
        }
    }
}
impl From<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<ParachainSystem, Runtime>> for Call { fn from(call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<ParachainSystem, Runtime>) -> Self { Call::ParachainSystem(call) } }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Timestamp, Runtime>> for Call {
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Timestamp, Runtime>> {
        match self {
            Call::Timestamp(call) => Some(call),
            _ => None,
        }
    }
}
impl From<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Timestamp, Runtime>> for Call { fn from(call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Timestamp, Runtime>) -> Self { Call::Timestamp(call) } }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Balances, Runtime>> for Call {
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Balances, Runtime>> {
        match self {
            Call::Balances(call) => Some(call),
            _ => None,
        }
    }
}
impl From<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Balances, Runtime>> for Call { fn from(call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Balances, Runtime>) -> Self { Call::Balances(call) } }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Authorship, Runtime>> for Call {
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Authorship, Runtime>> {
        match self {
            Call::Authorship(call) => Some(call),
            _ => None,
        }
    }
}
impl From<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Authorship, Runtime>> for Call { fn from(call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Authorship, Runtime>) -> Self { Call::Authorship(call) } }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<CollatorSelection, Runtime>> for Call {
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<CollatorSelection, Runtime>> {
        match self {
            Call::CollatorSelection(call) => Some(call),
            _ => None,
        }
    }
}
impl From<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<CollatorSelection, Runtime>> for Call { fn from(call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<CollatorSelection, Runtime>) -> Self { Call::CollatorSelection(call) } }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Session, Runtime>> for Call {
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Session, Runtime>> {
        match self {
            Call::Session(call) => Some(call),
            _ => None,
        }
    }
}
impl From<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Session, Runtime>> for Call { fn from(call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<Session, Runtime>) -> Self { Call::Session(call) } }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<XcmpQueue, Runtime>> for Call {
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<XcmpQueue, Runtime>> {
        match self {
            Call::XcmpQueue(call) => Some(call),
            _ => None,
        }
    }
}
impl From<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<XcmpQueue, Runtime>> for Call { fn from(call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<XcmpQueue, Runtime>) -> Self { Call::XcmpQueue(call) } }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<PolkadotXcm, Runtime>> for Call {
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<PolkadotXcm, Runtime>> {
        match self {
            Call::PolkadotXcm(call) => Some(call),
            _ => None,
        }
    }
}
impl From<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<PolkadotXcm, Runtime>> for Call { fn from(call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<PolkadotXcm, Runtime>) -> Self { Call::PolkadotXcm(call) } }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<DmpQueue, Runtime>> for Call {
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<DmpQueue, Runtime>> {
        match self {
            Call::DmpQueue(call) => Some(call),
            _ => None,
        }
    }
}
impl From<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<DmpQueue, Runtime>> for Call { fn from(call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<DmpQueue, Runtime>) -> Self { Call::DmpQueue(call) } }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<TemplatePallet, Runtime>> for Call {
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<TemplatePallet, Runtime>> {
        match self {
            Call::TemplatePallet(call) => Some(call),
            _ => None,
        }
    }
}
impl From<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<TemplatePallet, Runtime>> for Call { fn from(call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<TemplatePallet, Runtime>) -> Self { Call::TemplatePallet(call) } }
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<ProxyPallet, Runtime>> for Call {
    #[allow(unreachable_patterns)]
    fn is_sub_type(&self) -> Option<&self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<ProxyPallet, Runtime>> {
        match self {
            Call::ProxyPallet(call) => Some(call),
            _ => None,
        }
    }
}
impl From<self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<ProxyPallet, Runtime>> for Call { fn from(call: self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<ProxyPallet, Runtime>) -> Self { Call::ProxyPallet(call) } }
impl Runtime {
    pub fn metadata() -> self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::RuntimeMetadataPrefixed {
        self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::RuntimeMetadataLastVersion::new((<[_]>::into_vec(
            #[rustc_box]
                ::alloc::boxed::Box::new([(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(System ), index: 0u8, storage: Some(frame_system::Pallet::<Runtime>::storage_metadata()), calls: Some(frame_system::Pallet::<Runtime>::call_functions()), event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata { ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<frame_system::Event::<Runtime>>() }), constants: frame_system::Pallet::<Runtime>::pallet_constants_metadata(), error: frame_system::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(ParachainSystem ), index: 1u8, storage: Some(cumulus_pallet_parachain_system::Pallet::<Runtime>::storage_metadata()), calls: Some(cumulus_pallet_parachain_system::Pallet::<Runtime>::call_functions()), event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata { ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<cumulus_pallet_parachain_system::Event::<Runtime>>() }), constants: cumulus_pallet_parachain_system::Pallet::<Runtime>::pallet_constants_metadata(), error: cumulus_pallet_parachain_system::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(Timestamp ), index: 2u8, storage: Some(pallet_timestamp::Pallet::<Runtime>::storage_metadata()), calls: Some(pallet_timestamp::Pallet::<Runtime>::call_functions()), event: None, constants: pallet_timestamp::Pallet::<Runtime>::pallet_constants_metadata(), error: pallet_timestamp::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(ParachainInfo ), index: 3u8, storage: Some(parachain_info::Pallet::<Runtime>::storage_metadata()), calls: None, event: None, constants: parachain_info::Pallet::<Runtime>::pallet_constants_metadata(), error: parachain_info::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(Balances ), index: 10u8, storage: Some(pallet_balances::Pallet::<Runtime>::storage_metadata()), calls: Some(pallet_balances::Pallet::<Runtime>::call_functions()), event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata { ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<pallet_balances::Event::<Runtime>>() }), constants: pallet_balances::Pallet::<Runtime>::pallet_constants_metadata(), error: pallet_balances::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(TransactionPayment ), index: 11u8, storage: Some(pallet_transaction_payment::Pallet::<Runtime>::storage_metadata()), calls: None, event: None, constants: pallet_transaction_payment::Pallet::<Runtime>::pallet_constants_metadata(), error: pallet_transaction_payment::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(Authorship ), index: 20u8, storage: Some(pallet_authorship::Pallet::<Runtime>::storage_metadata()), calls: Some(pallet_authorship::Pallet::<Runtime>::call_functions()), event: None, constants: pallet_authorship::Pallet::<Runtime>::pallet_constants_metadata(), error: pallet_authorship::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(CollatorSelection ), index: 21u8, storage: Some(pallet_collator_selection::Pallet::<Runtime>::storage_metadata()), calls: Some(pallet_collator_selection::Pallet::<Runtime>::call_functions()), event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata { ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<pallet_collator_selection::Event::<Runtime>>() }), constants: pallet_collator_selection::Pallet::<Runtime>::pallet_constants_metadata(), error: pallet_collator_selection::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(Session ), index: 22u8, storage: Some(pallet_session::Pallet::<Runtime>::storage_metadata()), calls: Some(pallet_session::Pallet::<Runtime>::call_functions()), event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata { ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<pallet_session::Event>() }), constants: pallet_session::Pallet::<Runtime>::pallet_constants_metadata(), error: pallet_session::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(Aura ), index: 23u8, storage: Some(pallet_aura::Pallet::<Runtime>::storage_metadata()), calls: None, event: None, constants: pallet_aura::Pallet::<Runtime>::pallet_constants_metadata(), error: pallet_aura::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(AuraExt ), index: 24u8, storage: Some(cumulus_pallet_aura_ext::Pallet::<Runtime>::storage_metadata()), calls: None, event: None, constants: cumulus_pallet_aura_ext::Pallet::<Runtime>::pallet_constants_metadata(), error: cumulus_pallet_aura_ext::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(XcmpQueue ), index: 30u8, storage: Some(cumulus_pallet_xcmp_queue::Pallet::<Runtime>::storage_metadata()), calls: Some(cumulus_pallet_xcmp_queue::Pallet::<Runtime>::call_functions()), event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata { ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<cumulus_pallet_xcmp_queue::Event::<Runtime>>() }), constants: cumulus_pallet_xcmp_queue::Pallet::<Runtime>::pallet_constants_metadata(), error: cumulus_pallet_xcmp_queue::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(PolkadotXcm ), index: 31u8, storage: None, calls: Some(pallet_xcm::Pallet::<Runtime>::call_functions()), event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata { ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<pallet_xcm::Event::<Runtime>>() }), constants: pallet_xcm::Pallet::<Runtime>::pallet_constants_metadata(), error: pallet_xcm::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(CumulusXcm ), index: 32u8, storage: None, calls: None, event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata { ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<cumulus_pallet_xcm::Event::<Runtime>>() }), constants: cumulus_pallet_xcm::Pallet::<Runtime>::pallet_constants_metadata(), error: cumulus_pallet_xcm::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(DmpQueue ), index: 33u8, storage: Some(cumulus_pallet_dmp_queue::Pallet::<Runtime>::storage_metadata()), calls: Some(cumulus_pallet_dmp_queue::Pallet::<Runtime>::call_functions()), event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata { ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<cumulus_pallet_dmp_queue::Event::<Runtime>>() }), constants: cumulus_pallet_dmp_queue::Pallet::<Runtime>::pallet_constants_metadata(), error: cumulus_pallet_dmp_queue::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(TemplatePallet ), index: 40u8, storage: Some(pallet_template::Pallet::<Runtime>::storage_metadata()), calls: Some(pallet_template::Pallet::<Runtime>::call_functions()), event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata { ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<pallet_template::Event::<Runtime>>() }), constants: pallet_template::Pallet::<Runtime>::pallet_constants_metadata(), error: pallet_template::Pallet::<Runtime>::error_metadata() }), (self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletMetadata { name: stringify!(ProxyPallet ), index: 41u8, storage: Some(proxy_pallet::Pallet::<Runtime>::storage_metadata()), calls: Some(proxy_pallet::Pallet::<Runtime>::call_functions()), event: Some(self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::PalletEventMetadata { ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<proxy_pallet::Event::<Runtime>>() }), constants: proxy_pallet::Pallet::<Runtime>::pallet_constants_metadata(), error: proxy_pallet::Pallet::<Runtime>::error_metadata() })])
        )), self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::ExtrinsicMetadata {
            ty: self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<UncheckedExtrinsic>(),
            version: <UncheckedExtrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::ExtrinsicMetadata>::VERSION,
            signed_extensions: <<UncheckedExtrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::ExtrinsicMetadata>::SignedExtensions as self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::SignedExtension>::metadata().into_iter().map(|meta| self::sp_api_hidden_includes_construct_runtime::hidden_include::metadata::SignedExtensionMetadata { identifier: meta.identifier, ty: meta.ty, additional_signed: meta.additional_signed }).collect(),
        }, self::sp_api_hidden_includes_construct_runtime::hidden_include::scale_info::meta_type::<Runtime>()).into()
    }
}
#[cfg(any(feature = "std", test))]
pub type SystemConfig = frame_system::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type ParachainSystemConfig = cumulus_pallet_parachain_system::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type ParachainInfoConfig = parachain_info::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type BalancesConfig = pallet_balances::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type CollatorSelectionConfig = pallet_collator_selection::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type SessionConfig = pallet_session::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type AuraConfig = pallet_aura::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type AuraExtConfig = cumulus_pallet_aura_ext::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type PolkadotXcmConfig = pallet_xcm::GenesisConfig;
#[cfg(any(feature = "std", test))]
use self::sp_api_hidden_includes_construct_runtime::hidden_include::serde   as __genesis_config_serde_import__;
#[cfg(any(feature = "std", test))]
#[derive(self::sp_api_hidden_includes_construct_runtime::hidden_include::serde::Serialize, self::sp_api_hidden_includes_construct_runtime::hidden_include::serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(crate = "__genesis_config_serde_import__")]
pub struct GenesisConfig {
    pub system: SystemConfig,
    pub parachain_system: ParachainSystemConfig,
    pub parachain_info: ParachainInfoConfig,
    pub balances: BalancesConfig,
    pub collator_selection: CollatorSelectionConfig,
    pub session: SessionConfig,
    pub aura: AuraConfig,
    pub aura_ext: AuraExtConfig,
    pub polkadot_xcm: PolkadotXcmConfig,
}
#[cfg(any(feature = "std", test))]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildStorage for GenesisConfig {
    fn assimilate_storage(&self, storage: &mut self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::Storage ) -> std::result::Result<(), String> {
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<Runtime, frame_system::__InherentHiddenInstance>::build_module_genesis_storage(&self.system, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<Runtime, cumulus_pallet_parachain_system::__InherentHiddenInstance>::build_module_genesis_storage(&self.parachain_system, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<Runtime, parachain_info::__InherentHiddenInstance>::build_module_genesis_storage(&self.parachain_info, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<Runtime, pallet_balances::__InherentHiddenInstance>::build_module_genesis_storage(&self.balances, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<Runtime, pallet_collator_selection::__InherentHiddenInstance>::build_module_genesis_storage(&self.collator_selection, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<Runtime, pallet_session::__InherentHiddenInstance>::build_module_genesis_storage(&self.session, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<Runtime, pallet_aura::__InherentHiddenInstance>::build_module_genesis_storage(&self.aura, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<Runtime, cumulus_pallet_aura_ext::__InherentHiddenInstance>::build_module_genesis_storage(&self.aura_ext, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildModuleGenesisStorage::<Runtime, pallet_xcm::__InherentHiddenInstance>::build_module_genesis_storage(&self.polkadot_xcm, storage)?;
        self::sp_api_hidden_includes_construct_runtime::hidden_include::BasicExternalities::execute_with_storage(storage, || { <AllPalletsWithSystem as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OnGenesis>::on_genesis(); });
        Ok(())
    }
}
trait InherentDataExt {
    fn create_extrinsics(&self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Vec<<Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::BlockT>::Extrinsic>;
    fn check_extrinsics(&self, block: &Block) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::CheckInherentsResult;
}
impl InherentDataExt for self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::InherentData {
    fn create_extrinsics(&self) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Vec<<Block as self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::BlockT>::Extrinsic> {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
        let mut inherents = Vec::new();
        if let Some(inherent) = ParachainSystem::create_inherent(self) {
            let inherent = <UncheckedExtrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Extrinsic>::new(inherent.into(), None ).expect("Runtime UncheckedExtrinsic is not Opaque, so it has to return \
							`Some`; qed");
            inherents.push(inherent);
        }
        if let Some(inherent) = Timestamp::create_inherent(self) {
            let inherent = <UncheckedExtrinsic as self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Extrinsic>::new(inherent.into(), None ).expect("Runtime UncheckedExtrinsic is not Opaque, so it has to return \
							`Some`; qed");
            inherents.push(inherent);
        }
        inherents
    }
    fn check_extrinsics(&self, block: &Block) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::CheckInherentsResult {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::{ProvideInherent, IsFatalError};
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{IsSubType, ExtrinsicCall};
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block   as _;
        let mut result = self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::CheckInherentsResult::new();
        for xt in block.extrinsics() {
            if self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Extrinsic::is_signed(xt).unwrap_or(false) { break; }
            let mut is_inherent = false;
            {
                let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                if let Some(call) = IsSubType::<_>::is_sub_type(call) {
                    if ParachainSystem::is_inherent(call) {
                        is_inherent = true;
                        if let Err(e) = ParachainSystem::check_inherent(call, self) {
                            result.put_error(ParachainSystem::INHERENT_IDENTIFIER, &e).expect("There is only one fatal error; qed");
                            if e.is_fatal_error() { return result; }
                        }
                    }
                }
            }
            {
                let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                if let Some(call) = IsSubType::<_>::is_sub_type(call) {
                    if Timestamp::is_inherent(call) {
                        is_inherent = true;
                        if let Err(e) = Timestamp::check_inherent(call, self) {
                            result.put_error(Timestamp::INHERENT_IDENTIFIER, &e).expect("There is only one fatal error; qed");
                            if e.is_fatal_error() { return result; }
                        }
                    }
                }
            }
            if !is_inherent { break; }
        }
        match ParachainSystem::is_inherent_required(self) {
            Ok(Some(e)) => {
                let found = block.extrinsics().iter().any(|xt| {
                    let is_signed = self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Extrinsic::is_signed(xt).unwrap_or(false);
                    if !is_signed {
                        let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                        if let Some(call) = IsSubType::<_>::is_sub_type(call) { ParachainSystem::is_inherent(&call) } else { false }
                    } else { false }
                });
                if !found {
                    result.put_error(ParachainSystem::INHERENT_IDENTIFIER, &e).expect("There is only one fatal error; qed");
                    if e.is_fatal_error() { return result; }
                }
            }
            Ok(None) => (),
            Err(e) => {
                result.put_error(ParachainSystem::INHERENT_IDENTIFIER, &e).expect("There is only one fatal error; qed");
                if e.is_fatal_error() { return result; }
            }
        }
        match Timestamp::is_inherent_required(self) {
            Ok(Some(e)) => {
                let found = block.extrinsics().iter().any(|xt| {
                    let is_signed = self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Extrinsic::is_signed(xt).unwrap_or(false);
                    if !is_signed {
                        let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                        if let Some(call) = IsSubType::<_>::is_sub_type(call) { Timestamp::is_inherent(&call) } else { false }
                    } else { false }
                });
                if !found {
                    result.put_error(Timestamp::INHERENT_IDENTIFIER, &e).expect("There is only one fatal error; qed");
                    if e.is_fatal_error() { return result; }
                }
            }
            Ok(None) => (),
            Err(e) => {
                result.put_error(Timestamp::INHERENT_IDENTIFIER, &e).expect("There is only one fatal error; qed");
                if e.is_fatal_error() { return result; }
            }
        }
        result
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::EnsureInherentsAreFirst<Block> for Runtime {
    fn ensure_inherents_are_first(block: &Block) -> Result<(), u32> {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{IsSubType, ExtrinsicCall};
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block   as _;
        let mut first_signed_observed = false;
        for (i, xt) in block.extrinsics().iter().enumerate() {
            let is_signed = self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::Extrinsic::is_signed(xt).unwrap_or(false);
            let is_inherent = if is_signed { false } else {
                let mut is_inherent = false;
                {
                    let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                    if let Some(call) = IsSubType::<_>::is_sub_type(call) { if ParachainSystem::is_inherent(&call) { is_inherent = true; } }
                }
                {
                    let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                    if let Some(call) = IsSubType::<_>::is_sub_type(call) { if Timestamp::is_inherent(&call) { is_inherent = true; } }
                }
                is_inherent
            };
            if !is_inherent { first_signed_observed = true; }
            if first_signed_observed && is_inherent { return Err(i as u32); }
        }
        Ok(())
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::ValidateUnsigned for Runtime {
    type Call = Call;
    fn pre_dispatch(call: &Self::Call) -> Result<(), self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionValidityError> {
        #[allow(unreachable_patterns)] match call {
            Call::ParachainSystem(inner_call) => ParachainSystem::pre_dispatch(inner_call),
            _ => Ok(()),
        }
    }
    fn validate_unsigned(#[allow(unused_variables)] source: self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionSource, call: &Self::Call ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionValidity {
        #[allow(unreachable_patterns)] match call {
            Call::ParachainSystem(inner_call) => ParachainSystem::validate_unsigned(source, inner_call),
            _ => self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::UnknownTransaction::NoUnsignedValidator.into(),
        }
    }
}
#[cfg(test)]
mod __construct_runtime_integrity_test {
    use super::*;

    #[test]
    pub fn runtime_integrity_tests() { <AllPalletsWithSystem as self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IntegrityTest>::integrity_test(); }
}
const _: () = assert!(
    <
    frame_system::Error<Runtime> as IntellijRustDollarCrate::traits::PalletError
    >::MAX_ENCODED_SIZE <= IntellijRustDollarCrate::MAX_MODULE_ERROR_ENCODED_SIZE,
    "The maximum encoded size of the error type in the `System` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"
);
const _: () = assert!(
    <
    cumulus_pallet_parachain_system::Error<Runtime> as IntellijRustDollarCrate::traits::PalletError
    >::MAX_ENCODED_SIZE <= IntellijRustDollarCrate::MAX_MODULE_ERROR_ENCODED_SIZE,
    "The maximum encoded size of the error type in the `ParachainSystem` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"
);
const _: () = assert!(
    <
    pallet_balances::Error<Runtime> as IntellijRustDollarCrate::traits::PalletError
    >::MAX_ENCODED_SIZE <= IntellijRustDollarCrate::MAX_MODULE_ERROR_ENCODED_SIZE,
    "The maximum encoded size of the error type in the `Balances` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"
);
const _: () = assert!(
    <
    pallet_authorship::Error<Runtime> as IntellijRustDollarCrate::traits::PalletError
    >::MAX_ENCODED_SIZE <= IntellijRustDollarCrate::MAX_MODULE_ERROR_ENCODED_SIZE,
    "The maximum encoded size of the error type in the `Authorship` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"
);
const _: () = assert!(
    <
    pallet_collator_selection::Error<Runtime> as IntellijRustDollarCrate::traits::PalletError
    >::MAX_ENCODED_SIZE <= IntellijRustDollarCrate::MAX_MODULE_ERROR_ENCODED_SIZE,
    "The maximum encoded size of the error type in the `CollatorSelection` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"
);
const _: () = assert!(
    <
    pallet_session::Error<Runtime> as IntellijRustDollarCrate::traits::PalletError
    >::MAX_ENCODED_SIZE <= IntellijRustDollarCrate::MAX_MODULE_ERROR_ENCODED_SIZE,
    "The maximum encoded size of the error type in the `Session` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"
);
const _: () = assert!(
    <
    cumulus_pallet_xcmp_queue::Error<Runtime> as IntellijRustDollarCrate::traits::PalletError
    >::MAX_ENCODED_SIZE <= IntellijRustDollarCrate::MAX_MODULE_ERROR_ENCODED_SIZE,
    "The maximum encoded size of the error type in the `XcmpQueue` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"
);
const _: () = assert!(
    <
    pallet_xcm::Error<Runtime> as IntellijRustDollarCrate::traits::PalletError
    >::MAX_ENCODED_SIZE <= IntellijRustDollarCrate::MAX_MODULE_ERROR_ENCODED_SIZE,
    "The maximum encoded size of the error type in the `PolkadotXcm` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"
);
const _: () = assert!(
    <
    cumulus_pallet_xcm::Error<Runtime> as IntellijRustDollarCrate::traits::PalletError
    >::MAX_ENCODED_SIZE <= IntellijRustDollarCrate::MAX_MODULE_ERROR_ENCODED_SIZE,
    "The maximum encoded size of the error type in the `CumulusXcm` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"
);
const _: () = assert!(
    <
    cumulus_pallet_dmp_queue::Error<Runtime> as IntellijRustDollarCrate::traits::PalletError
    >::MAX_ENCODED_SIZE <= IntellijRustDollarCrate::MAX_MODULE_ERROR_ENCODED_SIZE,
    "The maximum encoded size of the error type in the `DmpQueue` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"
);
const _: () = assert!(
    <
    pallet_template::Error<Runtime> as IntellijRustDollarCrate::traits::PalletError
    >::MAX_ENCODED_SIZE <= IntellijRustDollarCrate::MAX_MODULE_ERROR_ENCODED_SIZE,
    "The maximum encoded size of the error type in the `TemplatePallet` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"
);
const _: () = assert!(
    <
    proxy_pallet::Error<Runtime> as IntellijRustDollarCrate::traits::PalletError
    >::MAX_ENCODED_SIZE <= IntellijRustDollarCrate::MAX_MODULE_ERROR_ENCODED_SIZE,
    "The maximum encoded size of the error type in the `ProxyPallet` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"
);