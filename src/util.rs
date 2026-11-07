//! graphicsmagick model — image processing (ImageMagick fork).

#[derive(Debug, Clone)]
pub struct Image {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub format: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub images: Vec<Image>,
}

impl Default for Model {
    fn default() -> Self { Self { images: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn load(&mut self, image: Image) { self.images.push(image); }

    pub fn convert(&mut self, src: &str, dst: &str, format: &str) -> bool {
        if self.images.iter().any(|i| i.path == src) {
            self.images.push(Image { path: dst.into(), width: 0, height: 0, format: format.into() });
            true
        } else { false }
    }

    pub fn identify(&self, path: &str) -> Option<&Image> {
        self.images.iter().find(|i| i.path == path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_convert() {
        let mut g = Model::new();
        g.load(Image { path: "in.png".into(), width: 100, height: 100, format: "png".into() });
        assert!(g.convert("in.png", "out.jpg", "jpg"));
        assert!(g.identify("out.jpg").is_some());
    }
}
