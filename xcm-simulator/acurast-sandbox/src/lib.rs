// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

mod mock_acurast;
mod mock_cumulus;
mod relay_chain;
// mod EXPANDED_acurast_runtime;

use polkadot_parachain::primitives::Id as ParaId;
use sp_runtime::traits::AccountIdConversion;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};

pub const ALICE: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0u8; 32]);
pub const INITIAL_BALANCE: u128 = 1_000_000_000;

decl_test_parachain! {
	pub struct MockAcurast {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = mock_acurast::MsgQueue,
		DmpMessageHandler = mock_acurast::MsgQueue,
		new_ext = acurast_ext(1),
	}
}

decl_test_parachain! {
	pub struct MockCumulus {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = mock_cumulus::MsgQueue,
		DmpMessageHandler = mock_cumulus::MsgQueue,
		new_ext = cumulus_ext(2),
	}
}

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = relay_chain::Runtime,
		XcmConfig = relay_chain::XcmConfig,
		new_ext = relay_ext(),
	}
}

decl_test_network! {
	pub struct MockNet {
		relay_chain = Relay,
		parachains = vec![
			(1, MockAcurast),
			(2, MockCumulus),
		],
	}
}

pub fn para_account_id(id: u32) -> relay_chain::AccountId {
	ParaId::from(id).into_account_truncating()
}

pub fn acurast_ext(para_id: u32) -> sp_io::TestExternalities {
	use mock_acurast::{MsgQueue, Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> { balances: vec![(ALICE, INITIAL_BALANCE)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
		MsgQueue::set_para_id(para_id.into());
	});
	ext
}

pub fn cumulus_ext(para_id: u32) -> sp_io::TestExternalities {
	use mock_cumulus::{MsgQueue, Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> { balances: vec![(ALICE, INITIAL_BALANCE)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
		MsgQueue::set_para_id(para_id.into());
	});
	ext
}

pub fn relay_ext() -> sp_io::TestExternalities {
	use relay_chain::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(ALICE, INITIAL_BALANCE), (para_account_id(1), INITIAL_BALANCE)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub type RelayChainPalletXcm = pallet_xcm::Pallet<relay_chain::Runtime>;
pub type AcurastPalletXcm = pallet_xcm::Pallet<mock_acurast::Runtime>;
pub type CumulusPalletXcm = pallet_xcm::Pallet<mock_cumulus::Runtime>;

#[cfg(test)]
mod tests {
	use super::*;

	use codec::{Decode, Encode};
	use frame_support::{assert_ok, RuntimeDebug};
	use scale_info::TypeInfo;
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;
	use frame_support::traits::PalletInfoAccess;

	#[test]
	fn mock_calls() {
		MockNet::reset();
		// mock should equal this call when encoded
		let remark = mock_acurast::Call::TestPallet(simulator_test_pallet::Call::<mock_acurast::Runtime>
		::test_store{something: 42}).encode();

		let pallet_info = mock_acurast::Runtime::metadata();
		let pallet_index = mock_acurast::TestPallet::index();
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
		MockCumulus::execute_with(|| {
			let xcm_transaction = CumulusPalletXcm::send_xcm(
				Here,
				(Parent, Parachain(1)),
				Xcm(vec![Transact {
					origin_type: OriginKind::SovereignAccount,
					require_weight_at_most: INITIAL_BALANCE as u64,
					call: remark.into(),
				}]),
			);
			assert_ok!(xcm_transaction);
		});

		MockAcurast::execute_with(|| {
			use mock_acurast::{Event, System, TestPallet};
			assert!(
				System::events().iter().any(|event|
					matches!(event.event, Event::TestPallet(simulator_test_pallet::Event::TestStored { .. }))
				)
			);

		});
	}


	// Helper function for forming buy execution message
	fn buy_execution<C>(fees: impl Into<MultiAsset>) -> Instruction<C> {
		BuyExecution { fees: fees.into(), weight_limit: Unlimited }
	}

	#[test]
	fn dmp() {
		MockNet::reset();

		let remark =
			mock_acurast::Call::System(frame_system::Call::<mock_acurast::Runtime>::remark_with_event {
				remark: vec![1, 2, 3],
			});
		Relay::execute_with(|| {
			assert_ok!(RelayChainPalletXcm::send_xcm(
				Here,
				Parachain(1),
				Xcm(vec![Transact {
					origin_type: OriginKind::SovereignAccount,
					require_weight_at_most: INITIAL_BALANCE as u64,
					call: remark.encode().into(),
				}]),
			));
		});

		MockAcurast::execute_with(|| {
			use mock_acurast::{Event, System};
			assert!(System::events()
				.iter()
				.any(|r| matches!(r.event, Event::System(frame_system::Event::Remarked { .. }))));
		});
	}

	#[test]
	fn ump() {
		MockNet::reset();

		let remark = relay_chain::Call::System(
			frame_system::Call::<relay_chain::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
		);
		MockAcurast::execute_with(|| {
			assert_ok!(AcurastPalletXcm::send_xcm(
				Here,
				Parent,
				Xcm(vec![Transact {
					origin_type: OriginKind::SovereignAccount,
					require_weight_at_most: INITIAL_BALANCE as u64,
					call: remark.encode().into(),
				}]),
			));
		});

		Relay::execute_with(|| {
			use relay_chain::{Event, System};
			assert!(System::events()
				.iter()
				.any(|r| matches!(r.event, Event::System(frame_system::Event::Remarked { .. }))));
		});
	}

	#[test]
	fn xcmp() {
		MockNet::reset();

		let remark =
			mock_acurast::Call::System(frame_system::Call::<mock_acurast::Runtime>::remark_with_event {
				remark: vec![1, 2, 3],
			});
		MockAcurast::execute_with(|| {
			assert_ok!(AcurastPalletXcm::send_xcm(
				Here,
				(Parent, Parachain(2)),
				Xcm(vec![Transact {
					origin_type: OriginKind::SovereignAccount,
					require_weight_at_most: INITIAL_BALANCE as u64,
					call: remark.encode().into(),
				}]),
			));
		});

		MockCumulus::execute_with(|| {
			use mock_acurast::{Event, System};
			assert!(System::events()
				.iter()
				.any(|r| matches!(r.event, Event::System(frame_system::Event::Remarked { .. }))));
		});
	}

	#[test]
	fn reserve_transfer() {
		MockNet::reset();

		let withdraw_amount = 123;

		Relay::execute_with(|| {
			assert_ok!(RelayChainPalletXcm::reserve_transfer_assets(
				relay_chain::Origin::signed(ALICE),
				Box::new(X1(Parachain(1)).into().into()),
				Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
				Box::new((Here, withdraw_amount).into()),
				0,
			));
			assert_eq!(
				mock_acurast::Balances::free_balance(&para_account_id(1)),
				INITIAL_BALANCE + withdraw_amount
			);
		});

		MockAcurast::execute_with(|| {
			// free execution, full amount received
			assert_eq!(
				pallet_balances::Pallet::<mock_acurast::Runtime>::free_balance(&ALICE),
				INITIAL_BALANCE + withdraw_amount
			);
		});
	}

	/// Scenario:
	/// A parachain transfers funds on the relay chain to another parachain account.
	///
	/// Asserts that the parachain accounts are updated as expected.
	#[test]
	fn withdraw_and_deposit() {
		MockNet::reset();

		let send_amount = 10;

		MockAcurast::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((Here, send_amount).into()),
				buy_execution((Here, send_amount)),
				DepositAsset {
					assets: All.into(),
					max_assets: 1,
					beneficiary: Parachain(2).into(),
				},
			]);
			// Send withdraw and deposit
			assert_ok!(AcurastPalletXcm::send_xcm(Here, Parent, message.clone()));
		});

		Relay::execute_with(|| {
			assert_eq!(
				relay_chain::Balances::free_balance(para_account_id(1)),
				INITIAL_BALANCE - send_amount
			);
			assert_eq!(relay_chain::Balances::free_balance(para_account_id(2)), send_amount);
		});
	}

	/// Scenario:
	/// A parachain wants to be notified that a transfer worked correctly.
	/// It sends a `QueryHolding` after the deposit to get notified on success.
	///
	/// Asserts that the balances are updated correctly and the expected XCM is sent.
	#[test]
	fn query_holding() {
		MockNet::reset();

		let send_amount = 10;
		let query_id_set = 1234;

		// Send a message which fully succeeds on the relay chain
		MockAcurast::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((Here, send_amount).into()),
				buy_execution((Here, send_amount)),
				DepositAsset {
					assets: All.into(),
					max_assets: 1,
					beneficiary: Parachain(2).into(),
				},
				QueryHolding {
					query_id: query_id_set,
					dest: Parachain(1).into(),
					assets: All.into(),
					max_response_weight: 1_000_000_000,
				},
			]);
			// Send withdraw and deposit with query holding
			assert_ok!(AcurastPalletXcm::send_xcm(Here, Parent, message.clone(),));
		});

		// Check that transfer was executed
		Relay::execute_with(|| {
			// Withdraw executed
			assert_eq!(
				relay_chain::Balances::free_balance(para_account_id(1)),
				INITIAL_BALANCE - send_amount
			);
			// Deposit executed
			assert_eq!(relay_chain::Balances::free_balance(para_account_id(2)), send_amount);
		});

		// Check that QueryResponse message was received
		MockAcurast::execute_with(|| {
			assert_eq!(
				mock_acurast::MsgQueue::received_dmp(),
				vec![Xcm(vec![QueryResponse {
					query_id: query_id_set,
					response: Response::Assets(MultiAssets::new()),
					max_weight: 1_000_000_000,
				}])],
			);
		});
	}


}
