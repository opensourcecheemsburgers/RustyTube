use std::ops::RangeBounds;

use invidious::Format;
use leptos::{
	create_rw_signal, error::Result, expect_context, web_sys, RwSignal,
	SignalGet, SignalSet,
};
use rustytube_error::RustyTubeError;
use utils::get_element_by_id;
use web_sys::{HtmlAudioElement, HtmlVideoElement};

use crate::{
	contexts::PlayerConfigCtx,
	resources::SponsorBlockResource,
	utils::{i18n, is_webkit},
};

use super::{toast, Toast};

pub const VIDEO_CONTAINER_ID: &str = "video_container";
pub const VIDEO_PLAYER_ID: &str = "video_player";
pub const VIDEO_CONTROLS_ID: &str = "video_controls";
pub const AUDIO_PLAYER_ID: &str = "audio_player";

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PlaybackState {
	Playing,
	Paused,
	Loading,
	Initial,
}

#[derive(Clone, Copy)]
pub struct PlayerState {
	format: RwSignal<Option<Format>>,
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
	pub fn init() -> Self {
		let format = create_rw_signal(None);
		let playback_state = create_rw_signal(PlaybackState::Initial);
		let video_ready = create_rw_signal(false);
		let audio_ready = create_rw_signal(false);
		let volume = create_rw_signal(
			expect_context::<PlayerConfigCtx>().volume_slice.0.get(),
		);
		let current_time_str = create_rw_signal(String::from("0:00"));
		let duration_str = create_rw_signal(String::from("0:00"));
		let current_time = create_rw_signal(0f64);
		let duration = create_rw_signal(0f64);

		Self {
			format,
			playback_state,
			video_ready,
			audio_ready,
			volume,
			current_time,
			duration,
			current_time_str,
			duration_str,
		}
	}

	pub fn ready(&self) -> Result<bool, RustyTubeError> {
		let video = get_element_by_id::<HtmlVideoElement>(VIDEO_PLAYER_ID)?;
		let audio = get_element_by_id::<HtmlAudioElement>(AUDIO_PLAYER_ID)?;

		let ready = if is_webkit() {
			if self.format.get().map_or(false, |format| format.is_audio_only())
			{
				audio.ready_state() >= 3
			} else if self
				.format
				.get()
				.map_or(false, |format| format.is_legacy())
			{
				video.ready_state() >= 3
			} else {
				video.ready_state() >= 3 && audio.ready_state() >= 3
			}
		} else if self
			.format
			.get()
			.map_or(false, |format| format.is_audio_only())
		{
			self.audio_ready.get() && audio.ready_state() >= 3
		} else if self.format.get().map_or(false, |format| format.is_legacy()) {
			self.video_ready.get() && video.ready_state() >= 3
		} else {
			self.video_ready.get()
				&& self.audio_ready.get()
				&& video.ready_state() >= 3
				&& audio.ready_state() >= 3
		};

		Ok(ready)
	}

	pub fn play(&self) -> Result<(), RustyTubeError> {
		let video = get_element_by_id::<HtmlVideoElement>(VIDEO_PLAYER_ID)?;
		let audio = get_element_by_id::<HtmlAudioElement>(AUDIO_PLAYER_ID)?;

		if self.ready()? {
			audio.set_volume(self.volume.get());
			let video_play = video.play();
			video.set_current_time(audio.current_time());
			let audio_play = audio.play();

			if audio_play.is_ok() && video_play.is_ok() {
				self.playback_state.set(PlaybackState::Playing);
			}
		}

		Ok(())
	}

	pub fn resume(&self) -> Result<(), RustyTubeError> {
		let video = get_element_by_id::<HtmlVideoElement>(VIDEO_PLAYER_ID)?;
		let audio = get_element_by_id::<HtmlAudioElement>(AUDIO_PLAYER_ID)?;

		if self.playback_state.get() == PlaybackState::Loading
			|| self.playback_state.get() == PlaybackState::Paused
		{
			if is_webkit() {
				let video_play = video.play();
				let audio_play = audio.play();

				if audio_play.is_ok() && video_play.is_ok() {
					self.playback_state.set(PlaybackState::Playing);
				}
			} else {
				audio.set_volume(self.volume.get());
				let video_play = video.play();
				audio.set_current_time(video.current_time());
				let audio_play = audio.play();

				if audio_play.is_ok() && video_play.is_ok() {
					self.playback_state.set(PlaybackState::Playing);
				}
			}
		}
		Ok(())
	}

	pub fn pause(&self) -> Result<(), RustyTubeError> {
		let video = get_element_by_id::<HtmlVideoElement>(VIDEO_PLAYER_ID)?;
		let audio = get_element_by_id::<HtmlAudioElement>(AUDIO_PLAYER_ID)?;
		let video_pause = video.pause();
		let audio_pause = audio.pause();
		if audio_pause.is_ok() && video_pause.is_ok() {
			self.playback_state.set(PlaybackState::Paused);
		}
		Ok(())
	}

	pub fn toggle_playback(&self) -> Result<(), RustyTubeError> {
		match self.playback_state.get() {
			PlaybackState::Playing => self.pause()?,
			PlaybackState::Paused => self.resume()?,
			PlaybackState::Loading => (),
			PlaybackState::Initial => self.play()?,
		}
		Ok(())
	}

	pub fn set_video_ready(&self, ready: bool) -> Result<(), RustyTubeError> {
		self.video_ready.set(ready);
		if self.ready()? && self.playback_state.get() != PlaybackState::Initial
		{
			self.resume()?;
		}
		Ok(())
	}

	pub fn set_audio_ready(&self, ready: bool) -> Result<(), RustyTubeError> {
		self.audio_ready.set(ready);
		if self.ready()? && self.playback_state.get() != PlaybackState::Initial
		{
			self.resume()?;
		}
		Ok(())
	}

	pub fn sync(&self) -> Result<(), RustyTubeError> {
		if let Some(Some(format)) = self.format.try_get() {
			if !format.is_audio_only() && !format.is_legacy() {
				let video =
					get_element_by_id::<HtmlVideoElement>(VIDEO_PLAYER_ID)?;
				let audio =
					get_element_by_id::<HtmlAudioElement>(AUDIO_PLAYER_ID)?;

				let video_time = video.current_time();
				let audio_time = audio.current_time();

				let initial_start = video_time < 3.0 || audio_time < 3.0;
				let out_of_sync = video_time > audio_time + 0.125
					|| video_time + 0.125 < audio_time;
				if !initial_start && out_of_sync {
					video.set_current_time(audio_time);
				}
			}
		}

		Ok(())
	}

	pub fn seek(&self, time: f64) -> Result<(), RustyTubeError> {
		let video = get_element_by_id::<HtmlVideoElement>(VIDEO_PLAYER_ID)?;
		let audio = get_element_by_id::<HtmlAudioElement>(AUDIO_PLAYER_ID)?;

		self.pause()?;
		self.set_video_ready(false)?;
		self.playback_state.set(PlaybackState::Loading);

		if is_webkit() {
			video.set_current_time(time);
			audio.set_current_time(time);
		} else {
			self.set_audio_ready(false)?;
			let fast_seek_video = video.fast_seek(time);
			let fast_seek_audio = audio.fast_seek(time);
			if fast_seek_audio.is_err() || fast_seek_video.is_err() {
				video.set_current_time(time);
				audio.set_current_time(time);
			}
		}

		self.current_time.set(time);
		self.current_time_str.set(utils::unix_to_hours_secs_mins(time));
		self.play()?;
		Ok(())
	}

	pub fn update_time(&self) -> Result<(), RustyTubeError> {
		let video = get_element_by_id::<HtmlVideoElement>(VIDEO_PLAYER_ID)?;

		let current_time = video.current_time();
		let total_time = video.duration();
		self.current_time.set(current_time);
		self.duration.set(total_time);
		self.current_time_str.set(utils::unix_to_hours_secs_mins(current_time));
		self.duration_str.set(utils::unix_to_hours_secs_mins(total_time));
		self.check_sponsorblock(current_time);
		Ok(())
	}

	pub fn change_format(&self, format: Format) -> Result<(), RustyTubeError> {
		let video = get_element_by_id::<HtmlVideoElement>(VIDEO_PLAYER_ID)?;
		let audio = get_element_by_id::<HtmlAudioElement>(AUDIO_PLAYER_ID)?;

		let current_time = video.current_time();
		video.set_src(&format.video_url().unwrap_or_default());
		audio.set_src(&format.audio_url().unwrap_or_default());
		self.pause()?;
		self.set_video_ready(false)?;
		self.playback_state.set(PlaybackState::Loading);
		self.format.set(Some(format));
		video.set_current_time(current_time);
		audio.set_current_time(current_time);
		self.current_time.set(current_time);
		self.current_time_str.set(utils::unix_to_hours_secs_mins(current_time));
		Ok(())
	}

	pub fn set_volume(&self, volume: f64) -> Result<(), RustyTubeError> {
		let video = get_element_by_id::<HtmlVideoElement>(VIDEO_PLAYER_ID)?;
		let audio = get_element_by_id::<HtmlAudioElement>(AUDIO_PLAYER_ID)?;

		video.set_volume(volume);
		audio.set_volume(volume);
		self.volume.set(volume);
		expect_context::<PlayerConfigCtx>().volume_slice.1.set(volume);

		Ok(())
	}

	pub fn check_sponsorblock(&self, time: f64) {
		if let Some(segments) =
			expect_context::<SponsorBlockResource>().get_segments()
		{
			for segment in segments {
				let range = (segment.timeframe.0.round() - 1f64)
					..=(segment.timeframe.0.round() + 1f64);
				if range.contains(&time) {
					self.seek(segment.timeframe.1);
					toast(Toast::new(
						i18n("sponsorblock.skipped")(),
						Some(super::ToastDuration::Normal),
						Some(super::ToastType::Info),
					));
				}
			}
		}
	}
}

#[derive(Clone, Copy)]
pub struct VideoTime {
	pub current: RwSignal<String>,
	pub total: RwSignal<String>,
	pub current_ms: RwSignal<f64>,
	pub total_ms: RwSignal<f64>,
}

#[derive(Clone, Copy)]
pub struct PlayerStyle {
	pub controls_visible: RwSignal<bool>,
	pub full_window: RwSignal<bool>,
	pub fullscreen: RwSignal<bool>,
}

impl PlayerStyle {
	pub fn init() -> Self {
		let controls_visible = create_rw_signal(false);
		let full_window = create_rw_signal(false);
		let fullscreen = create_rw_signal(false);

		Self { controls_visible, full_window, fullscreen }
	}
}
