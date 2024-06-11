#!/usr/bin/env python3

import zilib
import unittest

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

        t = "有班#怪叔叔 睺緊個正太"
        p = "jau5 baan1 gwaai3 suk1 suk1 hau1 gan2 go3 zing3 taai3"
        self.assertEqual(rm(t, p), "有jau5 班baan1 #怪叔叔gwaai3suk1suk1   睺hau1 緊gan2 個go3 正zing3 太taai3")

        t = "有#怪叔叔 呀"
        p = "jau5 gwaai3 suk1 suk1 aa3"
        self.assertEqual(rm(t, p), "有jau5 #怪叔叔gwaai3suk1suk1   呀aa3")

        t = "嘅#食物 質"
        p = "ge3 sik6 zat1"
        self.assertEqual(rm(t, p), "嘅ge3 #食物sik6   質zat1")

        t = "一個二個"
        p = "gau2 m4 daap3 baat3" # Wrong pronunciation for testing
        self.assertEqual(rm(t, p), "一gau2 個m4 二daap3 個baat3")

        t = "劉博"
        p = "puk1 zaak6" # Wrong pronunciation for testing
        self.assertEqual(rm(t, p), "劉puk1 博zaak6")

        t = "劉博士"
        p = "puk1 bok3 si6" # Wrong pronunciation for testing
        self.assertEqual(rm(t, p), "劉puk1 博bok3 士si6")

        t = "劉博士劉博士劉博士劉博士"
        p = "puk1 bok3 si6 puk1 bok3 si6 puk1 bok3 si6 puk1 bok3 si6" # Wrong pronunciation for testing
        self.assertEqual(rm(t, p), "劉puk1 博bok3 士si6 劉puk1 博bok3 士si6 劉puk1 博bok3 士si6 劉puk1 博bok3 士si6")


        t = "約翰‧威克里夫"
        p = "joek3 hon6 wai1 haak1 lei5 fu1"
        self.assertEqual(rm(t, p), "約joek3 翰hon6 ‧ 威wai1 克haak1 里lei5 夫fu1")

        t = "揸BB車"
        p = "zaa1 bi4 bi1 ce1"
        self.assertEqual(rm(t, p), "揸zaa1 BBbi4bi1 車ce1")

        t = "袋咁多coins喺身，唔重㗎咩？"
        p = "doi6 gam3 do1 kon1 si2 hai2 san1, m4 cung5 gaa3 me1"
        print(rm(t, p))
        self.assertEqual(rm(t, p), "袋doi6 咁gam3 多do1 coinskon1si2 喺hai2 身san1 ， 唔m4 重cung5 㗎gaa3 咩me1 ？")

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
