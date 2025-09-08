use ark_std::UniformRand;
use rand::Rng;
use ark_serialize::{ CanonicalSerialize, SerializationError};


pub fn to_bytes<T: CanonicalSerialize>(serializable:  Vec<T>) -> Result<Vec<u8>, SerializationError> {
    let mut buffer = vec![];
    for item in serializable{
        let mut data = vec![];
        item.serialize_compressed(&mut data)?;
        buffer.push(data);
    }
    buffer.into()
}
