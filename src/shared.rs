pub fn into_bytes<T: ::prost::Message + Default>(src: &T) -> Result<Vec<u8>, prost::EncodeError> {
    let mut result = Vec::new();
    prost::Message::encode(src, &mut result)?;
    Ok(result)
}

pub fn from_bytes<T: ::prost::Message + Default>(bytes: &[u8]) -> Result<T, prost::DecodeError> {
    prost::Message::decode(bytes)
}
