mod config;
mod user;
mod player;

pub use config::{
    provide_config_context_slices,
    NetworkConfigCtx,
    UiConfigCtx,
    PlayerConfigCtx,
    PrivacyConfigCtx,
    HomepageCategoryCtx,
    ServerCtx,
    ThemeCtx,
    VolumeCtx,
    THEMES,
};

pub use user::{
    SubscriptionsCtx,
    SubsVideosCtx,
    InstancesCtx,
    ChannelsCtx,
    provide_user_contexts,
    provide_user_resources
};

pub use player::{
    PlaybackState,
    PlayerState,
    PlayerStyle,
    VideoTime,
    VideoFormatCtx,
    provide_player_contexts,
    AUDIO_PLAYER_ID,
    VIDEO_PLAYER_ID,
    VIDEO_CONTAINER_ID,
    VIDEO_CONTROLS_ID,
};