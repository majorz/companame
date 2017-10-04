#!/usr/bin/env python

# pylint: disable=C0111, R0911

import argparse
from itertools import product
from string import ascii_lowercase

CONSONANTS = 'bcdfghklmnprstvx'
VOWELS = 'aeiou'

def pair(first, second):
    bad = {
        'a': 'aehikouy',
        'b': 'abcdfghkmnprtuvxy',
        'c': 'abdefgiklmnprsuvxy',
        'd': 'bcdfghklmnpstvxy',
        'e': 'aeiou',
        'f': 'abcdeghikmnoprstuvxy',
        'g': 'bcdfghklmnpstuvxy',
        'h': 'bcdghklmnprstuvx',
        'i': 'abefhikovy',
        'k': 'abcdeghkmnopstuvx',
        'l': 'dfhlux',
        'm': 'dfghklmtuvxy',
        'n': 'bcfghklmnuvxy',
        'o': 'aehikuy',
        'p': 'bcdfgkmnpsvxu',
        'r': 'cfghknpruxy',
        's': 'bcdfgrsuvxy',
        't': 'bdfgklmnpstvx',
        'u': 'dghkou',
        'v': 'bcdfghkmnpstuvx',
        'x': 'abcdefghklmnoprstuvx',
        'y': 'adegihklmnrsuxy',
    }

    if second in bad[first]:
        return False

    return True


def start_valid(first, second):
    bad = {
        'a': 'bcdfgitv',
        'b': 'eiols',
        'c': 'ct',
        'd': 'o',
        'e': 'bghky',
        'g': 'eio',
        'i': 'cdglmnprstux',
        'k': 'ilry',
        'l': 'bcikgmnprstv',
        'm': 'bcnprs',
        'n': 'dprst',
        'o': 'flos',
        'p': 'txy',
        'r': 'bdlmstv',
        's': 'm',
        't': 'chy',
        'u': 'aceiflmstvy',
        'v': 'l',
        'x': 'y',
        'y': 'bcfptv',
    }

    if second in bad.get(first, ''):
        return False

    return True


def end_valid(first, second):
    if first + second in [
            'ad',
            'ag',
            'al',
            'am',
            'an',
            'ar',
            'ax',
            'bo',
            'be',
            'ce',
            'da',
            'de',
            'do',
            'du',
            'ed',
            'ee',
            'eg',
            'eh',
            'ek',
            'eo',
            'ex',
            'fa',
            'ga',
            'ge',
            'go',
            'ha',
            'he',
            'ho',
            'id',
            'ig',
            'il',
            'io',
            'la',
            'le',
            'ma',
            'me',
            'ne',
            'od',
            'og',
            'oo',
            'ol',
            'om',
            'or',
            'ox',
            'pa',
            'pe',
            'po',
            're',
            'ro',
            'ru',
            'sa',
            'se',
            'so',
            'su',
            'ta',
            'te',
            'tu',
            'ua',
            'ue',
            'ul',
            'un',
            'ur',
            'ux',
            'va',
            've',
            'vo',
    ]:
        return False

    if first in CONSONANTS and second in CONSONANTS:
        return False

    return True


def triple(first, second, third):
    if first in CONSONANTS and second in CONSONANTS and third in CONSONANTS:
        return False

    if first in VOWELS and second in VOWELS and third in VOWELS:
        return False

    return True


def valid(chars):
    for char in chars:
        if char in 'qjwz':
            return False

    if chars[-1] in 'bcfiptvy':
        return False

    if not start_valid(chars[0], chars[1]):
        return False

    if not end_valid(chars[-2], chars[-1]):
        return False

    for j in range(0, len(chars) - 2):
        if not triple(chars[j], chars[j+1], chars[j+2]):
            return False

    for j in range(0, len(chars) - 1):
        if not pair(chars[j], chars[j+1]):
            return False

    return True


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-c', type=int, default=5, help='character count (default: 5)')
    parser.add_argument('-s', type=str, default='', help='start sequence')
    parser.add_argument('-e', type=str, default='', help='end sequence')
    args = parser.parse_args()

    start = args.s
    end = args.e
    delta = args.c - len(start) - len(end)

    if delta < 0:
        print('Character count exceeds start and end sequence sum')
        exit(1)

    for delta_chars in product(ascii_lowercase, repeat=delta):
        chars = start + ''.join(delta_chars) + end
        if not valid(chars):
            continue
        print(chars)

if __name__ == '__main__':
    main()
