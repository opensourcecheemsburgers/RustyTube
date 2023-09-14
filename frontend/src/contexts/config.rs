use config::{Config, HomepageCategory, NetworkConfig, PlayerConfig, PrivacyConfig, UiConfig};
use leptos::{create_effect, create_rw_signal, create_slice, provide_context, Scope, Signal, SignalGet, SignalSetter};

pub fn provide_config_context_slices(cx: Scope, config: Config) {
    let config = create_rw_signal(cx, config);
    create_effect(cx, move |_| config.get().save());

    let server_slice = create_slice(
        cx,
        config,
        |config| config.network.server.clone(),
        |config, server| config.network.server = server,
    );
    let theme_slice = create_slice(
        cx,
        config,
        |config| config.ui.theme.clone(),
        |config, theme| config.ui.theme = theme,
    );
    let homepage_category_slice = create_slice(
        cx,
        config,
        |config| config.ui.homepage.clone(),
        |config, homepage| config.ui.homepage = homepage,
    );
    let volume_slice = create_slice(
        cx,
        config,
        |config| config.player.volume.clone(),
        |config, volume| config.player.volume = volume,
    );
    let network_slice = create_slice(
        cx,
        config,
        |config| config.network.clone(),
        |config, network| config.network = network,
    );
    let ui_slice = create_slice(
        cx,
        config,
        |config| config.ui.clone(),
        |config, ui| config.ui = ui,
    );
    let player_slice = create_slice(
        cx,
        config,
        |config| config.player.clone(),
        |config, player| config.player = player,
    );
    let privacy_slice = create_slice(
        cx,
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

    provide_context(cx, config);
    provide_context(cx, server_ctx);
    provide_context(cx, theme_ctx);
    provide_context(cx, network_ctx);
    provide_context(cx, ui_ctx);
    provide_context(cx, player_ctx);
    provide_context(cx, privacy_ctx);
    provide_context(cx, homepage_category_ctx);
    provide_context(cx, volume_ctx);
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

pub const THEMES: &'static [&'static str] = &[
    "dracula",
    "winter",
    "night",
    "synthwave",
    "aqua",
    "retro",
    "cyberpunk",
    "valentine",
    "halloween",
    "light",
    "garden",
    "forest",
    "dark",
    "black",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "luxury",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "coffee",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
];
