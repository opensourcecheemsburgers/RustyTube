mod config;
mod player;
mod user;

pub use config::{
    provide_config_context_slices, PlayerConfigCtx,
    ServerCtx, ThemeCtx, UiConfigCtx, VolumeCtx
};

pub use user::{
    provide_user_contexts, provide_user_resources, ChannelsCtx, InstancesCtx, SubsVideosCtx,
    SubscriptionsCtx,
};

pub use player::{
    PlaybackState, PlayerState, PlayerStyle, AUDIO_PLAYER_ID, VIDEO_CONTAINER_ID,
    VIDEO_CONTROLS_ID, VIDEO_PLAYER_ID,
};

