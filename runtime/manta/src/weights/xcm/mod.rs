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

mod pallet_xcm_benchmarks_fungible;
mod pallet_xcm_benchmarks_generic;

use crate::{xcm_config::MaxAssetsIntoHolding, Runtime};
use frame_support::weights::Weight;
use pallet_xcm_benchmarks_fungible::WeightInfo as XcmFungibleWeight;
use pallet_xcm_benchmarks_generic::WeightInfo as XcmGeneric;
use sp_std::prelude::*;
use xcm::{latest::prelude::*, DoubleEncoded};

trait WeighMultiAssets {
    fn weigh_multi_assets(&self, weight: Weight) -> Weight;
}

const MAX_ASSETS: u64 = 100;

impl WeighMultiAssets for MultiAssetFilter {
    fn weigh_multi_assets(&self, weight: Weight) -> Weight {
        match self {
            Self::Definite(assets) => weight.saturating_mul(assets.inner().iter().count() as u64),
            Self::Wild(asset) => match asset {
                All => weight.saturating_mul(MAX_ASSETS),
                AllOf { fun, .. } => match fun {
                    WildFungibility::Fungible => weight,
                    // Magic number 2 has to do with the fact that we could have up to 2 times
                    // MaxAssetsIntoHolding in the worst-case scenario.
                    WildFungibility::NonFungible => {
                        weight.saturating_mul((MaxAssetsIntoHolding::get() * 2) as u64)
                    }
                },
                AllCounted(count) => weight.saturating_mul(MAX_ASSETS.min(*count as u64)),
                AllOfCounted { count, .. } => weight.saturating_mul(MAX_ASSETS.min(*count as u64)),
            },
        }
    }
}

impl WeighMultiAssets for MultiAssets {
    fn weigh_multi_assets(&self, weight: Weight) -> Weight {
        weight.saturating_mul(self.inner().iter().count() as u64)
    }
}

pub struct MantaXcmWeight<Call>(core::marker::PhantomData<Call>);
impl<Call> XcmWeightInfo<Call> for MantaXcmWeight<Call> {
    fn withdraw_asset(assets: &MultiAssets) -> Weight {
        assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::withdraw_asset())
    }
    // Currently there is no trusted reserve
    fn reserve_asset_deposited(_assets: &MultiAssets) -> Weight {
        // TODO: hardcoded - fix https://github.com/paritytech/cumulus/issues/1974
        Weight::from_parts(1_000_000_000_u64, 0)
    }
    fn receive_teleported_asset(assets: &MultiAssets) -> Weight {
        assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::receive_teleported_asset())
    }
    fn query_response(
        _query_id: &u64,
        _response: &Response,
        _max_weight: &Weight,
        _querier: &Option<MultiLocation>,
    ) -> Weight {
        XcmGeneric::<Runtime>::query_response()
    }
    fn transfer_asset(assets: &MultiAssets, _dest: &MultiLocation) -> Weight {
        assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::transfer_asset())
    }
    fn transfer_reserve_asset(
        assets: &MultiAssets,
        _dest: &MultiLocation,
        _xcm: &Xcm<()>,
    ) -> Weight {
        assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::transfer_reserve_asset())
    }
    fn transact(
        _origin_type: &OriginKind,
        _require_weight_at_most: &Weight,
        _call: &DoubleEncoded<Call>,
    ) -> Weight {
        XcmGeneric::<Runtime>::transact()
    }
    fn hrmp_new_channel_open_request(
        _sender: &u32,
        _max_message_size: &u32,
        _max_capacity: &u32,
    ) -> Weight {
        // XCM Executor does not currently support HRMP channel operations
        Weight::MAX
    }
    fn hrmp_channel_accepted(_recipient: &u32) -> Weight {
        // XCM Executor does not currently support HRMP channel operations
        Weight::MAX
    }
    fn hrmp_channel_closing(_initiator: &u32, _sender: &u32, _recipient: &u32) -> Weight {
        // XCM Executor does not currently support HRMP channel operations
        Weight::MAX
    }
    fn clear_origin() -> Weight {
        XcmGeneric::<Runtime>::clear_origin()
    }
    fn descend_origin(_who: &InteriorMultiLocation) -> Weight {
        XcmGeneric::<Runtime>::descend_origin()
    }
    fn report_error(_query_response_info: &QueryResponseInfo) -> Weight {
        XcmGeneric::<Runtime>::report_error()
    }

    fn deposit_asset(assets: &MultiAssetFilter, _dest: &MultiLocation) -> Weight {
        // Hardcoded till the XCM pallet is fixed
        let hardcoded_weight = Weight::from_parts(1_000_000_000_u64, 0);
        let weight = assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::deposit_asset());
        hardcoded_weight.min(weight)
    }
    fn deposit_reserve_asset(
        assets: &MultiAssetFilter,
        _dest: &MultiLocation,
        _xcm: &Xcm<()>,
    ) -> Weight {
        assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::deposit_reserve_asset())
    }
    fn exchange_asset(_give: &MultiAssetFilter, _receive: &MultiAssets, _maximal: &bool) -> Weight {
        Weight::MAX
    }
    fn initiate_reserve_withdraw(
        assets: &MultiAssetFilter,
        _reserve: &MultiLocation,
        _xcm: &Xcm<()>,
    ) -> Weight {
        assets.weigh_multi_assets(XcmGeneric::<Runtime>::initiate_reserve_withdraw())
    }
    fn initiate_teleport(
        assets: &MultiAssetFilter,
        _dest: &MultiLocation,
        _xcm: &Xcm<()>,
    ) -> Weight {
        assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::initiate_teleport())
    }
    fn report_holding(_response_info: &QueryResponseInfo, _assets: &MultiAssetFilter) -> Weight {
        XcmGeneric::<Runtime>::report_holding()
    }
    fn buy_execution(_fees: &MultiAsset, _weight_limit: &WeightLimit) -> Weight {
        XcmGeneric::<Runtime>::buy_execution()
    }
    fn refund_surplus() -> Weight {
        XcmGeneric::<Runtime>::refund_surplus()
    }
    fn set_error_handler(_xcm: &Xcm<Call>) -> Weight {
        XcmGeneric::<Runtime>::set_error_handler()
    }
    fn set_appendix(_xcm: &Xcm<Call>) -> Weight {
        XcmGeneric::<Runtime>::set_appendix()
    }
    fn clear_error() -> Weight {
        XcmGeneric::<Runtime>::clear_error()
    }
    fn claim_asset(_assets: &MultiAssets, _ticket: &MultiLocation) -> Weight {
        XcmGeneric::<Runtime>::claim_asset()
    }
    fn trap(_code: &u64) -> Weight {
        XcmGeneric::<Runtime>::trap()
    }
    fn subscribe_version(_query_id: &QueryId, _max_response_weight: &Weight) -> Weight {
        XcmGeneric::<Runtime>::subscribe_version()
    }
    fn unsubscribe_version() -> Weight {
        XcmGeneric::<Runtime>::unsubscribe_version()
    }
    fn burn_asset(assets: &MultiAssets) -> Weight {
        assets.weigh_multi_assets(XcmGeneric::<Runtime>::burn_asset())
    }
    fn expect_asset(assets: &MultiAssets) -> Weight {
        assets.weigh_multi_assets(XcmGeneric::<Runtime>::expect_asset())
    }
    fn expect_origin(_origin: &Option<MultiLocation>) -> Weight {
        XcmGeneric::<Runtime>::expect_origin()
    }
    fn expect_error(_error: &Option<(u32, XcmError)>) -> Weight {
        XcmGeneric::<Runtime>::expect_error()
    }
    fn expect_transact_status(_transact_status: &MaybeErrorCode) -> Weight {
        XcmGeneric::<Runtime>::expect_transact_status()
    }
    fn query_pallet(_module_name: &Vec<u8>, _response_info: &QueryResponseInfo) -> Weight {
        XcmGeneric::<Runtime>::query_pallet()
    }
    fn expect_pallet(
        _index: &u32,
        _name: &Vec<u8>,
        _module_name: &Vec<u8>,
        _crate_major: &u32,
        _min_crate_minor: &u32,
    ) -> Weight {
        XcmGeneric::<Runtime>::expect_pallet()
    }
    fn report_transact_status(_response_info: &QueryResponseInfo) -> Weight {
        XcmGeneric::<Runtime>::report_transact_status()
    }
    fn clear_transact_status() -> Weight {
        XcmGeneric::<Runtime>::clear_transact_status()
    }
    fn universal_origin(_: &Junction) -> Weight {
        Weight::MAX
    }
    fn export_message(_: &NetworkId, _: &Junctions, _: &Xcm<()>) -> Weight {
        Weight::MAX
    }
    fn lock_asset(_: &MultiAsset, _: &MultiLocation) -> Weight {
        Weight::MAX
    }
    fn unlock_asset(_: &MultiAsset, _: &MultiLocation) -> Weight {
        Weight::MAX
    }
    fn note_unlockable(_: &MultiAsset, _: &MultiLocation) -> Weight {
        Weight::MAX
    }
    fn request_unlock(_: &MultiAsset, _: &MultiLocation) -> Weight {
        Weight::MAX
    }
    fn set_fees_mode(_: &bool) -> Weight {
        XcmGeneric::<Runtime>::set_fees_mode()
    }
    fn set_topic(_topic: &[u8; 32]) -> Weight {
        XcmGeneric::<Runtime>::set_topic()
    }
    fn clear_topic() -> Weight {
        XcmGeneric::<Runtime>::clear_topic()
    }
    fn alias_origin(_: &MultiLocation) -> Weight {
        // XCM Executor does not currently support alias origin operations
        Weight::MAX
    }
    fn unpaid_execution(_: &WeightLimit, _: &Option<MultiLocation>) -> Weight {
        XcmGeneric::<Runtime>::unpaid_execution()
    }
}
