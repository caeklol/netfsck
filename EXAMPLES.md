# Examples

## Sending data
an i32 would be:
[LSB][byte 2][byte 3][MSB]

| Action             | Effect                            |
|--------------------|----------------------------------|
| ^^^^^%             | sends the LSB 5 times            |
| ^^^^%              | sends the current cell           |
| ^^^%               | sends `LSB \| byte 2 \| byte 3`  |
| \>++++[<^>-]%      | sends the LSB 4 times            |
| ^+-^+-^+-^%        | sends the LSB 4 times       |

## Recieving data
these examples assume 5 bytes has been recieved and placed into the internal buffer from the current connection.

| Instruction       | Current Cell                        | Queue                      |
|-------------------|-------------------------------------|----------------------------|
| vvvvv             | [dropped][LSB][byte 2][byte 3][MSB] | no bytes left              |
| vvvv              | [LSB][byte 2][byte 3][MSB][queue]   | last byte left             |
| vvv               | [LSB][byte 2][byte 3][0][q][q]      | last 2 bytes left in queue |

implementation may be incorrect -- do report bugs
