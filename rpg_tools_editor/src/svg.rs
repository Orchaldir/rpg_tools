#[derive(Responder)]
#[response(status = 200, content_type = "image/svg+xml")]
pub struct RawSvg(String);

impl RawSvg {
    pub fn new(content: String) -> Self {
        Self(content)
    }
}
