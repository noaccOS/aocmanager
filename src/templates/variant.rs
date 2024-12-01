#[derive(Clone, Copy, Debug, clap::ValueEnum, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunVariant {
    A,
    B,
    Both,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Variant {
    A,
    B,
}

impl From<Variant> for RunVariant {
    fn from(value: Variant) -> Self {
        match value {
            Variant::A => RunVariant::A,
            Variant::B => RunVariant::B,
        }
    }
}

impl TryInto<Variant> for RunVariant {
    type Error = ();

    fn try_into(self) -> Result<Variant, Self::Error> {
        match self {
            RunVariant::A => Ok(Variant::A),
            RunVariant::B => Ok(Variant::B),
            RunVariant::Both => Err(()),
        }
    }
}
