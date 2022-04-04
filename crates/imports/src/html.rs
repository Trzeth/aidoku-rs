type Rid = i32;

use super::StringRef;

#[link(wasm_import_module = "html")]
extern "C" {
    #[link_name = "parse"]
    fn scraper_parse(string: *const u8, len: usize) -> i32;
    #[link_name = "close"]
    fn scraper_free(rid: i32);

    #[link_name = "select"]
    fn scraper_select(rid: i32, selector: *const u8, selector_len: usize) -> i32;
    #[link_name = "first"]
    fn scraper_first(rid: i32) -> i32;
    #[link_name = "body"]
    fn scraper_body(rid: i32) -> i32;
    #[link_name = "text"]
    fn scraper_text(rid: i32) -> i32;
    #[link_name = "array"]
    fn scraper_array(rid: i32) -> i32;
    #[link_name = "attr"]
    fn scraper_attr(rid: i32, selector: *const u8, selector_len: usize) -> i32;
    #[link_name = "html"]
    fn scraper_html(rid: i32) -> i32;
    #[link_name = "outer_html"]
    fn scraper_outer_html(rid: i32) -> i32;

    #[link_name = "id"]
    fn scraper_id(rid: i32) -> i32;
    #[link_name = "tag_name"]
    fn scraper_tag_name(rid: i32) -> i32;
    #[link_name = "class_name"]
    fn scraper_class_name(rid: i32) -> i32;
    #[link_name = "has_class"]
    fn scraper_has_class(rid: i32, class_name: *const u8, class_length: usize) -> bool;
}

pub struct Node(Rid);

impl Node {
    pub fn new(buf: &[u8]) -> Self {
        let rid = unsafe { scraper_parse(buf.as_ptr(), buf.len()) };
        Self(rid)
    }

    pub fn from(rid: Rid) -> Self {
        Self(rid)
    }

    pub fn select(&self, selector: &str) -> Self {
        let rid = unsafe { scraper_select(self.0, selector.as_ptr(), selector.len()) };
        Self(rid)
    }

    pub fn attr(&self, selector: &str) -> StringRef {
        let rid = unsafe { scraper_attr(self.0, selector.as_ptr(), selector.len()) };
        StringRef(rid)
    }

    pub fn text(&self) -> StringRef {
        let rid = unsafe { scraper_text(self.0) };
        StringRef(rid)
    }

    pub fn id(&self) -> StringRef {
        let rid = unsafe { scraper_id(self.0) };
        StringRef(rid)
    }

    pub fn close(&mut self) {
        drop(self)
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe { scraper_free(self.0) }
    }
}