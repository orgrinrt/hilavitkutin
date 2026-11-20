// Stands for hilavitkutin-providers on default features: the contract only.
pub trait InternerApi: 'static {
    fn intern(&self, s: &str) -> Option<u32>;
}
