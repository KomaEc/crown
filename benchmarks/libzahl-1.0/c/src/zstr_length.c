/* See LICENSE file for copyright and license details. */
#include "internals.h"

#define num  libzahl_tmp_str_num
#define mag  libzahl_tmp_str_mag
#define div  libzahl_tmp_str_div


size_t
zstr_length(z_t a, unsigned long long int radix)
{
	size_t size_total = 1, size_temp;
	zset(num, a);
	while (!zzero(num)) {
		zsetu(mag, radix);
		zset(div, mag);
		size_temp = 1;
		while (zcmpmag(mag, num) <= 0) {
			zset(div, mag);
			zsqr(mag, mag);
			size_temp <<= 1;
		}
		size_temp >>= 1;
		size_total += size_temp;
		zdiv(num, num, div);
	}
	return size_total + (zsignum(a) < 0);
}
