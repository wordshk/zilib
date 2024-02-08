#!/usr/bin/env python3

from pylib import latin as lib
import zilib
import unittest
import bz2
import csv



class StemmerTest():
    def stem(self, word):
        self.assertEqual(lib.american_english_stem(word), zilib.american_english_stem(word), f'Failed to stem {word}')

    def test_degenerate(self):
        self.stem('')
        self.stem('!')
        self.stem('\n')
        self.stem('\r\n')
        self.stem(' ')
        self.stem('  ')
        self.stem(' latinize ')
        self.stem('latinize greatest')
        self.stem('greatest latinize')

        # These are discrepancies between unicodedecode and stripping away non-ascii characters
        self.assertEqual('gretestlatin', zilib.american_english_stem('greätest latinize'))
        self.assertEqual('latinizegreatest', zilib.american_english_stem('latinize 我 greatest'))
        self.assertEqual('latinizegreatest', zilib.american_english_stem('我latinize我greatest我'))
        self.assertEqual('greatestlatin', zilib.american_english_stem('我greatest我latinize我'))

    def test_simple(self):
        self.stem('caresses')
        self.stem('ponies')
        self.stem('ties')
        self.stem('caress')
        self.stem('cats')
        self.stem('feed')
        self.stem('agreed')
        self.stem('plastered')
        self.stem('bled')
        self.stem('motoring')
        self.stem('sing')
        self.stem('conflated')
        self.stem('troubled')
        self.stem('sized')
        self.stem('hopping')
        self.stem('tanned')
        self.stem('falling')
        self.stem('hissing')
        self.stem('fizzed')
        self.stem('failing')
        self.stem('filing')
        self.stem('happy')
        self.stem('sky')
        self.stem('quickly')
        self.stem('running')
        self.stem('dying')
        self.stem('tying')
        self.stem('flying')

    def test_comprehensive(self):
        # Open ../lists/en_unigram_freq.csv.bz2 and test all words
        with bz2.open('../lists/en_unigram_freq.csv.bz2', 'rt') as f:
            csvreader = csv.reader(f)
            for row in csvreader:
                self.stem(row[0])

class LocalTests(unittest.TestCase):
    def test_ruby_match(self):
        def rm(a, b):
            return zilib.ruby_match(a, b)[0]

        t = "X丫X丫X丫X丫X丫"
        p = "tik1 waai1 tik1 waai1 tik1 waai1 tik1 waai1 tik1 waai1"
        self.assertEqual(rm(t, p), "Xtik1 丫waai1 Xtik1 丫waai1 Xtik1 丫waai1 Xtik1 丫waai1 Xtik1 丫waai1")

        t = ""
        p = ""
        self.assertEqual(rm(t, p), "")

        t = "x"
        p = ""
        self.assertEqual(rm(t, p), "x")

        t = "#x"
        p = ""
        self.assertEqual(rm(t, p), "#x")

        t = "#卜 #正 #卜 #正"
        p = "haa1 lou2 haa1 lou2"
        self.assertEqual(rm(t, p), "#卜haa1   #正lou2   #卜haa1   #正lou2")

    def test_segmentation(self):
        self.assertEqual(zilib.end_user_friendly_segment("你真係咩事屈機呀唔知死未!")[2], ['你', '真係', '咩事', '屈機', '呀', '唔知死', '未', '!'])
        self.assertEqual(zilib.end_user_friendly_segment("你真係咩事屈機呀死未知數!")[2], ['你', '真係', '咩事', '屈機', '呀', '死', '未知數', '!'])

        # More tests, but more polite
        self.assertEqual(zilib.end_user_friendly_segment('中國人')[2], '中國 人'.split())
        self.assertEqual(zilib.end_user_friendly_segment('唔知道')[2], '唔知 道'.split())  # Maybe we should use some other heuristic for this to ensure the singled out word is more commonly used as single word or something using frequency lists

if __name__ == '__main__':
    unittest.main()
