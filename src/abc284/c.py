
def dfs(c):
    vis[c-1] = True
    for i in range(1,n+1):
        if (not vis[i-1]) & g[c-1][i-1]:
            dfs(i)
    return 0

# main
n, m = list(map(int, input().split()))
g0 = [list(map(int, input().split())) for _ in range(m)]

g = list(list(False for _ in range(n)) for _ in range(n))
for (u,v) in g0:
    g[u-1][v-1] = True
    g[v-1][u-1] = True

vis = list(False for _ in range(n))

ans = 0
for i in range(1,n+1):
    if not vis[i-1]:
        ans += 1
        dfs(i)
print(ans)