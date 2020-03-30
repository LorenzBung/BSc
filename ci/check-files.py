#!/usr/bin/env python2
from __future__ import print_function

import re
import sys

from os import listdir
from os.path import dirname, isfile, join, isdir

ROOT_DIR = join(dirname(__file__), '..')
FILES_FOLDER = join(ROOT_DIR, 'files')

def natural_sort_key(value):
    return [int(s) if s.isdigit() else s.lower() for s in re.split(r"(\d+)", value)]


def get_file_lists():
    if not isdir(FILES_FOLDER):
        print("! no file lists found")
        return {}

    list_files = [f for f in listdir(FILES_FOLDER) if isfile(join(FILES_FOLDER, f))]
    file_lists = {}

    for list_file in list_files:
        with open(join(FILES_FOLDER, list_file)) as lf:
            file_lists[list_file] = [f.strip() for f in lf.readlines() if len(f.strip()) > 0]

    return file_lists


def check_files(file_list):
    optional_folders = []
    errors = 0

    for f in file_list:
        # optional dir
        if f.startswith('?'):
            optional_folders.append(f[1:].strip())
        # else not a file -> error
        elif not isfile(f):
            # ignore if path starts with optional folder path
            if len(filter(lambda x: f.startswith(x), optional_folders)) > 0:
                continue

            print("! '{}' is missing".format(f))
            errors += 1

    if errors == 0:
        return True
        print(" -> All Files found.")
    return False


def main():
    file_lists = get_file_lists()
    newest_folder = sorted(file_lists.keys(), key=natural_sort_key)[-1]
    print(newest_folder)
    if not check_files(file_lists[newest_folder]):
        sys.exit(1)


if __name__ == '__main__':
    main()
