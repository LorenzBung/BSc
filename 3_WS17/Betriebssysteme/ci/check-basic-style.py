#!/usr/bin/env python2
from __future__ import print_function

# Checks
# - trailing whitespaces (not allowed)
# - single trailing newline (required)
# - bad windows/mac line endings
# - tab characters
# - lines longer than XX chars
from os import walk
from os.path import join

import re

FOLDER = "hw"
EXTENSIONS = ["rs", "toml", "sh"]
MAX_WIDTH = 120


def get_files():
    file_list = []

    # for all files and folders in pwd
    for root, dirs, files in walk("."):
        # if folder starts with "hw"
        if root.startswith("./" + FOLDER):
            # for all files in sub folder of "hw*"
            for file in files:
                # if extension match
                for ext in EXTENSIONS:
                    if file.lower().endswith("." + ext):
                        file_list.append(join(root, file))

    return file_list


def check_file(file):
    line_num = 1
    error = False

    # open in binary mode to disable universal newlines (to see "\r")
    with open(file, mode='rb') as f:
        content = f.read().decode(encoding='utf-8')
        line_feed = re.compile("\r")

        if line_feed.search(content):
            error = True
            print("\t! Contains windows/mac line ending")

        if not content.endswith("\n"):
            error = True
            print("\t! Has no single trailing newline")

        for line in content.split("\n"):
            line_format = "Line {: >3d}: {}\n\t{}"
            doc_string = re.compile("^\s*//[/!]")
            trailing_whitespaces = re.compile(" +$")
            tab_char = re.compile("\t")

            # ignore doc string in rust
            if file.lower().endswith(".rs") and doc_string.match(line):
                continue

            if trailing_whitespaces.search(line):
                error = True
                print(line_format.format(line_num, line, "! Line has trailing whitespaces"))

            if tab_char.search(line):
                error = True
                print(line_format.format(line_num, line, "! Line has tab character"))

            if len(line) >= MAX_WIDTH:
                error = True
                print(line_format.format(line_num, line, "! Line is longer than {} characters".format(MAX_WIDTH)))

            line_num += 1

    return not error


def main():
    errors = 0

    for file in get_files():
        print(file)
        if not check_file(file):
            errors += 1

    exit(errors)

if __name__ == '__main__':
    main()

