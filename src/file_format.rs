use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Write};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct AudioBlock {
    pub note: u8, 
    pub ms: u16,  
}

#[derive(Debug)]
pub struct X360File {
    pub version: u8,
    pub name: [u8; 4], 
    pub pairs: Vec<AudioBlock>,
}

pub fn read(path: &Path) -> io::Result<X360File> {

    let file = File::open(path)?;

    let mut reader = BufReader::new(file);

    let mut buffer_version = [0u8; 1];

    reader.read_exact(&mut buffer_version)?;

    let version = buffer_version[0];

    let mut buffer_name = [0u8; 4];
    reader.read_exact(&mut buffer_name)?;
    
    if buffer_name != *b"X360" {
         return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Assinatura do arquivo inválida"
        ));
    }

    let mut pairs = Vec::new();
    let mut buffer_pairs = [0u8; 3]; 

    loop {
        match reader.read_exact(&mut buffer_pairs) {
            Ok(_) => {

                let note = buffer_pairs[0];

                let ms = u16::from_be_bytes([buffer_pairs[1], buffer_pairs[2]]);

                pairs.push(AudioBlock { note, ms });

            }
            Err(e) => {

                if e.kind() == ErrorKind::UnexpectedEof {
                    break;
                } else {
                    return Err(e);
                }

            }
        }
    }

    Ok(X360File {
        version,
        name: buffer_name,
        pairs,
    })

}

pub fn write(path: &Path, data: X360File) -> io::Result<()> {

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    writer.write_all(&[data.version])?;
    writer.write_all(&data.name)?;

    for p in &data.pairs {
        writer.write_all(&[p.note])?;
        writer.write_all(&p.ms.to_be_bytes())?;
    }

    writer.flush()?;
    Ok(())

}