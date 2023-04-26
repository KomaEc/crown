/* See LICENSE file for copyright and license details. */
#include "zahl.h"

#include <stdio.h>
#include <string.h>

#define assert(expr, expected)\
	do {\
		int got = (expr);\
		if (!(got expected)) {\
			fprintf(stderr,\
				"Failure at line %i: %s, expected %s, but got %i.\n",\
				__LINE__, #expr, #expected, got);\
			ret = 1;\
			goto done;\
		}\
	} while (0)

#define assert_zu(expr, expected)\
	do {\
		size_t got = (expr);\
		if (got != (expected)) {\
			fprintf(stderr,\
				"Failure at line %i: %s, expected %zu, but got %zu.\n",\
				__LINE__, #expr, (size_t)(expected), got);\
			ret = 1;\
			goto done;\
		}\
	} while (0)

#define assert_s(expr, expected)\
	do {\
		const char *got = (expr);\
		if (strcmp(got, expected)) {\
			fprintf(stderr,\
				"Failure at line %i: %s, expected %s, but got %s.\n",\
				__LINE__, #expr, expected, got);\
			ret = 1;\
			goto done;\
		}\
	} while (0)

#define assert_nr(expr)\
	do {\
		if (setjmp(env2)) {\
			ret = 0;\
			zsetup(env);\
		} else {\
			zsetup(env2);\
			expr;\
			fprintf(stderr,\
				"Failure at line %i: %s, should not have returned.\n",\
				__LINE__, #expr);\
			ret = 1;\
			goto done;\
		}\
	} while (0)

int
main(void)
{
	/* static because otherwise it would have to be volatile yeilding a lot of stupid
	 * warnings. auto variables are not guaranteed to be readable after a long jump. */
	static z_t a, b, c, d, _0, _1, _2, _3;
	static char buf[2000];
	static int ret = 0;
	static jmp_buf env, env2;
	static size_t n;

	if (setjmp(env)) {
		zperror(0);
		ret = 2;
		goto done;
	}

	zsetup(env);
	zinit(a), zinit(b), zinit(c), zinit(d), zinit(_0), zinit(_1), zinit(_2), zinit(_3);

	zsetu(_0, 0);
	zsetu(_1, 1);
	zsetu(_2, 2);
	zsetu(_3, 3);

	assert(zeven(_0), == 1);
	assert(zodd(_0), == 0);
	assert(zzero(_0), == 1);
	assert(zsignum(_0), == 0);
	assert(zeven(_1), == 0);
	assert(zodd(_1), == 1);
	assert(zzero(_1), == 0);
	assert(zsignum(_1), == 1);
	assert(zeven(_2), == 1);
	assert(zodd(_2), == 0);
	assert(zzero(_2), == 0);
	assert(zsignum(_2), == 1);

	zswap(_1, _2);
	assert(zeven(_2), == 0);
	assert(zodd(_2), == 1);
	assert(zzero(_2), == 0);
	assert(zsignum(_2), == 1);
	assert(zeven(_1), == 1);
	assert(zodd(_1), == 0);
	assert(zzero(_1), == 0);
	assert(zsignum(_1), == 1);
	zswap(_2, _1);
	assert(zeven(_1), == 0);
	assert(zodd(_1), == 1);
	assert(zzero(_1), == 0);
	assert(zsignum(_1), == 1);
	assert(zeven(_2), == 1);
	assert(zodd(_2), == 0);
	assert(zzero(_2), == 0);
	assert(zsignum(_2), == 1);
	
	assert((zneg(_2, _2), zsignum(_2)), == -1); zneg(_2, _2);
	assert(zsignum(_2), == 1);

	assert(zcmp(_0, _0), == 0);
	assert(zcmp(_1, _1), == 0);
	assert(zcmp(_0, _1), < 0);
	assert(zcmp(_1, _0), > 0);
	assert(zcmp(_1, _2), < 0);
	assert(zcmp(_2, _1), > 0);
	assert(zcmp(_0, _2), < 0);
	assert(zcmp(_2, _0), > 0);

	zbset(a, _0, 0, 1);
	assert(zcmp(a, _1), == 0);
	zbset(a, a, 1, 1);
	assert(zcmp(a, _3), == 0);
	zbset(a, a, 0, 0);
	assert(zcmp(a, _2), == 0);
	zbset(a, a, 0, 0);
	assert(zcmp(a, _2), == 0);
	zbset(a, a, 0, -1);
	assert(zcmp(a, _3), == 0);
	zbset(a, a, 0, -1);
	assert(zcmp(a, _2), == 0);

	zadd(a, _0, _1);
	assert(zsignum(a), == 1);
	assert(zcmp(a, _1), == 0);
	assert(zcmpi(a, 1), == 0);
	assert(zcmpu(a, 1), == 0);
	zneg(a, a);
	assert(zsignum(a), == -1);
	assert(zcmp(a, _1), < 0);
	assert(zcmpi(a, 1), < 0);
	assert(zcmpu(a, 1), < 0);
	zadd(a, _2, _0);
	assert(zsignum(a), == 1);
	assert(zcmp(a, _2), == 0);
	assert(zcmpi(a, 2), == 0);
	assert(zcmpu(a, 2), == 0);
	zneg(a, a);
	assert(zsignum(a), == -1);
	assert(zcmp(a, _2), < 0);
	assert(zcmpi(a, 2), < 0);
	assert(zcmpu(a, 2), < 0);
	assert(zsignum(_1), == 1);
	zadd(a, _1, _1);
	assert(zsignum(a), == 1);
	assert(zcmp(a, _2), == 0);
	assert(zcmpi(a, 2), == 0);
	assert(zcmpu(a, 2), == 0);
	zset(b, _1);
	zadd(a, b, _1);
	assert(zsignum(a), == 1);
	assert(zcmp(a, _2), == 0);
	assert(zcmpi(a, 2), == 0);
	assert(zcmpu(a, 2), == 0);
	zneg(a, a);
	zset(b, _2);
	zneg(b, b);
	assert(zsignum(a), == -1);
	assert(zcmp(a, b), == 0);
	assert(zcmp(a, _2), < 0);
	assert(zcmpmag(a, b), == 0);
	assert(zcmpmag(a, _2), == 0);
	assert(zcmpi(a, 2), < 0);
	assert(zcmpu(a, 2), < 0);
	assert(zcmpi(a, -2), == 0);
	assert((zneg(_2, _2), zcmp(a, _2)), == 0); zneg(_2, _2);
	zadd(a, _1, _2);
	assert(zsignum(a), == 1);
	assert(zcmp(a, _2), > 0);
	assert(zcmpi(a, 2), > 0);
	assert(zcmpu(a, 2), > 0);
	zneg(a, a);
	zset(b, _2);
	zneg(b, b);
	assert(zsignum(a), == -1);
	assert(zcmpmag(a, _2), > 0);
	assert(zcmpmag(a, b), > 0);
	assert(zcmp(a, b), < 0);
	assert(zcmp(a, _2), < 0);
	assert(zcmpi(a, 2), < 0);
	assert(zcmpu(a, 2), < 0);
	assert(zcmpi(a, -2), < 0);
	assert((zneg(_2, _2), zcmp(a, _2)), < 0); zneg(_2, _2);
	zneg(b, _3);
	assert(zcmp(a, b), == 0);

	zunsetup();
	zsetup(env);

	zsub(a, _2, _1);
	assert(zcmpmag(_2, _1), > 0);
	assert(zcmpmag(_2, _0), > 0);
	assert(zcmpmag(_1, _0), > 0);
	zsub(b, _1, _2);
	assert(zcmpmag(_2, _0), > 0);
	assert(zcmpmag(_1, _0), > 0);
	assert(zcmpmag(_2, _1), > 0);
	assert(zcmpmag(a, b), == 0);
	assert(zcmpmag(a, _1), == 0);
	assert(zcmp(a, b), > 0);
	assert(zcmp(a, _1), == 0);
	assert(zcmp(b, _1), < 0);
	zsub(a, _1, _1);
	assert(zcmp(a, _0), == 0);
	zseti(b, 0);
	zsetu(c, 0);
	zsub(a, b, c);
	assert(zcmp(a, _0), == 0);
	assert(zcmpmag(_2, _1), > 0);
	assert(zcmp(_2, _1), > 0);
	zsub(a, _2, _1);
	assert(zsignum(a), == 1);
	assert(zcmpmag(a, _1), == 0);
	assert(zcmp(a, _1), == 0);
	zsub(a, a, _1);
	assert(zcmp(a, _0), == 0);
	zsub(a, a, _0);
	assert(zcmp(a, _0), == 0);
	zsub(a, _1, _2);
	assert(zcmp(a, _1), < 0);
	assert(zcmpmag(a, _1), == 0);
	zabs(a, a);
	assert(zcmp(a, _1), == 0);
	zabs(a, a);
	assert(zcmp(a, _1), == 0);
	zabs(a, _1);
	assert(zcmp(a, _1), == 0);
	zabs(a, _0);
	assert(zcmp(a, _0), == 0);

	zseti(b, -1);
	zseti(c, -2);
	zadd(a, _0, b);
	assert(zcmp(a, _0), < 0);
	assert(zcmpi(a, -1), == 0);
	assert(zcmpmag(a, _1), == 0);
	assert(zcmp(a, _1), < 0);
	zadd(a, b, _0);
	assert(zcmp(a, _0), < 0);
	assert(zcmpi(a, -1), == 0);
	assert(zcmpmag(a, _1), == 0);
	assert(zcmp(a, _1), < 0);
	zadd(a, b, c);
	assert(zcmp(a, c), < 0);
	assert(zcmpmag(a, _2), > 0);
	zadd(a, c, b);
	assert(zcmp(a, c), < 0);
	assert(zcmpmag(a, _2), > 0);
	zadd(a, b, _1);
	assert(zcmp(a, _0), == 0);
	assert(zcmpmag(a, _0), == 0);
	zadd(a, _1, b);
	assert(zcmp(a, _0), == 0);
	assert(zcmpmag(a, _0), == 0);

	zneg(b, _1);
	zneg(c, _2);
	zsub(a, _0, b);
	assert(zcmp(a, _1), == 0);
	zsub(a, b, _0);
	assert(zcmpmag(a, _1), == 0);
	assert(zcmp(a, _1), < 0);
	zsub(a, b, c);
	assert(zcmpmag(a, _1), == 0);
	assert(zcmp(a, _1), == 0);
	zsub(a, c, b);
	assert(zcmpmag(a, _1), == 0);
	assert(zcmp(a, _1), < 0);
	zsub(a, b, _1);
	assert(zcmpmag(a, _2), == 0);
	assert(zcmp(a, _2), < 0);
	assert(zcmp(a, c), == 0);
	zsub(a, _1, b);
	assert(zcmp(b, _1), < 0);
	assert(zcmpmag(b, _1), == 0);
	assert(zcmp(a, _2), == 0);

	zsetu(a, 1000);
	zsetu(b, 0);
	assert(zcmp(a, b), != 0);
	n = zsave(a, buf);
	assert(n, > 0);
	assert_zu(zload(b, buf), n);
	assert(zcmp(a, b), == 0);

	zneg(b, _1);
	zneg(c, _2);

	assert((zadd_unsigned(a, b, c), zcmp(a, _3)), == 0);
	assert((zadd_unsigned(a, b, c), zcmp(a, _3)), == 0);
	assert((zadd_unsigned(a, b, _2), zcmp(a, _3)), == 0);
	assert((zadd_unsigned(a, _1, c), zcmp(a, _3)), == 0);

	assert((zadd_unsigned(a, _0, _0), zcmp(a, _0)), == 0);
	assert((zadd_unsigned(a, _0, _1), zcmp(a, _1)), == 0);
	assert((zadd_unsigned(a, _1, _1), zcmp(a, _2)), == 0);
	assert((zadd_unsigned(a, _1, _0), zcmp(a, _1)), == 0);
	zneg(_1, _1);
	assert((zadd_unsigned(a, _0, _0), zcmp(a, _0)), == 0);
	assert((zadd_unsigned(a, _0, _1), zcmp(a, _1)), != 0);
	assert((zadd_unsigned(a, _0, _1), zcmpmag(a, _1)), == 0);
	assert((zadd_unsigned(a, _1, _1), zcmp(a, _2)), == 0);
	assert((zadd_unsigned(a, _1, _0), zcmp(a, _1)), != 0);
	assert((zadd_unsigned(a, _1, _0), zcmpmag(a, _1)), == 0);
	zneg(_1, _1);

	assert((zsub_unsigned(a, _2, _1), zcmp(a, _1)), == 0);
	assert((zsub_unsigned(a, _2, b), zcmp(a, _1)), == 0);
	assert((zsub_unsigned(a, c, _1), zcmp(a, _1)), == 0);
	assert((zsub_unsigned(a, c, b), zcmp(a, _1)), == 0);

	assert((zsub_unsigned(a, _1, _2), zcmp(a, b)), == 0);
	assert((zsub_unsigned(a, b, _2), zcmp(a, b)), == 0);
	assert((zsub_unsigned(a, _1, c), zcmp(a, b)), == 0);
	assert((zsub_unsigned(a, b, c), zcmp(a, b)), == 0);

	assert_zu(zbits(_0), 1);
	assert_zu(zbits(_1), 1);
	assert_zu(zbits(_2), 2);
	assert_zu(zbits(_3), 2);

	assert_zu(zlsb(_0), SIZE_MAX);
	assert_zu(zlsb(_1), 0);
	assert_zu(zlsb(_2), 1);
	assert_zu(zlsb(_3), 0);

	assert((zand(a, _0, _0), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zand(a, _0, _1), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zand(a, _0, _2), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zand(a, _0, _3), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zand(a, _1, _1), zcmp(a, _1)), == 0);
	assert((zand(a, _1, _2), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zand(a, _1, _3), zcmp(a, _1)), == 0);
	assert((zand(a, _2, _2), zcmp(a, _2)), == 0);
	assert((zand(a, _2, _3), zcmp(a, _2)), == 0);
	assert((zand(a, _3, _3), zcmp(a, _3)), == 0);

	assert((zor(a, _0, _0), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zor(a, _0, _1), zcmp(a, _1)), == 0);
	assert((zor(a, _0, _2), zcmp(a, _2)), == 0);
	assert((zor(a, _0, _3), zcmp(a, _3)), == 0);
	assert((zor(a, _1, _1), zcmp(a, _1)), == 0);
	assert((zor(a, _1, _2), zcmp(a, _3)), == 0);
	assert((zor(a, _1, _3), zcmp(a, _3)), == 0);
	assert((zor(a, _2, _2), zcmp(a, _2)), == 0);
	assert((zor(a, _2, _3), zcmp(a, _3)), == 0);
	assert((zor(a, _3, _3), zcmp(a, _3)), == 0);

	assert((zxor(a, _0, _0), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zxor(a, _0, _1), zcmp(a, _1)), == 0);
	assert((zxor(a, _0, _2), zcmp(a, _2)), == 0);
	assert((zxor(a, _0, _3), zcmp(a, _3)), == 0);
	assert((zxor(a, _1, _1), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zxor(a, _1, _2), zcmp(a, _3)), == 0);
	assert((zxor(a, _1, _3), zcmp(a, _2)), == 0);
	assert((zxor(a, _2, _2), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zxor(a, _2, _3), zcmp(a, _1)), == 0);
	assert((zxor(a, _3, _3), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);

	zneg(b, _1);
	zneg(c, _3);
	zneg(_1, _1);
	zand(a, b, c);
	assert(zcmpmag(a, _1), == 0);
	assert(zcmp(a, _1), == 0);
	zneg(_1, _1);
	assert((zand(a, b, _3), zcmp(a, _1)), == 0);
	assert((zand(a, _1, c), zcmp(a, _1)), == 0);
	assert((zand(a, _0, c), zcmp(a, _0)), == 0);
	assert((zand(a, b, _0), zcmp(a, _0)), == 0);

	zneg(b, _1);
	zneg(c, _2);
	zneg(_3, _3);
	zor(a, b, c);
	assert(zcmpmag(a, _3), == 0);
	assert(zcmp(a, _3), == 0);
	zor(a, b, _2);
	assert(zcmpmag(a, _3), == 0);
	assert(zcmp(a, _3), == 0);
	zor(a, _1, c);
	assert((zcmpmag(a, _3)), == 0);
	assert((zcmp(a, _3)), == 0);
	assert((zor(a, _0, c), zcmp(a, c)), == 0);
	assert((zor(a, b, _0), zcmp(a, b)), == 0);
	zneg(_3, _3);

	zneg(b, _1);
	zneg(c, _2);
	zxor(a, b, c);
	assert(zcmpmag(a, _3), == 0);
	assert(zcmp(a, _3), == 0);
	zneg(_3, _3);
	zxor(a, b, _2);
	assert(zcmpmag(a, _3), == 0);
	assert(zcmp(a, _3), == 0);
	zxor(a, _1, c);
	assert(zcmpmag(a, _3), == 0);
	assert(zcmp(a, _3), == 0);
	zxor(a, b, _0);
	assert(zcmpmag(a, b), == 0);
	assert(zcmp(a, b), == 0);
	zxor(a, _0, c);
	assert(zcmpmag(a, c), == 0);
	assert(zcmp(a, c), == 0);
	zneg(_3, _3);

	assert((zlsh(a, _0, 0), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zlsh(a, _0, 1), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zlsh(a, _1, 0), zcmp(a, _1)), == 0);
	assert((zlsh(a, _1, 1), zcmp(a, _2)), == 0);
	assert((zlsh(a, _1, 2), zcmp(a, _2)), > 0);
	assert((zlsh(a, _2, 0), zcmp(a, _2)), == 0);
	assert((zlsh(a, _2, 1), zcmp(a, _2)), > 0);

	zset(a, _0);
	assert((zlsh(a, a, 0), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zlsh(a, a, 1), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	zset(a, _1);
	assert((zlsh(a, a, 0), zcmp(a, _1)), == 0);
	assert((zlsh(a, a, 1), zcmp(a, _2)), == 0);
	assert((zlsh(a, a, 2), zcmp(a, _2)), > 0);
	zset(a, _2);
	assert((zlsh(a, a, 0), zcmp(a, _2)), == 0);
	assert((zlsh(a, a, 1), zcmp(a, _2)), > 0);

	assert((zrsh(a, _0, 0), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zrsh(a, _0, 1), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zrsh(a, _1, 0), zcmp(a, _1)), == 0);
	assert((zrsh(a, _1, 1), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zrsh(a, _1, 2), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zrsh(a, _2, 0), zcmp(a, _2)), == 0);
	assert((zrsh(a, _2, 1), zcmp(a, _1)), == 0);
	assert((zrsh(a, _2, 2), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);

	zset(a, _0);
	assert((zrsh(a, a, 0), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zrsh(a, a, 1), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	zset(a, _1);
	assert((zrsh(a, a, 0), zcmp(a, _1)), == 0);
	assert((zrsh(a, a, 1), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	assert((zrsh(a, a, 2), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);
	zset(a, _2);
	assert((zrsh(a, a, 0), zcmp(a, _2)), == 0);
	assert((zrsh(a, a, 1), zcmp(a, _1)), == 0);
	assert((zrsh(a, a, 2), zcmp(a, _0)), == 0);
	assert(zzero(a), == 1);

	assert(zbtest(_0, 0), == 0);
	assert(zbtest(_1, 0), == 1);
	assert(zbtest(_2, 0), == 0);
	assert(zbtest(_3, 0), == 1);
	assert(zbtest(_0, 1), == 0);
	assert(zbtest(_1, 1), == 0);
	assert(zbtest(_2, 1), == 1);
	assert(zbtest(_3, 1), == 1);
	assert(zbtest(_0, 2), == 0);
	assert(zbtest(_1, 2), == 0);
	assert(zbtest(_2, 2), == 0);
	assert(zbtest(_3, 2), == 0);

	znot(a, _2);
	assert(zcmpmag(a, _1), == 0);
	assert(zcmp(a, _1), != 0);
	znot(a, a);
	assert(zcmp(a, _0), == 0);

	zsetu(a, 0x1234);
	zsetu(c, 0x234);
	ztrunc(a, a, 12);
	assert(zcmp(a, c), == 0);

	zsetu(a, 0xEEFF);
	zsetu(c, 0xEE);
	zsetu(d, 0xFF);
	zsplit(a, b, a, 8);
	assert(zcmpmag(a, c), == 0);
	assert(zcmpmag(b, d), == 0);
	zsetu(a, 0xEEFF);
	zsplit(b, a, a, 8);
	assert(zcmpmag(b, c), == 0);
	assert(zcmpmag(a, d), == 0);

	zmul(a, _2, _3);
	assert(zcmpi(a, 6), == 0);
	zneg(_3, _3);
	zmul(a, _2, _3);
	assert(zcmpi(a, -6), == 0);
	zneg(_3, _3);
	zneg(_2, _2);
	zmul(a, _2, _3);
	assert(zcmpi(a, -6), == 0);
	zneg(_3, _3);
	zmul(a, _2, _3);
	assert(zcmpi(a, 6), == 0);
	zneg(_3, _3);
	zneg(_2, _2);

	zmul(a, _3, _3);
	assert(zcmpi(a, 9), == 0);
	zsqr(a, _3);
	assert(zcmpi(a, 9), == 0);
	zneg(_3, _3);
	zmul(a, _3, _3);
	assert(zcmpi(a, 9), == 0);
	zsqr(a, _3);
	assert(zcmpi(a, 9), == 0);
	zneg(_3, _3);

	zseti(a, 8);
	zseti(b, 2);
	zdiv(c, a, b);
	assert(zcmpi(c, 4), == 0);
	zseti(b, -2);
	zdiv(c, a, b);
	assert(zcmpi(c, -4), == 0);
	zseti(a, -8);
	zseti(b, 2);
	zdiv(c, a, b);
	assert(zcmpi(c, -4), == 0);
	zseti(b, -2);
	zdiv(c, a, b);
	assert(zcmpi(c, 4), == 0);

	zseti(a, 1000);
	zseti(b, 10);
	zdiv(c, a, b);
	assert(zcmpi(c, 100), == 0);
	zseti(b, -10);
	zdiv(c, a, b);
	assert(zcmpi(c, -100), == 0);
	zseti(a, -1000);
	zseti(b, 10);
	zdiv(c, a, b);
	assert(zcmpi(c, -100), == 0);
	zseti(b, -10);
	zdiv(c, a, b);
	assert(zcmpi(c, 100), == 0);

	zseti(a, 7);
	zseti(b, 3);
	zmod(c, a, b);
	assert(zcmpi(c, 1), == 0);
	zseti(b, -3);
	zmod(c, a, b);
	assert(zcmpi(c, 1), == 0);
	zseti(a, -7);
	zseti(b, 3);
	zmod(c, a, b);
	assert(zcmpi(c, 1), == 0);
	zseti(b, -3);
	zmod(c, a, b);
	assert(zcmpi(c, 1), == 0);

	zseti(a, 7);
	zseti(b, 3);
	zdivmod(d, c, a, b);
	assert(zcmpi(d, 2), == 0);
	assert(zcmpi(c, 1), == 0);
	zseti(b, -3);
	zdivmod(d, c, a, b);
	assert(zcmpi(d, -2), == 0);
	assert(zcmpi(c, 1), == 0);
	zseti(a, -7);
	zseti(b, 3);
	zdivmod(d, c, a, b);
	assert(zcmpi(d, -2), == 0);
	assert(zcmpi(c, 1), == 0);
	zseti(b, -3);
	zdivmod(d, c, a, b);
	assert(zcmpi(d, 2), == 0);
	assert(zcmpi(c, 1), == 0);

	zseti(a, 10);
	zseti(b, -1);
	zpow(a, a, b);
	assert(zcmp(a, _0), == 0);

	zseti(a, 10);
	zseti(b, -1);
	zseti(a, 20);
	zmodpow(a, a, b, c);
	assert(zcmp(a, _0), == 0);

	zseti(a, 10);
	zseti(c, 100000L);
	zpowu(a, a, 5);
	assert(zcmpmag(a, c), == 0);
	assert(zcmp(a, c), == 0);

	zseti(a, -10);
	zseti(c, -100000L);
	zpowu(a, a, 5);
	assert(zcmpmag(a, c), == 0);
	assert(zcmp(a, c), == 0);

	zseti(a, -10);
	zseti(c, 10000L);
	zpowu(a, a, 4);
	assert(zcmpmag(a, c), == 0);
	assert(zcmp(a, c), == 0);

	zseti(a, 10);
	zseti(c, 3);
	zmodpowu(a, a, 5, c);
	assert(zcmpmag(a, _1), == 0);
	assert(zcmp(a, _1), == 0);

	zseti(a, 10);
	zseti(b, 5);
	zseti(c, 100000L);
	zpow(a, a, b);
	assert(zcmpmag(a, c), == 0);
	assert(zcmp(a, c), == 0);

	zseti(a, -10);
	zseti(b, 5);
	zseti(c, -100000L);
	zpow(a, a, b);
	assert(zcmpmag(a, c), == 0);
	assert(zcmp(a, c), == 0);

	zseti(a, -10);
	zseti(b, 4);
	zseti(c, 10000L);
	zpow(a, a, b);
	assert(zcmpmag(a, c), == 0);
	assert(zcmp(a, c), == 0);

	zseti(a, 10);
	zseti(b, 5);
	zseti(c, 3);
	zmodpow(a, a, b, c);
	assert(zcmpmag(a, _1), == 0);
	assert(zcmp(a, _1), == 0);

	zseti(a, 102);
	zseti(b, 501);
	zseti(c, 5);
	zmodmul(a, a, b, c);
	assert(zcmp(a, _2), == 0);

	zseti(b, 2 * 3 * 3 * 7);
	zseti(c, 3 * 7 * 11);
	zseti(d, 3 * 7);
	assert((zgcd(a, _0, _0), zcmp(a, _0)), == 0);
	assert((zgcd(a, b, _0), zcmp(a, b)), == 0);
	assert((zgcd(a, _0, c), zcmp(a, c)), == 0);
	assert((zgcd(a, b, b), zcmp(a, b)), == 0);
	assert((zgcd(a, b, _2), zcmp(a, _2)), == 0);
	assert((zgcd(a, _2, b), zcmp(a, _2)), == 0);
	assert((zgcd(a, _2, _2), zcmp(a, _2)), == 0);
	assert((zgcd(a, c, _2), zcmp(a, _1)), == 0);
	assert((zgcd(a, _2, c), zcmp(a, _1)), == 0);
	assert((zgcd(a, b, _1), zcmp(a, _1)), == 0);
	assert((zgcd(a, _1, c), zcmp(a, _1)), == 0);
	assert((zgcd(a, _1, _1), zcmp(a, _1)), == 0);
	assert((zgcd(a, b, c), zcmp(a, d)), == 0);
	assert((zgcd(a, c, b), zcmp(a, d)), == 0);

	zsets(a, "1234");
	assert(zcmpi(a, 1234), == 0);
	zsets(b, "+1234");
	assert(zcmp(a, b), == 0);
	assert_zu(zstr_length(_0, 10), 1);
	assert_zu(zstr_length(_1, 10), 1);
	assert_zu(zstr_length(_2, 10), 1);
	assert_zu(zstr_length(_3, 10), 1);
	zneg(_2, _2);
	assert_zu(zstr_length(_2, 10), 2);
	zneg(_2, _2);
	assert_zu(zstr_length(a, 10), 4);
	zstr(a, buf);
	assert_s(buf, "1234");
	zsets(a, "-1234");
	zseti(b, -1234);
	zseti(c, 1234);
	assert(zcmp(a, _0), < 0);
	assert(zcmp(a, b), == 0);
	assert(zcmpmag(a, c), == 0);
	assert(zcmp(a, c), < 0);
	zstr(a, buf);
	assert_s(buf, "-1234");
	assert_s(zstr(a, buf), "-1234");

	zsetu(d, 100000UL);
	zrand(a, FAST_RANDOM, UNIFORM, d);
	assert(zcmp(a, _0), >= 0);
	assert(zcmp(a, d), <= 0);
	zrand(b, SECURE_RANDOM, UNIFORM, d);
	assert(zcmp(b, _0), >= 0);
	assert(zcmp(b, d), <= 0);
	zrand(c, FAST_RANDOM, UNIFORM, d);
	assert(zcmp(c, _0), >= 0);
	assert(zcmp(c, d), <= 0);
	assert(zcmp(a, b), != 0);
	assert(zcmp(a, c), != 0);
	assert(zcmp(b, c), != 0);

	assert((zseti(a, -5), zptest(0, a, 100)), == NONPRIME);
	assert((zseti(a, -4), zptest(0, a, 100)), == NONPRIME);
	assert((zseti(a, -3), zptest(0, a, 100)), == NONPRIME);
	assert((zseti(a, -2), zptest(0, a, 100)), == NONPRIME);
	assert((zseti(a, -1), zptest(0, a, 100)), == NONPRIME);
	assert((zseti(a, 0), zptest(0, a, 100)), == NONPRIME);
	assert((zseti(a, 1), zptest(0, a, 100)), == NONPRIME);
	assert((zseti(a, 2), zptest(0, a, 100)), == PRIME);
	assert((zseti(a, 3), zptest(0, a, 100)), == PRIME);
	assert((zseti(a, 4), zptest(0, a, 100)), == NONPRIME);
	assert((zseti(a, 5), zptest(0, a, 100)), != NONPRIME);
	assert((zseti(a, 6), zptest(0, a, 100)), == NONPRIME);
	assert((zseti(a, 7), zptest(0, a, 100)), != NONPRIME);
	assert((zseti(a, 8), zptest(0, a, 100)), == NONPRIME);
	assert((zseti(a, 9), zptest(0, a, 100)), == NONPRIME);
	assert((zseti(a, 10), zptest(0, a, 100)), == NONPRIME);
	assert((zseti(a, 11), zptest(0, a, 100)), != NONPRIME);
	assert((zseti(a, 101), zptest(0, a, 100)), != NONPRIME);

	assert_nr(zdivmod(a, b, _0, _0));
	assert_nr(zdivmod(a, b, _1, _0));
	zdivmod(a, b, _0, _1);
	zdivmod(a, b, _1, _1);
	assert_nr(zdiv(a, _0, _0));
	assert_nr(zdiv(a, _1, _0));
	zdiv(a, _0, _1);
	zdiv(a, _1, _1);
	assert_nr(zmod(a, _0, _0));
	assert_nr(zmod(a, _1, _0));
	zmod(a, _0, _1);
	zmod(a, _1, _1);
	assert_nr(zpow(a, _0, _0));
	assert_nr((zneg(_1, _1), zpow(a, _0, _1))); zneg(_1, _1);
	zpow(a, _0, _1);
	zpow(a, _1, _0);
	zneg(_1, _1), zpow(a, _1, _0), zneg(_1, _1);
	assert_nr(zmodmul(a, _1, _1, _0));
	assert_nr(zmodpow(a, _0, _0, _1));
	assert_nr((zneg(_1, _1), zmodpow(a, _0, _1, _1))); zneg(_1, _1);
	zmodpow(a, _0, _1, _1);
	zmodpow(a, _1, _0, _1);
	zneg(_1, _1), zmodpow(a, _1, _0, _1), zneg(_1, _1);
	assert_nr(zmodpow(a, _0, _0, _0));
	assert_nr((zneg(_1, _1), zmodpow(a, _0, _1, _0))); zneg(_1, _1);
	assert_nr(zmodpow(a, _0, _1, _0));
	assert_nr(zmodpow(a, _1, _0, _0));
	assert_nr((zneg(_1, _1), zmodpow(a, _1, _0, _0))); zneg(_1, _1);
	assert_nr(zpowu(a, _0, 0));
	zpowu(a, _0, 1);
	zpowu(a, _1, 0);
	zneg(_1, _1), zpowu(a, _1, 0), zneg(_1, _1);
	assert_nr(zmodpowu(a, _0, 0, _1));
	zmodpowu(a, _0, 1, _1);
	zmodpowu(a, _1, 0, _1);
	zneg(_1, _1), zmodpowu(a, _1, 0, _1), zneg(_1, _1);
	assert_nr(zmodpowu(a, _0, 0, _0));
	assert_nr((zneg(_1, _1), zmodpowu(a, _0, 1, _0))); zneg(_1, _1);
	assert_nr(zmodpowu(a, _0, 1, _0));
	assert_nr(zmodpowu(a, _1, 0, _0));
	assert_nr((zneg(_1, _1), zmodpowu(a, _1, 0, _0))); zneg(_1, _1);

	zsetu(a, 1LL);
	assert_s(zstr(a, buf), "1");
	zsetu(a, 10LL);
	assert_s(zstr(a, buf), "10");
	zsetu(a, 100LL);
	assert_s(zstr(a, buf), "100");
	zsetu(a, 1000LL);
	assert_s(zstr(a, buf), "1000");
	zsetu(a, 10000LL);
	assert_s(zstr(a, buf), "10000");
	zsetu(a, 100000LL);
	assert_s(zstr(a, buf), "100000");
	zsetu(a, 1000000LL);
	assert_s(zstr(a, buf), "1000000");
	zsetu(a, 10000000LL);
	assert_s(zstr(a, buf), "10000000");
	zsetu(a, 100000000LL);
	assert_s(zstr(a, buf), "100000000");
	zsetu(a, 999999999LL);
	assert_s(zstr(a, buf), "999999999");
	zsetu(a, 1000000000LL);
	assert_s(zstr(a, buf), "1000000000");
	zsetu(a, 1000000001LL);
	assert_s(zstr(a, buf), "1000000001");
	zsetu(a, 2000000000LL);
	assert_s(zstr(a, buf), "2000000000");
	zsetu(a, 2050000000LL);
	assert_s(zstr(a, buf), "2050000000");
	zsetu(a, 2100000000LL);
	assert_s(zstr(a, buf), "2100000000");
	zsetu(a, 2140000000LL);
	assert_s(zstr(a, buf), "2140000000");
	zsetu(a, 2147000000LL);
	assert_s(zstr(a, buf), "2147000000");
	zsetu(a, 2147483000LL);
	assert_s(zstr(a, buf), "2147483000");
	zsetu(a, 2147483640LL);
	assert_s(zstr(a, buf), "2147483640");
	zsetu(a, 2147483646LL);
	assert_s(zstr(a, buf), "2147483646");

	zseti(a, 2147483647LL);
	assert_s(zstr(a, buf), "2147483647");
	zseti(a, -2147483647LL);
	assert_s(zstr(a, buf), "-2147483647");
	zseti(a, -2147483647LL - 1LL);
	assert_s(zstr(a, buf), "-2147483648");

	zsetu(a, 2147483647ULL);
	assert_s(zstr(a, buf), "2147483647");
	zsetu(a, 2147483648ULL);
	assert_s(zstr(a, buf), "2147483648");
	zsetu(a, 2147483649ULL);
	assert_s(zstr(a, buf), "2147483649");

	zsetu(a, 3000000000ULL);
	assert_s(zstr(a, buf), "3000000000");
	zsetu(a, 3100000000ULL);
	assert_s(zstr(a, buf), "3100000000");
	zsetu(a, 3200000000ULL);
	assert_s(zstr(a, buf), "3200000000");
	zsetu(a, 3300000000ULL);
	assert_s(zstr(a, buf), "3300000000");
	zsetu(a, 3400000000ULL);
	assert_s(zstr(a, buf), "3400000000");
	zsetu(a, 3500000000ULL);
	assert_s(zstr(a, buf), "3500000000");
	zsetu(a, 3600000000ULL);
	assert_s(zstr(a, buf), "3600000000");
	zsetu(a, 3700000000ULL);
	assert_s(zstr(a, buf), "3700000000");
	zsetu(a, 3800000000ULL);
	assert_s(zstr(a, buf), "3800000000");
	zsetu(a, 3900000000ULL);
	assert_s(zstr(a, buf), "3900000000");
	zsetu(a, 3999999999ULL);
	assert_s(zstr(a, buf), "3999999999");
	zsetu(a, 4000000000ULL);
	assert_s(zstr(a, buf), "4000000000");
	zsetu(a, 4000000001ULL);
	assert_zu(zstr_length(a, 10), 10);
	assert_s(zstr(a, buf), "4000000001");

	zsetu(a, 4000000000ULL);
	zsetu(b, 4000000000ULL);
	zadd(c, a, a);
	zsets(d, "8000000000");
	assert(zcmp(c, d), == 0);
	zadd(c, a, b);
	assert(zcmp(c, d), == 0);
	zadd(c, c, a);
	zsets(d, "12000000000");
	assert(zcmp(c, d), == 0);
	zsub(c, c, a);
	zsets(d, "8000000000");
	assert(zcmp(c, d), == 0);
	zsub(c, c, a);
	zsets(d, "4000000000");
	assert(zcmp(c, d), == 0);
	zsets(d, "8000000000");
	zrsh(d, d, 1);
	assert(zcmp(c, d), == 0);
	zsets(a, "6234216714");
	zsets(b, "9424614147");
	zsets(d, "830476546");
	zand(c, a, b);
	assert(zcmp(c, d), == 0);
	zsets(a, "234216714");
	zsets(b, "9424614147");
	zsets(d, "9629466379");
	zor(c, a, b);
	assert(zcmp(c, d), == 0);
	zsets(a, "6234216714");
	zsets(b, "9424614147");
	zsets(d, "13997877769");
	zxor(c, a, b);
	assert(zcmp(c, d), == 0);
	zsets(a, "34216714");
	zsets(b, "9424614147");
	zsets(d, "9458821129");
	zxor(c, a, b);
	assert(zcmp(c, d), == 0);
	zsetu(a, 1000000000ULL);
	zsets(d, "1000000000000000000");
	zmul(c, a, a);
	assert(zcmp(c, d), == 0);
	zdiv(c, c, a);
	assert(zcmp(c, a), == 0);
	zsetu(a, 1000000000ULL);
	zsets(d, "1000000000000000000");
	zsqr(c, a);
	assert(zcmp(c, d), == 0);
	zsetu(a, 1000000000ULL);
	zmodpowu(c, a, 5, _3);
	assert(zcmpu(c, 1), == 0);
	zsetu(a, 1000000000ULL);
	zsets(d, "1");
	zpowu(c, a, 0);
	assert(zcmp(c, d), == 0);
	zsetu(a, 1000000000ULL);
	zsets(d, "1000000000");
	zpowu(c, a, 1);
	assert(zcmp(c, d), == 0);
	zsetu(a, 1000000000ULL);
	zsets(d, "1000000000000000000");
	zpowu(c, a, 2);
	assert(zcmp(c, d), == 0);
	zsetu(a, 1000000000ULL);
	zsets(b, "1000000000000000000");
	zsets(d, "1000000000000000000000000000");
	zmul(c, a, b);
	assert(zcmp(c, d), == 0);
	zsetu(a, 1000000000ULL);
	zsets(d, "1000000000000000000000000000");
	zmul(b, a, a);
	zmul(b, b, a);
	assert(zcmp(c, d), == 0);
	zsetu(a, 1000000000ULL);
	zsets(d, "1000000000000000000000000000");
	zpowu(c, a, 3);
	assert(zcmp(c, d), == 0);
	zsetu(a, 1000000000ULL);
	zsets(d, "1000000000000000000000000000000000000");
	zpowu(c, a, 4);
	assert(zcmp(c, d), == 0);
	zsetu(a, 1000000000ULL);
	zsets(d, "1000000000000000000000000000000000000000000000");
	zpowu(c, a, 5);
	assert(zcmp(c, d), == 0);

	zsetu(a, 4294967294ULL);
	assert_s(zstr(a, buf), "4294967294");
	zsetu(a, 4294967295ULL);
	assert_s(zstr(a, buf), "4294967295");
	zsetu(a, 4294967296ULL);
	assert_s(zstr(a, buf), "4294967296");
	zsetu(a, 4294967297ULL);
	assert_s(zstr(a, buf), "4294967297");

	zseti(a, 9223372036854775807LL);
	assert_s(zstr(a, buf), "9223372036854775807");
	zseti(a, -9223372036854775807LL);
	assert_s(zstr(a, buf), "-9223372036854775807");
	zseti(a, -9223372036854775807LL - 1LL);
	assert_s(zstr(a, buf), "-9223372036854775808");

	zsetu(a, 18446744073709551614ULL);
	assert_s(zstr(a, buf), "18446744073709551614");
	zsetu(a, 18446744073709551615ULL);
	assert_s(zstr(a, buf), "18446744073709551615");
	zadd(a, a, _1);
	assert_s(zstr(a, buf), "18446744073709551616");
	zadd(a, a, _1);
	assert_s(zstr(a, buf), "18446744073709551617");

	zsets(a, "1000000000000000000000000000000");
	assert_s(zstr(a, buf), "1000000000000000000000000000000");
	zsets(a, "+1000000000000000000000000000000");
	assert_s(zstr(a, buf), "1000000000000000000000000000000");
	zsets(a, "-1000000000000000000000000000000");
	assert_s(zstr(a, buf), "-1000000000000000000000000000000");

	zsetu(a, 1000000000000000ULL);
	zsqr(a, a);
	assert_s(zstr(a, buf), "1000000000000000000000000000000");

#include "test-random.c"

done:
	zfree(a), zfree(b), zfree(c), zfree(d), zfree(_0), zfree(_1), zfree(_2), zfree(_3);
	zunsetup();
	return ret;
}

