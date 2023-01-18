use super::*;
use crate::mock::*;

use frame_support::{
	assert_ok,
	traits::{OnFinalize, OnInitialize},
};

pub fn staking_events() -> Vec<Event<Test>> {
	let result = System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(
			|e| if let mock::RuntimeEvent::Capacity(inner) = e { Some(inner) } else { None },
		)
		.collect::<Vec<_>>();

	System::reset_events();
	result
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Capacity::on_initialize(System::block_number());
	}
}

pub fn register_provider(target_id: MessageSourceId, name: String) {
	let name = Vec::from(name).try_into().expect("error");
	assert_ok!(Msa::create_registered_provider(target_id.into(), name));
}

/// Create Capacity account and set remaining and available amounts.
pub fn create_capacity_account_and_fund(
	target_msa_id: MessageSourceId,
	remaining: BalanceOf<Test>,
	available: BalanceOf<Test>,
) -> CapacityDetails<BalanceOf<Test>, <Test as frame_system::Config>::BlockNumber> {
	let mut capacity_details =
		CapacityDetails::<BalanceOf<Test>, <Test as frame_system::Config>::BlockNumber>::default();

	capacity_details.remaining = remaining;
	capacity_details.total_tokens_staked = available;
	capacity_details.total_available = available;
	capacity_details.last_replenished_epoch = 1u32.into();

	Capacity::set_capacity_for(target_msa_id, capacity_details);

	capacity_details
}
