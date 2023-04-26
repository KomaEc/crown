#!/usr/bin/env python3
# See LICENSE file for copyright and license details.

import random


def mod(a, b):
    return abs(a) % abs(b)

def div(a, b): # Python's division is floored, not truncated.
    r = abs(a) // abs(b)
    if a < 0:
        r = -r
    if b < 0:
        r = -r
    return r

def gcd(u, v):
    if u == 0:
        return v
    if v == 0:
        return u
    uneg = u < 0
    vneg = v < 0
    u = abs(u)
    v = abs(v)
    
    shift = 0
    while ((u | v) & 1) == 0:
        u >>= 1
        v >>= 1
        shift += 1
    
    while (u & 1) == 0:
        u >>= 1
    
    while True:
        while (v & 1) == 0:
            v >>= 1
        if u > v:
            (u, v) = (v, u)
        v -= u
        if v == 0:
            break
    
    u <<= shift
    if uneg and vneg:
        u = -u
    return u


def zabs():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    print('zsets(a, "%i");' % a)
    print('zabs(b, a);')
    print('zabs(a, a);')
    print('assert(zcmp(a, b), == 0);')
    print('assert_s(zstr(a, buf), "%i");' % abs(a))

def zadd():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = random.randint(-(1 << bits), 1 << bits)
    c = a + b
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zadd(c, a, b);')
    print('zset(d, b);')
    print('zadd(d, a, d);')
    print('zadd(a, a, b);')
    print('assert_s(zstr(c, buf), "%i");' % c)
    print('assert_s(zstr(d, buf), "%i");' % c)
    print('assert_s(zstr(a, buf), "%i");' % c)

def zadd_unsigned():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = random.randint(-(1 << bits), 1 << bits)
    c = abs(a) + abs(b)
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zadd_unsigned(c, a, b);')
    print('zset(d, b);')
    print('zadd_unsigned(d, a, d);')
    print('zadd_unsigned(a, a, b);')
    print('assert_s(zstr(c, buf), "%i");' % c)
    print('assert_s(zstr(d, buf), "%i");' % c)
    print('assert_s(zstr(a, buf), "%i");' % c)
    c = abs(b) * 2
    print('zadd_unsigned(c, b, b);')
    print('zadd_unsigned(b, b, b);')
    print('assert_s(zstr(c, buf), "%i");' % c)
    print('assert(zcmp(b, c), == 0);')

def zand():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = random.randint(-(1 << bits), 1 << bits)
    c = abs(a) & abs(b)
    if a < 0 and b < 0:
        c = -c
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zand(c, a, b);')
    print('zset(d, b);')
    print('zand(d, a, d);')
    print('zand(a, a, b);')
    print('assert_s(zstr(c, buf), "%i");' % c)
    print('assert_s(zstr(d, buf), "%i");' % c)
    print('assert_s(zstr(a, buf), "%i");' % c)
    print('zsets(a, "%i");' % a)
    print('zand(d, a, a);')
    print('zand(a, a, a);')
    print('assert_s(zstr(d, buf), "%i");' % a)
    print('assert_s(zstr(a, buf), "%i");' % a)

def zbits():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    print('zsets(a, "%i");' % a)
    a = abs(a)
    if a == 0:
        b = 1
    else:
        b = 0
        while a > 0:
            b += 1
            a >>= 1
    print('assert_zu(zbits(a), %i);' % b)

def zbset():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    b = random.randint(0, 2 * LIMIT)
    cs = (abs(a) |  (1 << b)) * (-1 if a < 0 else 1)
    cc = (abs(a) & ~(1 << b)) * (-1 if a < 0 else 1)
    cf = (abs(a) ^  (1 << b)) * (-1 if a < 0 else 1)
    print('zsets(a, "%i");' % a)
    print('zset(d, a);')
    print('zbset(b, a, %i, 1);' % b)
    print('assert_s(zstr(b, buf), "%i");' % cs)
    print('zbset(b, a, %i, 0);' % b)
    print('assert_s(zstr(b, buf), "%i");' % cc)
    print('zbset(b, a, %i, -1);' % b)
    print('assert_s(zstr(b, buf), "%i");' % cf)
    print('zset(a, d);')
    print('zbset(a, a, %i, 1);' % b)
    print('assert_s(zstr(a, buf), "%i");' % cs)
    print('zset(a, d);')
    print('zbset(a, a, %i, 0);' % b)
    print('assert_s(zstr(a, buf), "%i");' % cc)
    print('zset(a, d);')
    print('zbset(a, a, %i, -1);' % b)
    print('assert_s(zstr(a, buf), "%i");' % cf)

def zbtest():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    b = random.randint(0, 2 * LIMIT)
    c = (abs(a) >> b) & 1
    print('zsets(a, "%i");' % a)
    print('assert(zbtest(a, %i), == %i);' % (b, c))

def zcmp():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = random.randint(-(1 << bits), 1 << bits)
    c = -1 if a < b else (1 if a > b else 0)
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('assert(zcmp(a, b), == %i);' % c)

def zcmpmag():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = random.randint(-(1 << bits), 1 << bits)
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    a = abs(a)
    b = abs(b)
    c = -1 if a < b else (1 if a > b else 0)
    print('assert(zcmpmag(a, b), == %i);' % c)

def zlsb():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    print('zsets(a, "%i");' % a)
    a = abs(a)
    if a == 0:
        b = "SIZE_MAX"
    else:
        b = 0
        while (a & 1) == 0:
            b += 1
            a >>= 1
        b = str(b)
    print('assert_zu(zlsb(a), %s);' % b)

def zlsh():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, 2 * LIMIT)
    c = a << bits
    print('zsets(a, "%i");' % a)
    print('zlsh(b, a, %i);' % bits)
    print('zlsh(a, a, %i);' % bits)
    print('assert(zcmp(a, b), == 0);')
    print('assert_s(zstr(a, buf), "%i");' % c)

def zneg():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    print('zsets(a, "%i");' % a)
    print('zneg(b, a);')
    print('zneg(a, a);')
    print('assert(zcmp(a, b), == 0);')
    print('assert_s(zstr(a, buf), "%i");' % -a)

def zor():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = random.randint(-(1 << bits), 1 << bits)
    c = abs(a) | abs(b)
    if a < 0 or b < 0:
        c = -c
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zor(c, a, b);')
    print('zset(d, b);')
    print('zor(d, a, d);')
    print('zor(a, a, b);')
    print('assert_s(zstr(c, buf), "%i");' % c)
    print('assert_s(zstr(d, buf), "%i");' % c)
    print('assert_s(zstr(a, buf), "%i");' % c)
    print('zsets(a, "%i");' % a)
    print('zor(d, a, a);')
    print('zor(a, a, a);')
    print('assert_s(zstr(d, buf), "%i");' % a)
    print('assert_s(zstr(a, buf), "%i");' % a)

def zrsh():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    c = (abs(a) >> bits) * (-1 if a < 0 else 1)
    print('zsets(a, "%i");' % a)
    print('zrsh(b, a, %i);' % bits)
    print('zrsh(a, a, %i);' % bits)
    print('assert(zcmp(a, b), == 0);')
    print('assert_s(zstr(a, buf), "%i");' % c)

def zsplit():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, 2 * LIMIT)
    sign = -1 if a < 0 else 1
    c = (abs(a) >> bits) * sign
    d = (abs(a) - (abs(c) << bits)) * sign
    print('zsets(a, "%i");' % a)
    print('zset(b, a);')
    print('zsplit(b, d, b, %i);' % bits)
    print('assert_s(zstr(b, buf), "%i");' % c)
    print('assert_s(zstr(d, buf), "%i");' % d)
    print('zsplit(c, d, a, %i);' % bits)
    print('assert(zcmp(b, c), == 0);')
    print('assert_s(zstr(d, buf), "%i");' % d)
    print('zsplit(c, a, a, %i);' % bits)
    print('assert(zcmp(a, d), == 0);')
    print('assert(zcmp(b, c), == 0);')

def zstr():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    print('zsets(a, "%i");' % a)
    print('assert_s(zstr(a, buf), "%i");' % a)

def zstr_length():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    print('zsets(a, "%i");' % a)
    print('assert_zu(zstr_length(a, 10), %i);' % len(str(a)))

def zsub():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = random.randint(-(1 << bits), 1 << bits)
    c = a - b
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zsub(c, a, b);')
    print('zset(d, b);')
    print('zsub(d, a, d);')
    print('zsub(a, a, b);')
    print('assert_s(zstr(c, buf), "%i");' % c)
    print('assert_s(zstr(d, buf), "%i");' % c)
    print('assert_s(zstr(a, buf), "%i");' % c)

def zsub_unsigned():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = random.randint(-(1 << bits), 1 << bits)
    c = abs(a) - abs(b)
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zsub_unsigned(c, a, b);')
    print('zset(d, b);')
    print('zsub_unsigned(d, a, d);')
    print('zsub_unsigned(a, a, b);')
    print('assert_s(zstr(c, buf), "%i");' % c)
    print('assert_s(zstr(d, buf), "%i");' % c)
    print('assert_s(zstr(a, buf), "%i");' % c)
    print('zsub_unsigned(a, b, b);')
    print('assert(zzero(a), == 1);')
    print('zsub_unsigned(b, b, b);')
    print('assert(zzero(b), == 1);')

def ztrunc():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, 2 * LIMIT)
    c = (abs(a) & ((1 << bits) - 1)) * (-1 if a < 0 else 1)
    print('zsets(a, "%i");' % a)
    print('ztrunc(b, a, %i);' % bits)
    print('ztrunc(a, a, %i);' % bits)
    print('assert(zcmp(a, b), == 0);')
    print('assert_s(zstr(a, buf), "%i");' % c)

def zxor():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = random.randint(-(1 << bits), 1 << bits)
    c = abs(a) ^ abs(b)
    if (a < 0) != (b < 0):
        c = -c
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zxor(c, a, b);')
    print('zset(d, b);')
    print('zxor(d, a, d);')
    print('zxor(a, a, b);')
    print('assert_s(zstr(c, buf), "%i");' % c)
    print('assert_s(zstr(d, buf), "%i");' % c)
    print('assert_s(zstr(a, buf), "%i");' % c)
    print('zsets(a, "%i");' % a)
    print('zxor(d, a, a);')
    print('zxor(a, a, a);')
    print('assert(zzero(d), == 1);')
    print('assert(zzero(a), == 1);')

def zeven():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    b = 1 if (abs(a) & 1) == 0 else 0
    print('zsets(a, "%i");' % a)
    print('assert(zeven(a), == %i);' % b)

def zodd():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    b = 1 if (abs(a) & 1) != 0 else 0
    print('zsets(a, "%i");' % a)
    print('assert(zodd(a), == %i);' % b)

def zeven_nonzero():
    bits = random.randint(0, LIMIT)
    a = 0
    while a == 0:
        a = random.randint(-(1 << bits), 1 << bits)
    b = 1 if (abs(a) & 1) == 0 else 0
    print('zsets(a, "%i");' % a)
    print('assert(zeven_nonzero(a), == %i);' % b)

def zodd_nonzero():
    bits = random.randint(0, LIMIT)
    a = 0
    while a == 0:
        a = random.randint(-(1 << bits), 1 << bits)
    b = 1 if (abs(a) & 1) != 0 else 0
    print('zsets(a, "%i");' % a)
    print('assert(zodd_nonzero(a), == %i);' % b)

def zzero():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    b = 1 if a == 0 else 0
    print('zsets(a, "%i");' % a)
    print('assert(zzero(a), == %i);' % b)

def zsignum():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    b = -1 if a < 0 else (1 if a > 0 else 0)
    print('zsets(a, "%i");' % a)
    print('assert(zsignum(a), == %i);' % b)

def zdiv():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = 0
    while b == 0:
        b = random.randint(-(1 << bits), 1 << bits)
    c = div(a, b)
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zsets(d, "%i");' % c)
    print('zdiv(c, a, b);')
    print('zdiv(a, a, b);')
    print('assert(zcmp(c, d), == 0);')
    print('assert(zcmp(a, d), == 0);')
    print('zsets(a, "%i");' % a)
    print('zdiv(b, a, b);')
    print('assert(zcmp(b, d), == 0);')

def zmod():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = 0
    while b == 0:
        b = random.randint(-(1 << bits), 1 << bits)
    c = mod(a, b)
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zsets(d, "%i");' % c)
    print('zmod(c, a, b);')
    print('zmod(a, a, b);')
    print('assert(zcmp(c, d), == 0);')
    print('assert(zcmp(a, d), == 0);')
    print('zsets(a, "%i");' % a)
    print('zmod(b, a, b);')
    print('assert(zcmp(b, d), == 0);')

def zdivmod():
    bits = random.randint(0, LIMIT)
    ap = random.randint(0, 1 << bits)
    bits = random.randint(0, LIMIT)
    bp = 0
    while bp == 0:
        bp = random.randint(0, 1 << bits)
    for (a_sign, b_sign) in ((1, 1), (1, -1), (-1, 1), (-1, -1)):
        a = ap * a_sign
        b = bp * b_sign
        (c, d) = (div(a, b), mod(a, b))
        print('zsets(a, "%i");' % a)
        print('zsets(b, "%i");' % b)
        print('zdivmod(c, d, a, b);')
        print('assert_s(zstr(c, buf), "%i");' % c)
        print('assert_s(zstr(d, buf), "%i");' % d)
        print('zdivmod(a, b, a, b);')
        print('assert(zcmp(a, c), == 0);')
        print('assert(zcmp(b, d), == 0);')
        print('zsets(a, "%i");' % a)
        print('zsets(b, "%i");' % b)
        print('zdivmod(b, a, a, b);')
        print('assert(zcmp(b, c), == 0);')
        print('assert(zcmp(a, d), == 0);')
        print('zsets(b, "%i");' % b)
        print('zdivmod(b, a, b, b);')
        print('assert(zcmpu(b, 1), == 0);')
        print('assert(zcmpu(a, 0), == 0);')
        print('zsets(b, "%i");' % b)
        print('zdivmod(a, b, b, b);')
        print('assert(zcmpu(a, 1), == 0);')
        print('assert(zcmpu(b, 0), == 0);')
        print('zsets(a, "%i");' % a)
        print('zsets(b, "%i");' % b)
        print('zdivmod(a, d, a, b);')
        print('assert_s(zstr(a, buf), "%i");' % c)
        print('assert_s(zstr(d, buf), "%i");' % d)
        print('zsets(a, "%i");' % a)
        print('zdivmod(c, b, a, b);')
        print('assert_s(zstr(c, buf), "%i");' % c)
        print('assert_s(zstr(b, buf), "%i");' % d)
        a = bp * a_sign
        b = bp * b_sign
        (c, d) = (div(a, b), mod(a, b))
        print('zsets(a, "%i");' % a)
        print('zsets(b, "%i");' % b)
        print('zdivmod(c, d, a, b);')
        print('assert_s(zstr(c, buf), "%i");' % c)
        print('assert_s(zstr(d, buf), "%i");' % d)
        print('zdivmod(a, b, a, b);')
        print('assert(zcmp(a, c), == 0);')
        print('assert(zcmp(b, d), == 0);')
        print('zsets(a, "%i");' % a)
        print('zsets(b, "%i");' % b)
        print('zdivmod(b, a, a, b);')
        print('assert(zcmp(b, c), == 0);')
        print('assert(zcmp(a, d), == 0);')
        print('zsets(b, "%i");' % b)
        print('zdivmod(b, a, b, b);')
        print('assert(zcmpu(b, 1), == 0);')
        print('assert(zcmpu(a, 0), == 0);')
        print('zsets(b, "%i");' % b)
        print('zdivmod(a, b, b, b);')
        print('assert(zcmpu(a, 1), == 0);')
        print('assert(zcmpu(b, 0), == 0);')
        print('zsets(a, "%i");' % a)
        print('zsets(b, "%i");' % b)
        print('zdivmod(a, d, a, b);')
        print('assert_s(zstr(a, buf), "%i");' % c)
        print('assert_s(zstr(d, buf), "%i");' % d)
        print('zsets(a, "%i");' % a)
        print('zdivmod(c, b, a, b);')
        print('assert_s(zstr(c, buf), "%i");' % c)
        print('assert_s(zstr(b, buf), "%i");' % d)

def zmul():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = random.randint(-(1 << bits), 1 << bits)
    c = a * b
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zsets(d, "%i");' % c)
    print('zmul(c, a, b);')
    print('assert(zcmp(c, d), == 0);')
    print('zmul(c, b, a);')
    print('assert(zcmp(c, d), == 0);')
    print('zmul(a, a, b);')
    print('assert(zcmp(a, d), == 0);')
    print('zsets(a, "%i");' % a)
    print('zmul(b, a, b);')
    print('assert(zcmp(b, d), == 0);')
    c = a * a
    print('zsets(d, "%i");' % c)
    print('zmul(c, a, a);')
    print('assert(zcmp(c, d), == 0);')
    print('zmul(a, a, a);')
    print('assert(zcmp(a, d), == 0);')

def zsqr():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    c = a * a
    print('zsets(a, "%i");' % a)
    print('zsets(d, "%i");' % c)
    print('zsqr(c, a);')
    print('assert(zcmp(c, d), == 0);')
    print('zsqr(a, a);')
    print('assert(zcmp(a, d), == 0);')

def zmodmul():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    c = 0
    while c == 0:
        c = random.randint(-(1 << bits), 1 << bits)
    d = mod(a * b, c)
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zsets(c, "%i");' % c)
    print('zmodmul(d, a, b, c);')
    print('assert_s(zstr(d, buf), "%i");' % d)
    print('zmodmul(a, a, b, c);')
    print('assert_s(zstr(a, buf), "%i");' % d)
    print('zsets(a, "%i");' % a)
    print('zmodmul(b, a, b, c);')
    print('assert_s(zstr(b, buf), "%i");' % d)
    print('zsets(b, "%i");' % b)
    print('zmodmul(c, a, b, c);')
    print('assert_s(zstr(c, buf), "%i");' % d)
    print('zsets(c, "%i");' % c)
    print('zmodmul(d, b, a, c);')
    print('assert_s(zstr(d, buf), "%i");' % d)
    print('zmodmul(a, b, a, c);')
    print('assert_s(zstr(a, buf), "%i");' % d)
    print('zsets(a, "%i");' % a)
    print('zmodmul(b, b, a, c);')
    print('assert_s(zstr(b, buf), "%i");' % d)
    print('zsets(b, "%i");' % b)
    print('zmodmul(c, b, a, c);')
    print('assert_s(zstr(c, buf), "%i");' % d)
    print('zsets(c, "%i");' % c)
    d = mod(a * a, c)
    print('zmodmul(d, a, a, c);')
    print('assert_s(zstr(d, buf), "%i");' % d)
    print('zmodmul(a, a, a, c);')
    print('assert_s(zstr(a, buf), "%i");' % d)
    print('zsets(a, "%i");' % a)
    print('zmodmul(c, a, a, c);')
    print('assert_s(zstr(c, buf), "%i");' % d)
    if a != 0:
        d = mod(a * b, a)
        print('zsets(d, "%i");' % d)
        print('zmodmul(c, a, b, a);')
        print('assert_s(zstr(c, buf), "%i");' % d)
        print('zmodmul(a, a, b, a);')
        print('assert_s(zstr(a, buf), "%i");' % d)
        print('zsets(a, "%i");' % a)
        print('zmodmul(b, a, b, a);')
        print('assert_s(zstr(b, buf), "%i");' % d)
        print('zsets(b, "%i");' % b)
        print('zmodmul(c, b, a, a);')
        print('assert_s(zstr(c, buf), "%i");' % d)
        print('zmodmul(a, b, a, a);')
        print('assert_s(zstr(a, buf), "%i");' % d)
        print('zsets(a, "%i");' % a)
        print('zmodmul(b, b, a, a);')
        print('assert_s(zstr(b, buf), "%i");' % d)
        print('zmodmul(b, a, a, a);')
        print('assert(zzero(b), == 1);')
        print('zmodmul(a, a, a, a);')
        print('assert(zzero(a), == 1);')

def zmodsqr():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = 0
    while b == 0:
        b = random.randint(-(1 << bits), 1 << bits)
    c = mod(a ** 2, b)
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zsets(d, "%i");' % c)
    print('zmodsqr(c, a, b);')
    print('assert(zcmp(c, d), == 0);')
    print('zset(c, a);')
    print('zmodsqr(a, a, b);')
    print('assert(zcmp(a, d), == 0);')
    print('zset(a, c);')
    print('zset(c, b);')
    print('zmodsqr(b, a, b);')
    print('assert(zcmp(b, d), == 0);')
    if a != 0:
        c = mod(a ** 2, a)
        print('zmodsqr(b, a, a);')
        print('assert(zzero(b), == 1);')
        print('zmodsqr(a, a, a);')
        print('assert(zzero(a), == 1);')

def zcmpi():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    b = random.randint(-(1 << 63), (1 << 63) - 1)
    c = -1 if a < b else (1 if a > b else 0)
    print('zsets(a, "%i");' % a)
    if b >= 0:
        print('assert(zcmpi(a, %iLL), == %i);' % (b, c))
    else:
        print('assert(zcmpi(a, %iLL - 1LL), == %i);' % (b + 1, c))

def zcmpu():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    b = random.randint(0, (1 << 64) - 1)
    c = -1 if a < b else (1 if a > b else 0)
    print('zsets(a, "%i");' % a)
    print('assert(zcmpu(a, %iULL), == %i);' % (b, c))

def zgcd():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    bits = random.randint(0, LIMIT)
    b = random.randint(-(1 << bits), 1 << bits)
    c = gcd(a, b)
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zsets(d, "%i");' % c)
    print('zgcd(c, a, b);')
    print('assert(zcmp(c, d), == 0);')

def zpow():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    b = random.randint(1, 16)
    c = a ** b
    print('zsets(a, "%i");' % a)
    print('zsetu(b, %i);' % b)
    print('zsets(d, "%i");' % c)
    print('zpow(c, a, b);')
    print('zpow(a, a, b);')
    print('assert(zcmp(c, d), == 0);')
    print('assert(zcmp(a, d), == 0);')
    print('zsets(a, "%i");' % a)
    print('zpow(b, a, b);')
    print('assert(zcmp(b, d), == 0);')

def zpowu():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    b = random.randint(1, 16)
    c = a ** b
    print('zsets(a, "%i");' % a)
    print('zsets(d, "%i");' % c)
    print('zpowu(c, a, %i);' % b)
    print('zpowu(a, a, %i);' % b)
    print('assert(zcmp(c, d), == 0);')
    print('assert(zcmp(a, d), == 0);')

def zmodpowu():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    b = random.randint(1, 16)
    bits = random.randint(0, LIMIT)
    c = 0
    while c == 0:
        c = random.randint(-(1 << bits), 1 << bits)
    d = mod(a ** b, c)
    print('zsets(a, "%i");' % a)
    print('zsets(c, "%i");' % c)
    print('zsets(d, "%i");' % d)
    print('zmodpowu(b, a, %i, c);' % b)
    print('zmodpowu(a, a, %i, c);' % b)
    print('assert(zcmp(b, d), == 0);')
    print('assert(zcmp(a, d), == 0);')
    print('zsets(a, "%i");' % a)
    print('zmodpowu(c, a, %i, c);' % b)
    print('assert(zcmp(c, d), == 0);')

def zmodpow():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    b = random.randint(1, 16)
    bits = random.randint(0, LIMIT)
    c = 0
    while c == 0:
        c = random.randint(-(1 << bits), 1 << bits)
    d = mod(a ** b, c)
    print('zsets(a, "%i");' % a)
    print('zsets(b, "%i");' % b)
    print('zsets(c, "%i");' % c)
    print('zsets(d, "%i");' % d)
    print('zmodpow(d, a, b, c);')
    print('zmodpow(a, a, b, c);')
    print('assert_s(zstr(d, buf), "%i");' % d)
    print('assert(zcmp(a, d), == 0);')
    print('zsets(a, "%i");' % a)
    print('zmodpow(b, a, b, c);')
    print('assert(zcmp(b, d), == 0);')
    print('zsets(b, "%i");' % b)
    print('zmodpow(c, a, b, c);')
    print('assert(zcmp(c, d), == 0);')

def znot():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    sign = -(-1 if a < 0 else 1)
    b = abs(a)
    bits = 0
    x = b
    while x > 0:
        bits += 1
        x >>= 1
    b = ~b
    b &= (1 << bits) - 1
    b *= sign
    print('zsets(a, "%i");' % a)
    print('zsets(c, "%i");' % b)
    print('znot(b, a);')
    print('znot(a, a);')
    print('assert(zcmp(b, c), == 0);')
    print('assert(zcmp(a, c), == 0);')

def zsave_zload():
    bits = random.randint(0, LIMIT)
    a = random.randint(-(1 << bits), 1 << bits)
    print('zsets(a, "%i");' % a)
    print('n = zsave(a, 0);')
    print('assert_zu(zsave(a, buf), n);')
    print('assert_zu(zload(b, buf), n);')
    print('assert(zcmp(a, b), == 0);')



functions = [zzero, zsignum, zeven_nonzero, zodd_nonzero, zeven, zcmp, zcmpi, zcmpu, zcmpmag,
             zodd, zabs, zneg, zlsh, zrsh, ztrunc, zsplit, zand, zor, zxor, zbits, zlsb, znot,
             zbtest, zbset, zadd_unsigned, zsub_unsigned, zadd, zsub, zmul, zsqr, zdivmod,
             zdiv, zmod, zmodmul, zmodsqr, zsave_zload, zgcd, zpow, zpowu, zmodpow, zmodpowu,
             zstr_length, zstr] # untested: zptest, zrand

limits = [200]

for LIMIT in limits:
    for function in functions:
        print('/* %s */' % function.__qualname__)
        for i in range(100):
            function()
            print()
        print()

