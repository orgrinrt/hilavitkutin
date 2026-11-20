// Stands for hilavitkutin-str: the interner generic over a consumer's arena.
pub trait ArenaInterner {
    fn arena_intern(&self, s: &str) -> u32;
}
pub struct StringInterner<A>(pub A);
impl<A: ArenaInterner> StringInterner<A> {
    pub fn intern(&self, s: &str) -> Option<u32> {
        let id = self.0.arena_intern(s);
        if id >> 28 != 0 { None } else { Some(id) }
    }
}
