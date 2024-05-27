use crate::interface::{Args, usage};
use crate::error::{Error, Result};
use crate::huffman::Huffman;
use crate::utils::{in_file_to_out_file, out_file_to_in_file};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Read, Write, self};

enum Out {
    File(PathBuf),
    StdOut
}


pub fn run(args: Args) -> Result<()> {
    let out_path: Out;
    let mut out_data: Vec<u8> = Vec::new();
    if let Some(in_file) = args.in_file {
        let mut file = File::open(in_file.as_str()).map_err(|err| Error::Compress(format!("could not open file '{}'", in_file), err.to_string()))?;
        if args.decompress {
            //it's a compression
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).map_err(|err| Error::DeCompress(format!("could not read file '{}'", in_file), err.to_string()))?;
            let mut huf = Huffman::from_bytes(&buf);
            let _ = huf.decode()?;
            out_path = if args.print_out {
                Out::StdOut
            } else {Out::File(out_file_to_in_file(PathBuf::from(in_file)))
        }; 
            huf.data_to_bytes(&mut out_data);
        } else {
            //it's a compression from file
            let mut buf = String::new();
            let mut file = File::open(in_file.as_str()).map_err(|err| Error::Compress(format!("could not open file '{}'", in_file), err.to_string()))?;
        file.read_to_string(&mut buf).map_err(|err| Error::Compress(format!("could not read file '{}'", in_file), err.to_string()))?;
            let mut huffman = Huffman::from_str(buf.as_str());
            huffman.create_tree();
            huffman.codes();
            let _ = huffman.encode()?;
            out_path = Out::File(in_file_to_out_file(PathBuf::from(in_file)));
            huffman.read_bytes_into(&mut out_data);
        }
    } else if let Some(data) = args.data {
        //can only be a compression
        let mut huffman = Huffman::from_str(data.as_str());
        huffman.create_tree();
        huffman.codes();
        let _ = huffman.encode()?;
        huffman.data_to_bytes(&mut out_data);
        if args.print_out {
            out_path = Out::StdOut;
        } else {
            out_path = Out::File(PathBuf::from("out.huf"));
        }
        huffman.data_to_bytes(&mut out_data);
    } else {
        usage(args.program.as_str());
        return Err(Error::Args("Insufficient args!".to_string(), "".to_string()))
    }
    //write file or print
    match out_path {
        Out::File(path) => {
            let mut out = OpenOptions::new().create(true).write(true).open(path.as_path()).map_err(|err| Error::Finalizing("Could not create out file".to_string(), err.to_string()))?;

                out.write_all(out_data.as_slice()).map_err(|err| Error::Finalizing("Could not write to stdout".to_string(), err.to_string()))?;
        },
        Out::StdOut => {
            let mut handle = io::stdout().lock();

                handle.write_all(out_data.as_slice()).map_err(|err| Error::Finalizing("Could not write to stdout".to_string(), err.to_string()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    //NOTE these tests do nothing
    #[test]
    fn compress_file() {
        let args = Args {
            program: String::new(),
            in_file: Some(String::from("test_input.txt")),
            decompress: false,
            data: None,
            print_out: false 
        };
        run(args);
    }

   #[test]
   fn de_compress_file() {

       let args = Args {
           program: String::new(),
           in_file: Some(String::from("test_output.huf")),
           decompress: true,
           data: None,
           print_out: true 
       };
       run(args);
   }
}


