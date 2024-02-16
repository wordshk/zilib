#!/usr/bin/env python3

import zilib
import unittest
import bz2
import csv

class LocalTests(unittest.TestCase):
    def test_degenerate(self):
        self.assertEqual('', zilib.american_english_stem(''))
        self.assertEqual('', zilib.american_english_stem('!'))
        self.assertEqual('', zilib.american_english_stem('\n'))
        self.assertEqual('', zilib.american_english_stem('\r\n'))
        self.assertEqual('', zilib.american_english_stem(' '))
        self.assertEqual('', zilib.american_english_stem('  '))
        self.assertEqual('latin', zilib.american_english_stem(' latinize '))
        self.assertEqual('latinizegreatest', zilib.american_english_stem('latinize greatest'))
        self.assertEqual('greatestlatin', zilib.american_english_stem('greatest latinize'))
        self.assertEqual('gretestlatin', zilib.american_english_stem('greätest latinize'))
        self.assertEqual('latinizegreatest', zilib.american_english_stem('latinize 我 greatest'))
        self.assertEqual('latinizegreatest', zilib.american_english_stem('我latinize我greatest我'))
        self.assertEqual('greatestlatin', zilib.american_english_stem('我greatest我latinize我'))

    def test_ruby_match(self):
        def rm(a, b):
            return zilib.ruby_match_plain(a, b)

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

    def test_simple_loading(self):
        # Just check whether there's some data here
        self.assertTrue(len(zilib.wordshk_charset()) > 1000)
        self.assertTrue(len(zilib.wordshk_variantmap()) > 1000)
        self.assertTrue(len(zilib.wordshk_autoconvertmap()) > 10)

if __name__ == '__main__':
    unittest.main()
