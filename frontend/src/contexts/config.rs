use config::{Config, HomepageCategory, RememberPosition};
use invidious::{AudioQuality, VideoQuality};
use leptos::*;
use locales::RustyTubeLocale;

pub fn provide_config_context_slices(config: Config) {
	let config = create_rw_signal(config);
	create_effect(move |_| config.get().save());

	let server_ctx = NetworkConfigCtx {
		server_slice: slice!(config.network.server),
		custom_servers_slice: slice!(config.network.custom_servers),
		auto_fetch_subs_slice: slice!(config.network.auto_fetch_subs),
		fetch_rss_slice: slice!(config.network.fetch_rss),
	};

	let ui_ctx = UiConfigCtx {
		theme_slice: slice!(config.ui.theme),
		homepage_slice: slice!(config.ui.homepage),
	};

	let player_ctx = PlayerConfigCtx {
		auto_play_slice: slice!(config.player.auto_play),
		fast_forward_interval_slice: slice!(config.player.fast_forward_interval),
		default_video_quality_slice: slice!(config.player.default_video_quality),
		default_audio_quality_slice: slice!(config.player.default_audio_quality),
		remember_position_slice: slice!(config.player.remember_position),
		volume_slice: slice!(config.player.volume),
	};

	let region_ctx = RegionConfigCtx {
		locale_slice: slice!(config.region.locale),
		trending_region_slice: slice!(config.region.trending_region),
	};

	let privacy_ctx = PrivacyConfigCtx { keep_history_slice: slice!(config.privacy.keep_history) };

	provide_context(server_ctx);
	provide_context(ui_ctx);
	provide_context(player_ctx);
	provide_context(region_ctx);
	provide_context(privacy_ctx);
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NetworkConfigCtx {
	pub server_slice: (Signal<String>, SignalSetter<String>),
	pub custom_servers_slice: (Signal<Option<Vec<String>>>, SignalSetter<Option<Vec<String>>>),
	pub auto_fetch_subs_slice: (Signal<bool>, SignalSetter<bool>),
	pub fetch_rss_slice: (Signal<bool>, SignalSetter<bool>),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct UiConfigCtx {
	pub theme_slice: (Signal<String>, SignalSetter<String>),
	pub homepage_slice: (Signal<HomepageCategory>, SignalSetter<HomepageCategory>),
}

#[derive(Copy, Clone, PartialEq)]
pub struct PlayerConfigCtx {
	pub auto_play_slice: (Signal<bool>, SignalSetter<bool>),
	pub fast_forward_interval_slice: (Signal<u8>, SignalSetter<u8>),
	pub default_video_quality_slice: (Signal<VideoQuality>, SignalSetter<VideoQuality>),
	pub default_audio_quality_slice: (Signal<AudioQuality>, SignalSetter<AudioQuality>),
	pub remember_position_slice: (Signal<RememberPosition>, SignalSetter<RememberPosition>),
	pub volume_slice: (Signal<f64>, SignalSetter<f64>),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RegionConfigCtx {
	pub locale_slice: (Signal<RustyTubeLocale>, SignalSetter<RustyTubeLocale>),
	pub trending_region_slice:
		(Signal<isocountry::CountryCode>, SignalSetter<isocountry::CountryCode>),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct PrivacyConfigCtx {
	pub keep_history_slice: (Signal<bool>, SignalSetter<bool>),
}
