"""A utility to calculate text insersions using Diff Match Patch.
Copyright 2020 Bill Dengler

licensed under the Apache licence, Version 2.0 (the "licence") with specific authorization to be distributed with NVDA;
you may not use this file except in compliance with the licence.
You may obtain a copy of the licence at

    http://www.apache.org/licences/licence-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the licence is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the licence for the specific language governing permissions and
limitations under the licence."""

import struct
import sys

from fast_diff_match_patch import diff


if __name__ == "__main__":
    while True:
        oldLen, newLen = struct.unpack("=II", sys.stdin.buffer.read(8))
        if not oldLen and not newLen:
            break  # sentinal value
        oldText = sys.stdin.buffer.read(oldLen).decode("utf-8")
        newText = sys.stdin.buffer.read(newLen).decode("utf-8")
        res = ""
        for op, text in diff(oldText, newText, counts_only=False):
            if op == "+":
                res += text
                if not text.endswith(("\n", "\r")):
                    res += "\n"
        res = res.encode("utf-8")
        sys.stdout.buffer.write(struct.pack("=I", len(res)))
        sys.stdout.buffer.write(res)
        sys.stdin.flush()
        sys.stdout.flush()
