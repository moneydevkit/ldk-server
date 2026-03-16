// This file is Copyright its original authors, visible in version control
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

use ldk_node::config::{ChannelConfig, MaxDustHTLCExposure};
use ldk_node::lightning::routing::router::RouteParametersConfig;
use ldk_server_protos::types::channel_config::MaxDustHtlcExposure;

use crate::api::error::LdkServerError;
use crate::api::error::LdkServerErrorCode::InvalidRequestError;

pub mod bolt11_receive;
pub mod bolt11_send;
pub mod bolt12_receive;
pub mod bolt12_send;
pub mod close_channel;
pub mod connect_peer;
pub mod disconnect_peer;
pub mod error;
pub mod export_pathfinding_scores;
pub mod get_balances;
pub mod get_node_info;
pub mod get_payment_details;
pub mod graph_get_channel;
pub mod graph_get_node;
pub mod graph_list_channels;
pub mod graph_list_nodes;
pub mod list_channels;
pub mod list_forwarded_payments;
pub mod list_payments;
pub mod onchain_receive;
pub mod onchain_send;
pub mod open_channel;
pub mod sign_message;
pub mod splice_channel;
pub mod spontaneous_send;
pub mod update_channel_config;
pub mod verify_signature;

pub(crate) fn build_channel_config_from_proto(
	default_config: ChannelConfig, proto_channel_config: ldk_server_protos::types::ChannelConfig,
) -> Result<ChannelConfig, LdkServerError> {
	let max_dust_htlc_exposure = proto_channel_config
		.max_dust_htlc_exposure
		.map(|max_dust_htlc_exposure| match max_dust_htlc_exposure {
			MaxDustHtlcExposure::FixedLimitMsat(limit_msat) => {
				MaxDustHTLCExposure::FixedLimit { limit_msat }
			},
			MaxDustHtlcExposure::FeeRateMultiplier(multiplier) => {
				MaxDustHTLCExposure::FeeRateMultiplier { multiplier }
			},
		})
		.unwrap_or(default_config.max_dust_htlc_exposure);

	let cltv_expiry_delta = match proto_channel_config.cltv_expiry_delta {
		Some(c) => Some(u16::try_from(c).map_err(|_| {
			LdkServerError::new(
				InvalidRequestError,
				format!("Invalid cltv_expiry_delta, must be between 0 and {}", u16::MAX),
			)
		})?),
		None => None,
	}
	.unwrap_or(default_config.cltv_expiry_delta);

	Ok(ChannelConfig {
		forwarding_fee_proportional_millionths: proto_channel_config
			.forwarding_fee_proportional_millionths
			.unwrap_or(default_config.forwarding_fee_proportional_millionths),
		forwarding_fee_base_msat: proto_channel_config
			.forwarding_fee_base_msat
			.unwrap_or(default_config.forwarding_fee_base_msat),
		cltv_expiry_delta,
		max_dust_htlc_exposure,
		force_close_avoidance_max_fee_satoshis: proto_channel_config
			.force_close_avoidance_max_fee_satoshis
			.unwrap_or(default_config.force_close_avoidance_max_fee_satoshis),
		accept_underpaying_htlcs: proto_channel_config
			.accept_underpaying_htlcs
			.unwrap_or(default_config.accept_underpaying_htlcs),
	})
}

pub(crate) fn build_route_parameters_config_from_proto(
	proto_route_params: Option<ldk_server_protos::types::RouteParametersConfig>,
) -> Result<Option<RouteParametersConfig>, LdkServerError> {
	match proto_route_params {
		Some(params) => {
			let max_path_count = params.max_path_count.try_into().map_err(|_| {
				LdkServerError::new(
					InvalidRequestError,
					format!("Invalid max_path_count, must be between 0 and {}", u8::MAX),
				)
			})?;
			let max_channel_saturation_power_of_half =
				params.max_channel_saturation_power_of_half.try_into().map_err(|_| {
					LdkServerError::new(
						InvalidRequestError,
						format!(
							"Invalid max_channel_saturation_power_of_half, must be between 0 and {}",
							u8::MAX
						),
					)
				})?;
			Ok(Some(RouteParametersConfig {
				max_total_routing_fee_msat: params.max_total_routing_fee_msat,
				max_total_cltv_expiry_delta: params.max_total_cltv_expiry_delta,
				max_path_count,
				max_channel_saturation_power_of_half,
			}))
		},
		None => Ok(None),
	}
}
