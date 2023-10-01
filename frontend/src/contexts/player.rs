use leptos::{*, html::{Video, Audio}};
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
    video_player: NodeRef<Video>,
    audio_player: NodeRef<Audio>,
    format: RwSignal<Option<VideoFormat>>,
    pub playback_state: RwSignal<PlaybackState>,
    video_ready: RwSignal<bool>,
    audio_ready: RwSignal<bool>,
    pub volume: RwSignal<f64>,
    pub current_time: RwSignal<f64>,
    pub duration: RwSignal<f64>,
    pub current_time_str: RwSignal<String>,
    pub duration_str: RwSignal<String>,
}

impl PlayerState {
    pub fn init(video_player: NodeRef<Video>, audio_player: NodeRef<Audio>) -> Self {
        let format = create_rw_signal(None);
        let playback_state = create_rw_signal(PlaybackState::Paused);
        let video_ready = create_rw_signal(false);
        let audio_ready = create_rw_signal(false);
        let volume = create_rw_signal(expect_context::<VolumeCtx>().0.0.get());
        let current_time_str = create_rw_signal(String::from("0:00"));
        let duration_str = create_rw_signal(String::from("0:00"));
        let current_time = create_rw_signal(0f64);
        let duration = create_rw_signal(0f64);

        Self {
            video_player,
            audio_player,
            format,
            playback_state,
            video_ready,
            audio_ready,
            volume,
            current_time,
            current_time_str,
            duration, 
            duration_str
        }
    }

    pub fn ready(&self) -> Result<bool, RustyTubeError> {
        let video = self.video_player.get().ok_or(RustyTubeError::element_not_found(VIDEO_PLAYER_ID))?;
        let audio = self.audio_player.get().ok_or(RustyTubeError::element_not_found(AUDIO_PLAYER_ID))?;

        let ready = self.audio_ready.get() && self.video_ready.get();
        Ok(ready)
    }

    pub fn play(&self) -> Result<(), RustyTubeError> {
        let video = self.video_player.get().ok_or(RustyTubeError::element_not_found(VIDEO_PLAYER_ID))?;
        let audio = self.audio_player.get().ok_or(RustyTubeError::element_not_found(AUDIO_PLAYER_ID))?;

        if self.playback_state.get() != PlaybackState::Playing && self.ready()? {
            audio.set_volume(self.volume.get());
            let video_play = video.play();
            audio.set_current_time(video.current_time());
            let audio_play = audio.play();
            if let Ok(_) = audio_play && let Ok(_) = video_play {
                self.playback_state.set(PlaybackState::Playing);
            }
        }
        Ok(())
    }

    pub fn load(&self) -> Result<(), RustyTubeError> {
        let video = self.video_player.get().ok_or(RustyTubeError::element_not_found(VIDEO_PLAYER_ID))?;
        let audio = self.audio_player.get().ok_or(RustyTubeError::element_not_found(AUDIO_PLAYER_ID))?;

        if video.network_state() != 2 || audio.network_state() != 2 {
            video.load();
            audio.load();
        }
        Ok(())
    }

    pub fn pause(&self) -> Result<(), RustyTubeError> {
        let video = self.video_player.get().ok_or(RustyTubeError::element_not_found(VIDEO_PLAYER_ID))?;
        let audio = self.audio_player.get().ok_or(RustyTubeError::element_not_found(AUDIO_PLAYER_ID))?;
        let video_pause = video.pause();
        let audio_pause = audio.pause();
        if let Ok(_) = video_pause && let Ok(_) = audio_pause {
            self.playback_state.set(PlaybackState::Paused);
        }
        Ok(())
    }

    pub async fn toggle_playback(&self) -> Result<(), RustyTubeError> {
        match self.playback_state.get() {
            PlaybackState::Playing => self.pause()?,
            PlaybackState::Paused => self.play()?,
            PlaybackState::Loading => (),
        }
        Ok(())
    }

    pub fn set_video_ready(&self, ready: bool) {
        self.video_ready.set(ready);
        if ready && self.audio_ready.get() {
            self.play();
        }
    }

    pub fn set_audio_ready(&self, ready: bool) {
        self.audio_ready.set(ready);
        if ready && self.video_ready.get() {
            self.play();
        }
    }

    pub fn sync(&self) -> Result<(), RustyTubeError> {
        let video = self.video_player.get().ok_or(RustyTubeError::element_not_found(VIDEO_PLAYER_ID))?;
        let audio = self.audio_player.get().ok_or(RustyTubeError::element_not_found(AUDIO_PLAYER_ID))?;

        audio.set_current_time(video.current_time());
        Ok(())
    }

    pub fn seek(&self, time: f64) -> Result<(), RustyTubeError> {
        let video = self.video_player.get().ok_or(RustyTubeError::element_not_found(VIDEO_PLAYER_ID))?;
        let audio = self.audio_player.get().ok_or(RustyTubeError::element_not_found(AUDIO_PLAYER_ID))?;

        let fast_seek_video = video.fast_seek(time);
        let fast_seek_audio = audio.fast_seek(time);

        if fast_seek_audio.is_err() || fast_seek_video.is_err() {
            video.set_current_time(time);
            audio.set_current_time(time);
        }

        self.pause()?;
        self.set_video_ready(false);
        self.set_audio_ready(false);
        self.playback_state.set(PlaybackState::Loading);
        video.set_current_time(audio.current_time());
        self.current_time.set(time);
        self.current_time_str.set(utils::unix_to_hours_secs_mins(audio.current_time()));
        Ok(())
    }

    pub fn update_time(&self) -> Result<(), RustyTubeError> {
        let video = self.video_player.get().ok_or(RustyTubeError::element_not_found(VIDEO_PLAYER_ID))?;

        let current_time = video.current_time();
        let total_time = video.duration();
        self.current_time.set(current_time);
        self.duration.set(total_time);
        self.current_time_str.set(utils::unix_to_hours_secs_mins(current_time));
        self.duration_str.set(utils::unix_to_hours_secs_mins(total_time));
        Ok(())
    }

    pub async fn change_quality(&self, format: VideoFormat) -> Result<(), RustyTubeError> {
        let video = self.video_player.get().ok_or(RustyTubeError::element_not_found(VIDEO_PLAYER_ID))?;
        let audio = self.audio_player.get().ok_or(RustyTubeError::element_not_found(AUDIO_PLAYER_ID))?;

        let current_time = video.current_time();
        video.set_src(&format.url);
        self.pause()?;
        self.set_video_ready(false);
        self.playback_state.set(PlaybackState::Loading);
        self.format.set(Some(format));
        video.set_current_time(current_time);
        audio.set_current_time(current_time);
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





















































































