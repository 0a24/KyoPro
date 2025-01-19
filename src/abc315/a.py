

S = input()

aiueo = ["a", "i", "u", "e", "o"]

ans = ""

for a in S:
    if a in aiueo:
        pass
    else:
        ans += a

print(ans)