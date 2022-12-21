#include <stdint.h>
#include <stddef.h>

#ifdef __linux__
// Cross-checks are only supported on Linux,
// and this doesn't compile on Mac OS anyway

uint64_t __c2rust_hash_internal_state_struct(void *x, size_t depth) {
    return 0xABCD0001;
}

uint64_t __c2rust_hash_lzma_internal_s_struct(void *x, size_t depth) {
    return 0xABCD0002;
}

uint64_t xcheck_hash_xmlSchemaValDate(void *x, size_t depth) {
    return 0xABCD0003;
}

uint64_t xcheck_hash_xmlSchemaValDecimal(void *x, size_t depth) {
    return 0xABCD0004;
}

uint64_t xcheck_hash_xmlSchemaType(void *x, size_t depth) {
    return (0xABCD0005ULL << 32) | (*(uint32_t*)x);
}

uint64_t xcheck_hash_xmlSchemaAttribute(void *x, size_t depth) {
    return (0xABCD0005ULL << 32) | (*(uint32_t*)x);
}

uint64_t xcheck_hash_xmlSchemaWildcard(void *x, size_t depth) {
    return (0xABCD0005ULL << 32) | (*(uint32_t*)x);
}

uint64_t xcheck_hash_xmlSchemaAttributeGroup(void *x, size_t depth) {
    return (0xABCD0005ULL << 32) | (*(uint32_t*)x);
}

uint64_t xcheck_hash_xmlSchemaElement(void *x, size_t depth) {
    return (0xABCD0005ULL << 32) | (*(uint32_t*)x);
}

uint64_t xcheck_hash_xmlSchemaFacet(void *x, size_t depth) {
    return (0xABCD0005ULL << 32) | (*(uint32_t*)x);
}

uint64_t xcheck_hash_xmlSchemaNotation(void *x, size_t depth) {
    return (0xABCD0005ULL << 32) | (*(uint32_t*)x);
}

struct _xmlNodeCommon {
    void *ptr1;
    unsigned type;
};

uint64_t xcheck_hash_xmlNode(void *x, size_t depth) {
    struct _xmlNodeCommon *nc = (struct _xmlNodeCommon*) x;
    return ((0xABCD0006ULL) << 32) | nc->type;
}

uint64_t xcheck_hash_xmlDoc(void *x, size_t depth)
    __attribute__((alias("xcheck_hash_xmlNode")));

uint64_t xcheck_hash_xmlDtd(void *x, size_t depth)
    __attribute__((alias("xcheck_hash_xmlNode")));

uint64_t xcheck_hash_xmlAttr(void *x, size_t depth)
    __attribute__((alias("xcheck_hash_xmlNode")));

uint64_t xcheck_hash_xmlAttribute(void *x, size_t depth)
    __attribute__((alias("xcheck_hash_xmlNode")));

uint64_t xcheck_hash_xmlElement(void *x, size_t depth)
    __attribute__((alias("xcheck_hash_xmlNode")));

uint64_t xcheck_hash_xmlNs(void *x, size_t depth)
    __attribute__((alias("xcheck_hash_xmlNode")));

#endif // __linux__
