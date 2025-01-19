import sys
sys.setrecursionlimit(10**6)

def dfs(c):
    vis[c] = 1
    for i in g[c]:
        if vis[i]==0:
            dfs(i)
    return


# main
n,m = map(int, input().split())

g = [[] for _ in range(n)]
for _ in range(m):
    u,v = map(int, input().split())
    g[u-1].append(v-1)
    g[v-1].append(u-1)

vis = [0] * n
cnt = 0
for i in range(n):
    if vis[i] == 0:
        dfs(i)
        cnt += 1

print(m - (n-cnt))