# netfuck

my implementation of [netfuck](https://esolangs.org/wiki/NetFuck) with some changes to make it useful in something other than netfuck <-> netfuck communication \
95% of the spec is from [here](https://github.com/animehunter/netfuck) \
very poorly written, may have implementation bugs

## instructions

### networking
` set the port to the current cell value \
~ connect to the ip address of the current cell value. stores socket handle (handle>=0) into current cell \
& use the current cell value as the socket handle for all networking operations \
! close the current connection \
v recieve one byte from the current connection and store it at the current cell \
^ write one byte to the connection \
% flush all bytes \
\
errors will set the current cell value to -1 \
the data handling instructions (^, v) sends the least significant byte unless multiple of them are chained

**examples:** \
i32 = [LSB][byte 2][byte 3][MSB]

^^^^^ -> sends the LSB 5 times \
^^^^ -> sends the current cell \
^^^ -> [LSB][byte 2][byte 3] \
\>++++[<^>-] -> sends the LSB 4 times \
^+-^+-^+-^ -> also sends the LSB 4 times

vvvvv -> 5 bytes sent - [dropped][LSB][byte 2][byte 3][MSB], no bytes left in queue \
vvvv -> 5 bytes sent -> [LSB][byte 2][byte 3][MSB][queue], last byte left in queue \
vvv -> 5 bytes sent -> [LSB][byte 2][byte 3][0], 2 bytes left in queue \

implementation may be incorrect

### threading
does not exist (soon™)

### endianess
ip/port is big endian \
data is in little endian

<!--
todo:
multithreading (probably never going to be implemented)
-->