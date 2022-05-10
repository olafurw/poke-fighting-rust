pub trait RandomlyGeneratable {
    fn generate_randomly() -> Box<dyn Iterator<Item = Self>>;
}

pub trait Colored {
    fn color(&self) -> nannou::image::Rgb<u8>;
}
