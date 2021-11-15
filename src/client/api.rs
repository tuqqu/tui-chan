pub trait ApiUrlProvider {
    fn boards(&self) -> String;

    fn threads(&self, board: &str, page: u8) -> String;

    fn thread(&self, board: &str, no: u64) -> String;
}

pub(crate) trait ContentUrlProvider {
    fn url_board(&self, board: &str) -> String;

    fn url_thread(&self, board: &str, no: u64) -> String;

    fn url_thread_post(&self, board: &str, no: u64, post_no: u64) -> String;

    fn url_file(&self, board: &str, filename: String) -> String;
}

pub(crate) trait ChannelProvider: ContentUrlProvider + ApiUrlProvider {
    fn as_api(&self) -> &dyn ApiUrlProvider;

    fn as_content(&self) -> &dyn ContentUrlProvider;
}

pub(crate) struct Api4chan;

impl Api4chan {
    const NAME: &'static str = "4chan";
    const BASE_API_URL: &'static str = "https://a.4cdn.org";
    const BASE_URL: &'static str = "https://boards.4chan.org";
    const BASE_MEDIA_URL: &'static str = "https://i.4cdn.org";
}

impl ApiUrlProvider for Api4chan {
    fn boards(&self) -> String {
        format!("{}/boards.json", Self::BASE_API_URL)
    }

    fn threads(&self, board: &str, page: u8) -> String {
        format!("{}/{}/{}.json", Self::BASE_API_URL, board, page)
    }

    fn thread(&self, board: &str, no: u64) -> String {
        format!("{}/{}/thread/{}.json", Self::BASE_API_URL, board, no)
    }
}

impl ContentUrlProvider for Api4chan {
    fn url_board(&self, board: &str) -> String {
        format!("{}/{}/", Self::BASE_URL, board)
    }

    fn url_thread(&self, board: &str, no: u64) -> String {
        format!("{}/{}/thread/{}", Self::BASE_URL, board, no)
    }

    fn url_thread_post(&self, board: &str, no: u64, post_no: u64) -> String {
        format!("{}/{}/thread/{}#p{}", Self::BASE_URL, board, no, post_no)
    }

    fn url_file(&self, board: &str, filename: String) -> String {
        format!("{}/{}/{}", Self::BASE_MEDIA_URL, board, filename)
    }
}

impl ChannelProvider for Api4chan {
    fn as_api(&self) -> &dyn ApiUrlProvider {
        self
    }

    fn as_content(&self) -> &dyn ContentUrlProvider {
        self
    }
}

const DEFAULT_API: &str = "default";

pub(crate) fn from_name(name: &str) -> Option<&'static dyn ChannelProvider> {
    match name {
        DEFAULT_API | Api4chan::NAME => Some(&Api4chan {}),
        _ => None,
    }
}
