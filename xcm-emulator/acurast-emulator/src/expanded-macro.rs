pub struct PolkadotRelay;
pub const EXT_POLKADOTRELAY: ::std::thread::LocalKey<::xcm_emulator::RefCell<::xcm_emulator::TestExternalities>
> =
    {
        #[inline]
        fn __init() -> ::xcm_emulator::RefCell<::xcm_emulator::TestExternalities>
        { (::xcm_emulator::RefCell::new((polkadot_ext()))) }

        #[cfg_attr(not(windows), inline)]
        unsafe fn __getit(
            init: ::std::option::Option<&mut ::std::option::Option<::xcm_emulator::RefCell<::xcm_emulator::TestExternalities>
            >>,
        ) -> ::std::option::Option<&'static ::xcm_emulator::RefCell<::xcm_emulator::TestExternalities>
        > {
            #[cfg(all(target_family = "wasm", not(target_feature = "atomics")))]
            static __KEY: ::std::thread::__StaticLocalKeyInner<::xcm_emulator::RefCell<::xcm_emulator::TestExternalities>
            > =
                ::std::thread::__StaticLocalKeyInner::new();

            #[thread_local]
            #[cfg(all(
            target_thread_local,
            not(all(target_family = "wasm", not(target_feature = "atomics"))),
            ))]
            static __KEY: ::std::thread::__FastLocalKeyInner<::xcm_emulator::RefCell<::xcm_emulator::TestExternalities>
            > =
                ::std::thread::__FastLocalKeyInner::new();

            #[cfg(all(
            not(target_thread_local),
            not(all(target_family = "wasm", not(target_feature = "atomics"))),
            ))]
            static __KEY: ::std::thread::__OsLocalKeyInner<::xcm_emulator::RefCell<::xcm_emulator::TestExternalities>
            > =
                ::std::thread::__OsLocalKeyInner::new();

            #[allow(unused_unsafe)]
            unsafe {
                __KEY.get(move || {
                    if let ::std::option::Option::Some(init) = init {
                        if let ::std::option::Option::Some(value) = init.take() {
                            return value;
                        } else if IntellijRustDollarCrate::cfg!( debug_assertions ) {
                            IntellijRustDollarCrate::unreachable!("missing default value");
                        }
                    }
                    __init()
                })
            }
        }

        unsafe {
            ::std::thread::LocalKey::new(__getit)
        }
    };
impl ::xcm_emulator::TestExt for PolkadotRelay {
    fn new_ext() -> ::xcm_emulator::TestExternalities {
        (polkadot_ext())
    }

    fn reset_ext() {
        EXT_POLKADOTRELAY.with(|v| *v.borrow_mut() = (polkadot_ext()));
    }

    fn execute_with<R>(execute: impl FnOnce() -> R) -> R {
        let r = EXT_POLKADOTRELAY.with(|v| v.borrow_mut().execute_with(execute));


        EXT_POLKADOTRELAY.with(|v| {
            v.borrow_mut().execute_with(|| {
                use ::xcm_emulator::polkadot_primitives::runtime_api::runtime_decl_for_ParachainHost::ParachainHost;

                for para_id in _para_ids() {
                    let downward_messages = <polkadot_runtime::Runtime>::dmq_contents(para_id.into())
                        .into_iter()
                        .map(|inbound| (inbound.sent_at, inbound.msg));
                    if downward_messages.len() == 0 {
                        continue;
                    }
                    _Messenger::send_downward_messages(para_id, downward_messages.into_iter());
                }
            })
        });

        _process_messages();

        r
    }
}
impl ::xcm_emulator::UmpSink for PolkadotRelay {
    fn process_upward_message(
        origin: ::xcm_emulator::ParaId,
        msg: &[u8],
        max_weight: ::xcm_emulator::Weight,
    ) -> Result<::xcm_emulator::Weight, (::xcm_emulator::MessageId, ::xcm_emulator::Weight)> {
        use ::xcm_emulator::{TestExt, UmpSink};

        Self::execute_with(|| {
            ::xcm_emulator::XcmSink::<::xcm_emulator::XcmExecutor<polkadot_runtime::xcm_config::XcmConfig>, polkadot_runtime::Runtime>::process_upward_message(
                origin, msg, max_weight,
            )
        })
    }
}