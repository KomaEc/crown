/* See LICENSE file for copyright and license details. */
#include "../zahl.h"

#include <string.h>
#include <stdlib.h>
#include <errno.h>

#define BITS_PER_CHAR                32
#define LB_BITS_PER_CHAR             5
#define ZAHL_CHAR_MAX                UINT32_MAX

#define FLOOR_BITS_TO_CHARS(bits)    ((bits) >> LB_BITS_PER_CHAR)
#define CEILING_BITS_TO_CHARS(bits)  (((bits) + (BITS_PER_CHAR - 1)) >> LB_BITS_PER_CHAR)
#define BITS_IN_LAST_CHAR(bits)      ((bits) & (BITS_PER_CHAR - 1))

#define LIST_TEMPS\
	X(libzahl_tmp_cmp)\
	X(libzahl_tmp_str_num)\
	X(libzahl_tmp_str_mag)\
	X(libzahl_tmp_str_div)\
	X(libzahl_tmp_str_rem)\
	X(libzahl_tmp_gcd_u)\
	X(libzahl_tmp_gcd_v)\
	X(libzahl_tmp_sub)\
	X(libzahl_tmp_modmul)\
	X(libzahl_tmp_div)\
	X(libzahl_tmp_mod)\
	X(libzahl_tmp_pow_b)\
	X(libzahl_tmp_pow_c)\
	X(libzahl_tmp_pow_d)\
	X(libzahl_tmp_modsqr)\
	X(libzahl_tmp_divmod_a)\
	X(libzahl_tmp_divmod_b)\
	X(libzahl_tmp_divmod_d)\
	X(libzahl_tmp_ptest_x)\
	X(libzahl_tmp_ptest_a)\
	X(libzahl_tmp_ptest_d)\
	X(libzahl_tmp_ptest_n1)\
	X(libzahl_tmp_ptest_n4)

#define LIST_CONSTS\
	X(libzahl_const_1e19, zsetu, 10000000000000000000ULL) /* The largest power of 10 < 2⁶⁴. */\
	X(libzahl_const_1e9,  zsetu, 1000000000ULL)           /* The largest power of 10 < 2³². */\
	X(libzahl_const_1,    zsetu, 1)\
	X(libzahl_const_2,    zsetu, 2)\
	X(libzahl_const_4,    zsetu, 4)

#define X(x)  extern z_t x;
LIST_TEMPS
#undef X
#define X(x, f, v)  extern z_t x;
LIST_CONSTS
#undef X

extern z_t libzahl_tmp_divmod_ds[BITS_PER_CHAR];
extern jmp_buf libzahl_jmp_buf;
extern int libzahl_set_up;
extern int libzahl_error;
extern zahl_char_t **libzahl_pool[sizeof(size_t) * 8];
extern size_t libzahl_pool_n[sizeof(size_t) * 8];
extern size_t libzahl_pool_alloc[sizeof(size_t) * 8];

#define FAILURE(error)               (libzahl_error = (error), longjmp(libzahl_jmp_buf, 1))
#define zmemcpy(d, s, n)             memcpy(d, s, (n) * sizeof(zahl_char_t))
#define zmemmove(d, s, n)            memmove(d, s, (n) * sizeof(zahl_char_t))
#define zmemset(a, v, n)             memset(a, v, (n) * sizeof(zahl_char_t))
#define zmemcmp(a, b, n)             memcmp(a, b, (n) * sizeof(zahl_char_t))

#define SET_SIGNUM(a, signum)        ((a)->sign = (signum))
#define SET(a, b)                    do { if ((a) != (b)) zset(a, b); } while (0)
#define ENSURE_SIZE(a, n)            do { if ((a)->alloced < (n)) libzahl_realloc(a, (n)); } while (0)

#define MIN(a, b)                    ((a) < (b) ? (a) : (b))
#define MAX(a, b)                    ((a) > (b) ? (a) : (b))

void libzahl_realloc(z_t a, size_t need);
