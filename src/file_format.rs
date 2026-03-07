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