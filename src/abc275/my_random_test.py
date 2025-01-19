#! /usr/bin/python3

import sys
import subprocess
from random import randint


# target
c_p = input("contest, problem = ")
if c_p == "": # デフォルト入力
    c = "abc275"
    p = "c"
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
    # intxt += "atcoder\n"
    # intxt += "?????\n"
    intxt += ''.join(''.join('#' for _ in range(9)) + '\n' for _ in range(9))


    print("* INPUT:")
    print(intxt)
    return intxt


def runpythontest(intxt):
    print("\n\n* DEBUG:")

    
    res = 0
    return res

def runrust(c,p,intxt):
    print("* RUN: ")
    cmd = "./target/debug/"+ c + "_" + p
    out = subprocess.run(cmd, shell=True, input=intxt, capture_output=True,text=True)
    # print(out.stdout)
    return out.stdout

def runrusttest(c,p,intxt):
    print("* RUN test: ")
    cmd = "./target/debug/"+ c + "_" + p + "_test"
    out = subprocess.run(cmd, shell=True, input=intxt, capture_output=True,text=True)
    # print(out.stdout)
    return out.stdout


res = True
for i in range(1):
    intxt = maketest()
    # a = runpythontest(c,p,intxt)
    # a = runrusttest(c,p,intxt)
    b = runrust(c,p,intxt)
    print(b)

    # res &= int(a)==int(b)
    if not res:
        print("err", intxt)
        break

print("* done:", res)