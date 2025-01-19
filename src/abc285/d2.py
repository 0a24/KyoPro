import sys
sys.setrecursionlimit(10**6)

def dfs(c):
    res = 1
    if vis[c] == 1:
        return 0
    else:
        vis[c] = 1
        vis_all[c] = 1
        if names[c] != -1:
            res = dfs(names[c])
        return res*1

# main
n = int(input())

g = {}
name_ima = [0] * n
name_ato = [0] * n
for i in range(n):
    u,v = input().split()
    g[u] = v
    name_ima[i] = u
    name_ato[i] = v

names = [0] * n
for i in range(n):
    if name_ato[i] in name_ima:
        names[i] = name_ima.index(name_ato[i])
    else:
        names[i] = -1
    
vis_all = [0] * n
for i in range(n):
    if vis_all[i] == 0:
        vis = [0] * n
        res = dfs(i)
        if not res:
            break

if res:
    print("Yes")
else:
    print("No")
