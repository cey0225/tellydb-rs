//! RESP protocol parser and serializer.
//!
//! Zero-copy — parsed data borrows directly from the input buffer.
//!
//! ## Supported types
//!
//! | Type | First byte | Status |
//! |------|-----------|--------|
//! | Simple String | `+` | ✓ |
//! | Simple Error | `-` | ✓ |
//! | Integer | `:` | ✓ |
//! | Bulk String | `$` | ✓ |
//! | Null Bulk String | `$-1` | ✓ |
//! | Array | `*` | ✓ |
//! | Null Array | `*-1` | ✓ |
//! | Null | `_` | ✗ |
//! | Boolean | `#` | ✗ |
//! | Double | `,` | ✗ |
//! | Big Number | `(` | ✗ |
//! | Bulk Error | `!` | ✗ |
//! | Verbatim String | `=` | ✗ |
//! | Map | `%` | ✗ |
//! | Attribute | `\|` | ✗ |
//! | Set | `~` | ✗ |
//! | Push | `>` | ✗ |

/// Represents a parsed RESP value.
///
/// All string and byte references borrow from the original input buffer.
#[derive(Debug, PartialEq)]
pub enum RespData<'a> {
    SimpleString(&'a str),
    Error(&'a str),
    Integer(i64),
    BulkString(&'a [u8]),
    Array(Vec<RespData<'a>>),
    NullBulkString,
    NullArray,
}

impl RespData<'_> {
    /// Serializes the value into RESP wire format.
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            RespData::SimpleString(s) => format!("+{s}\r\n").into_bytes(),
            RespData::Error(s) => format!("-{s}\r\n").into_bytes(),
            RespData::Integer(i) => format!(":{i}\r\n").into_bytes(),
            RespData::NullBulkString => b"$-1\r\n".to_vec(),
            RespData::NullArray => b"*-1\r\n".to_vec(),
            RespData::BulkString(data) => {
                let mut buf = format!("${}\r\n", data.len()).into_bytes();
                buf.extend_from_slice(data);
                buf.extend_from_slice(b"\r\n");
                buf
            }
            RespData::Array(items) => {
                let mut buf = format!("*{}\r\n", items.len()).into_bytes();
                for item in items {
                    buf.extend(item.serialize());
                }
                buf
            }
        }
    }
}

/// A parse error.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Not enough data in the buffer to complete parsing.
    Incomplete,
    /// The buffer contains malformed RESP data.
    InvalidProtocol,
}

type ParseResult<'a> = Result<(usize, RespData<'a>), ParseError>;

/// Finds the position of the first `\r\n` sequence in the buffer.
fn find_crlf(buffer: &[u8]) -> Option<usize> {
    buffer.windows(2).position(|w| w == b"\r\n")
}

/// Parses a single RESP value from the buffer.
///
/// Returns the number of bytes consumed and the parsed value.
pub fn parse(buffer: &(impl AsRef<[u8]> + ?Sized)) -> ParseResult<'_> {
    if buffer.as_ref().is_empty() {
        return Err(ParseError::Incomplete);
    }

    match buffer.as_ref()[0] {
        b'+' => parse_simple_string(buffer.as_ref()),
        b'-' => parse_error(buffer.as_ref()),
        b':' => parse_integer(buffer.as_ref()),
        b'$' => parse_bulk_string(buffer.as_ref()),
        b'*' => parse_array(buffer.as_ref()),
        _ => Err(ParseError::InvalidProtocol),
    }
}

/// Parses a simple string: `+OK\r\n`
fn parse_simple_string(buffer: &[u8]) -> ParseResult<'_> {
    let pos = find_crlf(buffer).ok_or(ParseError::Incomplete)?;
    let data = &buffer[1..pos];
    let s = std::str::from_utf8(data).map_err(|_| ParseError::InvalidProtocol)?;
    Ok((pos + 2, RespData::SimpleString(s)))
}

/// Parses an error: `-ERR message\r\n`
fn parse_error(buffer: &[u8]) -> ParseResult<'_> {
    let pos = find_crlf(buffer).ok_or(ParseError::Incomplete)?;
    let data = &buffer[1..pos];
    let s = std::str::from_utf8(data).map_err(|_| ParseError::InvalidProtocol)?;
    Ok((pos + 2, RespData::Error(s)))
}

/// Parses an integer: `:1000\r\n`
fn parse_integer(buffer: &[u8]) -> ParseResult<'_> {
    let pos = find_crlf(buffer).ok_or(ParseError::Incomplete)?;
    let data = &buffer[1..pos];
    let s = std::str::from_utf8(data).map_err(|_| ParseError::InvalidProtocol)?;
    let int = s.parse::<i64>().map_err(|_| ParseError::InvalidProtocol)?;
    Ok((pos + 2, RespData::Integer(int)))
}

/// Parses a bulk string: `$5\r\nhello\r\n`, or null: `$-1\r\n`
fn parse_bulk_string(buffer: &[u8]) -> ParseResult<'_> {
    let length_end = find_crlf(buffer).ok_or(ParseError::Incomplete)?;
    let length_str =
        std::str::from_utf8(&buffer[1..length_end]).map_err(|_| ParseError::InvalidProtocol)?;
    let length: isize = length_str
        .parse()
        .map_err(|_| ParseError::InvalidProtocol)?;

    if length == -1 {
        return Ok((length_end + 2, RespData::NullBulkString));
    }

    let data_len = length as usize;
    let data_start = length_end + 2;
    let data_end = data_start + data_len;
    let total_len = data_end + 2;

    if buffer.len() < total_len {
        return Err(ParseError::Incomplete);
    }

    if &buffer[data_end..total_len] != b"\r\n" {
        return Err(ParseError::InvalidProtocol);
    }

    let data = &buffer[data_start..data_end];
    Ok((total_len, RespData::BulkString(data)))
}

/// Parses an array: `*2\r\n...`, or null: `*-1\r\n`
fn parse_array(buffer: &[u8]) -> ParseResult<'_> {
    let length_end = find_crlf(buffer).ok_or(ParseError::Incomplete)?;
    let length_str =
        std::str::from_utf8(&buffer[1..length_end]).map_err(|_| ParseError::InvalidProtocol)?;
    let length: isize = length_str
        .parse()
        .map_err(|_| ParseError::InvalidProtocol)?;

    if length == -1 {
        return Ok((length_end + 2, RespData::NullArray));
    }

    let count = length as usize;
    let mut offset = length_end + 2;
    let mut items = Vec::with_capacity(count);

    for _ in 0..count {
        let (consumed, item) = parse(&buffer[offset..])?;
        offset += consumed;
        items.push(item);
    }

    Ok((offset, RespData::Array(items)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_string() {
        let (len, data) = parse(b"+OK\r\n").unwrap();
        assert_eq!(len, 5);
        assert_eq!(data, RespData::SimpleString("OK"));
    }

    #[test]
    fn error() {
        let (len, data) = parse(b"-ERR unknown\r\n").unwrap();
        assert_eq!(len, 14);
        assert_eq!(data, RespData::Error("ERR unknown"));
    }

    #[test]
    fn integer() {
        let (len, data) = parse(b":1000\r\n").unwrap();
        assert_eq!(len, 7);
        assert_eq!(data, RespData::Integer(1000));
    }

    #[test]
    fn negative_integer() {
        let (len, data) = parse(b":-42\r\n").unwrap();
        assert_eq!(len, 6);
        assert_eq!(data, RespData::Integer(-42));
    }

    #[test]
    fn bulk_string() {
        let (len, data) = parse(b"$5\r\nhello\r\n").unwrap();
        assert_eq!(len, 11);
        assert_eq!(data, RespData::BulkString(b"hello"));
    }

    #[test]
    fn empty_bulk_string() {
        let (len, data) = parse(b"$0\r\n\r\n").unwrap();
        assert_eq!(len, 6);
        assert_eq!(data, RespData::BulkString(b""));
    }

    #[test]
    fn null_bulk_string() {
        let (len, data) = parse(b"$-1\r\n").unwrap();
        assert_eq!(len, 5);
        assert_eq!(data, RespData::NullBulkString);
    }

    #[test]
    fn array() {
        let input = b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";
        let (len, data) = parse(input).unwrap();
        assert_eq!(len, input.len());
        assert_eq!(
            data,
            RespData::Array(vec![
                RespData::BulkString(b"foo"),
                RespData::BulkString(b"bar"),
            ])
        );
    }

    #[test]
    fn empty_array() {
        let (len, data) = parse(b"*0\r\n").unwrap();
        assert_eq!(len, 4);
        assert_eq!(data, RespData::Array(vec![]));
    }

    #[test]
    fn null_array() {
        let (len, data) = parse(b"*-1\r\n").unwrap();
        assert_eq!(len, 5);
        assert_eq!(data, RespData::NullArray);
    }

    #[test]
    fn mixed_types_array() {
        let input = b"*3\r\n+OK\r\n:-1\r\n$4\r\ntest\r\n";
        let (_, data) = parse(input).unwrap();
        assert_eq!(
            data,
            RespData::Array(vec![
                RespData::SimpleString("OK"),
                RespData::Integer(-1),
                RespData::BulkString(b"test"),
            ])
        );
    }

    #[test]
    fn incomplete() {
        assert_eq!(parse(b"+OK"), Err(ParseError::Incomplete));
        assert_eq!(parse(b"$5\r\nhel"), Err(ParseError::Incomplete));
        assert_eq!(parse(b"*2\r\n+OK\r\n"), Err(ParseError::Incomplete));
        assert_eq!(parse(b""), Err(ParseError::Incomplete));
    }

    #[test]
    fn invalid_protocol() {
        assert_eq!(parse(b"?what\r\n"), Err(ParseError::InvalidProtocol));
        assert_eq!(parse(b":notanumber\r\n"), Err(ParseError::InvalidProtocol));
    }

    #[test]
    fn roundtrip() {
        let inputs: Vec<&[u8]> = vec![
            b"+OK\r\n",
            b"-ERR fail\r\n",
            b":42\r\n",
            b"$5\r\nhello\r\n",
            b"$-1\r\n",
            b"*2\r\n$3\r\nfoo\r\n:1\r\n",
            b"*0\r\n",
        ];

        for input in inputs {
            let (_, data) = parse(input).unwrap();
            let serialized = data.serialize();
            let (_, reparsed) = parse(&serialized).unwrap();
            assert_eq!(data, reparsed);
        }
    }
}
