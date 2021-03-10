use crate::LTexture;
use sdl2::mixer::{Chunk, Music};
// Esto queda fuera del tutorial, pero dedidi crear un contendor, para los datos.
pub struct BoxData<'a> {
    pub picture: LTexture<'a>,
    pub beat: Music<'a>,
    pub scratch: Chunk,
    pub hint: Chunk,
    pub medium: Chunk,
    pub low: Chunk,
}

impl<'a> BoxData<'a> {
    pub fn new(
        picture: LTexture<'a>,
        beat: Music<'a>,
        scratch: Chunk,
        hint: Chunk,
        medium: Chunk,
        low: Chunk,
    ) -> Self {
        Self {
            picture,
            beat,
            scratch,
            hint,
            medium,
            low,
        }
    }
}
