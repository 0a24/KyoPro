

# S = "0010011010111001"

###
# def oracle(qu):
#     return int(S[qu-1])
    

###
n = int(input())
# n = len(S)

n_loop = 20

i_zero = 1
i_one = n

for _ in range(n_loop):
    qu = (i_zero + i_one) // 2
    print("?", qu)
    s = int(input())
    # s = oracle(qu)

    if s==0:
        i_zero = qu
    elif s==1:
        i_one = qu
    

    if i_one-i_zero == 1:
        break

print("!", i_zero)

# print(S[i_zero-1], S[i_zero])