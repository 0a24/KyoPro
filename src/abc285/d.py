import sys
sys.setrecursionlimit(10**6)

def dfs(c):
    res = 1
    if c in vis.keys():
        return 0
    else:
        vis[c] = True
        vis_all[c] = True
        if g[c] in g.keys():
            res = dfs(g[c])
        return res*1

# main
n = int(input())

g = {}
vis_all = {}
for i in range(n):
    u,v = input().split()
    g[u] = v
    vis_all[u] = False

for i in g.keys():
    if vis_all[i] == False:
        vis = {}
        res = dfs(i)
        if not res:
            break

if res:
    print("Yes")
else:
    print("No")
