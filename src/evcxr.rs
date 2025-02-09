use crate::coord::Shift;
use crate::drawing::{DrawingArea, IntoDrawingArea, SVGBackend};

/// The wrapper for the generated SVG
pub struct SVGWrapper(Vec<u8>, String);

impl SVGWrapper {
    pub fn evcxr_display(&self) {
        println!("{:?}", self);
    }

    pub fn style<S: Into<String>>(mut self, style: S) -> Self {
        self.1 = style.into();
        self
    }
}

impl std::fmt::Debug for SVGWrapper {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let svg = String::from_utf8_lossy(self.0.as_slice());
        write!(
            formatter,
            "EVCXR_BEGIN_CONTENT text/html\n<div style=\"{}\">{}</div>\nEVCXR_END_CONTENT",
            self.1, svg
        )
    }
}

/// Start drawing an evcxr figure
pub fn evcxr_figure<
    Draw: FnOnce(DrawingArea<SVGBackend, Shift>) -> Result<(), Box<dyn std::error::Error>>,
>(
    size: (u32, u32),
    draw: Draw,
) -> SVGWrapper {
    let mut buffer = vec![];
    let root = SVGBackend::with_buffer(&mut buffer, size).into_drawing_area();
    draw(root).expect("Drawing failure");
    SVGWrapper(buffer, "".to_string())
}
