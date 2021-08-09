import codecs
import contextlib
import os
import random
import re
import shutil
import string
import sys
import time
from itertools import chain

from flashtext.keyword import KeywordProcessor


# 复制
def copy_target_as_pyd():
    with contextlib.suppress(Exception):
        shutil.copy(
            os.path.realpath(
                os.path.join(__file__, '..', '..', '..', 'target', 'release',
                             'python_comm.dll')),
            os.path.realpath(os.path.join(__file__, '..', 'python_comm.pyd')))

    shutil.copy(
        os.path.realpath(
            os.path.join(__file__, '..', '..', '..', 'target', 'release',
                         'libpython_comm.so')),
        os.path.realpath(os.path.join(__file__, '..', 'python_comm.so')))


copy_target_as_pyd()
import python_comm


def benchmark(story_size):
    # 创建随机单词
    all_words = [
        generate_random_word_of_length(random.choice([3, 4, 5, 6, 7, 8]))
        for i in range(max(100000, story_size + 100))
    ]

    print('Keys   | Length    | FlashText |     Regex |      Rust')
    print('------------------------------------------------------')
    for keywords_count in chain(
            range(1, 100, 10),
            range(100, 1000, 100),
            range(1000, 10000, 1000),
    ):
        # 选 5000 个词组成一篇随机文章
        all_words_chosen = random.sample(all_words, story_size)
        story = ' '.join(all_words_chosen)

        # keywords_count 个关键字
        unique_keywords_sublist = list(
            set(random.sample(all_words, keywords_count)))

        # 编译正则表达式
        rep = dict([(key, '_keyword_') for key in unique_keywords_sublist])
        compiled_re = re.compile("|".join(rep.keys()))

        # 初始化 FlashText
        keyword_processor = KeywordProcessor()
        for keyword in unique_keywords_sublist:
            keyword_processor.add_keyword(keyword, '_keyword_')

        # 初始化 TextSearch
        tsid = python_comm.text_search_ex_init([
            (x, '_keyword_') for x in unique_keywords_sublist
        ])

        # Replace 计时
        t0 = time.time()
        r1 = keyword_processor.replace_keywords(story)
        t1 = time.time()
        r2 = compiled_re.sub(lambda m: rep[re.escape(m.group(0))], story)
        t2 = time.time()
        r3 = python_comm.text_search_ex_subst(tsid, story)
        t3 = time.time()
        r4 = python_comm.text_search_ex_match(tsid, story, "")
        t4 = time.time()

        python_comm.text_search_ex_free(tsid)

        # 汇总
        print(
            str(keywords_count).ljust(6),
            '|',
            str(len(story)).ljust(9),
            '|',
            "{0:.5f}".format(t1 - t0).rjust(9),
            '|',
            "{0:.5f}".format(t2 - t1).rjust(9),
            '|',
            "{0:.5f}".format(t3 - t2).rjust(9),
            '|',
            "{0:.5f}".format(t4 - t3).rjust(9),
        )

        # 校验
        # if r3 != r2:
        #     print('story: ', story)
        #     print('keywords: ', unique_keywords_sublist)
        #     print('r2', r2)
        #     print('r3', r3)
        #     return


# 创建随机单词
def generate_random_word_of_length(str_length):
    return ''.join(
        random.choice(string.ascii_lowercase) for _ in range(str_length))


def test():
    if len(sys.argv) >= 3:
        with codecs.open(sys.argv[1], 'r', 'utf-8') as ifile:
            text = ifile.read()

    # 初始化 TextSearch
    tsid = python_comm.text_search_ex_init([(x, None) for x in sys.argv[2:]])
    for line, _, _ in python_comm.text_search_ex_match(tsid, text, "l"):
        print(line)

    python_comm.text_search_ex_free(tsid)


if __name__ == '__main__':
    benchmark(200000)
    test()
