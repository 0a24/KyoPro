

# main
n,k = map(int, input().split())

s = []
for _ in range(n):
    s.append(input())

s1 = sorted(s[:k])

for i in range(k):
    print(s1[i])