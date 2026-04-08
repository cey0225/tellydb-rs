# telly-proto

RESP protocol parser and serializer.

Zero-copy — parsed data borrows directly from the input buffer.

## Supported types

| Type             | First byte | Status |
|------------------|------------|--------|
| Simple String    | `+`        | ✓      |
| Simple Error     | `-`        | ✓      |
| Integer          | `:`        | ✓      |
| Bulk String      | `$`        | ✓      |
| Null Bulk String | `$-1`      | ✓      |
| Array            | `*`        | ✓      |
| Null Array       | `*-1`      | ✓      |
| Null             | `_`        | ✗      |
| Boolean          | `#`        | ✗      |
| Double           | `,`        | ✗      |
| Big Number       | `(`        | ✗      |
| Bulk Error       | `!`        | ✗      |
| Verbatim String  | `=`        | ✗      |
| Map              | `%`        | ✗      |
| Attribute        | `\|`       | ✗      |
| Set              | `~`        | ✗      |
| Push             | `>`        | ✗      |