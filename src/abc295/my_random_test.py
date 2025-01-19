#! /usr/bin/python3

import sys
import subprocess
from random import randint


# target
c_p = input("contest, problem = ")
if c_p == "": # デフォルト入力
    c = "abc295"
    p = "d"
else:
    c, p = c_p.split()

# BUILD
bld = "cargo build --bin "+ c + "_" + p
res = subprocess.run(bld, shell=True)

if res.returncode != 0:
    print("build error")
    sys.exit()


# RANDOM TEST
def maketest():
    print("\n* TEST")

    ### 制約
    intxt: str = "" 
    intxt += "atcoder\n"
    intxt += "?????\n"

    print("* INPUT:")
    print(intxt)
    return intxt


def rundebug(intxt):
    print("\n\n* DEBUG:")

    cnt = [0 for _ in range(10)]
    res = 0
    N = len(intxt)
    for l in range(0,N):
        for r in range(l+1,N+1):
            cnt = [0 for _ in range(10)]
            # print(l,r, intxt[l:r])
            for i in intxt[l:r]:
                cnt[int(i)] += 1
            for i in range(10):
                cnt[i] %= 2
            if not any(cnt): res += 1
    print(res)
    return res

def runrust(c,p,intxt):
    print("* RUN: ")
    cmd = "./target/debug/"+ c + "_" + p
    out = subprocess.run(cmd, shell=True, input=intxt, capture_output=True,text=True)
    print(out.stdout)
    return out.stdout

def runrusttest(c,p,intxt):
    print("* RUN test: ")
    cmd = "./target/debug/"+ c + "_" + p + "_test"
    out = subprocess.run(cmd, shell=True, input=intxt, capture_output=True,text=True)
    print(out.stdout)
    return out.stdout


for i in range(1):
    intxt = maketest()
    a = rundebug(intxt)
    b = runrust(c,p,intxt)
    # print(a,b)
    if int(a)!=int(b):
        print("err", intxt)
        break
print("done")