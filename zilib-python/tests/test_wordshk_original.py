# vim: tabstop=4 expandtab shiftwidth=4 softtabstop=4

"""
Unit tests
"""

import unittest

import zilib
# from pylib import cantonese
# from pylib import util

class BaseTestCase(unittest.TestCase):
    """ Just a class to silence a pylint warning """
    def __init__(self, x):
        unittest.TestCase.__init__(self, x)

class SkipTestCase:
    # Don't test this
    pass

class Pronunciation(SkipTestCase):
    """Tests"""

    def test_ping3jam1(self):
        self.assertEqual(cantonese.get_ping3jam1('朝晚'), [['ziu1', 'ciu4'], ['maan5', 'maan1']])

        self.assertEqual(cantonese.get_n_ping3jam1('朝晚', 2), ["ziu1 maan5", "ziu1 maan1"])
        self.assertEqual(cantonese.get_n_ping3jam1('朝晚', 4), ["ziu1 maan5", "ziu1 maan1", "ciu4 maan5", "ciu4 maan1"])
        self.assertEqual(cantonese.get_n_ping3jam1('朝晚', 5), ["ziu1 maan5", "ziu1 maan1", "ciu4 maan5", "ciu4 maan1"])

        self.assertEqual(cantonese.canonicalize_jyutping("gaa1ze1"), "gaa1 ze1")
        self.assertEqual(cantonese.canonicalize_jyutping("gaa1 ze1"), "gaa1 ze1")
        self.assertEqual(cantonese.canonicalize_jyutping("gaa1  ze1"), "gaa1 ze1")
        self.assertEqual(cantonese.canonicalize_jyutping(" gaa1  ze1"), "gaa1 ze1")
        self.assertEqual(cantonese.canonicalize_jyutping(" gaa1  ze1 "), "gaa1 ze1")

        self.assertEqual(cantonese.canonicalize_jyutping("!T seot1", allow_exceptions=True), "!T seot1")
        self.assertEqual(cantonese.canonicalize_jyutping(" !T seot1", allow_exceptions=True), "!T seot1")
        self.assertEqual(cantonese.canonicalize_jyutping("  !T   seot1  ", allow_exceptions=True), "!T seot1")
        self.assertEqual(cantonese.canonicalize_jyutping("!T", allow_exceptions=True), "!T")
        self.assertEqual(cantonese.canonicalize_jyutping("!foobar", allow_exceptions=True), "!foobar")
        self.assertEqual(cantonese.canonicalize_jyutping(" seot1  !T ", allow_exceptions=True), "seot1 !T")
        self.assertEqual(cantonese.canonicalize_jyutping(" seot1  !foobar  ", allow_exceptions=True), "seot1 !foobar")
        self.assertEqual(cantonese.canonicalize_jyutping("seot1  !foobar", allow_exceptions=True), "seot1 !foobar")
        self.assertEqual(cantonese.canonicalize_jyutping(" !foobar seot1  !foobar", allow_exceptions=True), "!foobar seot1 !foobar")

        self.assertEqual(cantonese.canonicalize_jyutping("gaa ze1"), None)
        self.assertEqual(cantonese.canonicalize_jyutping("gaa1 ze"), None)
        self.assertEqual(cantonese.canonicalize_jyutping("foo1 bar1"), None)
        self.assertEqual(cantonese.canonicalize_jyutping("1 ga1 ze1"), None)
        self.assertEqual(cantonese.canonicalize_jyutping("!1ga1 ze"), None)

    def test_hk_variant(self):
        self.assertEqual(cantonese.hk_variant("留住溫度速度溫柔和憤怒"), "留住温度速度温柔和憤怒")
        self.assertEqual(cantonese.hk_variant("幫你淥個麵"), "幫你淥個麪")

        # Don't auto-convert lossy items (i.e. no 著=>着)
        self.assertEqual(cantonese.hk_variant("著作"), "著作")

        # OpenCC data converts to-and-from these 才 and 纔 characters. They
        # shouldn't be converted.
        self.assertEqual(cantonese.hk_variant("才"), "才")
        self.assertEqual(cantonese.hk_variant("纔"), "才")
        self.assertEqual(cantonese.hk_variant("衆"), "眾")
        self.assertEqual(cantonese.hk_variant("糉"), "粽")

        # Don't be overzelous
        self.assertEqual(cantonese.hk_variant("既"), "既")

        # Not supported yet
        self.assertEqual(cantonese.hk_variant("o既"), "o既")

        # Don't do simplified
        self.assertEqual(cantonese.hk_variant("或者個OL會問點解我放棄呢一段咁難得嘅姻緣"), "或者個OL會問點解我放棄呢一段咁難得嘅姻緣")

    def test_hk_std(self):
        self.assertEqual(cantonese.conformant("一二三"), [])


    def test_generate_variants(self):
        self.assertEqual(cantonese.generate_variants("我搭巴士返學"), [])
        # 一二三四五六七八九十
        # ① ② ③ ④ ⑤ ⑥ ⑦ ⑧ ⑨ ⑩
        # ❶ ❷ ❸ ❹ ❺ ❻ ❼ ❽ ❾ ➓
        numbers1 = {
            "一": "①",
            "二": "②",
            "三": "③",
            "四": "④",
            "五": "⑤",
            "六": "⑥",
            "七": "⑦",
            "八": "⑧",
            "九": "⑨",
            "十": "⑩",
        }
        self.assertEqual(cantonese.generate_variants("一一一一", reverse_map=numbers1), ["①①①①"])
        self.assertEqual(cantonese.generate_variants("一二", reverse_map=numbers1), (['一②', '①二', '①②']))
        self.assertEqual(cantonese.generate_variants("一二三四", reverse_map=numbers1), ([
            '一二三④', '一二③四', '一二③④',
            '一②三四', '一②三④', '一②③四', '一②③④',
            '①二三四', '①二三④', '①二③四', '①二③④',
            '①②三四', '①②三④', '①②③四', '①②③④']))

        self.assertEqual(cantonese.generate_variants("一二三四五", reverse_map=numbers1), ([
            '一二三四⑤', '一二三④五', '一二三④⑤', '一二③四五',
            '一二③四⑤', '一二③④五', '一二③④⑤', '一②三四五',
            '一②三四⑤', '一②三④五', '一②三④⑤', '一②③四五',
            '一②③四⑤', '一②③④五', '一②③④⑤', '①二三四五']))

        numbers2 = {
            "一": "①❶",
            "二": "②❷",
            "三": "③❸",
            "四": "④❹",
            "五": "⑤❺",
            "六": "⑥❻",
            "七": "⑦❼",
            "八": "⑧❽",
            "九": "⑨❾",
            "十": "⑩➓",
        }
        self.assertEqual(cantonese.generate_variants("一二", reverse_map=numbers2), (['一②', '一❷', '①二', '①②', '①❷', '❶二', '❶②', '❶❷']))
        self.assertEqual(cantonese.generate_variants("一二三", reverse_map=numbers2), ([
            '一二③', '一二❸', '一②三', '一②③', '一②❸', '一❷三',
            '一❷③', '一❷❸', '①二三', '①二③', '①二❸', '①②三',
            '①②③', '①②❸', '①❷三', '①❷③']))

        self.assertEqual(cantonese.generate_variants("七七八八", reverse_map=numbers2),
                         ['七七⑧⑧', '七七❽❽', '⑦⑦八八', '⑦⑦⑧⑧', '⑦⑦❽❽', '❼❼八八', '❼❼⑧⑧', '❼❼❽❽'])

        self.assertEqual(cantonese.generate_variants("一" * 100, reverse_map=numbers1), ["①" * 100])
        self.assertEqual(cantonese.generate_variants("一" * 100, reverse_map=numbers2), ["①" * 100, "❶" * 100])


class Util(SkipTestCase):
    """Tests"""

    def test_find_until(self):
        self.assertEqual(util.find_until("驚驚。感到 #害怕。", 100, "。"), "驚驚。感到 #害怕。")
        self.assertEqual(util.find_until("驚驚。感到 #害怕。", 5, "。"), "驚驚...")
        self.assertEqual(util.find_until("驚驚感到#害怕。", 5, "。"), "驚驚感到#...")
        self.assertEqual(util.find_until("驚驚驚驚", 5, "。"), "驚驚驚驚")
        self.assertEqual(util.find_until("驚驚驚驚驚", 5, "。"), "驚驚驚驚驚")
        self.assertEqual(util.find_until("驚驚驚驚驚驚", 5, "。"), "驚驚驚驚驚...")

class TestCommon(BaseTestCase):
    """Tests"""

    def test_sentence(self):
        self.assertEqual(zilib.looks_like_a_sentence('你好'), False)
        self.assertEqual(zilib.looks_like_a_sentence('你好！'), True)
        self.assertEqual(zilib.looks_like_a_sentence('一本書'), False)
        self.assertEqual(zilib.looks_like_a_sentence('一本，書'), True)
        self.assertEqual(zilib.looks_like_a_sentence('一本書。'), True)
        self.assertEqual(zilib.looks_like_a_sentence('一本書？'), True)
        self.assertEqual(zilib.looks_like_a_sentence('屌你老母仆街陷家鏟'), True)

    def test_guess_language(self):
        CHINESE = "zh"
        ENGLISH = "en"
        UNKNOWN_LANGUAGE = "xx"
        self.assertEqual(zilib.guess_language("我的名字叫做「小明」"), CHINESE)
        self.assertEqual(zilib.guess_language("「小明」"), CHINESE)
        self.assertEqual(zilib.guess_language("Happy birthday"), ENGLISH)
        self.assertEqual(zilib.guess_language("Holy shit thank you!!!!"), ENGLISH)
        self.assertEqual(zilib.guess_language("你好，我叫Jessica"), CHINESE)
        self.assertEqual(zilib.guess_language("介紹我個boss俾你識"), CHINESE)
        self.assertEqual(zilib.guess_language("Come on, James，可不可以成熟一點呢"), CHINESE)

        # Marginal cases
        self.assertEqual(zilib.guess_language("Fuck 屌你"), CHINESE)
        self.assertEqual(zilib.guess_language("Fuck you 屌你"), ENGLISH)
        self.assertEqual(zilib.guess_language("Happy birthday 祝你生日快樂"), CHINESE)
        self.assertEqual(zilib.guess_language("多謝晒！Holy shit thank you!!!!"), ENGLISH)

        # Unknown
        self.assertEqual(zilib.guess_language("小"), UNKNOWN_LANGUAGE)
        self.assertEqual(zilib.guess_language("屌"), UNKNOWN_LANGUAGE)
        self.assertEqual(zilib.guess_language(""), UNKNOWN_LANGUAGE)
        self.assertEqual(zilib.guess_language("i"), UNKNOWN_LANGUAGE)
        self.assertEqual(zilib.guess_language("ib"), UNKNOWN_LANGUAGE)
        self.assertEqual(zilib.guess_language("..."), UNKNOWN_LANGUAGE)
        self.assertEqual(zilib.guess_language("!@#$%"), UNKNOWN_LANGUAGE)
        self.assertEqual(zilib.guess_language("。"), UNKNOWN_LANGUAGE)

    def test_ruby_match(self):
        def rm(a, b):
            return zilib.ruby_match_plain(a, b)

        t = "呢度嘅食物質素返咁上下"
        p = "ni1 dou1 ge3 sik6 zat1 sou3 dou1 jau5 faan1 gam3 soeng6 haa2"
        r = "呢ni1 度dou1 嘅ge3 食sik6 物 質zat1 素sou3dou1jau5 返faan1 咁gam3 上soeng6 下haa2"
        self.assertEqual(rm(t, p), r)

        t = "一二三四。"
        p = "jat1 ji6 saam1 sei3"
        r = "一jat1 二ji6 三saam1 四sei3 。"
        self.assertEqual(rm(t, p), r)

        t = "一二三四。"
        p = "jat1 ji6 saam1 sei3 aa1 aa1"
        r = "一jat1 二ji6 三saam1 四sei3aa1aa1 。"
        self.assertEqual(rm(t, p), r)

        t = "九唔搭八。"
        p = "jat1 ji6 saam1 sei3"
        r = "九jat1 唔ji6 搭saam1 八sei3 。"
        self.assertEqual(rm(t, p), r)

        t = "九唔搭八。"
        p = "jat1 gau2 ji6 saam1 sei3 m4"
        r = "九jat1gau2ji6saam1sei3 唔m4 搭 八 。"
        self.assertEqual(rm(t, p), r)

        t = "九唔搭八。"
        p = "jat1 gau2 ji6 saam1 sei3 daap3"
        r = "九jat1gau2ji6saam1 唔sei3 搭daap3 八 。"
        self.assertEqual(rm(t, p), r)

        t = "ＳＥＲＶＥＲ。"
        p = "soe1 faa2"
        r = "ＳＥＲＶＥＲsoe1faa2 。"
        self.assertEqual(rm(t, p), r)

        t = "ＳＥＲＶＥＲ死。"
        p = "soe1 faa2 sei2"
        r = "ＳＥＲＶＥＲsoe1faa2 死sei2 。"
        self.assertEqual(rm(t, p), r)

        t = "Hi，你好嗎？"
        p = "haai1 nei5 hou2 maa3"
        r = "Hihaai1 ， 你nei5 好hou2 嗎maa3 ？"
        self.assertEqual(rm(t, p), r)

        t = "Hi Hi，你好嗎？"
        p = "haai1 haai1 nei5 hou2 maa3"
        r = "Hi Hihaai1haai1 ， 你nei5 好hou2 嗎maa3 ？"
        self.assertEqual(rm(t, p), r)

        t = "你條 hi hi 好快啲返嚟！"
        p = "nei5 tiu4 haai1 haai1 hou2 faai3 di1 faan1 lei4"
        r = "你nei5 條tiu4   hi hihaai1haai1   好hou2 快faai3 啲di1 返faan1 嚟lei4 ！"
        self.assertEqual(rm(t, p), r)

        t = "個 server 死咗，點算好？"
        p = "go3 soe1 faa2 sei2 zo2 dim2 syun3 hou2"
        r = "個go3   serversoe1faa2   死sei2 咗zo2 ， 點dim2 算syun3 好hou2 ？"
        self.assertEqual(rm(t, p), r)

        t = "個 ｓｅｒｖｅｒ 死咗，點算好？"
        p = "go3 soe1 faa2 sei2 zo2 dim2 syun3 hou2"
        r = "個go3   ｓｅｒｖｅｒsoe1faa2   死sei2 咗zo2 ， 點dim2 算syun3 好hou2 ？"
        self.assertEqual(rm(t, p), r)

        t = "個 「ｓｅｒｖｅｒ」 死咗，點算好？"
        p = "go3 soe1 faa2 sei2 zo2 dim2 syun3 hou2"
        r = "個go3   「 ｓｅｒｖｅｒsoe1faa2 」   死sei2 咗zo2 ， 點dim2 算syun3 好hou2 ？"
        self.assertEqual(rm(t, p), r)

        t = "個 「『ｓｅｒｖｅｒ』」 死咗，點算好？"
        p = "go3 soe1 faa2 sei2 zo2 dim2 syun3 hou2"
        r = "個go3   「 『 ｓｅｒｖｅｒsoe1faa2 』 」   死sei2 咗zo2 ， 點dim2 算syun3 好hou2 ？"
        self.assertEqual(rm(t, p), r)

        t = "「『ｓｅｒｖｅｒ』」 死咗，點算好？"
        p = "soe1 faa2 sei2 zo2 dim2 syun3 hou2"
        r = "「 『 ｓｅｒｖｅｒsoe1faa2 』 」   死sei2 咗zo2 ， 點dim2 算syun3 好hou2 ？"
        self.assertEqual(rm(t, p), r)

        t = "個ＳＥＲＶＥＲ死咗，點算好？"
        p = "go3 soe1 faa2 sei2 zo2 dim2 syun3 hou2"
        r = "個go3 ＳＥＲＶＥＲsoe1faa2 死sei2 咗zo2 ， 點dim2 算syun3 好hou2 ？"
        self.assertEqual(rm(t, p), r)

        t = "我 hi five 你！"
        p = "ngo5 haai1 fai1 nei5"
        r = "我ngo5   hi fivehaai1fai1   你nei5 ！"
        self.assertEqual(rm(t, p), r)

        t = "我 hi asdf five 你！"
        p = "ngo5 haai1 ei1 e1 di1 e1 fai1 nei5"
        r = "我ngo5   hi asdf fivehaai1ei1e1di1e1fai1   你nei5 ！"
        self.assertEqual(rm(t, p), r)

        t = "一二唔見咗一堆三四。"
        p = "jat1 ji6 saam1 sei3"
        r = "一jat1 二ji6 唔 見 咗 一 堆 三saam1 四sei3 。"
        self.assertEqual(rm(t, p), r)

        t = "「ｓｅｒｖｅｒ」死咗"
        p = "soe1 faa2 sei2 zo2"
        r = "「 ｓｅｒｖｅｒsoe1faa2 」 死sei2 咗zo2"
        self.assertEqual(rm(t, p), r)

        t = "「server」同「server」之間！"
        p = "soe1 faa2 tung4 soe1 faa2 zi1 gaan1"
        r = "「 serversoe1faa2 」 同tung4 「 serversoe1faa2 」 之zi1 間gaan1 ！"
        self.assertEqual(rm(t, p), r)

        t = "「server」同「server」！"
        p = "soe1 faa2 tung4 soe1 faa2"
        r = "「 serversoe1faa2 」 同tung4 「 serversoe1faa2 」 ！"
        self.assertEqual(rm(t, p), r)

        t = "hi 搜 hi！"
        p = "haai1 sau1 sau2 sau3 sau4 sau5 sau6 haai1"
        r = "hihaai1sau1   搜sau2   hisau3sau4sau5sau6haai1 ！"
        self.assertEqual(rm(t, p), r)

        # Not obvious what the result should be.
        t = "九唔搭八。"
        p = "jat1 ji6 saam1 sei3 aa1 aa1"
        self.assertEqual(rm(t, p), "九jat1ji6saam1 唔sei3 搭aa1 八aa1 。")

        t = "九唔搭八。"
        p = "jat1 gau2 ji6 saam1 sei3"
        self.assertEqual(rm(t, p), "九jat1gau2 唔ji6 搭saam1 八sei3 。")

        t = "九唔搭八。"
        p = "jat1 ji6 saam1 sei3 gau2"
        r = "九jat1ji6saam1sei3gau2 唔 搭 八 。"
        self.assertEqual(rm(t, p), r)

        t = "九唔搭八九。"
        p = "jat1 ji6 saam1 sei3 gau2"
        r = "九jat1 唔ji6 搭saam1 八sei3 九gau2 。"
        self.assertEqual(rm(t, p), r)

        t = "九唔搭八。"
        p = "jat1 daap1 baat3 saam1 sei3 gau2"
        r = "九 唔jat1 搭daap1 八baat3saam1sei3gau2 。"
        self.assertEqual(rm(t, p), r)

        t = "我部XYZ死咗。"
        p = "ngo5 bou6 sei2 zo2"
        r = "我ngo5 部bou6 XYZ 死sei2 咗zo2 。"
        self.assertEqual(rm(t, p), r)

        rmm = zilib.ruby_match_max()
        t = "一 hi 搜 hi 一！" * (rmm // 10)
        p = ("jat1 haai1 sau1 sau2 sau3 sau4 sau5 sau6 haai1 jat1 " * (rmm // 10)).strip()
        r = ("一jat1   hihaai1sau1   搜sau2   hisau3sau4sau5sau6haai1   一jat1 ！ " * (rmm // 10)).strip()
        self.assertEqual(rm(t, p), r)

        t = "一" * rmm
        p = " ".join(["jat1"] * rmm)
        r = " ".join(["一jat1"] * rmm)
        self.assertEqual(rm(t, p), r)

        t = "一" * (rmm - 1)
        p = " ".join(["jat1"] * rmm)
        r = " ".join(["一jat1jat1",] + ["一jat1"] * (rmm - 2))
        self.assertEqual(rm(t, p), r)

        t = "一" * (rmm - 1) + "。"
        p = " ".join(["jat1"] * (rmm - 2))
        r = "一 " + " ".join(["一jat1"] * (rmm - 2)) + " 。"
        self.assertEqual(rm(t, p), r)

        t = "1微米係1000000分之1米。"
        p = "jat1 mei4 mai5 hai6 jat1 baak3 maan6 fan6 zi1 jat1 mai5."
        r = "1jat1 微mei4 米mai5 係hai6 1000000jat1baak3maan6 分fan6 之zi1 1jat1 米mai5 。"
        self.assertEqual(rm(t, p), r)

        t = "1微米係１００００００分之1米。"
        p = "jat1 mei4 mai5 hai6 jat1 baak3 maan6 fan6 zi1 jat1 mai5."
        r = "1jat1 微mei4 米mai5 係hai6 １００００００jat1baak3maan6 分fan6 之zi1 1jat1 米mai5 。"
        self.assertEqual(rm(t, p), r)

        t = "呢度嘅#食物 質素返咁上下"
        p = "ni1 dou1 ge3 sik6 zat1 sou3 dou1 jau5 faan1 gam3 soeng6 haa2"
        r = "呢ni1 度dou1 嘅ge3 #食物sik6   質zat1 素sou3dou1jau5 返faan1 咁gam3 上soeng6 下haa2"
        self.assertEqual(rm(t, p), r)

        t = "傻仔 #懵盛盛#condom#哈#condom#哈#哈 做#condom。"
        p = "so4 zai2 mung2 sing6 sing6 kon1 dam4 haa1 kon1 dam4 haa1 haa1 zou6 kon1 dam4"
        r = "傻so4 仔zai2   #懵盛盛mung2sing6sing6kon1 #condomdam4 #哈haa1 #condomkon1dam4 #哈haa1 #哈haa1   做zou6 #condomkon1dam4 。"
        self.assertEqual(rm(t, p), r)

        t = "#。"
        p = "zeng2"
        r = "#zeng2 。"
        self.assertEqual(rm(t, p), r)

        t = "井#。"
        p = "zeng2"
        r = "井zeng2 # 。"
        self.assertEqual(rm(t, p), r)

        t = "丁#井。"
        p = "zeng2"
        r = "丁 #井zeng2 。"
        self.assertEqual(rm(t, p), r)

        t = "井#丁"
        p = "zeng2"
        r = "井zeng2 #丁"
        self.assertEqual(rm(t, p), r)

        t = "#配圖"
        p = "pui3 tou4"
        r = "#配圖pui3tou4"
        self.assertEqual(rm(t, p), r)

        t = "#J圖"
        p = "zei1 tou4"
        r = "#J圖zei1tou4"
        self.assertEqual(rm(t, p), r)

        t = "#JJ圖"
        p = "zei1 zei1 tou4"
        r = "#JJ圖zei1zei1tou4"
        self.assertEqual(rm(t, p), r)

        t = "#JJ 圖"
        p = "zei1 zei1 tou4"
        r = "#JJzei1zei1   圖tou4"
        self.assertEqual(rm(t, p), r)

if __name__ == '__main__':
    unittest.main()
