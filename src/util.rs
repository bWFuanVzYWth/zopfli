use std::io::{ErrorKind, Read, self};

/// Number of distinct literal/length symbols in DEFLATE
pub const ZOPFLI_NUM_LL: usize = 288;
/// Number of distinct distance symbols in DEFLATE
pub const ZOPFLI_NUM_D: usize = 32;

/// The window size for deflate. Must be a power of two. This should be 32768, the
/// maximum possible by the deflate spec. Anything less hurts compression more than
/// speed.
pub const ZOPFLI_WINDOW_SIZE: usize = 32768;

/// The window mask used to wrap indices into the window. This is why the
/// window size must be a power of two.
pub const ZOPFLI_WINDOW_MASK: usize = 32767; // ZOPFLI_WINDOW_SIZE - 1

/// Maximum length that can be encoded in deflate.
pub const ZOPFLI_MAX_MATCH: usize = 258;
/// Minimum length that can be encoded in deflate.
pub const ZOPFLI_MIN_MATCH: usize = 3;

/// For longest match cache. max 256. Uses huge amounts of memory but makes it
/// faster. Uses this many times three bytes per single byte of the input data.
/// This is so because longest match finding has to find the exact distance
/// that belongs to each length for the best lz77 strategy.
/// Good values: e.g. 5, 8.
pub const ZOPFLI_CACHE_LENGTH: usize = 8;

/// limit the max hash chain hits for this hash value. This has an effect only
/// on files where the hash value is the same very often. On these files, this
/// gives worse compression (the value should ideally be 32768, which is the
/// `ZOPFLI_WINDOW_SIZE`, while zlib uses 4096 even for best level), but makes it
/// faster on some specific files.
/// Good value: e.g. 8192.
pub const ZOPFLI_MAX_CHAIN_HITS: usize = 8192;

/// A block structure of huge, non-smart, blocks to divide the input into, to allow
/// operating on huge files without exceeding memory, such as the 1GB wiki9 corpus.
/// The whole compression algorithm, including the smarter block splitting, will
/// be executed independently on each huge block.
/// Dividing into huge blocks hurts compression, but not much relative to the size.
/// This must be equal or greater than `ZOPFLI_WINDOW_SIZE`.
pub const ZOPFLI_MASTER_BLOCK_SIZE: usize = 1000000;

/// Reads all bytes from a source to a buffer until either the buffer is full or EOF is
/// reached. The return value is a tuple whose first element signals whether the buffer
/// was completely filled, and its second element the count of bytes read and placed into
/// `buf`.
pub fn read_to_fill<R: Read>(mut in_data: R, mut buf: &mut [u8]) -> io::Result<(bool, usize)> {
    let mut bytes_read = 0;

    while !buf.is_empty() {
        match in_data.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                bytes_read += n;
                buf = &mut buf[n..];
            }
            Err(err) if err.kind() == ErrorKind::Interrupted => {},
            Err(err) => return Err(err)
        }
    }

    Ok((buf.is_empty(), bytes_read))
}
