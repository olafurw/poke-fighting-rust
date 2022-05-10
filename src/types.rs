use rand::Rng;

pub trait GenerateRandomly {
    fn generate_randomly<R>(rng: &mut R) -> Self
    where
        R: Rng;
}

pub trait Colored {
    fn color(&self) -> nannou::image::Rgb<u8>;
}
