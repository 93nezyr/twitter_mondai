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

def loidy_tano_singo() -> None:
    """loidyが実装したドドスコプログラムをリファクタする場所です.

    ## WIP
    TODO: リファクタリング
    """
    box = ["ドド", "スコ"]
    x = 0
    res = ""
    while x != 2:
        for n in range(9):
            x = random.randint(0, 1)
            res = res + box[x]
        print(res)
        if res == "ドドスコスコスコドドスコスコスコドドスコスコス":
            break
        else:
            res = ""
    print("ラブ注入♡")
