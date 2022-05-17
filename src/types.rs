use rand::Rng;

pub trait Generator: Sized {
    type Generator: GenerateRandomly<Self>;

    fn generator() -> Self::Generator;
}

pub trait GenerateRandomly<T> {
    fn generate_randomly<R>(&self, rng: &mut R) -> Option<T>
    where
        R: Rng;
}

pub trait Colored {
    fn color(&self) -> nannou::image::Rgb<u8>;
}
