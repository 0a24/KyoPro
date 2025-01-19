#! /usr/bin/python3

import sys
import subprocess
from random import randint


# target
c_p = input("contest, problem = ")
if c_p == "": # デフォルト入力
    c = "abc287"
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

    nt = randint(1,10)
    ns = randint(nt+1,11)
    cs = ['a', 'b', 'c', '?']
    intxt: str = "" 
    # intxt += "atcoder\n"
    # intxt += "?????\n"
    intxt += ''.join(cs[randint(0,3)] for _ in range(ns)) + '\n'
    intxt += ''.join(cs[randint(0,3)] for _ in range(nt)) + '\n'

    print("* INPUT:")
    print(intxt)
    return intxt


def runtest(intxt):
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
for i in range(100):
    intxt = maketest()
    a = runrusttest(c,p,intxt)
    b = runrust(c,p,intxt)
    # print(a,b)

    res &= a==b
    if not res:
        print("err", intxt)
        break

print("* done:", res)