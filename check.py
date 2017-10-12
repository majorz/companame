#!/usr/bin/env python

# pylint: disable=C0111

import sys
from dns import resolver
import requests
import html
import time

def get_dns(domain):
    print('::', domain)

    try:
        answer = resolver.query(domain)
    except resolver.NXDOMAIN:
        print('No DNS')
        return
    except resolver.NoAnswer:
        print('No answer')
        return
    except resolver.Timeout:
        print('Timeout')
        return
    except resolver.NoNameservers:
        print('No nameservers')
        return
    
    print(' / '.join(answer.rrset.to_text().split('\n')))

def google_count(text):
    print('::', text)
    
    text.replace(' ', '+')
    response = requests.get('https://www.google.com/search?q="{}"'.format(text))

    content = requests.utils.get_unicode_from_response(response)

    if content.find('Our systems have detected unusual traffic from your computer network.') != -1:
        print('Captcha by Google')
        return

    if content.find('<span class="spell_orig">') != -1 or \
            content.find(' <b>"' + text + '"</b>.') != -1:
        print('No results found')
        return
    
    results_start = content.find('id="resultStats">')
    if results_start == -1:
        print('Error parsing results (start)')
        return

    results_end_first = content.find('</div>', results_start)
    results_end_second = content.find('<nobr>', results_start)
    if results_end_second == -1:
        results_end = results_end_first
    else:
        results_end = min(results_end_first, results_end_second)

    string = html.unescape(content[results_start+17:results_end])
    count = ''
    for char in string:
        if char.isdigit():
            count += char

    print(count, 'results')


def main():
    resolver.get_default_resolver().lifetime = 4.0

    for line in sys.stdin:
        name = line.strip()

        if not name:
            continue

        print('=============================')
        print('  ', name)
        print('-----------------------------')

        get_dns(name + '.com')

        get_dns(name + '.io')

        google_count(name + ' os')

        time.sleep(2)

        google_count(name + ' application')

        time.sleep(2)

        google_count(name + ' server')

        print('')


if __name__ == '__main__':
    main()
