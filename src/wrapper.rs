use clap::ValueEnum;

#[derive(Copy, Clone, ValueEnum)]
pub enum CompressionMethod {
    /// Store the file as is
    Stored,
    /// Compress the file using Deflate, compress level range from 0 to 9. Default is 6
    Deflated,
    /// Compress the file using BZIP2, compress level range from 0 to 9. Default is 6
    Bzip2,
    /// Compress the file using ZStandard, compress level range from -7 to 22,
    /// with zero being mapped to default level. Default is 3
    Zstd,
}

impl Into<zip::CompressionMethod> for CompressionMethod {
    fn into(self) -> zip::CompressionMethod {
        match self {
            CompressionMethod::Stored => zip::CompressionMethod::Stored,
            CompressionMethod::Deflated => zip::CompressionMethod::Deflated,
            CompressionMethod::Bzip2 => zip::CompressionMethod::Bzip2,
            CompressionMethod::Zstd => zip::CompressionMethod::Zstd,
        }
    }
}
