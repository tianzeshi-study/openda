# NVDA_dmp
A utility to calculate text insersions using Diff Match Patch.

Copyright 2020 Bill Dengler, licensed under [apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) with explicit authorization to be distrinbuted with the [NVDA screen reader](https://nvaccess.org).

## Protocol
NVDA_dmp expects to receive old and new text on stdin, and writes on stdout text that was inserted (i.e. new changes that NVDA should speak). The protocol is as follows:

* Input: two 32-bit ints (in machine byte order) containing the lengths (in bytes) of the old and new texts respectively. Sending two lengths of 0 will cause nvda_dmp to exit. The old text, then new (encoded in utf-8) should follow the lengths.
* Output: a 32-bit int containing the length of the output, then any text which was inserted (i.e. present in new but not in old).