# netfsck

my implementation of [netfuck](https://esolangs.org/wiki/NetFuck) \
goal: make this useful in something other than netfuck <-> netfuck communication \
95% of the spec is from [here](https://github.com/animehunter/netfuck) \
very poorly written, may include implementation bugs

## instructions

### networking

| Instruction | Action                                                  |
| :---------- | :------------------------------------------------------ |
| \`          | set the port to the current cell value (mod 65535). negative values will cause an error                                        |
| \~          | connect to the ip address of the current cell value. stores socket handle (handle>=0) into current cell                        |
| \&          | use the current cell value as the socket handle for all networking operations                                                  |
| \!          | close the current connection                                                                                                   |
| v           | recieve one byte from the current connection and store it at the current cell                                                  |
| \^          | write one byte to an internal buffer                                                                                           |
| \%          | flush the internal buffer and send it through the socket                                                                       |
| \$          | set read/write timeout in ms. negative numbers remove timeout, causing the program to wait infinitely on read/write failure.   |

errors will set the current cell value to -1 \
ip is in big endian. (hopefully) \
the data handling instructions (^, v) are in little endian. they send the least significant byte unless multiple of them are chained.
view [examples](EXAMPLES.md) for more examples

### threading
does not exist (soon™)
