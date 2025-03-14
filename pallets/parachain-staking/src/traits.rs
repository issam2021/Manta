// Copyright 2020-2024 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! traits for parachain-staking

use frame_support::pallet_prelude::Weight;

pub trait OnCollatorPayout<AccountId, Balance> {
    fn on_collator_payout(
        for_round: crate::RoundIndex,
        collator_id: AccountId,
        amount: Balance,
    ) -> Weight;
}
impl<AccountId, Balance> OnCollatorPayout<AccountId, Balance> for () {
    fn on_collator_payout(
        _for_round: crate::RoundIndex,
        _collator_id: AccountId,
        _amount: Balance,
    ) -> Weight {
        Weight::zero()
    }
}

pub trait OnNewRound {
    fn on_new_round(round_index: crate::RoundIndex) -> Weight;
}
impl OnNewRound for () {
    fn on_new_round(_round_index: crate::RoundIndex) -> Weight {
        Weight::zero()
    }
}
