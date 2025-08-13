use super::enums::RespFrameEnum;

pub trait RespDecode {
    fn decode(bytes: &mut Self) -> Result<RespFrameEnum, String>
    where
        Self: Sized;
}
