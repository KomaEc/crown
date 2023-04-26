/* See LICENSE file for copyright and license details. */

/* Warning: libzahl is not thread-safe. */
/* Caution: Do not use libzahl for cryptographic applications, use a specialised library. */


#include <stddef.h>
#include <setjmp.h>
#include <stdint.h>



/* You should pretend like this typedef does not exist. */
typedef uint32_t zahl_char_t;

/* This structure should be considered opaque. */
typedef struct {
        int sign;
        size_t used;
        size_t alloced;
        zahl_char_t *chars;
} z_t[1];


enum zprimality { NONPRIME = 0, PROBABLY_PRIME, PRIME };
enum zranddev { FAST_RANDOM = 0, SECURE_RANDOM };
enum zranddist { QUASIUNIFORM = 0, UNIFORM };

enum zerror {
	ZERROR_ERRNO_SET = 0
};



/* The parameters in the functions below are numbers a, b, c, ... */


/* Library initialisation and destruction. */

void zsetup(jmp_buf);                  /* Prepare libzahl for use. */
void zunsetup(void);                   /* Free resources used by libzahl */


/* Memory functions. */

void zfree(z_t);                       /* Free resources in a. */
void zswap(z_t, z_t);                  /* (a, b) := (b, a) */
size_t zsave(z_t, void *);             /* Store a into b (if !!b), and return number of written bytes. */
size_t zload(z_t, const void *);       /* Restore a from b, and return number of read bytes. */


/* Assignment functions. */

/* a := b */
void zset(z_t, z_t);
void zseti(z_t, long long int);
void zsetu(z_t, unsigned long long int);


/* Comparison functions. */

/* signum (a - b) */
int zcmp(z_t, z_t);
int zcmpi(z_t, long long int);
int zcmpu(z_t, unsigned long long int);

int zcmpmag(z_t, z_t);                 /* signum (|a| - |b|) */


/* Arithmetic functions. */

void zadd(z_t, z_t, z_t);              /* a := b + c */
void zsub(z_t, z_t, z_t);              /* a := b - c */
void zmul(z_t, z_t, z_t);              /* a := b * c */
void zmodmul(z_t, z_t, z_t, z_t);      /* a := (b * c) % d */
void zdiv(z_t, z_t, z_t);              /* a := b / c */
void zdivmod(z_t, z_t, z_t, z_t);      /* a := c / d, b = c % d */
void zmod(z_t, z_t, z_t);              /* a := b % c */
void zsqr(z_t, z_t);                   /* a := b² */
void zmodsqr(z_t, z_t, z_t);           /* a := b² % c */
void zneg(z_t, z_t);                   /* a := -b */
void zabs(z_t, z_t);                   /* a := |b| */
void zpow(z_t, z_t, z_t);              /* a := b ↑ c */
void zmodpow(z_t, z_t, z_t, z_t);      /* a := (b ↑ c) % d */
void zpowu(z_t, z_t, unsigned long long int);
void zmodpowu(z_t, z_t, unsigned long long int, z_t);

/* These are used internally and may be removed in a future version. */
void zadd_unsigned(z_t, z_t, z_t);     /* a := |b| + |c| */
void zsub_unsigned(z_t, z_t, z_t);     /* a := |b| - |c| */


/* Bitwise operations. */

void zand(z_t, z_t, z_t);              /* a := b & c */
void zor(z_t, z_t, z_t);               /* a := b | c */
void zxor(z_t, z_t, z_t);              /* a := b ^ c */
void znot(z_t, z_t);                   /* a := ~b */
void zlsh(z_t, z_t, size_t);           /* a := b << c */
void zrsh(z_t, z_t, size_t);           /* a := b >> c */
void ztrunc(z_t, z_t, size_t);         /* a := b & ((1 << c) - 1) */
int zbtest(z_t, size_t);               /* (a >> b) & 1 */
void zsplit(z_t, z_t, z_t, size_t);    /* a := c >> d, b := c - (a << d) */
size_t zbits(z_t);                     /* ⌊log₂ |a|⌋ + 1, 1 if a = 0 */
size_t zlsb(z_t);                      /* Index of first set bit, SIZE_MAX if none are set. */

/* If d > 0: a := b | (1 << c), f d = 0: a := b & ~(1 << c), if d < 0: a := b ^ (1 << c) */
void zbset(z_t, z_t, size_t, int);


/* Number theory. */

void zgcd(z_t, z_t, z_t);              /* a := gcd(b, c) */

/* NONPRIME if b ∉ ℙ, PROBABLY_PRIME, if b ∈ ℙ with (1 − 4↑−c) certainty, 2 if PRIME ∈ ℙ.
 * If NONPRIME is returned the witness of b's compositeness is stored in a. */
enum zprimality zptest(z_t, z_t, int);


/* Random number generation. */

/* Pick a randomly from [0, d] ∩ ℤ. */
void zrand(z_t, enum zranddev, enum zranddist, z_t);


/* String conversion. */

char *zstr(z_t, char *);               /* Write a in decimal onto b. */
int zsets(z_t, const char *);          /* a := b */

/* Length of a in radix b. */
size_t zstr_length(z_t, unsigned long long int);


/* Error handling functions. */

enum zerror zerror(const char **);     /* Return the current error code, and unless a is 0, a description in *a. */
void zperror(const char *);            /* Identical to perror(3p) except it supports libzahl errors. */


/* Inline functions. */

static inline void zinit(z_t a)        { a->alloced = 0; a->chars = 0; }          /* Prepare a for use. */
static inline int zeven(z_t a)         { return !a->sign || !(a->chars[0] & 1); } /* Is a even? */
static inline int zodd(z_t a)          { return a->sign && (a->chars[0] & 1); }   /* Is a odd? */
static inline int zeven_nonzero(z_t a) { return !(a->chars[0] & 1); }             /* Is a even? Assumes a ≠ 0. */
static inline int zodd_nonzero(z_t a)  { return (a->chars[0] & 1); }              /* Is a odd? Assumes a ≠ 0. */
static inline int zzero(z_t a)         { return !a->sign; }                       /* Is a zero? */
static inline int zsignum(z_t a)       { return a->sign; }                        /* a/|a|, 0 if a is zero. */
