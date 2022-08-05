import random


def tano_singo():
    box = ["ドド", "スコ"]
    res = ""
    while True:
        x = random.randint(0, 1)
        res = res + box[x]
        if res.endswith("ドドスコスコスコドドスコスコスコドドスコスコスコ"):
            res = res + "ラブ注入♡"
            break
    print(res)
