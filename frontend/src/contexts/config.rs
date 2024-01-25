use config::{Config, HomepageCategory, NetworkConfig, PlayerConfig, PrivacyConfig, UiConfig};
use leptos::*;

pub fn provide_config_context_slices(config: Config) {
	let config = create_rw_signal(config);
	create_effect(move |_| config.get().save());

	let server_slice = create_slice(
		config,
		|config| config.network.server.clone(),
		|config, server| config.network.server = server,
	);
	let theme_slice = create_slice(
		config,
		|config| config.ui.theme.clone(),
		|config, theme| config.ui.theme = theme,
	);
	let homepage_category_slice = create_slice(
		config,
		|config| config.ui.homepage.clone(),
		|config, homepage| config.ui.homepage = homepage,
	);
	let volume_slice = create_slice(
		config,
		|config| config.player.volume.clone(),
		|config, volume| config.player.volume = volume,
	);
	let network_slice = create_slice(
		config,
		|config| config.network.clone(),
		|config, network| config.network = network,
	);
	let ui_slice = create_slice(config, |config| config.ui.clone(), |config, ui| config.ui = ui);
	let player_slice = create_slice(
		config,
		|config| config.player.clone(),
		|config, player| config.player = player,
	);
	let privacy_slice = create_slice(
		config,
		|config| config.privacy.clone(),
		|config, privacy| config.privacy = privacy,
	);

	let server_ctx = ServerCtx(server_slice);
	let theme_ctx = ThemeCtx(theme_slice);
	let network_ctx = NetworkConfigCtx(network_slice);
	let ui_ctx = UiConfigCtx(ui_slice);
	let player_ctx = PlayerConfigCtx(player_slice);
	let privacy_ctx = PrivacyConfigCtx(privacy_slice);
	let homepage_category_ctx = HomepageCategoryCtx(homepage_category_slice);
	let volume_ctx = VolumeCtx(volume_slice);

	provide_context(config);
	provide_context(server_ctx);
	provide_context(theme_ctx);
	provide_context(network_ctx);
	provide_context(ui_ctx);
	provide_context(player_ctx);
	provide_context(privacy_ctx);
	provide_context(homepage_category_ctx);
	provide_context(volume_ctx);
}

#[derive(Copy, Clone)]
pub struct NetworkConfigCtx(pub (Signal<NetworkConfig>, SignalSetter<NetworkConfig>));

#[derive(Copy, Clone)]
pub struct UiConfigCtx(pub (Signal<UiConfig>, SignalSetter<UiConfig>));

#[derive(Copy, Clone)]
pub struct PlayerConfigCtx(pub (Signal<PlayerConfig>, SignalSetter<PlayerConfig>));

#[derive(Copy, Clone)]
pub struct PrivacyConfigCtx(pub (Signal<PrivacyConfig>, SignalSetter<PrivacyConfig>));
#[derive(Copy, Clone)]
pub struct ServerCtx(pub (Signal<String>, SignalSetter<String>));

#[derive(Copy, Clone)]
pub struct ThemeCtx(pub (Signal<String>, SignalSetter<String>));

#[derive(Copy, Clone)]
pub struct HomepageCategoryCtx(pub (Signal<HomepageCategory>, SignalSetter<HomepageCategory>));

#[derive(Copy, Clone)]
pub struct VolumeCtx(pub (Signal<f64>, SignalSetter<f64>));
