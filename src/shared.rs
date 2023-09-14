pub fn into_bytes<T: ::prost::Message + Default>(src: &T) -> Result<Vec<u8>, prost::EncodeError> {
    let mut result = Vec::new();
    prost::Message::encode(src, &mut result)?;
    Ok(result)
}

pub fn from_bytes<T: ::prost::Message + Default>(bytes: &[u8]) -> Result<T, prost::DecodeError> {
    prost::Message::decode(bytes)
}

pub fn into_bytes_with_version<T: ::prost::Message + Default>(
    src: &T
) -> Result<Vec<u8>, prost::EncodeError> {
    let mut result = vec![0];
    prost::Message::encode(src, &mut result)?;
    Ok(result)
}

pub fn from_bytes_with_version<T: ::prost::Message + Default>(
    bytes: &[u8],
) -> Result<T, prost::DecodeError> {
    prost::Message::decode(&bytes[1..])
}
