use std::convert::TryInto;
use super::error::{Result, Error};
use std::path::PathBuf;

fn to_array_4(data: &[u8]) -> Result<&[u8; 4]> {
    data.try_into().map_err(|err| Error::Conversion("to_array_4".to_string(), format!("{err}")))
}

pub fn read_u32(input: &[u8]) -> Result<u32> {
    if input.len() != 4 {
        Error::Conversion("expected exactly 4 bytes".to_string(), "no".to_string());
    }
    let bytes = *to_array_4(input)?;
    Ok(u32::from_be_bytes(bytes))
}

pub fn inc_bit(bit: &mut u8) -> bool {
    if *bit == 7 {
        *bit = 0;
        return true;
    } 
    *bit += 1;
    false
}

pub fn in_file_to_out_file(mut in_file: PathBuf) -> PathBuf {
    in_file.set_extension("huf");
    if in_file.exists() {
        let file = in_file.file_stem().unwrap();
        in_file.set_file_name(format!("{}_copy.huf",file.to_str().unwrap()).as_str());
    }
    in_file
}
pub fn out_file_to_in_file(mut out_file: PathBuf) -> PathBuf {
    out_file.set_extension("txt"); //TODO at compression add in_file extension
    if out_file.exists() {
        let file = out_file.file_stem().unwrap();
        out_file.set_file_name(format!("{}_copy.txt", file.to_str().unwrap()).as_str());
    }
    out_file
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_read_u32() {
        let bytes = &[0,0,0,1];
        assert_eq!(read_u32(bytes).unwrap(), 1);
        assert!(read_u32(&[0,0,0]).is_err());
    }

    #[test]
    fn do_inc_bit() {
        let mut bit = 6;
        assert!(!inc_bit(&mut bit));
        assert_eq!(bit, 7);
        assert!(inc_bit(&mut bit));
        assert_eq!(bit, 0);

    }

    #[test]
    fn in_to_out_file() {
        let in_file = PathBuf::from("test.txt");
        let out_file = in_file_to_out_file(in_file);
        assert_eq!(out_file.as_path(), Path::new("test.huf"));
    }
    #[test]
    fn out_to_in_file() {
        let out_file = PathBuf::from("test.huf");
        let in_file = out_file_to_in_file(out_file);
        assert_eq!(in_file.as_path(), Path::new("test.txt"));
    }

    #[test]
    fn file_that_exists() {
        let in_file = PathBuf::from("blank.huf");
        let out_file = out_file_to_in_file(in_file);
        assert_eq!(out_file.as_path(), Path::new("blank_copy.txt"));
    }
}
