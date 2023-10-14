use std::str::FromStr;

use anyhow::bail;

#[derive(PartialEq, PartialOrd, Debug, Eq)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.0;
    }

    /// For convenience in description and in examining PNG files, type codes are restricted to consist of uppercase and lowercase ASCII
    /// letters (A-Z and a-z, or 65-90 and 97-122 decimal).
    pub fn is_valid(&self) -> bool {
        self.0.iter().all(u8::is_ascii_alphabetic) && self.is_reserved_bit_valid()
    }

    /// Chunks that are not strictly necessary in order to meaningfully display the contents of the file are known as "ancillary" chunks.
    /// A decoder encountering an unknown chunk in which the ancillary bit is 1 can safely ignore the chunk and proceed to display the image.
    /// The time chunk (tIME) is an example of an ancillary chunk.
    ///
    /// Chunks that are necessary for successful display of the file's contents are called "critical" chunks. A decoder encountering an unknown
    /// chunk in which the ancillary bit is 0 must indicate to the user that the image contains information it cannot safely interpret. The image
    ///  header chunk (IHDR) is an example of a critical chunk.
    pub fn is_critical(&self) -> bool {
        self.0[0].is_ascii_uppercase()
    }

    /// A public chunk is one that is part of the PNG specification or is registered in the list of PNG special-purpose public chunk types.
    /// Applications can also define private (unregistered) chunks for their own purposes. The names of private chunks must have a lowercase
    /// second letter, while public chunks will always be assigned names with uppercase second letters. Note that decoders do not need to
    /// test the private-chunk property bit, since it has no functional significance; it is simply an administrative convenience to ensure
    /// that public and private chunk names will not conflict. See Additional chunk types, and Recommendations for Encoders: Use of private chunks.
    pub fn is_public(&self) -> bool {
        self.0[1].is_ascii_uppercase()
    }

    /// The significance of the case of the third letter of the chunk name is reserved for possible future expansion. At the present time all
    /// chunk names must have uppercase third letters. (Decoders should not complain about a lowercase third letter, however, as some future
    /// version of the PNG specification could define a meaning for this bit. It is sufficient to treat a chunk with a lowercase third letter
    /// in the same way as any other unknown chunk type.)
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2].is_ascii_uppercase()
    }

    /// This property bit is not of interest to pure decoders, but it is needed by PNG editors (programs that modify PNG files). This bit defines
    /// the proper handling of unrecognized chunks in a file that is being modified.
    ///
    /// If a chunk's safe-to-copy bit is 1, the chunk may be copied to a modified PNG file whether or not the software recognizes the chunk type,
    /// and regardless of the extent of the file modifications.
    ///
    /// If a chunk's safe-to-copy bit is 0, it indicates that the chunk depends on the image data. If the program has made any changes to critical
    /// chunks, including addition, modification, deletion, or reordering of critical chunks, then unrecognized unsafe chunks must not be copied
    /// to the output PNG file. (Of course, if the program does recognize the chunk, it can choose to output an appropriately modified version.)
    ///
    /// A PNG editor is always allowed to copy all unrecognized chunks if it has only added, deleted, modified, or reordered ancillary chunks. This
    /// implies that it is not permissible for ancillary chunks to depend on other ancillary chunks.
    ///
    ///  PNG editors that do not recognize a critical chunk must report an error and refuse to process that PNG file at all. The safe/unsafe mechanism
    /// is intended for use with ancillary chunks. The safe-to-copy bit will always be 0 for critical chunks.
    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = super::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let result = ChunkType(value);

        if !value.iter().all(u8::is_ascii_alphabetic) {
            bail!("Invalid bytes passed, need to be all ascii alphabetic!")
        }

        Ok(result)
    }
}

impl FromStr for ChunkType {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 4] = s.as_bytes().try_into()?;

        ChunkType::try_from(bytes)
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(&self.0).expect("ASCII should be valid utf")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
