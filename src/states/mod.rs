pub mod feeding;
pub mod landing;
pub mod reading;

use crate::Buffer;
use crate::Result;
pub trait State {
    fn handle_key_then_next(self: Box<Self>, byte: u8) -> Result<Option<Box<dyn State>>>;
    fn buf_ref(&self) -> &Buffer;
}
