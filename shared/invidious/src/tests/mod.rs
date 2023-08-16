#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use gloo::file::{Blob, File};
    use crate::channel::Channel;
    use crate::comments::Comments;
    use crate::fetch::fetch;
    use crate::hidden::CountryCode;
    use crate::subs::{NewpipeSubscriptions, Subscriptions, YoutubeSubscriptions};
    use crate::universal::{Duration, Feature, Playlist, Popular, ResponseType, Search, SearchArgs, Sort, TimeSpan, Trending, LocalPlaylist, CsvPlaylist, read_playlist_csv, read_libretube_playlists, read_freetube_playlists};
    use crate::universal::TrendingCategory::{Default, Gaming, Movies, Music, News};
    use crate::video::Video;

    const TEST_SERVER: &'static str = "https://iv.nboeck.de";
    const TEST_VIDEO: &'static str = "wsmHCfSZM70";
    const TEST_CHANNEL: &'static str = "UC7YOGHUfC1Tb6E4pudI9STA";
    const TEST_VIDEO_COMMENTS: &'static str = "sjC9rxq0LMc";
    const TEST_PLAYLIST: &'static str = "PLMogWd-g0jAM34EC316Y7UT9-xp_mcAke";
    const TEST_REGION: CountryCode = CountryCode::IE;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn can_fetch_api_data() {
        fetch(&format!("{}/api/v1/videos/{}", TEST_SERVER, TEST_VIDEO)).await.unwrap();
    }

    #[wasm_bindgen_test]
    async fn get_video() {
        let video = Video::fetch_video(TEST_SERVER, TEST_VIDEO, None).await.unwrap();

        let local_json = include_str!("./files/video.json");
        let local: Video = serde_json::from_str(local_json).unwrap();

        assert_eq!(local, video)
    }

    #[wasm_bindgen_test]
    async fn get_trending() {
        Trending::fetch_trending(TEST_SERVER, Default, TEST_REGION).await.unwrap();
        Trending::fetch_trending(TEST_SERVER, Music, TEST_REGION).await.unwrap();
        Trending::fetch_trending(TEST_SERVER, Gaming, TEST_REGION).await.unwrap();
        Trending::fetch_trending(TEST_SERVER, News, TEST_REGION).await.unwrap();
        Trending::fetch_trending(TEST_SERVER, Movies, TEST_REGION).await.unwrap();
    }

    #[wasm_bindgen_test]
    async fn get_popular() {
        Popular::fetch_popular(TEST_SERVER).await.unwrap();
    }

    #[wasm_bindgen_test]
    async fn search() {
        let mut args = SearchArgs {
            page: 1,
            query: "".to_string(),
            sort: Sort::Relevance,
            timespan: None,
            duration: None,
            response_type: None,
            features: None,
            region: CountryCode::IE,
        };
        Search::search(TEST_SERVER, &args).await.unwrap();
        args.timespan = Some(TimeSpan::Year);
        Search::search(TEST_SERVER, &args).await.unwrap();
        args.duration = Some(Duration::Long);
        Search::search(TEST_SERVER, &args).await.unwrap();
        args.response_type = Some(ResponseType::All);
        Search::search(TEST_SERVER, &args).await.unwrap();
        args.features = Some(vec![Feature::_4K, Feature::Subtitles, Feature::HighDynamicRange]);
        Search::search(TEST_SERVER, &args).await.unwrap();
    }

    #[wasm_bindgen_test]
    async fn get_channel() {
        let channel = Channel::fetch_channel(TEST_SERVER, TEST_CHANNEL,None).await.unwrap();

        let local_json = include_str!("./files/channel.json");
        let local: Channel = serde_json::from_str(local_json).unwrap();

        assert_eq!(local, channel)
    }

    #[wasm_bindgen_test]
    async fn get_comments() {
        let comments = Comments::fetch_comments(TEST_SERVER, TEST_VIDEO_COMMENTS, None).await.unwrap();

        let local_json = include_str!("./files/comments.json");
        let local: Comments = serde_json::from_str(local_json).unwrap();
    }

    #[wasm_bindgen_test]
    async fn get_playlist() {
        let playlist = Playlist::fetch_playlist(TEST_SERVER, TEST_PLAYLIST, None).await.unwrap();

        let local_json = include_str!("./files/playlist.json");
        let local: Playlist = serde_json::from_str(local_json).unwrap();

        assert_eq!(playlist.id, local.id);
        assert_eq!(playlist.author_id, local.author_id);
    }

    // #[wasm_bindgen_test]
    // async fn read_playlists() {
    //     let playlist_csv = Blob::new(include_bytes!("files/playlist.csv"));
    //     let playlist = LocalPlaylist::read_playlists(playlist_csv).await.unwrap();
    // }

    #[wasm_bindgen_test]
    async fn parse_csv_playlist() {
        let csv_bytes = include_bytes!("files/playlist.csv");
        let playlist = read_playlist_csv("test_csv", csv_bytes);
    }

    #[wasm_bindgen_test]
    async fn parse_libretube_playlists_json() {
        let libretube_json = include_str!("files/libretube_playlists.json");
        let playlist = read_libretube_playlists(libretube_json).await.unwrap();
    }

    #[wasm_bindgen_test]
    async fn parse_freetube_playlists_json() {
        let freetube_json = include_str!("files/freetube_playlists.json");
        let playlist = read_freetube_playlists(freetube_json).await.unwrap();
    }

    #[wasm_bindgen_test]
    async fn read_newpipe_json_subs() {
        let subs_json = include_str!("./files/subscriptions.json");
        let np_subs: NewpipeSubscriptions = NewpipeSubscriptions::read_subs_from_file(subs_json).unwrap();
        let subs: Subscriptions = np_subs.into();
    }

    #[wasm_bindgen_test]
    async fn read_youtube_csv_subs() {
        let subs_json: &[u8] = include_bytes!("./files/subscriptions.csv");
        let yt_subs: YoutubeSubscriptions = YoutubeSubscriptions::read_subs_from_csv(subs_json).unwrap();
        assert_eq!(yt_subs.subscriptions.len(), 54);
        let subs: Subscriptions = yt_subs.into();
        assert_eq!(subs.channels.len(), 54);
    }
}
