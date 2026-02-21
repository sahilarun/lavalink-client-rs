use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref DEFAULT_SOURCES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        // youtubemusic
        m.insert("youtube music", "ytmsearch");
        m.insert("youtubemusic", "ytmsearch");
        m.insert("ytmsearch", "ytmsearch");
        m.insert("ytm", "ytmsearch");
        m.insert("musicyoutube", "ytmsearch");
        m.insert("music youtube", "ytmsearch");
        // youtube
        m.insert("youtube", "ytsearch");
        m.insert("yt", "ytsearch");
        m.insert("ytsearch", "ytsearch");
        // soundcloud
        m.insert("soundcloud", "scsearch");
        m.insert("scsearch", "scsearch");
        m.insert("sc", "scsearch");
        // apple music
        m.insert("apple music", "amsearch");
        m.insert("apple", "amsearch");
        m.insert("applemusic", "amsearch");
        m.insert("amsearch", "amsearch");
        m.insert("am", "amsearch");
        m.insert("musicapple", "amsearch");
        m.insert("music apple", "amsearch");
        // spotify
        m.insert("spotify", "spsearch");
        m.insert("spsearch", "spsearch");
        m.insert("sp", "spsearch");
        m.insert("spotify.com", "spsearch");
        m.insert("spotifycom", "spsearch");
        m.insert("sprec", "sprec");
        m.insert("spsuggestion", "sprec");
        // deezer
        m.insert("deezer", "dzsearch");
        m.insert("dz", "dzsearch");
        m.insert("dzsearch", "dzsearch");
        m.insert("dzisrc", "dzisrc");
        m.insert("dzrec", "dzrec");
        // yandexmusic
        m.insert("yandex music", "ymsearch");
        m.insert("yandexmusic", "ymsearch");
        m.insert("yandex", "ymsearch");
        m.insert("ymsearch", "ymsearch");
        m.insert("ymrec", "ymrec");
        // VK Music (lavasrc)
        m.insert("vksearch", "vksearch");
        m.insert("vkmusic", "vksearch");
        m.insert("vk music", "vksearch");
        m.insert("vkrec", "vkrec");
        m.insert("vk", "vksearch");
        // Qobuz (lavasrc)
        m.insert("qbsearch", "qbsearch");
        m.insert("qobuz", "qbsearch");
        m.insert("qbisrc", "qbisrc");
        m.insert("qbrec", "qbrec");
        // pandora (lavasrc)
        m.insert("pandora", "pdsearch");
        m.insert("pd", "pdsearch");
        m.insert("pdsearch", "pdsearch");
        m.insert("pdisrc", "pdisrc");
        m.insert("pdrec", "pdrec");
        m.insert("pandora music", "pdsearch");
        m.insert("pandoramusic", "pdsearch");
        // speak PLUGIN
        m.insert("speak", "speak");
        m.insert("tts", "tts");
        m.insert("ftts", "ftts");
        m.insert("flowery", "ftts");
        m.insert("flowery.tts", "ftts");
        m.insert("flowerytts", "ftts");
        // Client sided search platforms (after lavalinkv4.0.6 it will search via bcsearch on the node itself)
        m.insert("bandcamp", "bcsearch");
        m.insert("bc", "bcsearch");
        m.insert("bcsearch", "bcsearch");
        // other searches:
        m.insert("phsearch", "phsearch");
        m.insert("pornhub", "phsearch");
        m.insert("porn", "phsearch");
        // local files
        m.insert("local", "local");
        // http requests
        m.insert("http", "http");
        m.insert("https", "https");
        m.insert("link", "link");
        m.insert("uri", "uri");
        // tidal
        m.insert("tidal", "tdsearch");
        m.insert("td", "tdsearch");
        m.insert("tidal music", "tdsearch");
        m.insert("tdsearch", "tdsearch");
        m.insert("tdrec", "tdrec");
        // jiosaavn
        m.insert("jiosaavn", "jssearch");
        m.insert("js", "jssearch");
        m.insert("jssearch", "jssearch");
        m.insert("jsrec", "jsrec");
        m
    };

    pub static ref YOUTUBE_REGEX: Regex = Regex::new(r"https?://(?:www\.)?(?:(m|www)\.)?(?:youtu\.be/|youtube\.com/(?:embed/|v/|shorts|playlist\?|watch\?v=|watch\?.+(?:&|&#38;);v=))([a-zA-Z0-9\-_]{11})?(?:(?:\?|&|&#38;)index=((?:\d){1,3}))?(?:(?:\?|&|&#38;)?list=([a-zA-Z\-_0-9]{34}))?(?:\S+)?").unwrap();
    pub static ref YOUTUBE_MUSIC_REGEX: Regex = Regex::new(r"https?://(?:www\.)?(?:(music|m|www)\.)?(?:youtu\.be/|youtube\.com/(?:embed/|v/|shorts|playlist\?|watch\?v=|watch\?.+(?:&|&#38;);v=))([a-zA-Z0-9\-_]{11})?(?:(?:\?|&|&#38;)index=((?:\d){1,3}))?(?:(?:\?|&|&#38;)?list=([a-zA-Z\-_0-9]{34}))?(?:\S+)?").unwrap();
    pub static ref SOUNDCLOUD_REGEX: Regex = Regex::new(r"https?://(?:on\.)?soundcloud\.com/").unwrap();
    pub static ref SOUNDCLOUD_MOBILE_REGEX: Regex = Regex::new(r"https?://(soundcloud\.app\.goo\.gl)/(\S+)").unwrap();
    pub static ref BANDCAMP_REGEX: Regex = Regex::new(r"https?://(?:www\.)?([\d|\w]+)\.bandcamp\.com/(\S+)").unwrap();
    pub static ref TWITCH_TV_REGEX: Regex = Regex::new(r"https?://(?:www\.)?twitch\.tv/\w+").unwrap();
    pub static ref VIMEO_REGEX: Regex = Regex::new(r"https?://(www\.)?vimeo.com/(?:channels/(?:\w+/)?|groups/([^/]*)/videos/|)(\d+)(?:|/\?)").unwrap();
    pub static ref MP3_URL_REGEX: Regex = Regex::new(r"(https?|ftp|file)://(www.)?(.*?)\.(mp3)$").unwrap();
    pub static ref M3U_URL_REGEX: Regex = Regex::new(r"(https?|ftp|file)://(www.)?(.*?)\.(m3u)$").unwrap();
    pub static ref M3U8_URL_REGEX: Regex = Regex::new(r"(https?|ftp|file)://(www.)?(.*?)\.(m3u8)$").unwrap();
    pub static ref MP4_URL_REGEX: Regex = Regex::new(r"(https?|ftp|file)://(www.)?(.*?)\.(mp4)$").unwrap();
    pub static ref M4A_URL_REGEX: Regex = Regex::new(r"(https?|ftp|file)://(www.)?(.*?)\.(m4a)$").unwrap();
    pub static ref WAV_URL_REGEX: Regex = Regex::new(r"(https?|ftp|file)://(www.)?(.*?)\.(wav)$").unwrap();
    pub static ref AACP_URL_REGEX: Regex = Regex::new(r"(https?|ftp|file)://(www.)?(.*?)\.(aacp)$").unwrap();
    pub static ref DEEZER_TRACK_REGEX: Regex = Regex::new(r"(https?://|)?(?:www\.)?deezer\.com/(?:\w{2}/)?track/(\d+)").unwrap();
    pub static ref DEEZER_PAGE_LINK_REGEX: Regex = Regex::new(r"(https?://|)?(?:www\.)?deezer\.page\.link/(\S+)").unwrap();
    pub static ref DEEZER_PLAYLIST_REGEX: Regex = Regex::new(r"(https?://|)?(?:www\.)?deezer\.com/(?:\w{2}/)?playlist/(\d+)").unwrap();
    pub static ref DEEZER_ALBUM_REGEX: Regex = Regex::new(r"(https?://|)?(?:www\.)?deezer\.com/(?:\w{2}/)?album/(\d+)").unwrap();
    pub static ref DEEZER_ARTIST_REGEX: Regex = Regex::new(r"(https?://|)?(?:www\.)?deezer\.com/(?:\w{2}/)?artist/(\d+)").unwrap();
    pub static ref DEEZER_MIXES_REGEX: Regex = Regex::new(r"(https?://|)?(?:www\.)?deezer\.com/(?:\w{2}/)?mixes/genre/(\d+)").unwrap();
    pub static ref DEEZER_EPISODE_REGEX: Regex = Regex::new(r"(https?://|)?(?:www\.)?deezer\.com/(?:\w{2}/)?episode/(\d+)").unwrap();
    pub static ref ALL_DEEZER_REGEX_WITHOUT_PAGE_LINK: Regex = Regex::new(r"(https?://|)?(?:www\.)?deezer\.com/(?:\w{2}/)?(track|playlist|album|artist|mixes/genre|episode)/(\d+)").unwrap();
    pub static ref ALL_DEEZER_REGEX: Regex = Regex::new(r"((https?://|)?(?:www\.)?deezer\.com/(?:\w{2}/)?(track|playlist|album|artist|mixes/genre|episode)/(\d+)|(https?://|)?(?:www\.)?deezer\.page\.link/(\S+))").unwrap();
    pub static ref SPOTIFY_SONG_REGEX: Regex = Regex::new(r"(https?://)(www\.)?open\.spotify\.com/((?P<region>[a-zA-Z-]+)/)?(user/(?P<user>[a-zA-Z0-9-_]+)/)?track/(?P<identifier>[a-zA-Z0-9-_]+)").unwrap();
    pub static ref SPOTIFY_PLAYLIST_REGEX: Regex = Regex::new(r"(https?://)(www\.)?open\.spotify\.com/((?P<region>[a-zA-Z-]+)/)?(user/(?P<user>[a-zA-Z0-9-_]+)/)?playlist/(?P<identifier>[a-zA-Z0-9-_]+)").unwrap();
    pub static ref SPOTIFY_ARTIST_REGEX: Regex = Regex::new(r"(https?://)(www\.)?open\.spotify\.com/((?P<region>[a-zA-Z-]+)/)?(user/(?P<user>[a-zA-Z0-9-_]+)/)?artist/(?P<identifier>[a-zA-Z0-9-_]+)").unwrap();
    pub static ref SPOTIFY_EPISODE_REGEX: Regex = Regex::new(r"(https?://)(www\.)?open\.spotify\.com/((?P<region>[a-zA-Z-]+)/)?(user/(?P<user>[a-zA-Z0-9-_]+)/)?episode/(?P<identifier>[a-zA-Z0-9-_]+)").unwrap();
    pub static ref SPOTIFY_SHOW_REGEX: Regex = Regex::new(r"(https?://)(www\.)?open\.spotify\.com/((?P<region>[a-zA-Z-]+)/)?(user/(?P<user>[a-zA-Z0-9-_]+)/)?show/(?P<identifier>[a-zA-Z0-9-_]+)").unwrap();
    pub static ref SPOTIFY_ALBUM_REGEX: Regex = Regex::new(r"(https?://)(www\.)?open\.spotify\.com/((?P<region>[a-zA-Z-]+)/)?(user/(?P<user>[a-zA-Z0-9-_]+)/)?album/(?P<identifier>[a-zA-Z0-9-_]+)").unwrap();
    pub static ref ALL_SPOTIFY_REGEX: Regex = Regex::new(r"(https?://)(www\.)?open\.spotify\.com/((?P<region>[a-zA-Z-]+)/)?(user/(?P<user>[a-zA-Z0-9-_]+)/)?(?P<type>track|album|playlist|artist|episode|show)/(?P<identifier>[a-zA-Z0-9-_]+)").unwrap();
    pub static ref APPLE_MUSIC_REGEX: Regex = Regex::new(r"https?://(?:www\.)?music\.apple\.com/(\S+)").unwrap();
    pub static ref TIDAL_REGEX: Regex = Regex::new(r"https?://(?:www\.)?(?:tidal|listen)\.tidal\.com/(?P<type>track|album|playlist|artist)/(?P<identifier>[a-zA-Z0-9-_]+)").unwrap();
    pub static ref JIOSAAVN_REGEX: Regex = Regex::new(r"(https?://)(www\.)?jiosaavn\.com/(?P<type>song|album|featured|artist)/([a-zA-Z0-9-_/,]+)").unwrap();
    pub static ref PANDORA_TRACK_REGEX: Regex = Regex::new(r"^@?(?:https?://)?(?:www\.)?pandora\.com/artist/[\w-]+(?:/[\w-]+)*/(?P<identifier>TR[A-Za-z0-9]+)(?:[?#].*)?$").unwrap();
    pub static ref PANDORA_ALBUM_REGEX: Regex = Regex::new(r"^@?(?:https?://)?(?:www\.)?pandora\.com/artist/[\w-]+(?:/[\w-]+)*/(?P<identifier>AL[A-Za-z0-9]+)(?:[?#].*)?$").unwrap();
    pub static ref PANDORA_ARTIST_REGEX: Regex = Regex::new(r"^@?(?:https?://)?(?:www\.)?pandora\.com/artist/[\w-]+/(?P<identifier>AR[A-Za-z0-9]+)(?:[?#].*)?$").unwrap();
    pub static ref PANDORA_PLAYLIST_REGEX: Regex = Regex::new(r"^@?(?:https?://)?(?:www\.)?pandora\.com/playlist/(?P<identifier>PL:[\d:]+)(?:[?#].*)?$").unwrap();
    pub static ref ALL_PANDORA_REGEX: Regex = Regex::new(r"^@?(?:https?://)?(?:www\.)?pandora\.com/(?:playlist/(?P<playlistId>PL:[\d:]+)|artist/[\w-]+(?:/[\w-]+)*/(?P<identifier>(?:TR|AL|AR)[A-Za-z0-9]+))(?:[?#].*)?$").unwrap();
    pub static ref TIKTOK_REGEX: Regex = Regex::new(r"https://www\.tiktok\.com/").unwrap();
    pub static ref MIXCLOUD_REGEX: Regex = Regex::new(r"https://www\.mixcloud\.com/").unwrap();
    pub static ref MUSIC_YANDEX_REGEX: Regex = Regex::new(r"https://music\.yandex\.ru/").unwrap();
    pub static ref RADIOHOST_REGEX: Regex = Regex::new(r"https?://[^.\s]+\.radiohost\.de/(\S+)").unwrap();
}
