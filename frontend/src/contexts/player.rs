use leptos::*;
use invidious::VideoFormat;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlVideoElement, HtmlAudioElement};
use rustytube_error::RustyTubeError;
use utils::get_element_by_id;
use crate::contexts::VolumeCtx;

pub const VIDEO_CONTAINER_ID: &'static str = "video_container";
pub const VIDEO_PLAYER_ID: &'static str = "video_player";
pub const VIDEO_CONTROLS_ID: &'static str = "video_controls";
pub const AUDIO_PLAYER_ID: &'static str = "audio_player";


#[derive(Copy, Clone)]
pub struct VideoFormatCtx(pub RwSignal<VideoFormat>);

#[derive(Copy, Clone, PartialEq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Loading,
}

#[derive(Clone, Copy)]
pub struct PlayerState {
    format: RwSignal<Option<VideoFormat>>,
    pub playback_state: RwSignal<PlaybackState>,
    video_ready: RwSignal<bool>,
    audio_ready: RwSignal<bool>,
    buffering: RwSignal<bool>,
    pub volume: RwSignal<f64>,
    pub current_time: RwSignal<f64>,
    pub duration: RwSignal<f64>,
    pub current_time_str: RwSignal<String>,
    pub duration_str: RwSignal<String>,
}

impl PlayerState {
    pub fn init() -> Self {
        let format = create_rw_signal(None);
        let playback_state = create_rw_signal(PlaybackState::Paused);
        let video_ready = create_rw_signal(false);
        let audio_ready = create_rw_signal(false);
        let buffering = create_rw_signal(true);
        let volume = create_rw_signal(expect_context::<VolumeCtx>().0.0.get());
        let current_time_str = create_rw_signal(String::from("0:00"));
        let duration_str = create_rw_signal(String::from("0:00"));
        let current_time = create_rw_signal(0f64);
        let duration = create_rw_signal(0f64);

        Self {
            format,
            playback_state,
            video_ready,
            audio_ready,
            buffering,
            volume,
            current_time,
            current_time_str,
            duration, 
            duration_str
        }
    }

    pub fn ready(&self) -> Result<bool, RustyTubeError> {
        let video: HtmlVideoElement = get_element_by_id(VIDEO_PLAYER_ID)?;
        let audio: HtmlAudioElement = get_element_by_id(AUDIO_PLAYER_ID)?;

        let ready = video.ready_state() >= 3 && self.video_ready.get() && audio.ready_state() >= 3 && self.audio_ready.get();
        Ok(ready)
    }

    pub async fn play(&self) -> Result<(), RustyTubeError> {
        let video: HtmlVideoElement = get_element_by_id(VIDEO_PLAYER_ID)?;
        let audio: HtmlAudioElement = get_element_by_id(AUDIO_PLAYER_ID)?;

        audio.set_volume(self.volume.get());
        if self.ready()? {
            let video_play = JsFuture::from(video.play()?).await;
            audio.set_current_time(video.current_time());
            let audio_play = JsFuture::from(audio.play()?).await;
            if let Ok(_) = audio_play && let Ok(_) = video_play {
                self.playback_state.set(PlaybackState::Playing);
            }
        } else {
            self.playback_state.set(PlaybackState::Loading);
            self.load()?;
        }
        Ok(())
    }

    pub fn load(&self) -> Result<(), RustyTubeError> {
        let video: HtmlVideoElement = get_element_by_id(VIDEO_PLAYER_ID)?;
        let audio: HtmlAudioElement = get_element_by_id(AUDIO_PLAYER_ID)?;

        if video.network_state() != 2 || audio.network_state() != 2 {
            video.load();
            audio.load();
        }
        Ok(())
    }

    pub fn pause(&self) -> Result<(), RustyTubeError> {
        let video: HtmlVideoElement = get_element_by_id(VIDEO_PLAYER_ID)?;
        let audio: HtmlAudioElement = get_element_by_id(AUDIO_PLAYER_ID)?;

        video.pause()?;
        audio.pause()?;
        audio.set_current_time(video.current_time());
        self.playback_state.set(PlaybackState::Paused);
        Ok(())
    }

    pub async fn toggle_playback(&self) -> Result<(), RustyTubeError> {
        match self.playback_state.get() {
            PlaybackState::Playing => self.pause()?,
            PlaybackState::Paused => self.play().await?,
            PlaybackState::Loading => (),
        }
        Ok(())
    }

    pub async fn toggle_buffering(&self, buffering: bool) -> Result<(), RustyTubeError> {
        match buffering {
            true => {
                self.buffering.set(true);
                self.pause()?;
            }
            false => {
                self.buffering.set(false);
                self.play().await?;
            }
        }
        Ok(())
    }

    pub async fn toggle_video_ready(&self, ready: bool) -> Result<(), RustyTubeError> {
        self.video_ready.set(ready);
        if ready && self.playback_state.get() == PlaybackState::Loading && self.ready()? {
            self.play().await?
        }
        Ok(())
    }

    pub async fn toggle_audio_ready(&self, ready: bool) -> Result<(), RustyTubeError> {
        self.audio_ready.set(ready);
        if ready && self.playback_state.get() == PlaybackState::Loading && self.ready()? {
            self.play().await?
        }
        Ok(())
    }

    pub fn sync(&self) -> Result<(), RustyTubeError> {
        let video: HtmlVideoElement = get_element_by_id(VIDEO_PLAYER_ID)?;
        let audio: HtmlAudioElement = get_element_by_id(AUDIO_PLAYER_ID)?;

        audio.set_current_time(video.current_time());
        Ok(())
    }

    pub async fn seek(&self, time: f64) -> Result<(), RustyTubeError> {
        let video: HtmlVideoElement = get_element_by_id(VIDEO_PLAYER_ID)?;
        let audio: HtmlAudioElement = get_element_by_id(AUDIO_PLAYER_ID)?;

        video.fast_seek(time)?;
        audio.fast_seek(time)?;

        self.pause()?;
        self.toggle_video_ready(false).await?;
        self.toggle_video_ready(false).await?;
        self.playback_state.set(PlaybackState::Loading);
        self.current_time.set(time);
        self.current_time_str.set(utils::unix_to_hours_secs_mins(audio.current_time()));
        Ok(())
    }

    pub fn update_time(&self) -> Result<(), RustyTubeError> {
        let video: HtmlVideoElement = get_element_by_id(VIDEO_PLAYER_ID)?;

        let current_time = video.current_time();
        let total_time = video.duration();
        self.current_time.set(current_time);
        self.duration.set(total_time);
        self.current_time_str.set(utils::unix_to_hours_secs_mins(current_time));
        self.duration_str.set(utils::unix_to_hours_secs_mins(total_time));
        Ok(())
    }

    pub async fn change_quality(&self, format: VideoFormat) -> Result<(), RustyTubeError> {
        let video: HtmlVideoElement = get_element_by_id(VIDEO_PLAYER_ID)?;
        let audio: HtmlAudioElement = get_element_by_id(AUDIO_PLAYER_ID)?;

        let current_time = video.current_time();
        video.set_src(&format.url);
        self.toggle_video_ready(false).await?;
        self.toggle_audio_ready(false).await?;
        self.toggle_buffering(true).await?;
        self.format.set(Some(format));
        video.set_current_time(current_time);
        audio.set_current_time(current_time);
        self.load()?;
        self.play().await?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct VideoTime {
	pub current: RwSignal<String>,
	pub total: RwSignal<String>,
	pub current_ms: RwSignal<f64>,
	pub total_ms: RwSignal<f64>
}

#[derive(Clone, Copy)]
pub struct PlayerStyle {
    pub controls_visible: RwSignal<bool>,
    pub full_window: RwSignal<bool>,
    pub fullscreen: RwSignal<bool>
}

impl PlayerStyle {
    pub fn init() -> Self {
        let controls_visible = create_rw_signal(false);
        let full_window = create_rw_signal(false);
        let fullscreen = create_rw_signal(false);

        Self { controls_visible, full_window, fullscreen }
    }
}

pub fn provide_player_contexts() {
    let player_state = PlayerState::init();
    let player_style = PlayerStyle::init();

    provide_context(player_state);
    provide_context(player_style);
}
