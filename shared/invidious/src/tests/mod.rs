// #[cfg(test)]
// mod tests {
// 	use gloo::file::Blob;
// 	use wasm_bindgen_test::{console_log, wasm_bindgen_test, wasm_bindgen_test_configure};

// 	use crate::{
// 		channel::Channel,
// 		fetch::fetch,
// 		formats::{
// 			AdaptiveFormat, AudioFormat, Container, Formats, LegacyFormat, Resolution, VideoFormat,
// 			VideoQuality,
// 		},
// 		hidden::CountryCode,
// 		instance::fetch_instance_info,
// 		subs::{NewpipeSubscriptions, Subscriptions, YoutubeSubscriptions},
// 		universal::{
// 			read_freetube_playlists, read_libretube_playlists, read_playlist_csv, LocalPlaylist,
// 			Playlist, Popular, SearchArgs, Trending,
// 			TrendingCategory::{Default, Gaming, Movies, Music},
// 		},
// 		video::Video,
// 		Comments, Replies, SearchResults,
// 	};

// 	const TEST_SERVER: &'static str = "https://iv.nboeck.de";
// 	const TEST_VIDEO: &'static str = "wsmHCfSZM70";
// 	const TEST_CHANNEL: &'static str = "UC7YOGHUfC1Tb6E4pudI9STA";
// 	const TEST_VIDEO_COMMENTS: &'static str = "sjC9rxq0LMc";
// 	const TEST_PLAYLIST: &'static str = "PLMogWd-g0jAM34EC316Y7UT9-xp_mcAke";
// 	const TEST_REGION: CountryCode = CountryCode::IE;

// 	wasm_bindgen_test_configure!(run_in_browser);

// 	#[wasm_bindgen_test]
// 	async fn can_fetch_api_data() {
// 		fetch(&format!("{}/api/v1/videos/{}", TEST_SERVER, TEST_VIDEO)).await.unwrap();
// 	}

// 	#[wasm_bindgen_test]
// 	async fn can_fetch_instance_data() {
// 		fetch_instance_info().await.unwrap();
// 	}

// 	#[wasm_bindgen_test]
// 	async fn get_video() {
// 		let video = Video::fetch_video(TEST_SERVER, TEST_VIDEO).await.unwrap();
// 	}

// 	#[wasm_bindgen_test]
// 	async fn parse_formats() {
// 		let video: Video = serde_json::from_str(include_str!("./files/video.json")).unwrap();
// 		let formats: Formats = Formats::from((video.adaptive_formats, video.format_streams));

// 		assert_eq!(formats.audio_formats.len(), 7);
// 		assert_eq!(formats.legacy_formats.len(), 3);
// 		assert_eq!(formats.video_formats.len(), 16);
// 	}

// 	#[wasm_bindgen_test]
// 	async fn parse_all_formats() {
// 		let video: Video = serde_json::from_str(include_str!("./files/video.json")).unwrap();
// 		let formats: Formats = Formats::from((video.adaptive_formats, video.format_streams));

// 		assert_eq!(formats.video_formats.len(), 16);
// 		assert_eq!(formats.audio_formats.len(), 7);
// 		assert_eq!(formats.legacy_formats.len(), 3);
// 	}

// 	#[wasm_bindgen_test]
// 	async fn parse_video_format() {
// 		let video_formats: Vec<VideoFormat> =
// 			serde_json::from_str(include_str!("./files/video_format.json")).unwrap();

// 		assert_eq!(video_formats.len(), 2);

// 		let first = video_formats.first().unwrap().clone();
// 		assert_eq!(first.resolution, Resolution::_144p);
// 		assert_eq!(first.quality_label, VideoQuality::_144p);
// 		assert_eq!(first.container.unwrap(), Container::MP4);

// 		let second = video_formats.last().unwrap().clone();
// 		assert_eq!(second.resolution, Resolution::_144p);
// 		assert_eq!(second.quality_label, VideoQuality::_144p);
// 		assert_eq!(second.container.unwrap(), Container::WEBM);
// 	}

// 	#[wasm_bindgen_test]
// 	async fn parse_adaptive_format_to_video_format() {
// 		let adaptive_formats: Vec<AdaptiveFormat> =
// 			serde_json::from_str(include_str!("./files/video_format.json")).unwrap();
// 		let mut video_formats: Vec<VideoFormat> = vec![];

// 		adaptive_formats.into_iter().for_each(|adaptive_format| {
// 			video_formats.push(VideoFormat::try_from(adaptive_format).unwrap());
// 		});

// 		assert_eq!(video_formats.len(), 2)
// 	}

// 	#[wasm_bindgen_test]
// 	async fn parse_audio_format() {
// 		let audio_formats: Vec<AudioFormat> =
// 			serde_json::from_str(include_str!("./files/audio_format.json")).unwrap();

// 		assert_eq!(audio_formats.len(), 7)
// 	}

// 	#[wasm_bindgen_test]
// 	async fn parse_legacy_format() {
// 		let legacy_formats: Vec<LegacyFormat> =
// 			serde_json::from_str(include_str!("./files/legacy_format.json")).unwrap();

// 		assert_eq!(legacy_formats.len(), 3)
// 	}

// 	#[wasm_bindgen_test]
// 	async fn get_trending() {
// 		Trending::fetch_trending(TEST_SERVER, Default, TEST_REGION).await.unwrap();
// 		Trending::fetch_trending(TEST_SERVER, Music, TEST_REGION).await.unwrap();
// 		Trending::fetch_trending(TEST_SERVER, Gaming, TEST_REGION).await.unwrap();
// 		Trending::fetch_trending(TEST_SERVER, Movies, TEST_REGION).await.unwrap();
// 	}

// 	#[wasm_bindgen_test]
// 	async fn get_popular() {
// 		Popular::fetch_popular(TEST_SERVER).await.unwrap();
// 	}

// 	#[wasm_bindgen_test]
// 	async fn search() {
// 		let args = SearchArgs::from_str("test".to_string());
// 		let search = SearchResults::fetch_search_results(TEST_SERVER, args, 1).await.unwrap();
// 		assert_ne!(0, search.items.len());

// 		// Search::search(TEST_SERVER, &args).await.unwrap();
// 		// args.timespan = Some(TimeSpan::Year);
// 		// Search::search(TEST_SERVER, &args).await.unwrap();
// 		// args.duration = Some(Duration::Long);
// 		// Search::search(TEST_SERVER, &args).await.unwrap();
// 		// args.response_type = Some(ResponseType::All);
// 		// Search::search(TEST_SERVER, &args).await.unwrap();
// 		// args.features = Some(vec![Feature::_4K, Feature::Subtitles, Feature::HighDynamicRange]);
// 		// Search::search(TEST_SERVER, &args).await.unwrap();
// 	}

// 	#[wasm_bindgen_test]
// 	async fn get_channel() {
// 		let channel = Channel::fetch_channel(TEST_SERVER, TEST_CHANNEL).await.unwrap();

// 		let local_json = include_str!("./files/channel.json");
// 		let local: Channel = serde_json::from_str(local_json).unwrap();

// 		assert_eq!(local, channel)
// 	}

// 	#[wasm_bindgen_test]
// 	async fn get_comments() {
// 		let comments =
// 			Comments::fetch_comments(TEST_SERVER, TEST_VIDEO_COMMENTS, None).await.unwrap();

// 		let local_json = include_str!("./files/comments.json");
// 		let local: Comments = serde_json::from_str(local_json).unwrap();
// 	}

// 	#[wasm_bindgen_test]
// 	async fn get_comment_replies() {
// 		let comments =
// 			Comments::fetch_comments(TEST_SERVER, TEST_VIDEO_COMMENTS, None).await.unwrap();

// 		let first_comment = comments.comments.first().unwrap();
// 		let first_comment_replies_info = first_comment.replies_info.clone().unwrap();
// 		let replies = Replies::fetch_replies(
// 			&first_comment_replies_info.continuation,
// 			TEST_SERVER,
// 			&first_comment.id,
// 		)
// 		.await
// 		.unwrap();

// 		assert_eq!(replies.comments.len(), 10);
// 	}

// 	#[wasm_bindgen_test]
// 	async fn get_playlist() {
// 		let playlist = Playlist::fetch_playlist(TEST_SERVER, TEST_PLAYLIST, None).await.unwrap();

// 		let local_json = include_str!("./files/playlist.json");
// 		let local: Playlist = serde_json::from_str(local_json).unwrap();

// 		assert_eq!(playlist.id, local.id);
// 		assert_eq!(playlist.author_id, local.author_id);
// 	}

// 	#[wasm_bindgen_test]
// 	async fn read_playlists() {
// 		let playlist_bytes = include_bytes!("files/playlist.csv");
// 		let playlist_csv = Blob::new(playlist_bytes.as_slice());
// 		let playlist = LocalPlaylist::read_playlists(playlist_csv).await.unwrap();
// 	}

// 	#[wasm_bindgen_test]
// 	async fn parse_csv_playlist() {
// 		let csv_bytes = include_bytes!("files/playlist.csv");
// 		let playlist =
// 			read_playlist_csv(&utils::get_current_time().to_string(), csv_bytes).await.unwrap();

// 		let first_playlist_item = playlist.videos.first().unwrap().clone().id;
// 		let last_playlist_item = playlist.videos.last().unwrap().clone().id;
// 		assert_eq!(first_playlist_item, "E2hZDzJp9Pc");
// 		assert_eq!(last_playlist_item, "fBYVlFXsEME");
// 	}

// 	#[wasm_bindgen_test]
// 	async fn parse_libretube_playlists_json() {
// 		let libretube_json = include_str!("files/libretube_playlists.json");
// 		let playlist = read_libretube_playlists(libretube_json).await.unwrap();
// 	}

// 	#[wasm_bindgen_test]
// 	async fn parse_freetube_playlists_json() {
// 		let freetube_json = include_str!("files/freetube_playlists.json");
// 		let playlist = read_freetube_playlists(freetube_json).await.unwrap();
// 	}

// 	#[wasm_bindgen_test]
// 	async fn read_newpipe_json_subs() {
// 		let subs_json = include_str!("./files/subscriptions.json");
// 		let np_subs: NewpipeSubscriptions =
// 			NewpipeSubscriptions::read_subs_from_file(subs_json).unwrap();
// 		let subs: Subscriptions = np_subs.into();
// 	}

// 	#[wasm_bindgen_test]
// 	async fn read_youtube_csv_subs() {
// 		let subs_json: &[u8] = include_bytes!("./files/subscriptions.csv");
// 		let yt_subs: YoutubeSubscriptions =
// 			YoutubeSubscriptions::read_subs_from_csv(subs_json).unwrap();
// 		assert_eq!(yt_subs.subscriptions.len(), 54);
// 		let subs: Subscriptions = yt_subs.into();
// 		assert_eq!(subs.channels.len(), 54);
// 	}

// 	#[wasm_bindgen_test]
// 	async fn can_fetch_subs() {
// 		let mut fail = 0;
// 		let mut success = 0;

// 		let subs_json: &[u8] = include_bytes!("./files/subscriptions.csv");
// 		let yt_subs: YoutubeSubscriptions =
// 			YoutubeSubscriptions::read_subs_from_csv(subs_json).unwrap();
// 		let subs: Subscriptions = yt_subs.into();
// 		let subs_videos = subs.fetch_videos(TEST_SERVER, false).await.unwrap();

// 		subs_videos.into_iter().for_each(|sub_videos| match sub_videos {
// 			Ok(_) => success = success + 1,
// 			Err(_) => fail = fail + 1,
// 		});

// 		console_log!("JSON Sub Fetch Fail: {}", fail);
// 		console_log!("JSON Sub Fetch Success: {}", success);
// 	}

// 	#[wasm_bindgen_test]
// 	async fn can_fetch_subs_with_rss() {
// 		let mut fail = 0;
// 		let mut success = 0;

// 		let subs_json: &[u8] = include_bytes!("./files/subscriptions.csv");
// 		let yt_subs: YoutubeSubscriptions =
// 			YoutubeSubscriptions::read_subs_from_csv(subs_json).unwrap();
// 		let subs: Subscriptions = yt_subs.into();

// 		let subs_videos = subs.fetch_videos(TEST_SERVER, true).await.unwrap();
// 		subs_videos.into_iter().for_each(|sub_videos| match sub_videos {
// 			Ok(videos) => success = success + 1,
// 			Err(_) => fail = fail + 1,
// 		});

// 		console_log!("RSS Sub Fetch Fail: {}", fail);
// 		console_log!("RSS Sub Fetch Success: {}", success);
// 	}
// }
