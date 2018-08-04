# rust-db2
An attempt to piece together odbc-rs + r2d2 + r2d2-odbc with IBM DB2 linuxx64 drivers

## Running

`DSN='Driver=/path/to/libdb2.dylib;Database=REDACTED;Hostname=REDACTED;UID=REDACTED;PWD=REDACTED;PORT=REDACTED;' QUERY='SELECT foo FROM REDACTED.REDACTED' cargo run`
