// A consumer on a contracts-only build with its own arena, binding the str
// crate's interner to the providers contract as the gated bridge does.
pub struct MyArena;
impl strcrate::ArenaInterner for MyArena {
    fn arena_intern(&self, _s: &str) -> u32 {
        0
    }
}
impl providers::InternerApi for strcrate::StringInterner<MyArena> {
    fn intern(&self, s: &str) -> Option<u32> {
        strcrate::StringInterner::intern(self, s)
    }
}
