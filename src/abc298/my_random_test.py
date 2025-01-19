import sys
import subprocess
from random import randint


### target
c_p = input("contest, problem = ")
if c_p == "": # デフォルト入力
    c = "abc298"
    p = "d"
else:
    c, p = c_p.split()


### BUILD
bld = "cargo build --bin "+ c + "_" + p
res = subprocess.run(bld, shell=True)

if res.returncode != 0:
    print("build error")
    sys.exit()


### RANDOM TEST
print("### TEST")

intxt = ""

### 制約
# Q = andint(1, 6*10**5)
Q = 30
intxt += str(Q)+"\n"

x1 = "1"

i = 0
i1 = 1
i2 = 0
while i<Q-1:
    qu = str(randint(1,3))
    
    if qu=="1":
        x = str(randint(1,9))
        print(x, end="")
        qu += " " + x
        x1 += x
        i1 += 1
    if qu=="2":
        if i1 - i2 > 2:
            i2 += 1
        else:
            continue

    if qu=="3":
        continue

    intxt += qu+"\n"
    i += 1

intxt += "3\n\n"

# print("\n\n### INPUT:\n")
# print(intxt)

print("\n\n### DEBUG:\n")

print(i2,x1)
print(x1[i2:])
n = 998244353
x = int(x1[i2:])
print("ANS: ", x % n)

print("\n###\n")



### 実行 ###
cmd = "./target/debug/"+ c + "_" + p
print("RUN: ")
out = subprocess.run(cmd, shell=True, input=intxt, capture_output=True,text=True)
print(out.stdout)
