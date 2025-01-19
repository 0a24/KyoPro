import sys
import subprocess
from random import randint


### target ##### ##### ##### ##### ##### ##### 
c_p = input("contest, problem = ")
if c_p == "": # デフォルト入力
    c = "abc299"
    p = "d"
else:
    c, p = c_p.split()


### BUILD ##### ##### ##### ##### ##### ##### 
bld = "cargo build --bin "+ c + "_" + p
res = subprocess.run(bld, shell=True)

if res.returncode != 0:
    print("build error")
    sys.exit()


### RANDOM TEST ##### ##### ##### ##### ##### ##### 
print("### TEST ##### ##### ##### ##### ##### ##### \n")

### 制約 ##### ##### ##### ##### ##### ##### 
N = randint(2, 2*10**1)
S = randint(2, 2**N)
o_txt = "0" + bin(S)[2:] + "1"
print(o_txt)

intxt = ""
intxt += str(len(o_txt))+"\n"


print("\n\n### INPUT ##### ##### ##### ##### ##### ##### \n")
print(intxt)

print("\n\n### DEBUG ##### ##### ##### ##### ##### ##### \n")

print("\n ##### ##### ##### ##### ##### ##### \n")



### 実行 ### ##### ##### ##### ##### ##### ##### 
# cmd = "./target/debug/"+ c + "_" + p
# print("RUN: ")
# out = subprocess.run(cmd, shell=True, input=intxt, capture_output=True,text=True)
# print(out.stdout)
