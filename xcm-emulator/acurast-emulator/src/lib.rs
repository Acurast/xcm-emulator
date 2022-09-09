use frame_support::traits::GenesisBuild;
use sp_runtime::AccountId32;

use xcm_emulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};

decl_test_relay_chain! {
	pub struct PolkadotRelay {
		Runtime = polkadot_runtime::Runtime,
		XcmConfig = polkadot_runtime::xcm_config::XcmConfig,
		new_ext = polkadot_ext(),
	}
}

decl_test_parachain! {
	pub struct AcurastParachain {
		Runtime = acurast_runtime::Runtime,
		Origin = acurast_runtime::Origin,
		XcmpMessageHandler = acurast_runtime::XcmpQueue,
		DmpMessageHandler = acurast_runtime::DmpQueue,
		new_ext = acurast_ext(2000),
	}
}

decl_test_parachain! {
	pub struct CumulusParachain {
		Runtime = parachain_template_runtime::Runtime,
		Origin = parachain_template_runtime::Origin,
		XcmpMessageHandler = parachain_template_runtime::XcmpQueue,
		DmpMessageHandler = parachain_template_runtime::DmpQueue,
		new_ext = cumulus_ext(2001),
	}
}

decl_test_network! {
	pub struct Network {
		relay_chain = PolkadotRelay,
		parachains = vec![
			(2000, AcurastParachain),
			(2001, CumulusParachain),
		],
	}
}

pub const ALICE: AccountId32 = AccountId32::new([0u8; 32]);
pub const INITIAL_BALANCE: u128 = 1_000_000_000_000;

pub fn acurast_ext(para_id: u32) -> sp_io::TestExternalities {
	use acurast_runtime::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap();

	let parachain_info_config = parachain_info::GenesisConfig {
		parachain_id: para_id.into(),
	};

	<parachain_info::GenesisConfig as GenesisBuild<Runtime, _>>::assimilate_storage(&parachain_info_config, &mut t)
		.unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(ALICE, INITIAL_BALANCE)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn cumulus_ext(para_id: u32) -> sp_io::TestExternalities {
	use parachain_template_runtime::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap();

	let parachain_info_config = parachain_info::GenesisConfig {
		parachain_id: para_id.into(),
	};

	<parachain_info::GenesisConfig as GenesisBuild<Runtime, _>>::assimilate_storage(&parachain_info_config, &mut t)
		.unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(ALICE, INITIAL_BALANCE)],
	}
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

fn default_parachains_host_configuration(
) -> polkadot_runtime_parachains::configuration::HostConfiguration<polkadot_primitives::v2::BlockNumber> {
	use polkadot_primitives::v2::{MAX_CODE_SIZE, MAX_POV_SIZE};

	polkadot_runtime_parachains::configuration::HostConfiguration {
		minimum_validation_upgrade_delay: 5,
		validation_upgrade_cooldown: 10u32,
		validation_upgrade_delay: 10,
		code_retention_period: 1200,
		max_code_size: MAX_CODE_SIZE,
		max_pov_size: MAX_POV_SIZE,
		max_head_data_size: 32 * 1024,
		group_rotation_frequency: 20,
		chain_availability_period: 4,
		thread_availability_period: 4,
		max_upward_queue_count: 8,
		max_upward_queue_size: 1024 * 1024,
		max_downward_message_size: 1024,
		ump_service_total_weight: 4 * 1_000_000_000,
		max_upward_message_size: 50 * 1024,
		max_upward_message_num_per_candidate: 5,
		hrmp_sender_deposit: 0,
		hrmp_recipient_deposit: 0,
		hrmp_channel_max_capacity: 8,
		hrmp_channel_max_total_size: 8 * 1024,
		hrmp_max_parachain_inbound_channels: 4,
		hrmp_max_parathread_inbound_channels: 4,
		hrmp_channel_max_message_size: 1024 * 1024,
		hrmp_max_parachain_outbound_channels: 4,
		hrmp_max_parathread_outbound_channels: 4,
		hrmp_max_message_num_per_candidate: 5,
		dispute_period: 6,
		no_show_slots: 2,
		n_delay_tranches: 25,
		needed_approvals: 2,
		relay_vrf_modulo_samples: 2,
		zeroth_delay_tranche_width: 0,
		..Default::default()
	}
}

pub fn polkadot_ext() -> sp_io::TestExternalities {
	use polkadot_runtime::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(ALICE, INITIAL_BALANCE)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	polkadot_runtime_parachains::configuration::GenesisConfig::<Runtime> {
		config: default_parachains_host_configuration(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

#[cfg(test)]
mod tests {
	use super::*;
	use codec::{Decode, Encode};

	use cumulus_primitives_core::ParaId;
	use frame_support::{assert_ok, RuntimeDebug, traits::Currency};
	use frame_support::dispatch::TypeInfo;
	use sp_runtime::traits::{AccountIdConversion, Dispatchable};
	use xcm::{latest::prelude::*, VersionedMultiLocation, VersionedXcm};
	use frame_support::traits::PalletInfoAccess;
	use xcm_emulator::TestExt;
	type CumulusXcmPallet = pallet_xcm::Pallet<parachain_template_runtime::Runtime>;
	type AcurastXcmPallet = pallet_xcm::Pallet<acurast_runtime::Runtime>;

	#[test]
	fn dmp() {
		Network::reset();

		let remark = acurast_runtime::Call::System(frame_system::Call::<acurast_runtime::Runtime>::remark_with_event {
			remark: "Hello from Atera".as_bytes().to_vec(),
		});
		PolkadotRelay::execute_with(|| {
			assert_ok!(polkadot_runtime::XcmPallet::force_default_xcm_version(
				polkadot_runtime::Origin::root(),
				Some(0)
			));
			assert_ok!(polkadot_runtime::XcmPallet::send_xcm(
				Here,
				Parachain(2000),
				Xcm(vec![Transact {
					origin_type: OriginKind::SovereignAccount,
					require_weight_at_most: INITIAL_BALANCE as u64,
					call: remark.encode().into(),
				}]),
			));
		});

		AcurastParachain::execute_with(|| {
			use acurast_runtime::{Event, System};
			System::events().iter().for_each(|r| println!(">>> {:?}", r.event));

			assert!(System::events().iter().any(|r| matches!(
				r.event,
				Event::System(frame_system::Event::Remarked { sender: _, hash: _ })
			)));
		});
	}

	#[test]
	fn ump() {
		Network::reset();

		PolkadotRelay::execute_with(|| {
			let _ = polkadot_runtime::Balances::deposit_creating(
				&ParaId::from(2000).into_account_truncating(),
				1_000_000_000_000,
			);
		});

		let remark = polkadot_runtime::Call::System(frame_system::Call::<polkadot_runtime::Runtime>::remark_with_event {
			remark: "Hello from Acurast!".as_bytes().to_vec(),
		});

		let send_amount = 1_000_000_000_000;
		AcurastParachain::execute_with(|| {
			assert_ok!(acurast_runtime::PolkadotXcm::send_xcm(
				Here,
				Parent,
				Xcm(vec![
						WithdrawAsset((Here, send_amount).into()),
						buy_execution((Here, send_amount)),
						Transact {
							origin_type: OriginKind::SovereignAccount,
							require_weight_at_most: INITIAL_BALANCE as u64,
							call: remark.encode().into(),
						}
				]),
			));
		});

		PolkadotRelay::execute_with(|| {
			use polkadot_runtime::{Event, System};
			let event_list = System::events();
			assert!(System::events().iter().any(|r| matches!(
				r.event,
				Event::System(frame_system::Event::Remarked { sender: _, hash: _ })
			)));
		});
	}

	#[test]
	fn xcmp() {
		Network::reset();

		let remark = parachain_template_runtime::Call::System(frame_system::Call::<parachain_template_runtime::Runtime>::remark_with_event {
			remark: "Hello from acurast!".as_bytes().to_vec(),
		});
		AcurastParachain::execute_with(|| {
			assert_ok!(acurast_runtime::PolkadotXcm::send_xcm(
				Here,
				MultiLocation::new(1, X1(Parachain(2001))),
				Xcm(vec![Transact {
					origin_type: OriginKind::SovereignAccount,
					require_weight_at_most: 10_000_000,
					call: remark.encode().into(),
				}]),
			));
		});

		CumulusParachain::execute_with(|| {
			use parachain_template_runtime::{Event, System};
			System::events().iter().for_each(|r| println!(">>> {:?}", r.event));

			assert!(System::events().iter().any(|r| matches!(
				r.event,
				Event::System(frame_system::Event::Remarked { sender: _, hash: _ })
			)));
		});
	}

	#[test]
	fn mock_call() {
		Network::reset();
		let remark = acurast_runtime::Call::TestPallet(test_pallet::Call::<acurast_runtime::Runtime>
		::test_store{something: 42}).encode();

		let pallet_info = acurast_runtime::Runtime::metadata();
		let pallet_index = acurast_runtime::TestPallet::index();

		#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
		#[allow(non_camel_case_types)]
		enum MockCall {
			#[codec(index = 0u8)]
			mock_test_store {
				mock_something: u32
			}
		}

		let mut mock_remark = MockCall::mock_test_store{ mock_something: 42}.encode();
		mock_remark.splice(..0, [pallet_index as u8]);

		assert_eq!(remark, mock_remark);

		CumulusParachain::execute_with(|| {
			let xcm_transaction = CumulusXcmPallet::send_xcm(
				Here,
				(Parent, Parachain(2000)),
				Xcm(vec![Transact {
					origin_type: OriginKind::SovereignAccount
					,
					require_weight_at_most: INITIAL_BALANCE as u64,
					call: mock_remark.into(),
				}]),
			);
			assert_ok!(xcm_transaction);
		});

		AcurastParachain::execute_with(|| {
			use acurast_runtime::{Event, System};
			let events = System::events();
			assert!(
				System::events().iter().any(|event|
					matches!(event.event, Event::TestPallet(test_pallet::Event::TestStored { .. }))
				)
			);
		});
	}

	#[test]
	fn proxy_call() {
		Network::reset();
		use frame_support::dispatch::Dispatchable;

		CumulusParachain::execute_with(|| {
			let message_call = parachain_template_runtime::Call::ProxyPallet(proxy_pallet::pallet::Call::<parachain_template_runtime::Runtime>::test_store{something: 42, pallet_index: 69});
			let alice_origin = parachain_template_runtime::Origin::signed(ALICE);
			let dispatch_status =  message_call.dispatch(alice_origin);
			assert_ok!(dispatch_status);
		});

		AcurastParachain::execute_with(|| {
			use acurast_runtime::{Event, System};
			let events = System::events();
			let p_store = test_pallet::TestStorage::<acurast_runtime::Runtime>::get();
			assert!(
				events.iter().any(|event|
					matches!(event.event, Event::TestPallet(test_pallet::Event::TestStored { .. }))
				)
			);
		});
	}

	fn send_base_asset() {
		Network::reset();

		AcurastParachain::execute_with(|| {

		})
	}

	// Helper function for forming buy execution message
	fn buy_execution<C>(fees: impl Into<MultiAsset>) -> Instruction<C> {
		BuyExecution { fees: fees.into(), weight_limit: Unlimited }
	}
}