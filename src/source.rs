//! Typed source traits and export helpers for Manatan extensions.

use crate::abi::ExtensionResult;
use crate::{
    AlternateCover, CatalogItem, HomeSection, MangaChapter, MangaPage, MangaPageImage,
    NovelChapter, NovelChapterPage, NovelText, Paged, ProcessedImage, UrlResolveResult,
    VideoEpisode, VideoHoster, VideoStream,
};
use serde_json::Value;

#[doc(hidden)]
#[macro_export]
macro_rules! __manatan_export_json {
    ($export_name:ident, $handler:path) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $export_name(ptr: u32, len: u32) -> u64 {
            let input = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };
            let result = (|| -> $crate::abi::ExtensionResult<Vec<u8>> {
                let value =
                    serde_json::from_slice(input).map_err(|error| $crate::abi::ExtensionError {
                        message: format!("request decode error: {error}"),
                    })?;
                let output = $handler(value)?;
                serde_json::to_vec(&Ok::<_, $crate::abi::ExtensionError>(output)).map_err(|error| {
                    $crate::abi::ExtensionError {
                        message: format!("response encode error: {error}"),
                    }
                })
            })();
            let bytes = match result {
                Ok(bytes) => bytes,
                Err(error) => serde_json::to_vec(&Err::<serde_json::Value, _>(error))
                    .unwrap_or_else(|_| b"{\"Err\":{\"message\":\"fatal encode error\"}}".to_vec()),
            };
            let len = bytes.len() as u32;
            let ptr = $crate::abi::manatan_alloc(bytes.len()) as *mut u8;
            unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len()) };
            $crate::abi::pack_ptr_len(ptr as u32, len)
        }
    };
}

/// A Manatan manga source.
///
/// Implement this trait for a zero-sized source struct, then expose it with
/// [`export_manga_source!`].
pub trait MangaSource {
    fn list(&self, request: Value) -> ExtensionResult<Paged<CatalogItem>>;
    fn search(&self, request: Value) -> ExtensionResult<Paged<CatalogItem>>;
    fn details(&self, request: Value) -> ExtensionResult<CatalogItem>;
    fn chapters(&self, request: Value) -> ExtensionResult<Vec<MangaChapter>>;
    fn pages(&self, request: Value) -> ExtensionResult<Vec<MangaPage>>;

    fn home(&self, _request: Value) -> ExtensionResult<Vec<HomeSection<CatalogItem>>> {
        Ok(Vec::new())
    }

    fn manga_url(&self, request: Value) -> ExtensionResult<Option<String>> {
        Ok(request
            .get("manga")
            .and_then(|manga| manga.get("url").or_else(|| manga.get("key")))
            .and_then(Value::as_str)
            .map(ToString::to_string))
    }

    fn chapter_url(&self, request: Value) -> ExtensionResult<Option<String>> {
        Ok(request
            .get("chapter")
            .and_then(|chapter| chapter.get("url").or_else(|| chapter.get("key")))
            .and_then(Value::as_str)
            .map(ToString::to_string))
    }

    fn handle_url(&self, _request: Value) -> ExtensionResult<Option<UrlResolveResult>> {
        Ok(None)
    }

    fn prepare_chapter(&self, request: Value) -> ExtensionResult<MangaChapter> {
        serde_json::from_value(
            request
                .get("chapter")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
        )
        .map_err(|error| crate::abi::ExtensionError {
            message: format!("chapter decode error: {error}"),
        })
    }

    fn resolve_page_image(&self, request: Value) -> ExtensionResult<MangaPageImage> {
        let page = request
            .get("page")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let url = page
            .get("content")
            .and_then(|content| {
                content
                    .get("url")
                    .and_then(|value| value.get("url").or(Some(value)))
                    .and_then(Value::as_str)
            })
            .or_else(|| page.get("url").and_then(Value::as_str))
            .unwrap_or_default()
            .to_string();
        Ok(MangaPageImage {
            url,
            ..Default::default()
        })
    }

    fn process_page_image(&self, request: Value) -> ExtensionResult<ProcessedImage> {
        Ok(ProcessedImage {
            image_base64: request
                .get("imageBase64")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            mime_type: request
                .get("mimeType")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            ..Default::default()
        })
    }

    fn alternate_covers(&self, _request: Value) -> ExtensionResult<Vec<AlternateCover>> {
        Ok(Vec::new())
    }

    fn related(&self, _request: Value) -> ExtensionResult<Vec<CatalogItem>> {
        Ok(Vec::new())
    }

    fn migrate(&self, _request: Value) -> ExtensionResult<Vec<CatalogItem>> {
        Ok(Vec::new())
    }
}

/// A Manatan video source.
///
/// Hoster exports are optional for sources that resolve streams directly from
/// episodes. The default hoster methods return empty lists.
pub trait VideoSource {
    fn list(&self, request: Value) -> ExtensionResult<Paged<CatalogItem>>;
    fn search(&self, request: Value) -> ExtensionResult<Paged<CatalogItem>>;
    fn details(&self, request: Value) -> ExtensionResult<CatalogItem>;
    fn episodes(&self, request: Value) -> ExtensionResult<Vec<VideoEpisode>>;
    fn streams(&self, request: Value) -> ExtensionResult<Vec<VideoStream>>;

    fn hosters(&self, _request: Value) -> ExtensionResult<Vec<VideoHoster>> {
        Ok(Vec::new())
    }

    fn resolve_hoster(&self, _request: Value) -> ExtensionResult<Vec<VideoStream>> {
        Ok(Vec::new())
    }

    fn home(&self, _request: Value) -> ExtensionResult<Vec<HomeSection<CatalogItem>>> {
        Ok(Vec::new())
    }

    fn item_url(&self, request: Value) -> ExtensionResult<Option<String>> {
        Ok(request
            .get("item")
            .and_then(|item| item.get("url").or_else(|| item.get("key")))
            .and_then(Value::as_str)
            .map(ToString::to_string))
    }

    fn episode_url(&self, request: Value) -> ExtensionResult<Option<String>> {
        Ok(request
            .get("episode")
            .and_then(|episode| episode.get("url").or_else(|| episode.get("key")))
            .and_then(Value::as_str)
            .map(ToString::to_string))
    }

    fn handle_url(&self, _request: Value) -> ExtensionResult<Option<UrlResolveResult>> {
        Ok(None)
    }
}

/// A Manatan novel source.
pub trait NovelSource {
    fn list(&self, request: Value) -> ExtensionResult<Paged<CatalogItem>>;
    fn search(&self, request: Value) -> ExtensionResult<Paged<CatalogItem>>;
    fn details(&self, request: Value) -> ExtensionResult<CatalogItem>;
    fn chapters(&self, request: Value) -> ExtensionResult<Vec<NovelChapter>>;
    fn text(&self, request: Value) -> ExtensionResult<NovelText>;

    fn home(&self, _request: Value) -> ExtensionResult<Vec<HomeSection<CatalogItem>>> {
        Ok(Vec::new())
    }

    fn chapters_page(&self, request: Value) -> ExtensionResult<NovelChapterPage> {
        let entries = self.chapters(request)?;
        Ok(NovelChapterPage {
            entries,
            has_next_page: false,
            ..Default::default()
        })
    }

    fn novel_url(&self, request: Value) -> ExtensionResult<Option<String>> {
        Ok(request
            .get("item")
            .and_then(|item| item.get("url").or_else(|| item.get("key")))
            .and_then(Value::as_str)
            .map(ToString::to_string))
    }

    fn chapter_url(&self, request: Value) -> ExtensionResult<Option<String>> {
        Ok(request
            .get("chapter")
            .and_then(|chapter| chapter.get("url").or_else(|| chapter.get("key")))
            .and_then(Value::as_str)
            .map(ToString::to_string))
    }

    fn handle_url(&self, _request: Value) -> ExtensionResult<Option<UrlResolveResult>> {
        Ok(None)
    }
}

#[macro_export]
macro_rules! export_manga_source {
    ($source:expr $(,)?) => {
        fn __manatan_manga_get_list(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::Paged<$crate::CatalogItem>> {
            $crate::source::MangaSource::list(&$source, request)
        }

        fn __manatan_manga_search(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::Paged<$crate::CatalogItem>> {
            $crate::source::MangaSource::search(&$source, request)
        }

        fn __manatan_manga_get_details(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::CatalogItem> {
            $crate::source::MangaSource::details(&$source, request)
        }

        fn __manatan_manga_get_chapters(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::MangaChapter>> {
            $crate::source::MangaSource::chapters(&$source, request)
        }

        fn __manatan_manga_get_pages(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::MangaPage>> {
            $crate::source::MangaSource::pages(&$source, request)
        }

        fn __manatan_manga_get_home(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::HomeSection<$crate::CatalogItem>>> {
            $crate::source::MangaSource::home(&$source, request)
        }

        fn __manatan_manga_get_manga_url(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Option<String>> {
            $crate::source::MangaSource::manga_url(&$source, request)
        }

        fn __manatan_manga_get_chapter_url(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Option<String>> {
            $crate::source::MangaSource::chapter_url(&$source, request)
        }

        fn __manatan_manga_handle_url(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Option<$crate::UrlResolveResult>> {
            $crate::source::MangaSource::handle_url(&$source, request)
        }

        fn __manatan_manga_prepare_chapter(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::MangaChapter> {
            $crate::source::MangaSource::prepare_chapter(&$source, request)
        }

        fn __manatan_manga_resolve_page_image(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::MangaPageImage> {
            $crate::source::MangaSource::resolve_page_image(&$source, request)
        }

        fn __manatan_manga_process_page_image(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::ProcessedImage> {
            $crate::source::MangaSource::process_page_image(&$source, request)
        }

        fn __manatan_manga_get_alternate_covers(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::AlternateCover>> {
            $crate::source::MangaSource::alternate_covers(&$source, request)
        }

        fn __manatan_manga_get_related(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::CatalogItem>> {
            $crate::source::MangaSource::related(&$source, request)
        }

        fn __manatan_manga_migrate(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::CatalogItem>> {
            $crate::source::MangaSource::migrate(&$source, request)
        }

        $crate::__manatan_export_json!(manatan_manga_get_list, __manatan_manga_get_list);
        $crate::__manatan_export_json!(manatan_manga_search, __manatan_manga_search);
        $crate::__manatan_export_json!(manatan_manga_get_details, __manatan_manga_get_details);
        $crate::__manatan_export_json!(manatan_manga_get_chapters, __manatan_manga_get_chapters);
        $crate::__manatan_export_json!(manatan_manga_get_pages, __manatan_manga_get_pages);
        $crate::__manatan_export_json!(manatan_manga_get_home, __manatan_manga_get_home);
        $crate::__manatan_export_json!(manatan_manga_get_manga_url, __manatan_manga_get_manga_url);
        $crate::__manatan_export_json!(
            manatan_manga_get_chapter_url,
            __manatan_manga_get_chapter_url
        );
        $crate::__manatan_export_json!(manatan_manga_handle_url, __manatan_manga_handle_url);
        $crate::__manatan_export_json!(
            manatan_manga_prepare_chapter,
            __manatan_manga_prepare_chapter
        );
        $crate::__manatan_export_json!(
            manatan_manga_resolve_page_image,
            __manatan_manga_resolve_page_image
        );
        $crate::__manatan_export_json!(
            manatan_manga_process_page_image,
            __manatan_manga_process_page_image
        );
        $crate::__manatan_export_json!(
            manatan_manga_get_alternate_covers,
            __manatan_manga_get_alternate_covers
        );
        $crate::__manatan_export_json!(manatan_manga_get_related, __manatan_manga_get_related);
        $crate::__manatan_export_json!(manatan_manga_migrate, __manatan_manga_migrate);
    };
}

#[macro_export]
macro_rules! export_video_source {
    ($source:expr $(,)?) => {
        fn __manatan_video_get_list(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::Paged<$crate::CatalogItem>> {
            $crate::source::VideoSource::list(&$source, request)
        }

        fn __manatan_video_search(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::Paged<$crate::CatalogItem>> {
            $crate::source::VideoSource::search(&$source, request)
        }

        fn __manatan_video_get_details(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::CatalogItem> {
            $crate::source::VideoSource::details(&$source, request)
        }

        fn __manatan_video_get_episodes(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::VideoEpisode>> {
            $crate::source::VideoSource::episodes(&$source, request)
        }

        fn __manatan_video_get_streams(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::VideoStream>> {
            $crate::source::VideoSource::streams(&$source, request)
        }

        fn __manatan_video_get_hosters(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::VideoHoster>> {
            $crate::source::VideoSource::hosters(&$source, request)
        }

        fn __manatan_video_resolve_hoster(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::VideoStream>> {
            $crate::source::VideoSource::resolve_hoster(&$source, request)
        }

        fn __manatan_video_get_home(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::HomeSection<$crate::CatalogItem>>> {
            $crate::source::VideoSource::home(&$source, request)
        }

        fn __manatan_video_get_item_url(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Option<String>> {
            $crate::source::VideoSource::item_url(&$source, request)
        }

        fn __manatan_video_get_episode_url(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Option<String>> {
            $crate::source::VideoSource::episode_url(&$source, request)
        }

        fn __manatan_video_handle_url(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Option<$crate::UrlResolveResult>> {
            $crate::source::VideoSource::handle_url(&$source, request)
        }

        $crate::__manatan_export_json!(manatan_video_get_list, __manatan_video_get_list);
        $crate::__manatan_export_json!(manatan_video_search, __manatan_video_search);
        $crate::__manatan_export_json!(manatan_video_get_details, __manatan_video_get_details);
        $crate::__manatan_export_json!(manatan_video_get_episodes, __manatan_video_get_episodes);
        $crate::__manatan_export_json!(manatan_video_get_streams, __manatan_video_get_streams);
        $crate::__manatan_export_json!(manatan_video_get_hosters, __manatan_video_get_hosters);
        $crate::__manatan_export_json!(
            manatan_video_resolve_hoster,
            __manatan_video_resolve_hoster
        );
        $crate::__manatan_export_json!(manatan_video_get_home, __manatan_video_get_home);
        $crate::__manatan_export_json!(manatan_video_get_item_url, __manatan_video_get_item_url);
        $crate::__manatan_export_json!(
            manatan_video_get_episode_url,
            __manatan_video_get_episode_url
        );
        $crate::__manatan_export_json!(manatan_video_handle_url, __manatan_video_handle_url);
    };
}

#[macro_export]
macro_rules! export_novel_source {
    ($source:expr $(,)?) => {
        fn __manatan_novel_get_list(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::Paged<$crate::CatalogItem>> {
            $crate::source::NovelSource::list(&$source, request)
        }

        fn __manatan_novel_search(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::Paged<$crate::CatalogItem>> {
            $crate::source::NovelSource::search(&$source, request)
        }

        fn __manatan_novel_get_details(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::CatalogItem> {
            $crate::source::NovelSource::details(&$source, request)
        }

        fn __manatan_novel_get_chapters(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::NovelChapter>> {
            $crate::source::NovelSource::chapters(&$source, request)
        }

        fn __manatan_novel_get_text(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::NovelText> {
            $crate::source::NovelSource::text(&$source, request)
        }

        fn __manatan_novel_get_home(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Vec<$crate::HomeSection<$crate::CatalogItem>>> {
            $crate::source::NovelSource::home(&$source, request)
        }

        fn __manatan_novel_get_chapters_page(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<$crate::NovelChapterPage> {
            $crate::source::NovelSource::chapters_page(&$source, request)
        }

        fn __manatan_novel_get_novel_url(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Option<String>> {
            $crate::source::NovelSource::novel_url(&$source, request)
        }

        fn __manatan_novel_get_chapter_url(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Option<String>> {
            $crate::source::NovelSource::chapter_url(&$source, request)
        }

        fn __manatan_novel_handle_url(
            request: serde_json::Value,
        ) -> $crate::abi::ExtensionResult<Option<$crate::UrlResolveResult>> {
            $crate::source::NovelSource::handle_url(&$source, request)
        }

        $crate::__manatan_export_json!(manatan_novel_get_list, __manatan_novel_get_list);
        $crate::__manatan_export_json!(manatan_novel_search, __manatan_novel_search);
        $crate::__manatan_export_json!(manatan_novel_get_details, __manatan_novel_get_details);
        $crate::__manatan_export_json!(manatan_novel_get_chapters, __manatan_novel_get_chapters);
        $crate::__manatan_export_json!(
            manatan_novel_get_chapters_page,
            __manatan_novel_get_chapters_page
        );
        $crate::__manatan_export_json!(manatan_novel_get_text, __manatan_novel_get_text);
        $crate::__manatan_export_json!(manatan_novel_get_home, __manatan_novel_get_home);
        $crate::__manatan_export_json!(manatan_novel_get_novel_url, __manatan_novel_get_novel_url);
        $crate::__manatan_export_json!(
            manatan_novel_get_chapter_url,
            __manatan_novel_get_chapter_url
        );
        $crate::__manatan_export_json!(manatan_novel_handle_url, __manatan_novel_handle_url);
    };
}
