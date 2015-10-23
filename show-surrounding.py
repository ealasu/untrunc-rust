#!/usr/bin/env python

import sys

matches = open(sys.argv[1], 'r')
f = open(sys.argv[2], 'rb')

for m in matches:
    f.seek(int(m))
    assert f.read(4) == b'mdat'
    f.seek(int(m) - 4)
    print(f.read(4))
