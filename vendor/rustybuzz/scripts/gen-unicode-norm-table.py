#!/usr/bin/env python3

import urllib.request
import os

URL = 'https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt'
FILE_NAME = 'UnicodeData.txt'

EXCLUSIONS_URL = 'https://www.unicode.org/Public/draft/UCD/ucd/CompositionExclusions.txt'
EXCLUSIONS_FILE_NAME = 'CompositionExclusions.txt'


def hex_to_char_rs(c):
    return f"'\\u{{{c}}}'"


if not os.path.exists(FILE_NAME):
    urllib.request.urlretrieve(URL, FILE_NAME)

if not os.path.exists(EXCLUSIONS_FILE_NAME):
    urllib.request.urlretrieve(EXCLUSIONS_URL, EXCLUSIONS_FILE_NAME)

exclusions = set()

with open(EXCLUSIONS_FILE_NAME) as f:
    import re
    pattern = re.compile(r'^([0-9a-fA-F]+)\s+#\s+(.*)$')

    for line in f:
        match = pattern.match(line.strip())
        if match:
            exclusions.add(match.group(1))


print('// WARNING: this file was generated by ../scripts/gen-unicode-norm-table.py')
print()
print('//! This module provides Unicode tables for canonical (de)composition.')
print('//!')
print('//! The current implementation is not the fastest one. Just good enough.')
print()
print('#[allow(dead_code)]')
print('pub const UNICODE_VERSION: (u8, u8, u8) = (15, 0, 0);')
print()
print('// Rust support `Option<char>` layout optimization, so it will take only 4 bytes.')
print('pub const DECOMPOSITION_TABLE: &[(char, char, Option<char>)] = &[')

compose_data = []
with open(FILE_NAME) as f:
    for line in f:
        parts = line.split(';')
        if len(parts[5]) == 0:
            continue

        # Skip codepoints with compatibility formatting tags
        # since we care only about canonical mapping.
        if parts[5][0] == '<':
            continue

        # Print the decomposition table as is, since `UnicodeData` is already sorted.

        c = parts[0]
        mapping = parts[5].split(' ')
        if len(mapping) == 2:
            print(f"    ({hex_to_char_rs(c)}, {hex_to_char_rs(mapping[0])}, Some({hex_to_char_rs(mapping[1])})),")

            # Remember only codepoints that should be decomposed into two codepoints.
            compose_data.append([mapping[0], mapping[1], c])
        elif len(mapping) == 1:
            print(f'    ({hex_to_char_rs(c)}, {hex_to_char_rs(mapping[0])}, None),')
        else:
            raise 'invalid unicode data'

print('];')
print()


print('// The first value is `a << 32 | b`.')
print('// Sorted by the first value.')
print('pub const COMPOSITION_TABLE: &[(u64, char)] = &[')

pairs = []
for mapping in compose_data:
    needle = int(mapping[0], 16) << 32 | int(mapping[1], 16)
    pairs.append((needle, mapping[2]))

pairs.sort(key=lambda x: x[0])

# Make sure that needles are unique.
needles = set()
for pair in pairs:
    needles.add(pair[0])

assert len(pairs) == len(needles)

for pair in pairs:
    if pair[1] not in exclusions:
        print(f'    ({pair[0]}, {hex_to_char_rs(pair[1])}),')

print('];')