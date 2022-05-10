#[derive(Debug)]
pub enum FighterType
{
    Pokemon,
    RPS,
    StreetFighter,
}

impl std::str::FromStr for FighterType
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s.trim()
        {
            "pokemon" => Ok(FighterType::Pokemon),
            "rps" => Ok(FighterType::RPS),
            "streetfighter" => Ok(FighterType::StreetFighter),
            _ => Err("Unknown fighter type".to_string())
        }
    }
}

impl std::fmt::Display for FighterType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", match self
        {
            FighterType::Pokemon => "pokemon",
            FighterType::RPS => "rps",
            FighterType::StreetFighter => "streetfighter",
        })
    }
}

pub trait RandomlyGeneratable
{
    fn generate_randomly() -> Box<dyn Iterator<Item=Self>>;
}

pub trait Colored
{
    fn color(&self) -> nannou::image::Rgb<u8>;
}
