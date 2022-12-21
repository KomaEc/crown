
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlDict;
    /*
 * Summary: Chained hash tables
 * Description: This module implements the hash table support used in
 *		various places in the library.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Bjorn Reese <bjorn.reese@systematic.dk>
 */
    /*
 * The hash table.
 */
    pub type _xmlHashTable;
    pub type _xmlBuf;
    /* *
 * xmlAutomataStatePtr:
 *
 * A state int the automata description,
 */
    pub type _xmlAutomataState;
    /*
 * Summary: API to build regexp automata
 * Description: the API to build regexp automata
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /* *
 * xmlAutomataPtr:
 *
 * A libxml automata description, It can be compiled into a regexp
 */
    pub type _xmlAutomata;
    pub type _xmlValidState;
    pub type internal_state;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(__filename: *const std::os::raw::c_char, __modes: *const std::os::raw::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn fread(__ptr: *mut std::os::raw::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fwrite(__ptr: *const std::os::raw::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrncasecmp(str1: *const xmlChar, str2: *const xmlChar,
                      len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrPrintf(buf: *mut xmlChar, len: std::os::raw::c_int,
                    msg: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrstr(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strncpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn __errno_location() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __xstat(__ver: std::os::raw::c_int, __filename: *const std::os::raw::c_char,
               __stat_buf: *mut stat) -> std::os::raw::c_int;
    #[no_mangle]
    fn close(__fd: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn read(__fd: std::os::raw::c_int, __buf: *mut std::os::raw::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: std::os::raw::c_int, __buf: *const std::os::raw::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn getcwd(__buf: *mut std::os::raw::c_char, __size: size_t) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn dup(__fd: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> std::os::raw::c_int;
    #[no_mangle]
    fn gzdopen(fd: std::os::raw::c_int, mode: *const std::os::raw::c_char) -> gzFile;
    #[no_mangle]
    fn gzread(file: gzFile, buf: voidp, len: std::os::raw::c_uint) -> std::os::raw::c_int;
    #[no_mangle]
    fn gzwrite(file: gzFile, buf: voidpc, len: std::os::raw::c_uint) -> std::os::raw::c_int;
    #[no_mangle]
    fn gzdirect(file: gzFile) -> std::os::raw::c_int;
    #[no_mangle]
    fn gzclose(file: gzFile) -> std::os::raw::c_int;
    #[no_mangle]
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn deflateInit2_(strm: z_streamp, level: std::os::raw::c_int, method: std::os::raw::c_int,
                     windowBits: std::os::raw::c_int, memLevel: std::os::raw::c_int,
                     strategy: std::os::raw::c_int, version: *const std::os::raw::c_char,
                     stream_size: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn gzopen64(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> gzFile;
    /*
 * Summary: error handling
 * Description: the API used to report errors
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /* *
 * xmlErrorLevel:
 *
 * Indicates the level of an error
 */
    /* A simple warning */
    /* A recoverable error */
    /* A fatal error */
    /* *
 * xmlErrorDomain:
 *
 * Indicates where an error may have come from
 */
    /* The XML parser */
    /* The tree module */
    /* The XML Namespace module */
    /* The XML DTD validation with parser context*/
    /* The HTML parser */
    /* The memory allocator */
    /* The serialization code */
    /* The Input/Output stack */
    /* The FTP module */
    /* The HTTP module */
    /* The XInclude processing */
    /* The XPath module */
    /* The XPointer module */
    /* The regular expressions module */
    /* The W3C XML Schemas Datatype module */
    /* The W3C XML Schemas parser module */
    /* The W3C XML Schemas validation module */
    /* The Relax-NG parser module */
    /* The Relax-NG validator module */
    /* The Catalog module */
    /* The Canonicalization module */
    /* The XSLT engine from libxslt */
    /* The XML DTD validation with valid context */
    /* The error checking module */
    /* The xmlwriter module */
    /* The dynamically loaded module module*/
    /* The module handling character conversion */
    /* The Schematron validator module */
    /* The buffers module */
    /* The URI module */
    /* *
 * xmlError:
 *
 * An XML Error instance.
 */
    /* What part of the library raised this error */
    /* The error code, e.g. an xmlParserError */
    /* human-readable informative error message */
    /* how consequent is the error */
    /* the filename */
    /* the line number if available */
    /* extra string information */
    /* extra string information */
    /* extra string information */
    /* extra number information */
    /* error column # or 0 if N/A (todo: rename field when we would brk ABI) */
    /* the parser context if available */
    /* the node in the tree */
    /* *
 * xmlParserError:
 *
 * This is an error that the XML (or HTML) parser can generate
 */
    /* 1 */
    /* 2 */
    /* 3 */
    /* 4 */
    /* 5 */
    /* 6 */
    /* 7 */
    /* 8 */
    /* 9 */
    /* 10 */
    /* 11 */
    /* 12 */
    /* 13 */
    /* 14 */
    /* 15 */
    /* 16 */
    /* 17 */
    /* 18 */
    /* 19 */
    /* 20 */
    /* 21 */
    /* 22 */
    /* 23 */
    /* 24 */
    /* 25 */
    /* 26 */
    /* 27 */
    /* 28 */
    /* 29 */
    /* 30 */
    /* 31 */
    /* 32 */
    /* 33 */
    /* 34 */
    /* 35 */
    /* 36 */
    /* 37 */
    /* 38 */
    /* 39 */
    /* 40 */
    /* 41 */
    /* 42 */
    /* 43 */
    /* 44 */
    /* 45 */
    /* 46 */
    /* 47 */
    /* 48 */
    /* 49 */
    /* 50 */
    /* 51 */
    /* 52 */
    /* 53 */
    /* 54 */
    /* 55 */
    /* 56 */
    /* 57 */
    /* 58 */
    /* 59 */
    /* 60 */
    /* 61 */
    /* 62 */
    /* 63 */
    /* 64 */
    /* 65 */
    /* 66 */
    /* 67 */
    /* 68 */
    /* 69 */
    /* 70 */
    /* 71 */
    /* 72 */
    /* 73 */
    /* 74 */
    /* 75 */
    /* 76 */
    /* 77 */
    /* 78 */
    /* 79 */
    /* 80 */
    /* 81 */
    /* 82 */
    /* 83 */
    /* 84 */
    /* 85 */
    /* 86 */
    /* 87 */
    /* 88 */
    /* 89 */
    /* 90 */
    /* 91 */
    /* 92 */
    /* 93 */
    /* 94 */
    /* 95 */
    /* 96 */
    /* 97 */
    /* 98 */
    /* 99 */
    /* 100 */
    /* 101 */
    /* 102 */
    /* 103 */
    /* 104 */
    /* 105 */
    /* 106 */
    /* 107 */
    /* 108 */
    /* 109 */
    /* 110 */
    /* 111 */
    /* 201 */
    /* 202 */
    /* 203 */
    /* 204 */
    /* 205 */
    /* 501 */
    /* 502 */
    /* 503 */
    /* 504 */
    /* 505 */
    /* 506 */
    /* 507 */
    /* 508 */
    /* 509 */
    /* 510 */
    /* 511 */
    /* 512 */
    /* 513 */
    /* 514 */
    /* 515 */
    /* 516 */
    /* 517 */
    /* 518 */
    /* 519 */
    /* 520 */
    /* 521 */
    /* 522 */
    /* 523 */
    /* 524 */
    /* 525 */
    /* 526 */
    /* 527 */
    /* 528 */
    /* 529 */
    /* 530 */
    /* 531 */
    /* 532 */
    /* 533 */
    /* 534 */
    /* 535 */
    /* 536 */
    /* 537 */
    /* 538 */
    /* 539 */
    /* 540 */
    /* 541 */
    /* 801 */
    /* 1001 */
    /* 1002 */
    /* 1003 */
    /* 1004 */
    /* 1005 */
    /* 1006 */
    /* 1007 */
    /* 1008 */
    /* 1009 */
    /* 1010 */
    /* 1011 */
    /* 1012 */
    /* 1013 */
    /* 1014 */
    /* 1015 */
    /* 1016 */
    /* 1017 */
    /* 1018 */
    /* 1019 */
    /* 1020 */
    /* 1021 */
    /* 1022 */
    /* 1023 */
    /* 1024 */
    /* 1025 */
    /* 1026 */
    /* 1027 */
    /* 1028 */
    /* 1029 */
    /* 1030 */
    /* 1031 */
    /* 1032 */
    /* 1033 */
    /* 1034 */
    /* 1035 */
    /* 1036 */
    /* 1037 */
    /* 1038 */
    /* 1039 */
    /* 1040 */
    /* 1041 */
    /* 1042 */
    /* 1043 */
    /* 1044 */
    /* 1045 */
    /* 1046 */
    /* 1047 */
    /* 1048 */
    /* 1049 */
    /* 1050 */
    /* 1051 */
    /* 1052 */
    /* 1053 */
    /* 1054 */
    /* 1055 */
    /* 1056 */
    /* 1057 */
    /* 1058 */
    /* 1059 */
    /* 1060 */
    /* 1061 */
    /* 1062 */
    /* 1063 */
    /* 1064 */
    /* 1065 */
    /* 1066 */
    /* 1067 */
    /* 1068 */
    /* 1069 */
    /* 1070 */
    /* 1071 */
    /* 1072 */
    /* 1073 */
    /* 1074 */
    /* 1075 */
    /* 1076 */
    /* 1077 */
    /* 1078 */
    /* 1079 */
    /* 1080 */
    /* 1081 */
    /* 1082 */
    /* 1083 */
    /* 1084 */
    /* 1085 */
    /* 1086 */
    /* 1087 */
    /* 1088 */
    /* 1089 */
    /* 1090 */
    /* 1091 */
    /* 1092 */
    /* 1093 */
    /* 1094 */
    /* 1095 */
    /* 1096 */
    /* 1097 */
    /* 1098 */
    /* 1099 */
    /* 1100 */
    /* 1101 */
    /* 1102 */
    /* 1103 */
    /* 1104 */
    /* 1105 */
    /* 1106 */
    /* 1107 */
    /* 1108 */
    /* 1109 */
    /* 1110 */
    /* 1111 */
    /* 1112 */
    /* 1113 */
    /* 1114 */
    /* 1115 */
    /* 1116 */
    /* 1117 */
    /* 1118 */
    /* 1119 */
    /* 1120 */
    /* 1121 */
    /* 1122 */
    /* 1201 */
    /* 1202 */
    /* 1203 */
    /* 1204 */
    /* 1205 */
    /* 1206 */
    /* 1207 */
    /* 1208 */
    /* 1209 */
    /* 1210 */
    /* 1211 */
    /* 1212 */
    /* 1213 */
    /* 1214 */
    /* 1215 */
    /* 1216 */
    /* 1217 */
    /* 1218 */
    /* 1219 */
    /* 1220 */
    /* 1221 */
    /* 1301 */
    /* 1302 */
    /* 1303 */
    /* 1401 */
    /* 1402 */
    /* 1403 */
    /* 1501 */
    /* 1502 */
    /* 1503 */
    /* 1504 */
    /* 1505 */
    /* 1506 */
    /* 1507 */
    /* 1508 */
    /* 1509 */
    /* 1510 */
    /* 1511 */
    /* 1512 */
    /* 1513 */
    /* 1514 */
    /* 1515 */
    /* 1516 */
    /* 1517 */
    /* 1518 */
    /* 1519 */
    /* 1520 */
    /* 1521 */
    /* 1522 */
    /* 1523 */
    /* 1524 */
    /* 1525 */
    /* 1526 */
    /* 1527 */
    /* 1528 */
    /* 1529 */
    /* 1530 */
    /* 1531 */
    /* 1532 */
    /* 1533 */
    /* 1534 */
    /* 1535 */
    /* 1536 */
    /* 1537 */
    /* 1538 */
    /* 1539 */
    /* 1540 */
    /* 1541 */
    /* 1542 */
    /* 1543 */
    /* 1544 */
    /* 1545 */
    /* 1546 */
    /* 1547 */
    /* 1548 */
    /* 1549 */
    /* 1550 */
    /* 1551 */
    /* 1552 */
    /* 1553 */
    /* 1554 */
    /* 1555 */
    /* 1556 */
    /* 1601 */
    /* 1602 */
    /* 1603 */
    /* 1604 */
    /* 1605 */
    /* 1606 */
    /* 1607 */
    /* 1608 */
    /* 1609 */
    /* 1610 */
    /* 1611 */
    /* 1612 */
    /* 1613 */
    /* 1614 */
    /* 1615 */
    /* 1616 */
    /* 1617 */
    /* 1618 */
    /* 1651 */
    /* 1652 */
    /* 1653 */
    /* 1654 */
    /* 1701 */
    /* 1702 */
    /* 1703 */
    /* 1704 */
    /* 1705 */
    /* 1706 */
    /* 1707 */
    /* 1708 */
    /* 1709 */
    /* 1710 */
    /* 1711 */
    /* 1712 */
    /* 1713 */
    /* 1714 */
    /* 1715 */
    /* 1716 */
    /* 1717 */
    /* 1718 */
    /* 1719 */
    /* 1720 */
    /* 1721 */
    /* 1722 */
    /* 1723 */
    /* 1724 */
    /* 1725 */
    /* 1726 */
    /* 1727 */
    /* 1728 */
    /* 1729 */
    /* 1730 */
    /* 1731 */
    /* 1732 */
    /* 1733 */
    /* 1734 */
    /* 1735 */
    /* 1736 */
    /* 1737 */
    /* 1738 */
    /* 1739 */
    /* 1740 */
    /* 1741 */
    /* 1742 */
    /* 1743 */
    /* 1744 */
    /* 1745 */
    /* 1746 */
    /* 1747 */
    /* 1748 */
    /* 1749 */
    /* 1750 */
    /* 1751 */
    /* 1752 */
    /* 1753 */
    /* 1754 */
    /* 1755 */
    /* 1756 */
    /* 1757 */
    /* 1758 */
    /* 1759 */
    /* 1760 */
    /* 1761 */
    /* 1762 */
    /* 1763 */
    /* 1764 */
    /* 1765 */
    /* 1766 */
    /* 1767 */
    /* 1768 */
    /* 1769 */
    /* 1770 */
    /* 1771 */
    /* 1772 */
    /* 1773 */
    /* 1774 */
    /* 1775 */
    /* 1776 */
    /* 1777 */
    /* 1778 */
    /* 1779 */
    /* 1780 */
    /* 1781 */
    /* 1782 */
    /* 1783 */
    /* 1784 */
    /* 1785 */
    /* 1786 */
    /* 1787 */
    /* 1788 */
    /* 1789 */
    /* 1790 */
    /* 1791 */
    /* 1792 */
    /* 1793 */
    /* 1794 */
    /* 1795 */
    /* 1796 */
    /* 1797 */
    /* 1798 */
    /* 1799 */
    /* 1800 */
    /* 1802 */
    /* 1803 */
    /* 1804 */
    /* 1805 */
    /* 1806 */
    /* 1807 */
    /* 1808 */
    /* 1809 */
    /* 1810 */
    /* 1811 */
    /* 1812 */
    /* 1813 */
    /* 1814 */
    /* 1815 */
    /* 1816 */
    /* 1817 */
    /* 1818 */
    /* 1819 */
    /* 1820 */
    /* 1821 */
    /* 1822 */
    /* 1823 */
    /* 1824 */
    /* 1825 */
    /* 1826 */
    /* 1827 */
    /* 1828 */
    /* 1829 */
    /* 1830 */
    /* 1831 */
    /* 1832 */
    /* 1833 */
    /* 1834 */
    /* 1835 */
    /* 1836 */
    /* 1837 */
    /* 1838 */
    /* 1839 */
    /* 1840 */
    /* 1841 */
    /* 1842 */
    /* 1843 */
    /* 1844 */
    /* 1845 */
    /* 1846 */
    /* 1847 */
    /* 1848 */
    /* 1849 */
    /* 1850 */
    /* 1851 */
    /* 1852 */
    /* 1853 */
    /* 1854 */
    /* 1855 */
    /* 1856 */
    /* 1857 */
    /* 1858 */
    /* 1859 */
    /* 1860 */
    /* 1861 */
    /* 1862 */
    /* 1863 */
    /* 1864 */
    /* 1865 */
    /* 1866 */
    /* 1867 */
    /* 1868 */
    /* 1869 */
    /* 1870 */
    /* 1871 */
    /* 1872 */
    /* 1873 */
    /* 1874 */
    /* 1875 */
    /* 1876 */
    /* 1877 */
    /* 1878 */
    /* 1879 */
    /* 1901 */
    /* 1902 */
    /* 1903 */
    /* 1951 */
    /* 1952 */
    /* 1953 */
    /* 1954 */
    /* 1955 */
    /* 2001 */
    /* 2002 */
    /* 2003 */
    /* 2021 */
    /* 2022 */
    /* 3001 */
    /* 3002 */
    /* 3003 */
    /* 3004 */
    /* 3005 */
    /* 3006 */
    /* 3007 */
    /* 3008 */
    /* 3009 */
    /* 3010 */
    /* 3011 */
    /* 3012 */
    /* 3013 */
    /* 3014 */
    /* 3015 */
    /* 3016 */
    /* 3017 */
    /* 3018 */
    /* 3019 */
    /* 3020 */
    /* 3021 */
    /* 3022 */
    /* 3023 */
    /* 3024 */
    /* 3025 */
    /* 3026 */
    /* 3027 */
    /* 3028 */
    /* 3029 */
    /* 3030 */
    /* 3031 */
    /* 3032 */
    /* 3033 */
    /* 3034 */
    /* 3035 */
    /* 3036 */
    /* 3037 */
    /* 3038 */
    /* 3039 */
    /* 3040 */
    /* 3041 */
    /* 3042 */
    /* 3043 */
    /* 3044 */
    /* 3045 */
    /* 3046 */
    /* 3047 */
    /* 3048 */
    /* 3049 */
    /* 3050 */
    /* 3051 */
    /* 3052 */
    /* 3053 */
    /* 3054 */
    /* 3055 */
    /* 3056 */
    /* 3057 */
    /* 3058 */
    /* 3059 */
    /* 3060 */
    /* 3061 */
    /* 3062 */
    /* 3063 */
    /* 3064 */
    /* 3065 */
    /* 3066 */
    /* 3067 */
    /* 3068 */
    /* 3069 non-W3C */
    /* 3070 non-W3C */
    /* 3071 */
    /* 3072 */
    /* 3073 */
    /* 3074 */
    /* 3075 */
    /* 3076 */
    /* 3077 */
    /* 3078 */
    /* 3079 */
    /* 3080 */
    /* 3081 */
    /* 3082 */
    /* 3083 */
    /* 3084 */
    /* 3085 */
    /* 3085 */
    /* 3086 */
    /* 3087 */
    /* 3088 */
    /* 3089 */
    /* 3090 */
    /* 4000 */
    /* 4900 */
    /* 4901 */
    /* 5001 */
    /* 5002 */
    /* 5003 */
    /* 5004 */
    /* 5005 */
    /* 5006 */
    /* 5007 */
    /* 5008 */
    /* 5009 */
    /* 5010 */
    /* 5011 */
    /* 5012 */
    /* 5013 */
    /* 5014 */
    /* 5015 */
    /* 5016 */
    /* 5017 */
    /* 5018 */
    /* 5019 */
    /* 5020 */
    /* 5021 */
    /* 5022 */
    /* 5023 */
    /* 5024 */
    /* 5025 */
    /* 5026 */
    /* 5027 */
    /* 5028 */
    /* 5029 */
    /* 5030 */
    /* 5031 */
    /* 5032 */
    /* 5033 */
    /* 5034 */
    /* 5035 */
    /* 5036 */
    /* 5037 */
    /* 6001 */
    /* 6002 */
    /* 6003 */
    /* 6004 */
    /* *
 * xmlGenericErrorFunc:
 * @ctx:  a parsing context
 * @msg:  the message
 * @...:  the extra arguments of the varags to format the message
 *
 * Signature of the function to use when there is an error and
 * no parsing or validity context available .
 */
    /* *
 * xmlStructuredErrorFunc:
 * @userData:  user provided data for the error callback
 * @error:  the error being raised.
 *
 * Signature of the function to use when there is an error and
 * the module handles the new error reporting mechanism.
 */
    /*
 * Use the following function to reset the two global variables
 * xmlGenericError and xmlGenericErrorContext.
 */
    /*
 * Default message routines used by SAX and Valid context for error
 * and warning reporting.
 */
    /*
 * Extended error information routines
 */
    /*
 * Internal callback reporting routine
 */
    #[no_mangle]
    fn __xmlRaiseError(schannel: xmlStructuredErrorFunc,
                       channel: xmlGenericErrorFunc, data: *mut std::os::raw::c_void,
                       ctx: *mut std::os::raw::c_void, node: *mut std::os::raw::c_void,
                       domain: std::os::raw::c_int, code: std::os::raw::c_int,
                       level: xmlErrorLevel, file: *const std::os::raw::c_char,
                       line: std::os::raw::c_int, str1: *const std::os::raw::c_char,
                       str2: *const std::os::raw::c_char, str3: *const std::os::raw::c_char,
                       int1: std::os::raw::c_int, col: std::os::raw::c_int,
                       msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufEnd(buf: xmlBufPtr) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufUse(buf: xmlBufPtr) -> size_t;
    #[no_mangle]
    fn xmlBufShrink(buf: xmlBufPtr, len: size_t) -> size_t;
    #[no_mangle]
    fn xmlBufferAdd(buf: xmlBufferPtr, str: *const xmlChar, len: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlGetCharEncodingHandler(enc: xmlCharEncoding)
     -> xmlCharEncodingHandlerPtr;
    #[no_mangle]
    fn xmlFindCharEncodingHandler(name: *const std::os::raw::c_char)
     -> xmlCharEncodingHandlerPtr;
    #[no_mangle]
    fn xmlCharEncCloseFunc(handler: *mut xmlCharEncodingHandler)
     -> std::os::raw::c_int;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn __xmlSimpleError(domain: std::os::raw::c_int, code: std::os::raw::c_int,
                        node: xmlNodePtr, msg: *const std::os::raw::c_char,
                        extra: *const std::os::raw::c_char);
    #[no_mangle]
    fn __xmlDefaultBufferSize() -> *mut std::os::raw::c_int;
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    #[no_mangle]
    static mut xmlMemStrdup: xmlStrdupFunc;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    fn __xmlParserInputBufferCreateFilenameValue()
     -> *mut xmlParserInputBufferCreateFilenameFunc;
    #[no_mangle]
    fn __xmlOutputBufferCreateFilenameValue()
     -> *mut xmlOutputBufferCreateFilenameFunc;
    #[no_mangle]
    fn xmlFreeInputStream(input: xmlParserInputPtr);
    #[no_mangle]
    fn xmlSwitchInputEncoding(ctxt: xmlParserCtxtPtr,
                              input: xmlParserInputPtr,
                              handler: xmlCharEncodingHandlerPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn __xmlErrEncoding(ctxt: xmlParserCtxtPtr, xmlerr: xmlParserErrors,
                        msg: *const std::os::raw::c_char, str1: *const xmlChar,
                        str2: *const xmlChar);
    #[no_mangle]
    fn xmlNewInputFromFile(ctxt: xmlParserCtxtPtr,
                           filename: *const std::os::raw::c_char)
     -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlParseURI(str: *const std::os::raw::c_char) -> xmlURIPtr;
    #[no_mangle]
    fn xmlURIUnescapeString(str: *const std::os::raw::c_char, len: std::os::raw::c_int,
                            target: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn xmlFreeURI(uri: xmlURIPtr);
    #[no_mangle]
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlNanoHTTPMethod(URL: *const std::os::raw::c_char,
                         method: *const std::os::raw::c_char,
                         input: *const std::os::raw::c_char,
                         contentType: *mut *mut std::os::raw::c_char,
                         headers: *const std::os::raw::c_char, ilen: std::os::raw::c_int)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlNanoHTTPOpen(URL: *const std::os::raw::c_char,
                       contentType: *mut *mut std::os::raw::c_char)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlNanoHTTPReturnCode(ctx: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlNanoHTTPRedir(ctx: *mut std::os::raw::c_void) -> *const std::os::raw::c_char;
    #[no_mangle]
    fn xmlNanoHTTPEncoding(ctx: *mut std::os::raw::c_void) -> *const std::os::raw::c_char;
    #[no_mangle]
    fn xmlNanoHTTPMimeType(ctx: *mut std::os::raw::c_void) -> *const std::os::raw::c_char;
    #[no_mangle]
    fn xmlNanoHTTPRead(ctx: *mut std::os::raw::c_void, dest: *mut std::os::raw::c_void,
                       len: std::os::raw::c_int) -> std::os::raw::c_int;
    /* LIBXML_OUTPUT_ENABLED */
    #[no_mangle]
    fn xmlNanoHTTPClose(ctx: *mut std::os::raw::c_void);
    /*
 * Opening/closing session connections.
 */
    #[no_mangle]
    fn xmlNanoFTPOpen(URL: *const std::os::raw::c_char) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlNanoFTPClose(ctx: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlNanoFTPRead(ctx: *mut std::os::raw::c_void, dest: *mut std::os::raw::c_void,
                      len: std::os::raw::c_int) -> std::os::raw::c_int;
    /* LIBXML_OUTPUT_ENABLED */
    #[no_mangle]
    fn xmlCatalogResolve(pubID: *const xmlChar, sysID: *const xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlCatalogResolveURI(URI: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlCatalogLocalResolve(catalogs: *mut std::os::raw::c_void,
                              pubID: *const xmlChar, sysID: *const xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlCatalogLocalResolveURI(catalogs: *mut std::os::raw::c_void,
                                 URI: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlCatalogGetDefaults() -> xmlCatalogAllow;
    /*
 * Summary: Internal Interfaces for memory buffers in libxml2
 * Description: this module describes most of the new xmlBuf buffer
 *              entry points, those are private routines, with a
 *              few exceptions exported in tree.h. This was added
 *              in 2.9.0.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    #[no_mangle]
    fn xmlBufCreate() -> xmlBufPtr;
    #[no_mangle]
    fn xmlBufCreateSize(size: size_t) -> xmlBufPtr;
    #[no_mangle]
    fn xmlBufCreateStatic(mem: *mut std::os::raw::c_void, size: size_t) -> xmlBufPtr;
    #[no_mangle]
    fn xmlBufSetAllocationScheme(buf: xmlBufPtr,
                                 scheme: xmlBufferAllocationScheme)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufGetAllocationScheme(buf: xmlBufPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufFree(buf: xmlBufPtr);
    /* size_t xmlBufShrink(xmlBufPtr buf, size_t len); */
    #[no_mangle]
    fn xmlBufGrow(buf: xmlBufPtr, len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufAdd(buf: xmlBufPtr, str: *const xmlChar, len: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufAvail(buf: xmlBufPtr) -> size_t;
    #[no_mangle]
    fn xmlBufAddLen(buf: xmlBufPtr, len: size_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlCharEncInput(input: xmlParserInputBufferPtr, flush: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlCharEncOutput(output: xmlOutputBufferPtr, init: std::os::raw::c_int)
     -> std::os::raw::c_int;
    /* opaque lzma file descriptor */
    #[no_mangle]
    fn __libxml2_xzopen(path: *const std::os::raw::c_char, mode: *const std::os::raw::c_char)
     -> xzFile;
    #[no_mangle]
    fn __libxml2_xzdopen(fd: std::os::raw::c_int, mode: *const std::os::raw::c_char)
     -> xzFile;
    #[no_mangle]
    fn __libxml2_xzread(file: xzFile, buf: *mut std::os::raw::c_void,
                        len: std::os::raw::c_uint) -> std::os::raw::c_int;
    #[no_mangle]
    fn __libxml2_xzclose(file: xzFile) -> std::os::raw::c_int;
    #[no_mangle]
    fn __libxml2_xzcompressed(f: xzFile) -> std::os::raw::c_int;
}
pub type xmlChar = std::os::raw::c_uchar;
pub type size_t = std::os::raw::c_ulong;
pub type __dev_t = std::os::raw::c_ulong;
pub type __uid_t = std::os::raw::c_uint;
pub type __gid_t = std::os::raw::c_uint;
pub type __ino_t = std::os::raw::c_ulong;
pub type __mode_t = std::os::raw::c_uint;
pub type __nlink_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __time_t = std::os::raw::c_long;
pub type __blksize_t = std::os::raw::c_long;
pub type __blkcnt_t = std::os::raw::c_long;
pub type __ssize_t = std::os::raw::c_long;
pub type __syscall_slong_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut std::os::raw::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::os::raw::c_void,
    pub __pad5: size_t,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut _xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut std::os::raw::c_void,
    pub line: std::os::raw::c_ushort,
    pub extra: std::os::raw::c_ushort,
}
pub type xmlNs = _xmlNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut std::os::raw::c_void,
    pub context: *mut _xmlDoc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *mut std::os::raw::c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: std::os::raw::c_int,
    pub standalone: std::os::raw::c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut std::os::raw::c_void,
    pub refs: *mut std::os::raw::c_void,
    pub URL: *const xmlChar,
    pub charset: std::os::raw::c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut std::os::raw::c_void,
    pub parseFlags: std::os::raw::c_int,
    pub properties: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut std::os::raw::c_void,
    pub elements: *mut std::os::raw::c_void,
    pub attributes: *mut std::os::raw::c_void,
    pub entities: *mut std::os::raw::c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut std::os::raw::c_void,
}
pub type xmlElementType = std::os::raw::c_uint;
pub const XML_DOCB_DOCUMENT_NODE: xmlElementType = 21;
pub const XML_XINCLUDE_END: xmlElementType = 20;
pub const XML_XINCLUDE_START: xmlElementType = 19;
pub const XML_NAMESPACE_DECL: xmlElementType = 18;
pub const XML_ENTITY_DECL: xmlElementType = 17;
pub const XML_ATTRIBUTE_DECL: xmlElementType = 16;
pub const XML_ELEMENT_DECL: xmlElementType = 15;
pub const XML_DTD_NODE: xmlElementType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlElementType = 13;
pub const XML_NOTATION_NODE: xmlElementType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlElementType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlElementType = 10;
pub const XML_DOCUMENT_NODE: xmlElementType = 9;
pub const XML_COMMENT_NODE: xmlElementType = 8;
pub const XML_PI_NODE: xmlElementType = 7;
pub const XML_ENTITY_NODE: xmlElementType = 6;
pub const XML_ENTITY_REF_NODE: xmlElementType = 5;
pub const XML_CDATA_SECTION_NODE: xmlElementType = 4;
pub const XML_TEXT_NODE: xmlElementType = 3;
pub const XML_ATTRIBUTE_NODE: xmlElementType = 2;
pub const XML_ELEMENT_NODE: xmlElementType = 1;
pub type xmlNsType = xmlElementType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlAttr,
    pub prev: *mut _xmlAttr,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut std::os::raw::c_void,
}
pub type xmlAttributeType = std::os::raw::c_uint;
pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
pub const XML_IO_UNKNOWN: xmlParserErrors = 1500;
pub const XML_IO_EAFNOSUPPORT: xmlParserErrors = 1556;
pub const XML_IO_EALREADY: xmlParserErrors = 1555;
pub const XML_IO_EINPROGRESS: xmlParserErrors = 1513;
pub const XML_IO_EADDRINUSE: xmlParserErrors = 1554;
pub const XML_IO_ENETUNREACH: xmlParserErrors = 1553;
pub const XML_IO_ETIMEDOUT: xmlParserErrors = 1541;
pub const XML_IO_ECONNREFUSED: xmlParserErrors = 1552;
pub const XML_IO_EISCONN: xmlParserErrors = 1551;
pub const XML_IO_ENOTSOCK: xmlParserErrors = 1550;
pub const XML_IO_EXDEV: xmlParserErrors = 1542;
pub const XML_IO_ESRCH: xmlParserErrors = 1540;
pub const XML_IO_ESPIPE: xmlParserErrors = 1539;
pub const XML_IO_EROFS: xmlParserErrors = 1538;
pub const XML_IO_ERANGE: xmlParserErrors = 1537;
pub const XML_IO_EPIPE: xmlParserErrors = 1536;
pub const XML_IO_EPERM: xmlParserErrors = 1535;
pub const XML_IO_ENXIO: xmlParserErrors = 1534;
pub const XML_IO_ENOTTY: xmlParserErrors = 1533;
pub const XML_IO_ENOTSUP: xmlParserErrors = 1532;
pub const XML_IO_ENOTEMPTY: xmlParserErrors = 1531;
pub const XML_IO_ENOTDIR: xmlParserErrors = 1530;
pub const XML_IO_ENOSYS: xmlParserErrors = 1529;
pub const XML_IO_ENOSPC: xmlParserErrors = 1528;
pub const XML_IO_ENOMEM: xmlParserErrors = 1527;
pub const XML_IO_ENOLCK: xmlParserErrors = 1526;
pub const XML_IO_ENOEXEC: xmlParserErrors = 1525;
pub const XML_IO_ENOENT: xmlParserErrors = 1524;
pub const XML_IO_ENODEV: xmlParserErrors = 1523;
pub const XML_IO_ENFILE: xmlParserErrors = 1522;
pub const XML_IO_ENAMETOOLONG: xmlParserErrors = 1521;
pub const XML_IO_EMSGSIZE: xmlParserErrors = 1520;
pub const XML_IO_EMLINK: xmlParserErrors = 1519;
pub const XML_IO_EMFILE: xmlParserErrors = 1518;
pub const XML_IO_EISDIR: xmlParserErrors = 1517;
pub const XML_IO_EIO: xmlParserErrors = 1516;
pub const XML_IO_EINVAL: xmlParserErrors = 1515;
pub const XML_IO_EINTR: xmlParserErrors = 1514;
pub const XML_IO_EFBIG: xmlParserErrors = 1512;
pub const XML_IO_EFAULT: xmlParserErrors = 1511;
pub const XML_IO_EEXIST: xmlParserErrors = 1510;
pub const XML_IO_EDOM: xmlParserErrors = 1509;
pub const XML_IO_EDEADLK: xmlParserErrors = 1508;
pub const XML_IO_ECHILD: xmlParserErrors = 1507;
pub const XML_IO_ECANCELED: xmlParserErrors = 1506;
pub const XML_IO_EBUSY: xmlParserErrors = 1505;
pub const XML_IO_EBADMSG: xmlParserErrors = 1504;
pub const XML_IO_EBADF: xmlParserErrors = 1503;
pub const XML_IO_EAGAIN: xmlParserErrors = 1502;
pub const XML_IO_EACCES: xmlParserErrors = 1501;
pub type xmlErrorLevel = std::os::raw::c_uint;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
pub const XML_IO_LOAD_ERROR: xmlParserErrors = 1549;
pub const XML_FROM_IO: C2RustUnnamed_0 = 8;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlParserCtxt = _xmlParserCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserCtxt {
    pub sax: *mut _xmlSAXHandler,
    pub userData: *mut std::os::raw::c_void,
    pub myDoc: xmlDocPtr,
    pub wellFormed: std::os::raw::c_int,
    pub replaceEntities: std::os::raw::c_int,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub standalone: std::os::raw::c_int,
    pub html: std::os::raw::c_int,
    pub input: xmlParserInputPtr,
    pub inputNr: std::os::raw::c_int,
    pub inputMax: std::os::raw::c_int,
    pub inputTab: *mut xmlParserInputPtr,
    pub node: xmlNodePtr,
    pub nodeNr: std::os::raw::c_int,
    pub nodeMax: std::os::raw::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub record_info: std::os::raw::c_int,
    pub node_seq: xmlParserNodeInfoSeq,
    pub errNo: std::os::raw::c_int,
    pub hasExternalSubset: std::os::raw::c_int,
    pub hasPErefs: std::os::raw::c_int,
    pub external: std::os::raw::c_int,
    pub valid: std::os::raw::c_int,
    pub validate: std::os::raw::c_int,
    pub vctxt: xmlValidCtxt,
    pub instate: xmlParserInputState,
    pub token: std::os::raw::c_int,
    pub directory: *mut std::os::raw::c_char,
    pub name: *const xmlChar,
    pub nameNr: std::os::raw::c_int,
    pub nameMax: std::os::raw::c_int,
    pub nameTab: *mut *const xmlChar,
    pub nbChars: std::os::raw::c_long,
    pub checkIndex: std::os::raw::c_long,
    pub keepBlanks: std::os::raw::c_int,
    pub disableSAX: std::os::raw::c_int,
    pub inSubset: std::os::raw::c_int,
    pub intSubName: *const xmlChar,
    pub extSubURI: *mut xmlChar,
    pub extSubSystem: *mut xmlChar,
    pub space: *mut std::os::raw::c_int,
    pub spaceNr: std::os::raw::c_int,
    pub spaceMax: std::os::raw::c_int,
    pub spaceTab: *mut std::os::raw::c_int,
    pub depth: std::os::raw::c_int,
    pub entity: xmlParserInputPtr,
    pub charset: std::os::raw::c_int,
    pub nodelen: std::os::raw::c_int,
    pub nodemem: std::os::raw::c_int,
    pub pedantic: std::os::raw::c_int,
    pub _private: *mut std::os::raw::c_void,
    pub loadsubset: std::os::raw::c_int,
    pub linenumbers: std::os::raw::c_int,
    pub catalogs: *mut std::os::raw::c_void,
    pub recovery: std::os::raw::c_int,
    pub progressive: std::os::raw::c_int,
    pub dict: xmlDictPtr,
    pub atts: *mut *const xmlChar,
    pub maxatts: std::os::raw::c_int,
    pub docdict: std::os::raw::c_int,
    pub str_xml: *const xmlChar,
    pub str_xmlns: *const xmlChar,
    pub str_xml_ns: *const xmlChar,
    pub sax2: std::os::raw::c_int,
    pub nsNr: std::os::raw::c_int,
    pub nsMax: std::os::raw::c_int,
    pub nsTab: *mut *const xmlChar,
    pub attallocs: *mut std::os::raw::c_int,
    pub pushTab: *mut *mut std::os::raw::c_void,
    pub attsDefault: xmlHashTablePtr,
    pub attsSpecial: xmlHashTablePtr,
    pub nsWellFormed: std::os::raw::c_int,
    pub options: std::os::raw::c_int,
    pub dictNames: std::os::raw::c_int,
    pub freeElemsNr: std::os::raw::c_int,
    pub freeElems: xmlNodePtr,
    pub freeAttrsNr: std::os::raw::c_int,
    pub freeAttrs: xmlAttrPtr,
    pub lastError: xmlError,
    pub parseMode: xmlParserMode,
    pub nbentities: std::os::raw::c_ulong,
    pub sizeentities: std::os::raw::c_ulong,
    pub nodeInfo: *mut xmlParserNodeInfo,
    pub nodeInfoNr: std::os::raw::c_int,
    pub nodeInfoMax: std::os::raw::c_int,
    pub nodeInfoTab: *mut xmlParserNodeInfo,
    pub input_id: std::os::raw::c_int,
    pub sizeentcopy: std::os::raw::c_ulong,
}
pub type xmlParserNodeInfo = _xmlParserNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfo {
    pub node: *const _xmlNode,
    pub begin_pos: std::os::raw::c_ulong,
    pub begin_line: std::os::raw::c_ulong,
    pub end_pos: std::os::raw::c_ulong,
    pub end_line: std::os::raw::c_ulong,
}
pub type xmlParserMode = std::os::raw::c_uint;
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
pub type xmlError = _xmlError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlError {
    pub domain: std::os::raw::c_int,
    pub code: std::os::raw::c_int,
    pub message: *mut std::os::raw::c_char,
    pub level: xmlErrorLevel,
    pub file: *mut std::os::raw::c_char,
    pub line: std::os::raw::c_int,
    pub str1: *mut std::os::raw::c_char,
    pub str2: *mut std::os::raw::c_char,
    pub str3: *mut std::os::raw::c_char,
    pub int1: std::os::raw::c_int,
    pub int2: std::os::raw::c_int,
    pub ctxt: *mut std::os::raw::c_void,
    pub node: *mut std::os::raw::c_void,
}
pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlDictPtr = *mut xmlDict;
/*
 * Summary: string dictionary
 * Description: dictionary of reusable strings, just used to avoid allocation
 *         and freeing operations.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * The dictionary.
 */
pub type xmlDict = _xmlDict;
pub type xmlParserInputPtr = *mut xmlParserInput;
pub type xmlParserInput = _xmlParserInput;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInput {
    pub buf: xmlParserInputBufferPtr,
    pub filename: *const std::os::raw::c_char,
    pub directory: *const std::os::raw::c_char,
    pub base: *const xmlChar,
    pub cur: *const xmlChar,
    pub end: *const xmlChar,
    pub length: std::os::raw::c_int,
    pub line: std::os::raw::c_int,
    pub col: std::os::raw::c_int,
    pub consumed: std::os::raw::c_ulong,
    pub free: xmlParserInputDeallocate,
    pub encoding: *const xmlChar,
    pub version: *const xmlChar,
    pub standalone: std::os::raw::c_int,
    pub id: std::os::raw::c_int,
}
pub type xmlParserInputDeallocate
    =
    Option<unsafe extern "C" fn(_: *mut xmlChar) -> ()>;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInputBuffer {
    pub context: *mut std::os::raw::c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub raw: xmlBufPtr,
    pub compressed: std::os::raw::c_int,
    pub error: std::os::raw::c_int,
    pub rawconsumed: std::os::raw::c_ulong,
}
pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
/*
 * Block defining the handlers for non UTF-8 encodings.
 * If iconv is supported, there are two extra fields.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingHandler {
    pub name: *mut std::os::raw::c_char,
    pub input: xmlCharEncodingInputFunc,
    pub output: xmlCharEncodingOutputFunc,
    pub iconv_in: iconv_t,
    pub iconv_out: iconv_t,
}
pub type iconv_t = *mut std::os::raw::c_void;
/* *
 * xmlCharEncodingOutputFunc:
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @in:  a pointer to an array of UTF-8 chars
 * @inlen:  the length of @in
 *
 * Take a block of UTF-8 chars in and try to convert it to another
 * encoding.
 * Note: a first call designed to produce heading info is called with
 * in = NULL. If stateful this should also initialize the encoder state.
 *
 * Returns the number of bytes written, -1 if lack of space, or -2
 *     if the transcoding failed.
 * The value of @inlen after return is the number of octets consumed
 *     if the return value is positive, else unpredictiable.
 * The value of @outlen after return is the number of octets produced.
 */
pub type xmlCharEncodingOutputFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_uchar, _: *mut std::os::raw::c_int,
                                _: *const std::os::raw::c_uchar, _: *mut std::os::raw::c_int)
               -> std::os::raw::c_int>;
/*
 * Summary: interface for the encoding conversion functions
 * Description: interface for the encoding conversion functions needed for
 *              XML basic encoding and iconv() support.
 *
 * Related specs are
 * rfc2044        (UTF-8 and UTF-16) F. Yergeau Alis Technologies
 * [ISO-10646]    UTF-8 and UTF-16 in Annexes
 * [ISO-8859-1]   ISO Latin-1 characters codes.
 * [UNICODE]      The Unicode Consortium, "The Unicode Standard --
 *                Worldwide Character Encoding -- Version 1.0", Addison-
 *                Wesley, Volume 1, 1991, Volume 2, 1992.  UTF-8 is
 *                described in Unicode Technical Report #4.
 * [US-ASCII]     Coded Character Set--7-bit American Standard Code for
 *                Information Interchange, ANSI X3.4-1986.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * xmlCharEncoding:
 *
 * Predefined values for some standard encodings.
 * Libxml does not do beforehand translation on UTF8 and ISOLatinX.
 * It also supports ASCII, ISO-8859-1, and UTF16 (LE and BE) by default.
 *
 * Anything else would have to be translated to UTF8 before being
 * given to the parser itself. The BOM for UTF16 and the encoding
 * declaration are looked at and a converter is looked for at that
 * point. If not found the parser stops here as asked by the XML REC. A
 * converter can be registered by the user using xmlRegisterCharEncodingHandler
 * but the current form doesn't allow stateful transcoding (a serious
 * problem agreed !). If iconv has been found it will be used
 * automatically and allow stateful transcoding, the simplest is then
 * to be sure to enable iconv and to provide iconv libs for the encoding
 * support needed.
 *
 * Note that the generic "UTF-16" is not a predefined value.  Instead, only
 * the specific UTF-16LE and UTF-16BE are present.
 */
/* No char encoding detected */
/* No char encoding detected */
/* UTF-8 */
/* UTF-16 little endian */
/* UTF-16 big endian */
/* UCS-4 little endian */
/* UCS-4 big endian */
/* EBCDIC uh! */
/* UCS-4 unusual ordering */
/* UCS-4 unusual ordering */
/* UCS-2 */
/* ISO-8859-1 ISO Latin 1 */
/* ISO-8859-2 ISO Latin 2 */
/* ISO-8859-3 */
/* ISO-8859-4 */
/* ISO-8859-5 */
/* ISO-8859-6 */
/* ISO-8859-7 */
/* ISO-8859-8 */
/* ISO-8859-9 */
/* ISO-2022-JP */
/* Shift_JIS */
/* EUC-JP */
/* pure ASCII */
/* *
 * xmlCharEncodingInputFunc:
 * @out:  a pointer to an array of bytes to store the UTF-8 result
 * @outlen:  the length of @out
 * @in:  a pointer to an array of chars in the original encoding
 * @inlen:  the length of @in
 *
 * Take a block of chars in the original encoding and try to convert
 * it to an UTF-8 block of chars out.
 *
 * Returns the number of bytes written, -1 if lack of space, or -2
 *     if the transcoding failed.
 * The value of @inlen after return is the number of octets consumed
 *     if the return value is positive, else unpredictiable.
 * The value of @outlen after return is the number of octets consumed.
 */
pub type xmlCharEncodingInputFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_uchar, _: *mut std::os::raw::c_int,
                                _: *const std::os::raw::c_uchar, _: *mut std::os::raw::c_int)
               -> std::os::raw::c_int>;
pub type xmlInputCloseCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type xmlInputReadCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *mut std::os::raw::c_char,
                                _: std::os::raw::c_int) -> std::os::raw::c_int>;
pub type xmlParserInputState = std::os::raw::c_int;
pub const XML_PARSER_PUBLIC_LITERAL: xmlParserInputState = 16;
pub const XML_PARSER_IGNORE: xmlParserInputState = 15;
pub const XML_PARSER_EPILOG: xmlParserInputState = 14;
pub const XML_PARSER_SYSTEM_LITERAL: xmlParserInputState = 13;
pub const XML_PARSER_ATTRIBUTE_VALUE: xmlParserInputState = 12;
pub const XML_PARSER_ENTITY_VALUE: xmlParserInputState = 11;
pub const XML_PARSER_ENTITY_DECL: xmlParserInputState = 10;
pub const XML_PARSER_END_TAG: xmlParserInputState = 9;
pub const XML_PARSER_CDATA_SECTION: xmlParserInputState = 8;
pub const XML_PARSER_CONTENT: xmlParserInputState = 7;
pub const XML_PARSER_START_TAG: xmlParserInputState = 6;
pub const XML_PARSER_COMMENT: xmlParserInputState = 5;
pub const XML_PARSER_PROLOG: xmlParserInputState = 4;
pub const XML_PARSER_DTD: xmlParserInputState = 3;
pub const XML_PARSER_PI: xmlParserInputState = 2;
pub const XML_PARSER_MISC: xmlParserInputState = 1;
pub const XML_PARSER_START: xmlParserInputState = 0;
pub const XML_PARSER_EOF: xmlParserInputState = -1;
pub type xmlValidCtxt = _xmlValidCtxt;
/* *
 * XML_CTXT_FINISH_DTD_0:
 *
 * Special value for finishDtd field when embedded in an xmlParserCtxt
 */
/* *
 * XML_CTXT_FINISH_DTD_1:
 *
 * Special value for finishDtd field when embedded in an xmlParserCtxt
 */
/*
 * xmlValidCtxt:
 * An xmlValidCtxt is used for error reporting when validating.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlValidCtxt {
    pub userData: *mut std::os::raw::c_void,
    pub error: xmlValidityErrorFunc,
    pub warning: xmlValidityWarningFunc,
    pub node: xmlNodePtr,
    pub nodeNr: std::os::raw::c_int,
    pub nodeMax: std::os::raw::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub finishDtd: std::os::raw::c_uint,
    pub doc: xmlDocPtr,
    pub valid: std::os::raw::c_int,
    pub vstate: *mut xmlValidState,
    pub vstateNr: std::os::raw::c_int,
    pub vstateMax: std::os::raw::c_int,
    pub vstateTab: *mut xmlValidState,
    pub am: xmlAutomataPtr,
    pub state: xmlAutomataStatePtr,
}
pub type xmlAutomataStatePtr = *mut xmlAutomataState;
pub type xmlAutomataState = _xmlAutomataState;
pub type xmlAutomataPtr = *mut xmlAutomata;
pub type xmlAutomata = _xmlAutomata;
pub type xmlValidState = _xmlValidState;
pub type xmlDocPtr = *mut xmlDoc;
/* Namespace prefix */
/* *
 * xmlElementTypeVal:
 *
 * The different possibilities for an element content type.
 */
/* *
 * xmlElement:
 *
 * An XML Element declaration from a DTD.
 */
/* application data */
/* XML_ELEMENT_DECL, must be second ! */
/* Element name */
/* NULL */
/* NULL */
/* -> DTD */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* The type */
/* the allowed element content */
/* List of the declared attributes */
/* the namespace prefix if any */
/* the validating regexp */
/* *
 * XML_LOCAL_NAMESPACE:
 *
 * A namespace declaration node.
 */
/* *
 * xmlNs:
 *
 * An XML namespace.
 * Note that prefix == NULL is valid, it defines the default namespace
 * within the subtree (until overridden).
 *
 * xmlNsType is unified with xmlElementType.
 */
/* next Ns link for this node  */
/* global or local */
/* URL for the namespace */
/* prefix for the namespace */
/* application data */
/* normally an xmlDoc */
/* *
 * xmlDtd:
 *
 * An XML DTD, as defined by <!DOCTYPE ... There is actually one for
 * the internal subset and for the external subset.
 */
/* application data */
/* XML_DTD_NODE, must be second ! */
/* Name of the DTD */
/* the value of the property link */
/* last child link */
/* child->parent link */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* End of common part */
/* Hash table for notations if any */
/* Hash table for elements if any */
/* Hash table for attributes if any */
/* Hash table for entities if any */
/* External identifier for PUBLIC DTD */
/* URI for a SYSTEM or PUBLIC DTD */
/* Hash table for param entities if any */
/* *
 * xmlAttr:
 *
 * An attribute on an XML node.
 */
/* application data */
/* XML_ATTRIBUTE_NODE, must be second ! */
/* the name of the property */
/* the value of the property */
/* NULL */
/* child->parent link */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* pointer to the associated namespace */
/* the attribute type if validating */
/* for type/PSVI informations */
/* *
 * xmlID:
 *
 * An XML ID instance.
 */
/* next ID */
/* The ID name */
/* The attribute holding it */
/* The attribute if attr is not available */
/* The line number if attr is not available */
/* The document holding the ID */
/* *
 * xmlRef:
 *
 * An XML IDREF instance.
 */
/* next Ref */
/* The Ref name */
/* The attribute holding it */
/* The attribute if attr is not available */
/* The line number if attr is not available */
/* *
 * xmlNode:
 *
 * A node in an XML tree.
 */
/* application data */
/* type number, must be second ! */
/* the name of the node, or the entity */
/* parent->childs link */
/* last child link */
/* child->parent link */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* End of common part */
/* pointer to the associated namespace */
/* the content */
/* properties list */
/* namespace definitions on this node */
/* for type/PSVI informations */
/* line number */
/* extra data for XPath/XSLT */
/* *
 * XML_GET_CONTENT:
 *
 * Macro to extract the content pointer of a node.
 */
/* *
 * XML_GET_LINE:
 *
 * Macro to extract the line number of an element node.
 */
/* *
 * xmlDocProperty
 *
 * Set of properties of the document as found by the parser
 * Some of them are linked to similary named xmlParserOption
 */
/* document is XML well formed */
/* document is Namespace valid */
/* parsed with old XML-1.0 parser */
/* DTD validation was successful */
/* XInclude substitution was done */
/* Document was built using the API
                                           and not by parsing an instance */
/* built for internal processing */
/* parsed or built HTML document */
/* *
 * xmlDoc:
 *
 * An XML document.
 */
pub type xmlDoc = _xmlDoc;
/* *
 * xmlValidityWarningFunc:
 * @ctx:  usually an xmlValidCtxtPtr to a validity error context,
 *        but comes from ctxt->userData (which normally contains such
 *        a pointer); ctxt->userData can be changed by the user.
 * @msg:  the string to format *printf like vararg
 * @...:  remaining arguments to the format
 *
 * Callback called when a validity warning is found. This is a message
 * oriented function similar to an *printf function.
 */
pub type xmlValidityWarningFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/*
 * Summary: The DTD validation
 * Description: API for the DTD handling and the validity checking
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * Validation state added for non-determinist content model.
 */
/* *
 * xmlValidityErrorFunc:
 * @ctx:  usually an xmlValidCtxtPtr to a validity error context,
 *        but comes from ctxt->userData (which normally contains such
 *        a pointer); ctxt->userData can be changed by the user.
 * @msg:  the string to format *printf like vararg
 * @...:  remaining arguments to the format
 *
 * Callback called when a validity error is found. This is a message
 * oriented function similar to an *printf function.
 */
pub type xmlValidityErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlParserNodeInfoSeq = _xmlParserNodeInfoSeq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfoSeq {
    pub maximum: std::os::raw::c_ulong,
    pub length: std::os::raw::c_ulong,
    pub buffer: *mut xmlParserNodeInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXHandler {
    pub internalSubset: internalSubsetSAXFunc,
    pub isStandalone: isStandaloneSAXFunc,
    pub hasInternalSubset: hasInternalSubsetSAXFunc,
    pub hasExternalSubset: hasExternalSubsetSAXFunc,
    pub resolveEntity: resolveEntitySAXFunc,
    pub getEntity: getEntitySAXFunc,
    pub entityDecl: entityDeclSAXFunc,
    pub notationDecl: notationDeclSAXFunc,
    pub attributeDecl: attributeDeclSAXFunc,
    pub elementDecl: elementDeclSAXFunc,
    pub unparsedEntityDecl: unparsedEntityDeclSAXFunc,
    pub setDocumentLocator: setDocumentLocatorSAXFunc,
    pub startDocument: startDocumentSAXFunc,
    pub endDocument: endDocumentSAXFunc,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub reference: referenceSAXFunc,
    pub characters: charactersSAXFunc,
    pub ignorableWhitespace: ignorableWhitespaceSAXFunc,
    pub processingInstruction: processingInstructionSAXFunc,
    pub comment: commentSAXFunc,
    pub warning: warningSAXFunc,
    pub error: errorSAXFunc,
    pub fatalError: fatalErrorSAXFunc,
    pub getParameterEntity: getParameterEntitySAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub externalSubset: externalSubsetSAXFunc,
    pub initialized: std::os::raw::c_uint,
    pub _private: *mut std::os::raw::c_void,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub serror: xmlStructuredErrorFunc,
}
pub type xmlStructuredErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: xmlErrorPtr) -> ()>;
pub type xmlErrorPtr = *mut xmlError;
/* *
 * endElementNsSAX2Func:
 * @ctx:  the user data (XML parser context)
 * @localname:  the local name of the element
 * @prefix:  the element namespace prefix if available
 * @URI:  the element namespace name if available
 *
 * SAX2 callback when an element end has been detected by the parser.
 * It provides the namespace informations for the element.
 */
pub type endElementNsSAX2Func
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
/* *
 * startElementNsSAX2Func:
 * @ctx:  the user data (XML parser context)
 * @localname:  the local name of the element
 * @prefix:  the element namespace prefix if available
 * @URI:  the element namespace name if available
 * @nb_namespaces:  number of namespace definitions on that node
 * @namespaces:  pointer to the array of prefix/URI pairs namespace definitions
 * @nb_attributes:  the number of attributes on that node
 * @nb_defaulted:  the number of defaulted attributes. The defaulted
 *                  ones are at the end of the array
 * @attributes:  pointer to the array of (localname/prefix/URI/value/end)
 *               attribute values.
 *
 * SAX2 callback when an element start has been detected by the parser.
 * It provides the namespace informations for the element, as well as
 * the new namespace declarations on the element.
 */
pub type startElementNsSAX2Func
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar,
                                _: std::os::raw::c_int, _: *mut *const xmlChar,
                                _: std::os::raw::c_int, _: std::os::raw::c_int,
                                _: *mut *const xmlChar) -> ()>;
pub type externalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type cdataBlockSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int) -> ()>;
pub type getParameterEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> xmlEntityPtr>;
pub type xmlEntityPtr = *mut xmlEntity;
pub type xmlEntity = _xmlEntity;
/*
 * An unit of storage for an entity, contains the string, the value
 * and the linkind data needed for the linking in the hash table.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEntity {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub orig: *mut xmlChar,
    pub content: *mut xmlChar,
    pub length: std::os::raw::c_int,
    pub etype: xmlEntityType,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub nexte: *mut _xmlEntity,
    pub URI: *const xmlChar,
    pub owner: std::os::raw::c_int,
    pub checked: std::os::raw::c_int,
}
/*
 * Summary: interface for the XML entities handling
 * Description: this module provides some of the entity API needed
 *              for the parser and applications.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * The different valid entity types.
 */
pub type xmlEntityType = std::os::raw::c_uint;
pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
pub type fatalErrorSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type errorSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type warningSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type commentSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
pub type processingInstructionSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> ()>;
pub type ignorableWhitespaceSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int) -> ()>;
pub type charactersSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int) -> ()>;
pub type referenceSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
pub type endElementSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
pub type startElementSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *mut *const xmlChar) -> ()>;
pub type endDocumentSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
pub type startDocumentSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
pub type setDocumentLocatorSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: xmlSAXLocatorPtr)
               -> ()>;
pub type xmlSAXLocatorPtr = *mut xmlSAXLocator;
pub type xmlSAXLocator = _xmlSAXLocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXLocator {
    pub getPublicId: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                -> *const xmlChar>,
    pub getSystemId: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                -> *const xmlChar>,
    pub getLineNumber: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                  -> std::os::raw::c_int>,
    pub getColumnNumber: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                    -> std::os::raw::c_int>,
}
pub type unparsedEntityDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar,
                                _: *const xmlChar) -> ()>;
pub type elementDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int, _: xmlElementContentPtr)
               -> ()>;
pub type xmlElementContentPtr = *mut xmlElementContent;
pub type xmlElementContent = _xmlElementContent;
/* *
 * xmlElementContent:
 *
 * An XML Element content as stored after parsing an element definition
 * in a DTD.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElementContent {
    pub type_0: xmlElementContentType,
    pub ocur: xmlElementContentOccur,
    pub name: *const xmlChar,
    pub c1: *mut _xmlElementContent,
    pub c2: *mut _xmlElementContent,
    pub parent: *mut _xmlElementContent,
    pub prefix: *const xmlChar,
}
/* *
 * xmlElementContentOccur:
 *
 * Possible definitions of element content occurrences.
 */
pub type xmlElementContentOccur = std::os::raw::c_uint;
pub const XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
pub const XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
pub const XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
pub const XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
/* Enumeration name */
/* *
 * xmlAttribute:
 *
 * An Attribute declaration in a DTD.
 */
/* application data */
/* XML_ATTRIBUTE_DECL, must be second ! */
/* Attribute name */
/* NULL */
/* NULL */
/* -> DTD */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* next in hash table */
/* The attribute type */
/* the default */
/* or the default value */
/* or the enumeration tree if any */
/* the namespace prefix if any */
/* Element holding the attribute */
/* *
 * xmlElementContentType:
 *
 * Possible definitions of element content types.
 */
pub type xmlElementContentType = std::os::raw::c_uint;
pub const XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
pub const XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
pub const XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
pub const XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
pub type attributeDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: std::os::raw::c_int,
                                _: std::os::raw::c_int, _: *const xmlChar,
                                _: xmlEnumerationPtr) -> ()>;
pub type xmlEnumerationPtr = *mut xmlEnumeration;
pub type xmlEnumeration = _xmlEnumeration;
/*
 * Summary: interfaces for tree manipulation
 * Description: this module describes the structures found in an tree resulting
 *              from an XML or HTML parsing, as well as the API provided for
 *              various processing on that tree
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * Some of the basic types pointer to structures:
 */
/* xmlIO.h */
/* parser.h */
/* entities.h */
/* *
 * BASE_BUFFER_SIZE:
 *
 * default buffer size 4000.
 */
/* *
 * LIBXML_NAMESPACE_DICT:
 *
 * Defines experimental behaviour:
 * 1) xmlNs gets an additional field @context (a xmlDoc)
 * 2) when creating a tree, xmlNs->href is stored in the dict of xmlDoc.
 */
/* #define LIBXML_NAMESPACE_DICT */
/* *
 * xmlBufferAllocationScheme:
 *
 * A buffer allocation scheme can be defined to either match exactly the
 * need or double it's allocated size each time it is found too small.
 */
/* double each time one need to grow */
/* grow only to the minimal size */
/* immutable buffer */
/* special allocation scheme used for I/O */
/* exact up to a threshold, and doubleit thereafter */
/* limit the upper size of the buffer */
/* *
 * xmlBuffer:
 *
 * A buffer structure, this old construct is limited to 2GB and
 * is being deprecated, use API with xmlBuf instead
 */
/* The buffer content UTF8 */
/* The buffer size used */
/* The buffer size */
/* The realloc method */
/* in IO mode we may have a different base */
/* *
 * xmlBuf:
 *
 * A buffer structure, new one, the actual structure internals are not public
 */
/* *
 * xmlBufPtr:
 *
 * A pointer to a buffer structure, the actual structure internals are not
 * public
 */
/*
 * A few public routines for xmlBuf. As those are expected to be used
 * mostly internally the bulk of the routines are internal in buf.h
 */
/*
 * LIBXML2_NEW_BUFFER:
 *
 * Macro used to express that the API use the new buffers for
 * xmlParserInputBuffer and xmlOutputBuffer. The change was
 * introduced in 2.9.0.
 */
/* *
 * XML_XML_NAMESPACE:
 *
 * This is the namespace for the special xml: prefix predefined in the
 * XML Namespace specification.
 */
/* *
 * XML_XML_ID:
 *
 * This is the name for the special xml:id attribute
 */
/*
 * The different element types carried by an XML tree.
 *
 * NOTE: This is synchronized with DOM Level1 values
 *       See http://www.w3.org/TR/REC-DOM-Level-1/
 *
 * Actually this had diverged a bit, and now XML_DOCUMENT_TYPE_NODE should
 * be deprecated to use an XML_DTD_NODE.
 */
/* *
 * xmlNotation:
 *
 * A DTD Notation definition.
 */
/* Notation name */
/* Public identifier, if any */
/* System identifier, if any */
/* *
 * xmlAttributeType:
 *
 * A DTD Attribute type definition.
 */
/* *
 * xmlAttributeDefault:
 *
 * A DTD Attribute default definition.
 */
/* *
 * xmlEnumeration:
 *
 * List structure used when there is an enumeration in DTDs.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEnumeration {
    pub next: *mut _xmlEnumeration,
    pub name: *const xmlChar,
}
pub type notationDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type entityDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int, _: *const xmlChar,
                                _: *const xmlChar, _: *mut xmlChar) -> ()>;
pub type getEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> xmlEntityPtr>;
pub type resolveEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlParserInputPtr>;
pub type hasExternalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type hasInternalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type isStandaloneSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type internalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type ptrdiff_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: std::os::raw::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type Byte = std::os::raw::c_uchar;
pub type uInt = std::os::raw::c_uint;
pub type uLong = std::os::raw::c_ulong;
pub type Bytef = Byte;
pub type voidpc = *const std::os::raw::c_void;
pub type voidpf = *mut std::os::raw::c_void;
pub type voidp = *mut std::os::raw::c_void;
pub type alloc_func
    =
    Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut std::os::raw::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: std::os::raw::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: std::os::raw::c_uint,
    pub next: *mut std::os::raw::c_uchar,
    pub pos: off_t,
}
pub type gzFile = *mut gzFile_s;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const LZMA_PROG_ERROR: C2RustUnnamed = 11;
pub const LZMA_BUF_ERROR: C2RustUnnamed = 10;
pub const LZMA_DATA_ERROR: C2RustUnnamed = 9;
pub const LZMA_OPTIONS_ERROR: C2RustUnnamed = 8;
pub const LZMA_FORMAT_ERROR: C2RustUnnamed = 7;
pub const LZMA_MEMLIMIT_ERROR: C2RustUnnamed = 6;
pub const LZMA_MEM_ERROR: C2RustUnnamed = 5;
pub const LZMA_GET_CHECK: C2RustUnnamed = 4;
pub const LZMA_UNSUPPORTED_CHECK: C2RustUnnamed = 3;
pub const LZMA_NO_CHECK: C2RustUnnamed = 2;
pub const LZMA_STREAM_END: C2RustUnnamed = 1;
pub const LZMA_OK: C2RustUnnamed = 0;
/*
 * Summary: interface for the memory allocator
 * Description: provides interfaces for the memory allocator,
 *              including debugging capabilities.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * DEBUG_MEMORY:
 *
 * DEBUG_MEMORY replaces the allocator with a collect and debug
 * shell to the libc allocator.
 * DEBUG_MEMORY should only be activated when debugging
 * libxml i.e. if libxml has been configured with --with-debug-mem too.
 */
/* #define DEBUG_MEMORY_FREED */
/* #define DEBUG_MEMORY_LOCATION */
/* *
 * DEBUG_MEMORY_LOCATION:
 *
 * DEBUG_MEMORY_LOCATION should be activated only when debugging
 * libxml i.e. if libxml has been configured with --with-debug-mem too.
 */
/*
 * The XML memory wrapper support 4 basic overloadable functions.
 */
/* *
 * xmlFreeFunc:
 * @mem: an already allocated block of memory
 *
 * Signature for a free() implementation.
 */
pub type xmlFreeFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
/* *
 * xmlMallocFunc:
 * @size:  the size requested in bytes
 *
 * Signature for a malloc() implementation.
 *
 * Returns a pointer to the newly allocated block or NULL in case of error.
 */
pub type xmlMallocFunc
    =
    Option<unsafe extern "C" fn(_: size_t) -> *mut std::os::raw::c_void>;
/* *
 * xmlReallocFunc:
 * @mem: an already allocated block of memory
 * @size:  the new size requested in bytes
 *
 * Signature for a realloc() implementation.
 *
 * Returns a pointer to the newly reallocated block or NULL in case of error.
 */
pub type xmlReallocFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: size_t)
               -> *mut std::os::raw::c_void>;
/* *
 * xmlStrdupFunc:
 * @str: a zero terminated string
 *
 * Signature for an strdup() implementation.
 *
 * Returns the copy of the string or NULL in case of error.
 */
pub type xmlStrdupFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlOutputBuffer {
    pub context: *mut std::os::raw::c_void,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub conv: xmlBufPtr,
    pub written: std::os::raw::c_int,
    pub error: std::os::raw::c_int,
}
pub type xmlOutputCloseCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type xmlOutputWriteCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: std::os::raw::c_int) -> std::os::raw::c_int>;
pub type xmlOutputBuffer = _xmlOutputBuffer;
pub type xmlOutputBufferPtr = *mut xmlOutputBuffer;
pub type xmlBufferAllocationScheme = std::os::raw::c_uint;
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlBuffer {
    pub content: *mut xmlChar,
    pub use_0: std::os::raw::c_uint,
    pub size: std::os::raw::c_uint,
    pub alloc: xmlBufferAllocationScheme,
    pub contentIO: *mut xmlChar,
}
pub type xmlBuffer = _xmlBuffer;
pub type xmlBufferPtr = *mut xmlBuffer;
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const XML_FROM_URI: C2RustUnnamed_0 = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed_0 = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed_0 = 28;
pub const XML_FROM_I18N: C2RustUnnamed_0 = 27;
pub const XML_FROM_MODULE: C2RustUnnamed_0 = 26;
pub const XML_FROM_WRITER: C2RustUnnamed_0 = 25;
pub const XML_FROM_CHECK: C2RustUnnamed_0 = 24;
pub const XML_FROM_VALID: C2RustUnnamed_0 = 23;
pub const XML_FROM_XSLT: C2RustUnnamed_0 = 22;
pub const XML_FROM_C14N: C2RustUnnamed_0 = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed_0 = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed_0 = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed_0 = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed_0 = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed_0 = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed_0 = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed_0 = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed_0 = 13;
pub const XML_FROM_XPATH: C2RustUnnamed_0 = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed_0 = 11;
pub const XML_FROM_HTTP: C2RustUnnamed_0 = 10;
pub const XML_FROM_FTP: C2RustUnnamed_0 = 9;
pub const XML_FROM_OUTPUT: C2RustUnnamed_0 = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed_0 = 6;
pub const XML_FROM_HTML: C2RustUnnamed_0 = 5;
pub const XML_FROM_DTD: C2RustUnnamed_0 = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed_0 = 3;
pub const XML_FROM_TREE: C2RustUnnamed_0 = 2;
pub const XML_FROM_PARSER: C2RustUnnamed_0 = 1;
pub const XML_FROM_NONE: C2RustUnnamed_0 = 0;
pub type xmlParserErrors = std::os::raw::c_uint;
pub const XML_BUF_OVERFLOW: xmlParserErrors = 7000;
pub const XML_I18N_NO_OUTPUT: xmlParserErrors = 6004;
pub const XML_I18N_CONV_FAILED: xmlParserErrors = 6003;
pub const XML_I18N_EXCESS_HANDLER: xmlParserErrors = 6002;
pub const XML_I18N_NO_HANDLER: xmlParserErrors = 6001;
pub const XML_I18N_NO_NAME: xmlParserErrors = 6000;
pub const XML_CHECK_NAME_NOT_NULL: xmlParserErrors = 5037;
pub const XML_CHECK_WRONG_NAME: xmlParserErrors = 5036;
pub const XML_CHECK_OUTSIDE_DICT: xmlParserErrors = 5035;
pub const XML_CHECK_NOT_NCNAME: xmlParserErrors = 5034;
pub const XML_CHECK_NO_DICT: xmlParserErrors = 5033;
pub const XML_CHECK_NOT_UTF8: xmlParserErrors = 5032;
pub const XML_CHECK_NS_ANCESTOR: xmlParserErrors = 5031;
pub const XML_CHECK_NS_SCOPE: xmlParserErrors = 5030;
pub const XML_CHECK_WRONG_PARENT: xmlParserErrors = 5029;
pub const XML_CHECK_NO_HREF: xmlParserErrors = 5028;
pub const XML_CHECK_NOT_NS_DECL: xmlParserErrors = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: xmlParserErrors = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: xmlParserErrors = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: xmlParserErrors = 5024;
pub const XML_CHECK_NOT_ATTR: xmlParserErrors = 5023;
pub const XML_CHECK_NOT_DTD: xmlParserErrors = 5022;
pub const XML_CHECK_WRONG_NEXT: xmlParserErrors = 5021;
pub const XML_CHECK_NO_NEXT: xmlParserErrors = 5020;
pub const XML_CHECK_WRONG_PREV: xmlParserErrors = 5019;
pub const XML_CHECK_NO_PREV: xmlParserErrors = 5018;
pub const XML_CHECK_WRONG_DOC: xmlParserErrors = 5017;
pub const XML_CHECK_NO_ELEM: xmlParserErrors = 5016;
pub const XML_CHECK_NO_NAME: xmlParserErrors = 5015;
pub const XML_CHECK_NO_DOC: xmlParserErrors = 5014;
pub const XML_CHECK_NO_PARENT: xmlParserErrors = 5013;
pub const XML_CHECK_ENTITY_TYPE: xmlParserErrors = 5012;
pub const XML_CHECK_UNKNOWN_NODE: xmlParserErrors = 5011;
pub const XML_CHECK_FOUND_NOTATION: xmlParserErrors = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: xmlParserErrors = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: xmlParserErrors = 5008;
pub const XML_CHECK_FOUND_COMMENT: xmlParserErrors = 5007;
pub const XML_CHECK_FOUND_PI: xmlParserErrors = 5006;
pub const XML_CHECK_FOUND_ENTITY: xmlParserErrors = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: xmlParserErrors = 5004;
pub const XML_CHECK_FOUND_CDATA: xmlParserErrors = 5003;
pub const XML_CHECK_FOUND_TEXT: xmlParserErrors = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: xmlParserErrors = 5001;
pub const XML_CHECK_FOUND_ELEMENT: xmlParserErrors = 5000;
pub const XML_MODULE_CLOSE: xmlParserErrors = 4901;
pub const XML_MODULE_OPEN: xmlParserErrors = 4900;
pub const XML_SCHEMATRONV_REPORT: xmlParserErrors = 4001;
pub const XML_SCHEMATRONV_ASSERT: xmlParserErrors = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: xmlParserErrors = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: xmlParserErrors = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: xmlParserErrors = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: xmlParserErrors = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: xmlParserErrors = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: xmlParserErrors = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: xmlParserErrors = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: xmlParserErrors = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: xmlParserErrors = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: xmlParserErrors = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: xmlParserErrors = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: xmlParserErrors = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: xmlParserErrors = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: xmlParserErrors = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: xmlParserErrors = 3077;
pub const XML_SCHEMAP_SRC_CT_1: xmlParserErrors = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: xmlParserErrors = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: xmlParserErrors = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: xmlParserErrors = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: xmlParserErrors = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: xmlParserErrors = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: xmlParserErrors = 3070;
pub const XML_SCHEMAP_INTERNAL: xmlParserErrors = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: xmlParserErrors = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: xmlParserErrors = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: xmlParserErrors = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: xmlParserErrors = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: xmlParserErrors = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: xmlParserErrors = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: xmlParserErrors = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: xmlParserErrors = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: xmlParserErrors = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: xmlParserErrors = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: xmlParserErrors = 3058;
pub const XML_SCHEMAP_NO_XSI: xmlParserErrors = 3057;
pub const XML_SCHEMAP_NO_XMLNS: xmlParserErrors = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: xmlParserErrors = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: xmlParserErrors = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: xmlParserErrors = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: xmlParserErrors = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: xmlParserErrors = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: xmlParserErrors = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: xmlParserErrors = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: xmlParserErrors = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: xmlParserErrors = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: xmlParserErrors = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: xmlParserErrors = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: xmlParserErrors = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: xmlParserErrors = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: xmlParserErrors = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: xmlParserErrors = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: xmlParserErrors = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: xmlParserErrors = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: xmlParserErrors = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: xmlParserErrors = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: xmlParserErrors = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: xmlParserErrors = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: xmlParserErrors = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: xmlParserErrors = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: xmlParserErrors = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: xmlParserErrors = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: xmlParserErrors = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: xmlParserErrors = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: xmlParserErrors = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: xmlParserErrors = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: xmlParserErrors = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: xmlParserErrors = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: xmlParserErrors = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: xmlParserErrors = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: xmlParserErrors = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: xmlParserErrors = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: xmlParserErrors = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: xmlParserErrors = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: xmlParserErrors = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: xmlParserErrors = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: xmlParserErrors = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: xmlParserErrors = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: xmlParserErrors = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: xmlParserErrors = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: xmlParserErrors = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: xmlParserErrors = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: xmlParserErrors = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: xmlParserErrors = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: xmlParserErrors = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: xmlParserErrors =
    3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: xmlParserErrors = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: xmlParserErrors =
    3005;
pub const XML_SCHEMAP_SRC_RESOLVE: xmlParserErrors = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: xmlParserErrors = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: xmlParserErrors = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: xmlParserErrors = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: xmlParserErrors = 3000;
pub const XML_HTTP_UNKNOWN_HOST: xmlParserErrors = 2022;
pub const XML_HTTP_USE_IP: xmlParserErrors = 2021;
pub const XML_HTTP_URL_SYNTAX: xmlParserErrors = 2020;
pub const XML_FTP_URL_SYNTAX: xmlParserErrors = 2003;
pub const XML_FTP_ACCNT: xmlParserErrors = 2002;
pub const XML_FTP_EPSV_ANSWER: xmlParserErrors = 2001;
pub const XML_FTP_PASV_ANSWER: xmlParserErrors = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: xmlParserErrors = 1955;
pub const XML_C14N_UNKNOW_NODE: xmlParserErrors = 1954;
pub const XML_C14N_INVALID_NODE: xmlParserErrors = 1953;
pub const XML_C14N_CREATE_STACK: xmlParserErrors = 1952;
pub const XML_C14N_REQUIRES_UTF8: xmlParserErrors = 1951;
pub const XML_C14N_CREATE_CTXT: xmlParserErrors = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: xmlParserErrors = 1903;
pub const XML_XPTR_EVAL_FAILED: xmlParserErrors = 1902;
pub const XML_XPTR_CHILDSEQ_START: xmlParserErrors = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: xmlParserErrors = 1900;
pub const XML_SCHEMAV_MISC: xmlParserErrors = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: xmlParserErrors = 1878;
pub const XML_SCHEMAV_CVC_IDC: xmlParserErrors = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: xmlParserErrors = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: xmlParserErrors = 1875;
pub const XML_SCHEMAV_CVC_AU: xmlParserErrors = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: xmlParserErrors = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: xmlParserErrors = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: xmlParserErrors = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: xmlParserErrors = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: xmlParserErrors = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: xmlParserErrors = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: xmlParserErrors = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: xmlParserErrors = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: xmlParserErrors = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: xmlParserErrors = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: xmlParserErrors = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: xmlParserErrors = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: xmlParserErrors = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: xmlParserErrors = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: xmlParserErrors = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: xmlParserErrors = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: xmlParserErrors = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: xmlParserErrors = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: xmlParserErrors = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: xmlParserErrors = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: xmlParserErrors = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: xmlParserErrors = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: xmlParserErrors = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: xmlParserErrors = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: xmlParserErrors = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: xmlParserErrors = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: xmlParserErrors = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: xmlParserErrors = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: xmlParserErrors = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: xmlParserErrors = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: xmlParserErrors = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: xmlParserErrors = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: xmlParserErrors = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: xmlParserErrors = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: xmlParserErrors = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: xmlParserErrors = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: xmlParserErrors = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: xmlParserErrors = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: xmlParserErrors = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: xmlParserErrors = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: xmlParserErrors = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: xmlParserErrors = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: xmlParserErrors = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: xmlParserErrors = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: xmlParserErrors = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: xmlParserErrors = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: xmlParserErrors = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: xmlParserErrors = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: xmlParserErrors = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: xmlParserErrors = 1824;
pub const XML_SCHEMAV_FACET: xmlParserErrors = 1823;
pub const XML_SCHEMAV_VALUE: xmlParserErrors = 1822;
pub const XML_SCHEMAV_ATTRINVALID: xmlParserErrors = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: xmlParserErrors = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: xmlParserErrors = 1819;
pub const XML_SCHEMAV_INTERNAL: xmlParserErrors = 1818;
pub const XML_SCHEMAV_CONSTRUCT: xmlParserErrors = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: xmlParserErrors = 1816;
pub const XML_SCHEMAV_INVALIDELEM: xmlParserErrors = 1815;
pub const XML_SCHEMAV_INVALIDATTR: xmlParserErrors = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: xmlParserErrors = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: xmlParserErrors = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: xmlParserErrors = 1811;
pub const XML_SCHEMAV_ELEMCONT: xmlParserErrors = 1810;
pub const XML_SCHEMAV_NOTEMPTY: xmlParserErrors = 1809;
pub const XML_SCHEMAV_ISABSTRACT: xmlParserErrors = 1808;
pub const XML_SCHEMAV_NOROLLBACK: xmlParserErrors = 1807;
pub const XML_SCHEMAV_NOTYPE: xmlParserErrors = 1806;
pub const XML_SCHEMAV_WRONGELEM: xmlParserErrors = 1805;
pub const XML_SCHEMAV_MISSING: xmlParserErrors = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: xmlParserErrors = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: xmlParserErrors = 1802;
pub const XML_SCHEMAV_NOROOT: xmlParserErrors = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: xmlParserErrors = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: xmlParserErrors = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: xmlParserErrors = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: xmlParserErrors = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: xmlParserErrors = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: xmlParserErrors = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: xmlParserErrors = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: xmlParserErrors = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: xmlParserErrors = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: xmlParserErrors = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: xmlParserErrors = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: xmlParserErrors = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: xmlParserErrors = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: xmlParserErrors = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: xmlParserErrors = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: xmlParserErrors = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: xmlParserErrors = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: xmlParserErrors = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: xmlParserErrors = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: xmlParserErrors = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: xmlParserErrors = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: xmlParserErrors = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: xmlParserErrors = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: xmlParserErrors = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: xmlParserErrors = 1776;
pub const XML_SCHEMAP_RECURSIVE: xmlParserErrors = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: xmlParserErrors = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: xmlParserErrors = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: xmlParserErrors = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: xmlParserErrors = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: xmlParserErrors = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: xmlParserErrors = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: xmlParserErrors = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: xmlParserErrors = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: xmlParserErrors = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: xmlParserErrors = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: xmlParserErrors = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: xmlParserErrors = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: xmlParserErrors = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: xmlParserErrors = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: xmlParserErrors = 1760;
pub const XML_SCHEMAP_NOROOT: xmlParserErrors = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: xmlParserErrors = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: xmlParserErrors = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: xmlParserErrors = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: xmlParserErrors = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: xmlParserErrors = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: xmlParserErrors = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: xmlParserErrors = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: xmlParserErrors = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: xmlParserErrors = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: xmlParserErrors = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: xmlParserErrors = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: xmlParserErrors = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: xmlParserErrors = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: xmlParserErrors = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: xmlParserErrors = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: xmlParserErrors = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: xmlParserErrors = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: xmlParserErrors = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: xmlParserErrors = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: xmlParserErrors = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: xmlParserErrors = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: xmlParserErrors = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: xmlParserErrors = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: xmlParserErrors = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: xmlParserErrors = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: xmlParserErrors = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: xmlParserErrors = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: xmlParserErrors = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: xmlParserErrors = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: xmlParserErrors = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: xmlParserErrors = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: xmlParserErrors = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: xmlParserErrors = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: xmlParserErrors = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: xmlParserErrors = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: xmlParserErrors = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: xmlParserErrors = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: xmlParserErrors = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: xmlParserErrors = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: xmlParserErrors = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: xmlParserErrors = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: xmlParserErrors = 1717;
pub const XML_SCHEMAP_INVALID_FACET: xmlParserErrors = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: xmlParserErrors = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: xmlParserErrors = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: xmlParserErrors = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: xmlParserErrors = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: xmlParserErrors = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: xmlParserErrors = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: xmlParserErrors = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: xmlParserErrors = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: xmlParserErrors = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: xmlParserErrors = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: xmlParserErrors = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: xmlParserErrors = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: xmlParserErrors = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: xmlParserErrors = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: xmlParserErrors = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: xmlParserErrors = 1700;
pub const XML_CATALOG_RECURSION: xmlParserErrors = 1654;
pub const XML_CATALOG_NOT_CATALOG: xmlParserErrors = 1653;
pub const XML_CATALOG_PREFER_VALUE: xmlParserErrors = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: xmlParserErrors = 1651;
pub const XML_CATALOG_MISSING_ATTR: xmlParserErrors = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: xmlParserErrors = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: xmlParserErrors = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: xmlParserErrors = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: xmlParserErrors = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: xmlParserErrors = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: xmlParserErrors = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: xmlParserErrors = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: xmlParserErrors = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: xmlParserErrors = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: xmlParserErrors = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: xmlParserErrors = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: xmlParserErrors = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: xmlParserErrors = 1606;
pub const XML_XINCLUDE_HREF_URI: xmlParserErrors = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: xmlParserErrors = 1604;
pub const XML_XINCLUDE_NO_HREF: xmlParserErrors = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: xmlParserErrors = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: xmlParserErrors = 1601;
pub const XML_XINCLUDE_RECURSION: xmlParserErrors = 1600;
pub const XML_IO_BUFFER_FULL: xmlParserErrors = 1548;
pub const XML_IO_NO_INPUT: xmlParserErrors = 1547;
pub const XML_IO_WRITE: xmlParserErrors = 1546;
pub const XML_IO_FLUSH: xmlParserErrors = 1545;
pub const XML_IO_ENCODER: xmlParserErrors = 1544;
pub const XML_IO_NETWORK_ATTEMPT: xmlParserErrors = 1543;
pub const XML_REGEXP_COMPILE_ERROR: xmlParserErrors = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: xmlParserErrors = 1403;
pub const XML_SAVE_NO_DOCTYPE: xmlParserErrors = 1402;
pub const XML_SAVE_CHAR_INVALID: xmlParserErrors = 1401;
pub const XML_SAVE_NOT_UTF8: xmlParserErrors = 1400;
pub const XML_TREE_NOT_UTF8: xmlParserErrors = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: xmlParserErrors = 1302;
pub const XML_TREE_INVALID_DEC: xmlParserErrors = 1301;
pub const XML_TREE_INVALID_HEX: xmlParserErrors = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: xmlParserErrors = 1221;
pub const XML_XPATH_ENCODING_ERROR: xmlParserErrors = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: xmlParserErrors = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: xmlParserErrors = 1218;
pub const XML_XPTR_RESOURCE_ERROR: xmlParserErrors = 1217;
pub const XML_XPTR_SYNTAX_ERROR: xmlParserErrors = 1216;
pub const XML_XPATH_MEMORY_ERROR: xmlParserErrors = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: xmlParserErrors = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: xmlParserErrors = 1213;
pub const XML_XPATH_INVALID_ARITY: xmlParserErrors = 1212;
pub const XML_XPATH_INVALID_TYPE: xmlParserErrors = 1211;
pub const XML_XPATH_INVALID_OPERAND: xmlParserErrors = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: xmlParserErrors = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: xmlParserErrors = 1208;
pub const XML_XPATH_EXPR_ERROR: xmlParserErrors = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: xmlParserErrors = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: xmlParserErrors = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: xmlParserErrors = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: xmlParserErrors = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: xmlParserErrors = 1202;
pub const XML_XPATH_NUMBER_ERROR: xmlParserErrors = 1201;
pub const XML_XPATH_EXPRESSION_OK: xmlParserErrors = 1200;
pub const XML_RNGP_XML_NS: xmlParserErrors = 1122;
pub const XML_RNGP_XMLNS_NAME: xmlParserErrors = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: xmlParserErrors = 1120;
pub const XML_RNGP_VALUE_EMPTY: xmlParserErrors = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: xmlParserErrors = 1118;
pub const XML_RNGP_URI_FRAGMENT: xmlParserErrors = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: xmlParserErrors = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: xmlParserErrors = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: xmlParserErrors = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: xmlParserErrors = 1113;
pub const XML_RNGP_TYPE_VALUE: xmlParserErrors = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: xmlParserErrors = 1111;
pub const XML_RNGP_TYPE_MISSING: xmlParserErrors = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: xmlParserErrors = 1109;
pub const XML_RNGP_TEXT_EXPECTED: xmlParserErrors = 1108;
pub const XML_RNGP_START_MISSING: xmlParserErrors = 1107;
pub const XML_RNGP_START_EMPTY: xmlParserErrors = 1106;
pub const XML_RNGP_START_CONTENT: xmlParserErrors = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: xmlParserErrors = 1103;
pub const XML_RNGP_REF_NO_NAME: xmlParserErrors = 1102;
pub const XML_RNGP_REF_NO_DEF: xmlParserErrors = 1101;
pub const XML_RNGP_REF_NAME_INVALID: xmlParserErrors = 1100;
pub const XML_RNGP_REF_CYCLE: xmlParserErrors = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: xmlParserErrors = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: xmlParserErrors = 1097;
pub const XML_RNGP_PAT_START_VALUE: xmlParserErrors = 1096;
pub const XML_RNGP_PAT_START_TEXT: xmlParserErrors = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: xmlParserErrors = 1094;
pub const XML_RNGP_PAT_START_LIST: xmlParserErrors = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: xmlParserErrors = 1092;
pub const XML_RNGP_PAT_START_GROUP: xmlParserErrors = 1091;
pub const XML_RNGP_PAT_START_EMPTY: xmlParserErrors = 1090;
pub const XML_RNGP_PAT_START_DATA: xmlParserErrors = 1089;
pub const XML_RNGP_PAT_START_ATTR: xmlParserErrors = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: xmlParserErrors = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: xmlParserErrors = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: xmlParserErrors = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: xmlParserErrors = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: xmlParserErrors = 1083;
pub const XML_RNGP_PAT_LIST_REF: xmlParserErrors = 1082;
pub const XML_RNGP_PAT_LIST_LIST: xmlParserErrors = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: xmlParserErrors = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: xmlParserErrors = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: xmlParserErrors = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: xmlParserErrors = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: xmlParserErrors = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: xmlParserErrors = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: xmlParserErrors = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: xmlParserErrors = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: xmlParserErrors = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: xmlParserErrors = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: xmlParserErrors = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: xmlParserErrors = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: xmlParserErrors = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: xmlParserErrors = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: xmlParserErrors = 1066;
pub const XML_RNGP_PARSE_ERROR: xmlParserErrors = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: xmlParserErrors = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: xmlParserErrors = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: xmlParserErrors = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: xmlParserErrors = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: xmlParserErrors = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: xmlParserErrors = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: xmlParserErrors = 1058;
pub const XML_RNGP_NSNAME_NO_NS: xmlParserErrors = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: xmlParserErrors = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: xmlParserErrors = 1055;
pub const XML_RNGP_NEED_COMBINE: xmlParserErrors = 1054;
pub const XML_RNGP_NAME_MISSING: xmlParserErrors = 1053;
pub const XML_RNGP_MISSING_HREF: xmlParserErrors = 1052;
pub const XML_RNGP_INVALID_VALUE: xmlParserErrors = 1051;
pub const XML_RNGP_INVALID_URI: xmlParserErrors = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: xmlParserErrors = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: xmlParserErrors = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: xmlParserErrors = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: xmlParserErrors = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: xmlParserErrors = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: xmlParserErrors = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: xmlParserErrors = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: xmlParserErrors = 1042;
pub const XML_RNGP_HREF_ERROR: xmlParserErrors = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: xmlParserErrors = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: xmlParserErrors = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: xmlParserErrors = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: xmlParserErrors = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: xmlParserErrors = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: xmlParserErrors = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: xmlParserErrors = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: xmlParserErrors = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: xmlParserErrors = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: xmlParserErrors = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: xmlParserErrors = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: xmlParserErrors = 1029;
pub const XML_RNGP_EXCEPT_MISSING: xmlParserErrors = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: xmlParserErrors = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: xmlParserErrors = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: xmlParserErrors = 1025;
pub const XML_RNGP_EMPTY_CONTENT: xmlParserErrors = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: xmlParserErrors = 1023;
pub const XML_RNGP_EMPTY: xmlParserErrors = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: xmlParserErrors = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: xmlParserErrors = 1020;
pub const XML_RNGP_ELEMENT_NAME: xmlParserErrors = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: xmlParserErrors = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: xmlParserErrors = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: xmlParserErrors = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: xmlParserErrors = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: xmlParserErrors = 1014;
pub const XML_RNGP_DEFINE_MISSING: xmlParserErrors = 1013;
pub const XML_RNGP_DEFINE_EMPTY: xmlParserErrors = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: xmlParserErrors = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1010;
pub const XML_RNGP_DATA_CONTENT: xmlParserErrors = 1009;
pub const XML_RNGP_CREATE_FAILURE: xmlParserErrors = 1008;
pub const XML_RNGP_CHOICE_EMPTY: xmlParserErrors = 1007;
pub const XML_RNGP_CHOICE_CONTENT: xmlParserErrors = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: xmlParserErrors = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: xmlParserErrors = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: xmlParserErrors = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: xmlParserErrors = 1002;
pub const XML_RNGP_ATTR_CONFLICT: xmlParserErrors = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: xmlParserErrors = 1000;
pub const XML_HTML_UNKNOWN_TAG: xmlParserErrors = 801;
pub const XML_HTML_STRUCURE_ERROR: xmlParserErrors = 800;
pub const XML_DTD_DUP_TOKEN: xmlParserErrors = 541;
pub const XML_DTD_XMLID_TYPE: xmlParserErrors = 540;
pub const XML_DTD_XMLID_VALUE: xmlParserErrors = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: xmlParserErrors = 538;
pub const XML_DTD_UNKNOWN_NOTATION: xmlParserErrors = 537;
pub const XML_DTD_UNKNOWN_ID: xmlParserErrors = 536;
pub const XML_DTD_UNKNOWN_ENTITY: xmlParserErrors = 535;
pub const XML_DTD_UNKNOWN_ELEM: xmlParserErrors = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: xmlParserErrors = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: xmlParserErrors = 532;
pub const XML_DTD_ROOT_NAME: xmlParserErrors = 531;
pub const XML_DTD_NOT_STANDALONE: xmlParserErrors = 530;
pub const XML_DTD_NOT_PCDATA: xmlParserErrors = 529;
pub const XML_DTD_NOT_EMPTY: xmlParserErrors = 528;
pub const XML_DTD_NOTATION_VALUE: xmlParserErrors = 527;
pub const XML_DTD_NOTATION_REDEFINED: xmlParserErrors = 526;
pub const XML_DTD_NO_ROOT: xmlParserErrors = 525;
pub const XML_DTD_NO_PREFIX: xmlParserErrors = 524;
pub const XML_DTD_NO_ELEM_NAME: xmlParserErrors = 523;
pub const XML_DTD_NO_DTD: xmlParserErrors = 522;
pub const XML_DTD_NO_DOC: xmlParserErrors = 521;
pub const XML_DTD_MULTIPLE_ID: xmlParserErrors = 520;
pub const XML_DTD_MIXED_CORRUPT: xmlParserErrors = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: xmlParserErrors = 518;
pub const XML_DTD_LOAD_ERROR: xmlParserErrors = 517;
pub const XML_DTD_INVALID_DEFAULT: xmlParserErrors = 516;
pub const XML_DTD_INVALID_CHILD: xmlParserErrors = 515;
pub const XML_DTD_ID_SUBSET: xmlParserErrors = 514;
pub const XML_DTD_ID_REDEFINED: xmlParserErrors = 513;
pub const XML_DTD_ID_FIXED: xmlParserErrors = 512;
pub const XML_DTD_ENTITY_TYPE: xmlParserErrors = 511;
pub const XML_DTD_EMPTY_NOTATION: xmlParserErrors = 510;
pub const XML_DTD_ELEM_REDEFINED: xmlParserErrors = 509;
pub const XML_DTD_ELEM_NAMESPACE: xmlParserErrors = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: xmlParserErrors = 507;
pub const XML_DTD_DIFFERENT_PREFIX: xmlParserErrors = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: xmlParserErrors = 505;
pub const XML_DTD_CONTENT_MODEL: xmlParserErrors = 504;
pub const XML_DTD_CONTENT_ERROR: xmlParserErrors = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: xmlParserErrors = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: xmlParserErrors = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: xmlParserErrors = 500;
pub const XML_NS_ERR_COLON: xmlParserErrors = 205;
pub const XML_NS_ERR_EMPTY: xmlParserErrors = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 203;
pub const XML_NS_ERR_QNAME: xmlParserErrors = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: xmlParserErrors = 201;
pub const XML_NS_ERR_XML_NAMESPACE: xmlParserErrors = 200;
pub const XML_ERR_USER_STOP: xmlParserErrors = 111;
pub const XML_ERR_NAME_TOO_LONG: xmlParserErrors = 110;
pub const XML_ERR_VERSION_MISMATCH: xmlParserErrors = 109;
pub const XML_ERR_UNKNOWN_VERSION: xmlParserErrors = 108;
pub const XML_WAR_ENTITY_REDEFINED: xmlParserErrors = 107;
pub const XML_WAR_NS_COLUMN: xmlParserErrors = 106;
pub const XML_ERR_NOTATION_PROCESSING: xmlParserErrors = 105;
pub const XML_ERR_ENTITY_PROCESSING: xmlParserErrors = 104;
pub const XML_ERR_NOT_STANDALONE: xmlParserErrors = 103;
pub const XML_WAR_SPACE_VALUE: xmlParserErrors = 102;
pub const XML_ERR_MISSING_ENCODING: xmlParserErrors = 101;
pub const XML_WAR_NS_URI_RELATIVE: xmlParserErrors = 100;
pub const XML_WAR_NS_URI: xmlParserErrors = 99;
pub const XML_WAR_LANG_VALUE: xmlParserErrors = 98;
pub const XML_WAR_UNKNOWN_VERSION: xmlParserErrors = 97;
pub const XML_ERR_VERSION_MISSING: xmlParserErrors = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: xmlParserErrors = 95;
pub const XML_ERR_NO_DTD: xmlParserErrors = 94;
pub const XML_WAR_CATALOG_PI: xmlParserErrors = 93;
pub const XML_ERR_URI_FRAGMENT: xmlParserErrors = 92;
pub const XML_ERR_INVALID_URI: xmlParserErrors = 91;
pub const XML_ERR_ENTITY_BOUNDARY: xmlParserErrors = 90;
pub const XML_ERR_ENTITY_LOOP: xmlParserErrors = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: xmlParserErrors = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: xmlParserErrors = 87;
pub const XML_ERR_EXTRA_CONTENT: xmlParserErrors = 86;
pub const XML_ERR_NOT_WELL_BALANCED: xmlParserErrors = 85;
pub const XML_ERR_VALUE_REQUIRED: xmlParserErrors = 84;
pub const XML_ERR_CONDSEC_INVALID: xmlParserErrors = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: xmlParserErrors = 82;
pub const XML_ERR_INVALID_ENCODING: xmlParserErrors = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: xmlParserErrors = 80;
pub const XML_ERR_ENCODING_NAME: xmlParserErrors = 79;
pub const XML_ERR_STANDALONE_VALUE: xmlParserErrors = 78;
pub const XML_ERR_TAG_NOT_FINISHED: xmlParserErrors = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: xmlParserErrors = 76;
pub const XML_ERR_EQUAL_REQUIRED: xmlParserErrors = 75;
pub const XML_ERR_LTSLASH_REQUIRED: xmlParserErrors = 74;
pub const XML_ERR_GT_REQUIRED: xmlParserErrors = 73;
pub const XML_ERR_LT_REQUIRED: xmlParserErrors = 72;
pub const XML_ERR_PUBID_REQUIRED: xmlParserErrors = 71;
pub const XML_ERR_URI_REQUIRED: xmlParserErrors = 70;
pub const XML_ERR_PCDATA_REQUIRED: xmlParserErrors = 69;
pub const XML_ERR_NAME_REQUIRED: xmlParserErrors = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: xmlParserErrors = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: xmlParserErrors = 66;
pub const XML_ERR_SPACE_REQUIRED: xmlParserErrors = 65;
pub const XML_ERR_RESERVED_XML_NAME: xmlParserErrors = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: xmlParserErrors = 63;
pub const XML_ERR_MISPLACED_CDATA_END: xmlParserErrors = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: xmlParserErrors = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: xmlParserErrors = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: xmlParserErrors = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: xmlParserErrors = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: xmlParserErrors = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: xmlParserErrors = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: xmlParserErrors = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: xmlParserErrors = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: xmlParserErrors = 53;
pub const XML_ERR_MIXED_NOT_STARTED: xmlParserErrors = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: xmlParserErrors = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: xmlParserErrors = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: xmlParserErrors = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: xmlParserErrors = 48;
pub const XML_ERR_PI_NOT_FINISHED: xmlParserErrors = 47;
pub const XML_ERR_PI_NOT_STARTED: xmlParserErrors = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: xmlParserErrors = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: xmlParserErrors = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: xmlParserErrors = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: xmlParserErrors = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: xmlParserErrors = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: xmlParserErrors = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: xmlParserErrors = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: xmlParserErrors = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: xmlParserErrors = 36;
pub const XML_ERR_NS_DECL_ERROR: xmlParserErrors = 35;
pub const XML_ERR_STRING_NOT_CLOSED: xmlParserErrors = 34;
pub const XML_ERR_STRING_NOT_STARTED: xmlParserErrors = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: xmlParserErrors = 32;
pub const XML_ERR_UNKNOWN_ENCODING: xmlParserErrors = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: xmlParserErrors = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: xmlParserErrors = 29;
pub const XML_ERR_UNPARSED_ENTITY: xmlParserErrors = 28;
pub const XML_WAR_UNDECLARED_ENTITY: xmlParserErrors = 27;
pub const XML_ERR_UNDECLARED_ENTITY: xmlParserErrors = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: xmlParserErrors = 25;
pub const XML_ERR_PEREF_NO_NAME: xmlParserErrors = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: xmlParserErrors = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: xmlParserErrors = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: xmlParserErrors = 21;
pub const XML_ERR_PEREF_IN_EPILOG: xmlParserErrors = 20;
pub const XML_ERR_PEREF_IN_PROLOG: xmlParserErrors = 19;
pub const XML_ERR_PEREF_AT_EOF: xmlParserErrors = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: xmlParserErrors = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: xmlParserErrors = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: xmlParserErrors = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: xmlParserErrors = 14;
pub const XML_ERR_CHARREF_IN_DTD: xmlParserErrors = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: xmlParserErrors = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: xmlParserErrors = 11;
pub const XML_ERR_CHARREF_AT_EOF: xmlParserErrors = 10;
pub const XML_ERR_INVALID_CHAR: xmlParserErrors = 9;
pub const XML_ERR_INVALID_CHARREF: xmlParserErrors = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: xmlParserErrors = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: xmlParserErrors = 6;
pub const XML_ERR_DOCUMENT_END: xmlParserErrors = 5;
pub const XML_ERR_DOCUMENT_EMPTY: xmlParserErrors = 4;
pub const XML_ERR_DOCUMENT_START: xmlParserErrors = 3;
pub const XML_ERR_NO_MEMORY: xmlParserErrors = 2;
pub const XML_ERR_INTERNAL_ERROR: xmlParserErrors = 1;
pub const XML_ERR_OK: xmlParserErrors = 0;
/* *
 * xmlExternalEntityLoader:
 * @URL: The System ID of the resource requested
 * @ID: The Public ID of the resource requested
 * @context: the XML parser context
 *
 * External entity loaders types.
 *
 * Returns the entity input parser.
 */
pub type xmlExternalEntityLoader
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                _: *const std::os::raw::c_char, _: xmlParserCtxtPtr)
               -> xmlParserInputPtr>;
pub type xmlCharEncoding = std::os::raw::c_int;
pub const XML_CHAR_ENCODING_ASCII: xmlCharEncoding = 22;
pub const XML_CHAR_ENCODING_EUC_JP: xmlCharEncoding = 21;
pub const XML_CHAR_ENCODING_SHIFT_JIS: xmlCharEncoding = 20;
pub const XML_CHAR_ENCODING_2022_JP: xmlCharEncoding = 19;
pub const XML_CHAR_ENCODING_8859_9: xmlCharEncoding = 18;
pub const XML_CHAR_ENCODING_8859_8: xmlCharEncoding = 17;
pub const XML_CHAR_ENCODING_8859_7: xmlCharEncoding = 16;
pub const XML_CHAR_ENCODING_8859_6: xmlCharEncoding = 15;
pub const XML_CHAR_ENCODING_8859_5: xmlCharEncoding = 14;
pub const XML_CHAR_ENCODING_8859_4: xmlCharEncoding = 13;
pub const XML_CHAR_ENCODING_8859_3: xmlCharEncoding = 12;
pub const XML_CHAR_ENCODING_8859_2: xmlCharEncoding = 11;
pub const XML_CHAR_ENCODING_8859_1: xmlCharEncoding = 10;
pub const XML_CHAR_ENCODING_UCS2: xmlCharEncoding = 9;
pub const XML_CHAR_ENCODING_UCS4_3412: xmlCharEncoding = 8;
pub const XML_CHAR_ENCODING_UCS4_2143: xmlCharEncoding = 7;
pub const XML_CHAR_ENCODING_EBCDIC: xmlCharEncoding = 6;
pub const XML_CHAR_ENCODING_UCS4BE: xmlCharEncoding = 5;
pub const XML_CHAR_ENCODING_UCS4LE: xmlCharEncoding = 4;
pub const XML_CHAR_ENCODING_UTF16BE: xmlCharEncoding = 3;
pub const XML_CHAR_ENCODING_UTF16LE: xmlCharEncoding = 2;
pub const XML_CHAR_ENCODING_UTF8: xmlCharEncoding = 1;
pub const XML_CHAR_ENCODING_NONE: xmlCharEncoding = 0;
pub const XML_CHAR_ENCODING_ERROR: xmlCharEncoding = -1;
pub type xmlInputMatchCallback
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> std::os::raw::c_int>;
pub type xmlInputOpenCallback
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_void>;
pub type xmlOutputMatchCallback
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> std::os::raw::c_int>;
pub type xmlOutputOpenCallback
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_void>;
/*
 * Input I/O callback sets
 */
pub type xmlInputCallback = _xmlInputCallback;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlInputCallback {
    pub matchcallback: xmlInputMatchCallback,
    pub opencallback: xmlInputOpenCallback,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
}
/* *
 * xzlib.h: header for the front end for the transparent suport of lzma
 *          compression at the I/O layer
 *
 * See Copyright for the status of this software.
 *
 * Anders F Bjorklund <afb@users.sourceforge.net>
 */
pub type xzFile = *mut std::os::raw::c_void;
pub type xmlParserInputBufferCreateFilenameFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char, _: xmlCharEncoding)
               -> xmlParserInputBufferPtr>;
/*
 * Output I/O callback sets
 */
pub type xmlOutputCallback = _xmlOutputCallback;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlOutputCallback {
    pub matchcallback: xmlOutputMatchCallback,
    pub opencallback: xmlOutputOpenCallback,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
}
/* LIBXML_LZMA_ENABLED */
/* ***********************************************************************
 *									*
 *			I/O for HTTP file accesses			*
 *									*
 ************************************************************************/
pub type xmlIOHTTPWriteCtxtPtr = *mut xmlIOHTTPWriteCtxt_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlIOHTTPWriteCtxt_ {
    pub compression: std::os::raw::c_int,
    pub uri: *mut std::os::raw::c_char,
    pub doc_buff: *mut std::os::raw::c_void,
}
/*
**  Data structure and functions to work with sending compressed data
**  via HTTP.
*/
pub type xmlZMemBuffPtr = *mut xmlZMemBuff_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlZMemBuff_ {
    pub size: std::os::raw::c_ulong,
    pub crc: std::os::raw::c_ulong,
    pub zbuff: *mut std::os::raw::c_uchar,
    pub zctrl: z_stream,
}
pub type xmlZMemBuff = xmlZMemBuff_;
pub type xmlIOHTTPWriteCtxt = xmlIOHTTPWriteCtxt_;
pub type xmlURIPtr = *mut xmlURI;
/* *
 * Summary: library of generic URI related routines
 * Description: library of generic URI related routines
 *              Implements RFC 2396
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlURI:
 *
 * A parsed URI reference. This is a struct containing the various fields
 * as described in RFC 2396 but separated for further processing.
 *
 * Note: query is a deprecated field which is incorrectly unescaped.
 * query_raw takes precedence over query if the former is set.
 * See: http://mail.gnome.org/archives/xml/2007-April/thread.html#00127
 */
pub type xmlURI = _xmlURI;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlURI {
    pub scheme: *mut std::os::raw::c_char,
    pub opaque: *mut std::os::raw::c_char,
    pub authority: *mut std::os::raw::c_char,
    pub server: *mut std::os::raw::c_char,
    pub user: *mut std::os::raw::c_char,
    pub port: std::os::raw::c_int,
    pub path: *mut std::os::raw::c_char,
    pub query: *mut std::os::raw::c_char,
    pub fragment: *mut std::os::raw::c_char,
    pub cleanup: std::os::raw::c_int,
    pub query_raw: *mut std::os::raw::c_char,
}
pub type xmlOutputBufferCreateFilenameFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                _: xmlCharEncodingHandlerPtr, _: std::os::raw::c_int)
               -> xmlOutputBufferPtr>;
pub const XML_CATA_ALLOW_GLOBAL: xmlCatalogAllow = 1;
pub type xmlCatalogAllow = std::os::raw::c_uint;
pub const XML_CATA_ALLOW_ALL: xmlCatalogAllow = 3;
pub const XML_CATA_ALLOW_DOCUMENT: xmlCatalogAllow = 2;
pub const XML_CATA_ALLOW_NONE: xmlCatalogAllow = 0;
/* Implement XInclude substitition  */
pub const XML_PARSE_NONET: C2RustUnnamed_1 = 2048;
/*
 * New set of simpler/more flexible APIs
 */
/* *
 * xmlParserOption:
 *
 * This is the set of XML parser options that can be passed down
 * to the xmlReadDoc() and similar calls.
 */
pub type C2RustUnnamed_1 = std::os::raw::c_uint;
/* Store big lines numbers in text PSVI field */
/* ignore internal document encoding hint */
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_1 = 4194304;
/* parse using SAX2 interface before 2.7.0 */
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_1 = 2097152;
/* relax any hardcoded limit from the parser */
pub const XML_PARSE_OLDSAX: C2RustUnnamed_1 = 1048576;
/* do not fixup XINCLUDE xml:base uris */
pub const XML_PARSE_HUGE: C2RustUnnamed_1 = 524288;
/* parse using XML-1.0 before update 5 */
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_1 = 262144;
/* compact small text nodes; no modification of
                                   the tree allowed afterwards (will possibly
				   crash if you try to modify the tree) */
pub const XML_PARSE_OLD10: C2RustUnnamed_1 = 131072;
/* do not generate XINCLUDE START/END nodes */
pub const XML_PARSE_COMPACT: C2RustUnnamed_1 = 65536;
/* merge CDATA as text nodes */
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_1 = 32768;
/* remove redundant namespaces declarations */
pub const XML_PARSE_NOCDATA: C2RustUnnamed_1 = 16384;
/* Do not reuse the context dictionary */
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_1 = 8192;
/* Forbid network access */
pub const XML_PARSE_NODICT: C2RustUnnamed_1 = 4096;
/* use the SAX1 interface internally */
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_1 = 1024;
/* remove blank nodes */
pub const XML_PARSE_SAX1: C2RustUnnamed_1 = 512;
/* pedantic error reporting */
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_1 = 256;
/* suppress warning reports */
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_1 = 128;
/* suppress error reports */
pub const XML_PARSE_NOWARNING: C2RustUnnamed_1 = 64;
/* validate with the DTD */
pub const XML_PARSE_NOERROR: C2RustUnnamed_1 = 32;
/* default DTD attributes */
pub const XML_PARSE_DTDVALID: C2RustUnnamed_1 = 16;
/* load the external subset */
pub const XML_PARSE_DTDATTR: C2RustUnnamed_1 = 8;
/* substitute entities */
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_1 = 4;
/* recover on errors */
pub const XML_PARSE_NOENT: C2RustUnnamed_1 = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed_1 = 1;
#[inline]
unsafe extern "C" fn stat(mut __path: *const std::os::raw::c_char,
                          mut __statbuf: *mut stat) -> std::os::raw::c_int {
    return __xstat(1 as std::os::raw::c_int, __path, __statbuf);
}
static mut xmlInputCallbackTable: [xmlInputCallback; 15] =
    [xmlInputCallback{matchcallback: None,
                      opencallback: None,
                      readcallback: None,
                      closecallback: None,}; 15];
static mut xmlInputCallbackNr: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlInputCallbackInitialized: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlOutputCallbackTable: [xmlOutputCallback; 15] =
    [xmlOutputCallback{matchcallback: None,
                       opencallback: None,
                       writecallback: None,
                       closecallback: None,}; 15];
static mut xmlOutputCallbackNr: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlOutputCallbackInitialized: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* LIBXML_OUTPUT_ENABLED */
/* ***********************************************************************
 *									*
 *		Tree memory error handler				*
 *									*
 ************************************************************************/
static mut IOerr: [*const std::os::raw::c_char; 57] =
    [b"Unknown IO error\x00" as *const u8 as *const std::os::raw::c_char,
     b"Permission denied\x00" as *const u8 as *const std::os::raw::c_char,
     b"Resource temporarily unavailable\x00" as *const u8 as
         *const std::os::raw::c_char,
     b"Bad file descriptor\x00" as *const u8 as *const std::os::raw::c_char,
     b"Bad message\x00" as *const u8 as *const std::os::raw::c_char,
     b"Resource busy\x00" as *const u8 as *const std::os::raw::c_char,
     b"Operation canceled\x00" as *const u8 as *const std::os::raw::c_char,
     b"No child processes\x00" as *const u8 as *const std::os::raw::c_char,
     b"Resource deadlock avoided\x00" as *const u8 as *const std::os::raw::c_char,
     b"Domain error\x00" as *const u8 as *const std::os::raw::c_char,
     b"File exists\x00" as *const u8 as *const std::os::raw::c_char,
     b"Bad address\x00" as *const u8 as *const std::os::raw::c_char,
     b"File too large\x00" as *const u8 as *const std::os::raw::c_char,
     b"Operation in progress\x00" as *const u8 as *const std::os::raw::c_char,
     b"Interrupted function call\x00" as *const u8 as *const std::os::raw::c_char,
     b"Invalid argument\x00" as *const u8 as *const std::os::raw::c_char,
     b"Input/output error\x00" as *const u8 as *const std::os::raw::c_char,
     b"Is a directory\x00" as *const u8 as *const std::os::raw::c_char,
     b"Too many open files\x00" as *const u8 as *const std::os::raw::c_char,
     b"Too many links\x00" as *const u8 as *const std::os::raw::c_char,
     b"Inappropriate message buffer length\x00" as *const u8 as
         *const std::os::raw::c_char,
     b"Filename too long\x00" as *const u8 as *const std::os::raw::c_char,
     b"Too many open files in system\x00" as *const u8 as *const std::os::raw::c_char,
     b"No such device\x00" as *const u8 as *const std::os::raw::c_char,
     b"No such file or directory\x00" as *const u8 as *const std::os::raw::c_char,
     b"Exec format error\x00" as *const u8 as *const std::os::raw::c_char,
     b"No locks available\x00" as *const u8 as *const std::os::raw::c_char,
     b"Not enough space\x00" as *const u8 as *const std::os::raw::c_char,
     b"No space left on device\x00" as *const u8 as *const std::os::raw::c_char,
     b"Function not implemented\x00" as *const u8 as *const std::os::raw::c_char,
     b"Not a directory\x00" as *const u8 as *const std::os::raw::c_char,
     b"Directory not empty\x00" as *const u8 as *const std::os::raw::c_char,
     b"Not supported\x00" as *const u8 as *const std::os::raw::c_char,
     b"Inappropriate I/O control operation\x00" as *const u8 as
         *const std::os::raw::c_char,
     b"No such device or address\x00" as *const u8 as *const std::os::raw::c_char,
     b"Operation not permitted\x00" as *const u8 as *const std::os::raw::c_char,
     b"Broken pipe\x00" as *const u8 as *const std::os::raw::c_char,
     b"Result too large\x00" as *const u8 as *const std::os::raw::c_char,
     b"Read-only file system\x00" as *const u8 as *const std::os::raw::c_char,
     b"Invalid seek\x00" as *const u8 as *const std::os::raw::c_char,
     b"No such process\x00" as *const u8 as *const std::os::raw::c_char,
     b"Operation timed out\x00" as *const u8 as *const std::os::raw::c_char,
     b"Improper link\x00" as *const u8 as *const std::os::raw::c_char,
     b"Attempt to load network entity %s\x00" as *const u8 as
         *const std::os::raw::c_char,
     b"encoder error\x00" as *const u8 as *const std::os::raw::c_char,
     b"flush error\x00" as *const u8 as *const std::os::raw::c_char,
     b"write error\x00" as *const u8 as *const std::os::raw::c_char,
     b"no input\x00" as *const u8 as *const std::os::raw::c_char,
     b"buffer full\x00" as *const u8 as *const std::os::raw::c_char,
     b"loading error\x00" as *const u8 as *const std::os::raw::c_char,
     b"not a socket\x00" as *const u8 as *const std::os::raw::c_char,
     b"already connected\x00" as *const u8 as *const std::os::raw::c_char,
     b"connection refused\x00" as *const u8 as *const std::os::raw::c_char,
     b"unreachable network\x00" as *const u8 as *const std::os::raw::c_char,
     b"adddress in use\x00" as *const u8 as *const std::os::raw::c_char,
     b"already in use\x00" as *const u8 as *const std::os::raw::c_char,
     b"unknown address familly\x00" as *const u8 as *const std::os::raw::c_char];
/* *
 * xmlIOErrMemory:
 * @extra:  extra informations
 *
 * Handle an out of memory condition
 */
unsafe extern "C" fn xmlIOErrMemory(mut extra: *const std::os::raw::c_char) {
    __xmlSimpleError(XML_FROM_IO as std::os::raw::c_int,
                     XML_ERR_NO_MEMORY as std::os::raw::c_int, 0 as xmlNodePtr,
                     0 as *const std::os::raw::c_char, extra);
}
/* *
 * __xmlIOErr:
 * @code:  the error number
 * @
 * @extra:  extra informations
 *
 * Handle an I/O error
 */
#[no_mangle]
pub unsafe extern "C" fn __xmlIOErr(mut domain: std::os::raw::c_int,
                                    mut code: std::os::raw::c_int,
                                    mut extra: *const std::os::raw::c_char) {
    let mut idx: std::os::raw::c_uint = 0;
    if code == 0 as std::os::raw::c_int {
        if *__errno_location() == 0 as std::os::raw::c_int {
            code = 0 as std::os::raw::c_int
        } else if *__errno_location() == 13 as std::os::raw::c_int {
            code = XML_IO_EACCES as std::os::raw::c_int
        } else if *__errno_location() == 11 as std::os::raw::c_int {
            code = XML_IO_EAGAIN as std::os::raw::c_int
        } else if *__errno_location() == 9 as std::os::raw::c_int {
            code = XML_IO_EBADF as std::os::raw::c_int
        } else if *__errno_location() == 74 as std::os::raw::c_int {
            code = XML_IO_EBADMSG as std::os::raw::c_int
        } else if *__errno_location() == 16 as std::os::raw::c_int {
            code = XML_IO_EBUSY as std::os::raw::c_int
        } else if *__errno_location() == 125 as std::os::raw::c_int {
            code = XML_IO_ECANCELED as std::os::raw::c_int
        } else if *__errno_location() == 10 as std::os::raw::c_int {
            code = XML_IO_ECHILD as std::os::raw::c_int
        } else if *__errno_location() == 35 as std::os::raw::c_int {
            code = XML_IO_EDEADLK as std::os::raw::c_int
        } else if *__errno_location() == 33 as std::os::raw::c_int {
            code = XML_IO_EDOM as std::os::raw::c_int
        } else if *__errno_location() == 17 as std::os::raw::c_int {
            code = XML_IO_EEXIST as std::os::raw::c_int
        } else if *__errno_location() == 14 as std::os::raw::c_int {
            code = XML_IO_EFAULT as std::os::raw::c_int
        } else if *__errno_location() == 27 as std::os::raw::c_int {
            code = XML_IO_EFBIG as std::os::raw::c_int
        } else if *__errno_location() == 115 as std::os::raw::c_int {
            code = XML_IO_EINPROGRESS as std::os::raw::c_int
        } else if *__errno_location() == 4 as std::os::raw::c_int {
            code = XML_IO_EINTR as std::os::raw::c_int
        } else if *__errno_location() == 22 as std::os::raw::c_int {
            code = XML_IO_EINVAL as std::os::raw::c_int
        } else if *__errno_location() == 5 as std::os::raw::c_int {
            code = XML_IO_EIO as std::os::raw::c_int
        } else if *__errno_location() == 21 as std::os::raw::c_int {
            code = XML_IO_EISDIR as std::os::raw::c_int
        } else if *__errno_location() == 24 as std::os::raw::c_int {
            code = XML_IO_EMFILE as std::os::raw::c_int
        } else if *__errno_location() == 31 as std::os::raw::c_int {
            code = XML_IO_EMLINK as std::os::raw::c_int
        } else if *__errno_location() == 90 as std::os::raw::c_int {
            code = XML_IO_EMSGSIZE as std::os::raw::c_int
        } else if *__errno_location() == 36 as std::os::raw::c_int {
            code = XML_IO_ENAMETOOLONG as std::os::raw::c_int
        } else if *__errno_location() == 23 as std::os::raw::c_int {
            code = XML_IO_ENFILE as std::os::raw::c_int
        } else if *__errno_location() == 19 as std::os::raw::c_int {
            code = XML_IO_ENODEV as std::os::raw::c_int
        } else if *__errno_location() == 2 as std::os::raw::c_int {
            code = XML_IO_ENOENT as std::os::raw::c_int
        } else if *__errno_location() == 8 as std::os::raw::c_int {
            code = XML_IO_ENOEXEC as std::os::raw::c_int
        } else if *__errno_location() == 37 as std::os::raw::c_int {
            code = XML_IO_ENOLCK as std::os::raw::c_int
        } else if *__errno_location() == 12 as std::os::raw::c_int {
            code = XML_IO_ENOMEM as std::os::raw::c_int
        } else if *__errno_location() == 28 as std::os::raw::c_int {
            code = XML_IO_ENOSPC as std::os::raw::c_int
        } else if *__errno_location() == 38 as std::os::raw::c_int {
            code = XML_IO_ENOSYS as std::os::raw::c_int
        } else if *__errno_location() == 20 as std::os::raw::c_int {
            code = XML_IO_ENOTDIR as std::os::raw::c_int
        } else if *__errno_location() == 39 as std::os::raw::c_int {
            code = XML_IO_ENOTEMPTY as std::os::raw::c_int
        } else if *__errno_location() == 95 as std::os::raw::c_int {
            code = XML_IO_ENOTSUP as std::os::raw::c_int
        } else if *__errno_location() == 25 as std::os::raw::c_int {
            code = XML_IO_ENOTTY as std::os::raw::c_int
        } else if *__errno_location() == 6 as std::os::raw::c_int {
            code = XML_IO_ENXIO as std::os::raw::c_int
        } else if *__errno_location() == 1 as std::os::raw::c_int {
            code = XML_IO_EPERM as std::os::raw::c_int
        } else if *__errno_location() == 32 as std::os::raw::c_int {
            code = XML_IO_EPIPE as std::os::raw::c_int
        } else if *__errno_location() == 34 as std::os::raw::c_int {
            code = XML_IO_ERANGE as std::os::raw::c_int
        } else if *__errno_location() == 30 as std::os::raw::c_int {
            code = XML_IO_EROFS as std::os::raw::c_int
        } else if *__errno_location() == 29 as std::os::raw::c_int {
            code = XML_IO_ESPIPE as std::os::raw::c_int
        } else if *__errno_location() == 3 as std::os::raw::c_int {
            code = XML_IO_ESRCH as std::os::raw::c_int
        } else if *__errno_location() == 110 as std::os::raw::c_int {
            code = XML_IO_ETIMEDOUT as std::os::raw::c_int
        } else if *__errno_location() == 18 as std::os::raw::c_int {
            code = XML_IO_EXDEV as std::os::raw::c_int
        } else if *__errno_location() == 88 as std::os::raw::c_int {
            code = XML_IO_ENOTSOCK as std::os::raw::c_int
        } else if *__errno_location() == 106 as std::os::raw::c_int {
            code = XML_IO_EISCONN as std::os::raw::c_int
        } else if *__errno_location() == 111 as std::os::raw::c_int {
            code = XML_IO_ECONNREFUSED as std::os::raw::c_int
        } else if *__errno_location() == 110 as std::os::raw::c_int {
            code = XML_IO_ETIMEDOUT as std::os::raw::c_int
        } else if *__errno_location() == 101 as std::os::raw::c_int {
            code = XML_IO_ENETUNREACH as std::os::raw::c_int
        } else if *__errno_location() == 98 as std::os::raw::c_int {
            code = XML_IO_EADDRINUSE as std::os::raw::c_int
        } else if *__errno_location() == 115 as std::os::raw::c_int {
            code = XML_IO_EINPROGRESS as std::os::raw::c_int
        } else if *__errno_location() == 114 as std::os::raw::c_int {
            code = XML_IO_EALREADY as std::os::raw::c_int
        } else if *__errno_location() == 97 as std::os::raw::c_int {
            code = XML_IO_EAFNOSUPPORT as std::os::raw::c_int
        } else { code = XML_IO_UNKNOWN as std::os::raw::c_int }
        /* HAVE_ERRNO_H */
    }
    idx = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if code >= XML_IO_UNKNOWN as std::os::raw::c_int {
        idx = (code - XML_IO_UNKNOWN as std::os::raw::c_int) as std::os::raw::c_uint
    }
    if idx as std::os::raw::c_ulong >=
           (::std::mem::size_of::<[*const std::os::raw::c_char; 57]>() as
                std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<*const std::os::raw::c_char>()
                                                as std::os::raw::c_ulong) {
        idx = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    __xmlSimpleError(domain, code, 0 as xmlNodePtr, IOerr[idx as usize],
                     extra);
}
/* *
 * xmlIOErr:
 * @code:  the error number
 * @extra:  extra informations
 *
 * Handle an I/O error
 */
unsafe extern "C" fn xmlIOErr(mut code: std::os::raw::c_int,
                              mut extra: *const std::os::raw::c_char) {
    __xmlIOErr(XML_FROM_IO as std::os::raw::c_int, code, extra);
}
/* *
 * __xmlLoaderErr:
 * @ctx: the parser context
 * @extra:  extra informations
 *
 * Handle a resource access error
 */
#[no_mangle]
pub unsafe extern "C" fn __xmlLoaderErr(mut ctx: *mut std::os::raw::c_void,
                                        mut msg: *const std::os::raw::c_char,
                                        mut filename: *const std::os::raw::c_char) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut level: xmlErrorLevel = XML_ERR_ERROR;
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as std::os::raw::c_int &&
           (*ctxt).instate as std::os::raw::c_int == XML_PARSER_EOF as std::os::raw::c_int {
        return
    }
    if !ctxt.is_null() && !(*ctxt).sax.is_null() {
        if (*ctxt).validate != 0 {
            channel = (*(*ctxt).sax).error;
            level = XML_ERR_ERROR
        } else { channel = (*(*ctxt).sax).warning; level = XML_ERR_WARNING }
        if (*(*ctxt).sax).initialized == 0xdeedbeaf as std::os::raw::c_uint {
            schannel = (*(*ctxt).sax).serror
        }
        data = (*ctxt).userData
    }
    __xmlRaiseError(schannel, channel, data, ctxt as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, XML_FROM_IO as std::os::raw::c_int,
                    XML_IO_LOAD_ERROR as std::os::raw::c_int, level,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, filename,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, 0 as std::os::raw::c_int, msg, filename);
}
/* ***********************************************************************
 *									*
 *		Tree memory error handler				*
 *									*
 ************************************************************************/
/* *
 * xmlNormalizeWindowsPath:
 * @path: the input file path
 *
 * This function is obsolete. Please see xmlURIFromPath in uri.c for
 * a better solution.
 *
 * Returns a canonicalized version of the path
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNormalizeWindowsPath(mut path: *const xmlChar)
 -> *mut xmlChar {
    return xmlCanonicPath(path);
}
/* *
 * xmlCleanupInputCallbacks:
 *
 * clears the entire input callback table. this includes the
 * compiled-in I/O.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupInputCallbacks() {
    let mut i: std::os::raw::c_int = 0;
    if xmlInputCallbackInitialized == 0 { return }
    i = xmlInputCallbackNr - 1 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        xmlInputCallbackTable[i as usize].matchcallback = None;
        xmlInputCallbackTable[i as usize].opencallback = None;
        xmlInputCallbackTable[i as usize].readcallback = None;
        xmlInputCallbackTable[i as usize].closecallback = None;
        i -= 1
    }
    xmlInputCallbackNr = 0 as std::os::raw::c_int;
    xmlInputCallbackInitialized = 0 as std::os::raw::c_int;
}
/* *
 * xmlPopInputCallbacks:
 *
 * Clear the top input callback from the input stack. this includes the
 * compiled-in I/O.
 *
 * Returns the number of input callback registered or -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlPopInputCallbacks() -> std::os::raw::c_int {
    if xmlInputCallbackInitialized == 0 { return -(1 as std::os::raw::c_int) }
    if xmlInputCallbackNr <= 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    xmlInputCallbackNr -= 1;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].matchcallback = None;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].opencallback = None;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].readcallback = None;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].closecallback = None;
    return xmlInputCallbackNr;
}
/* *
 * xmlCleanupOutputCallbacks:
 *
 * clears the entire output callback table. this includes the
 * compiled-in I/O callbacks.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupOutputCallbacks() {
    let mut i: std::os::raw::c_int = 0;
    if xmlOutputCallbackInitialized == 0 { return }
    i = xmlOutputCallbackNr - 1 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        xmlOutputCallbackTable[i as usize].matchcallback = None;
        xmlOutputCallbackTable[i as usize].opencallback = None;
        xmlOutputCallbackTable[i as usize].writecallback = None;
        xmlOutputCallbackTable[i as usize].closecallback = None;
        i -= 1
    }
    xmlOutputCallbackNr = 0 as std::os::raw::c_int;
    xmlOutputCallbackInitialized = 0 as std::os::raw::c_int;
}
/* LIBXML_OUTPUT_ENABLED */
/* ***********************************************************************
 *									*
 *		Standard I/O for file accesses				*
 *									*
 ************************************************************************/
/* *
 * xmlCheckFilename:
 * @path:  the path to check
 *
 * function checks to see if @path is a valid source
 * (file, socket...) for XML.
 *
 * if stat is not available on the target machine,
 * returns 1.  if stat fails, returns 0 (if calling
 * stat on the filename fails, it can't be right).
 * if stat succeeds and the file is a directory,
 * returns 2.  otherwise returns 1.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCheckFilename(mut path: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut stat_buffer: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if path.is_null() { return 0 as std::os::raw::c_int }
    if stat(path, &mut stat_buffer) == -(1 as std::os::raw::c_int) {
        return 0 as std::os::raw::c_int
    }
    if stat_buffer.st_mode & 0o170000 as std::os::raw::c_int as std::os::raw::c_uint ==
           0o40000 as std::os::raw::c_int as std::os::raw::c_uint {
        return 2 as std::os::raw::c_int
    }
    /* HAVE_STAT */
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlInputReadCallbackNop:
 *
 * No Operation xmlInputReadCallback function, does nothing.
 *
 * Returns zero
 */
#[no_mangle]
pub unsafe extern "C" fn xmlInputReadCallbackNop(mut context:
                                                     *mut std::os::raw::c_void,
                                                 mut buffer:
                                                     *mut std::os::raw::c_char,
                                                 mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlFdRead:
 * @context:  the I/O context
 * @buffer:  where to drop data
 * @len:  number of bytes to read
 *
 * Read @len bytes to @buffer from the I/O channel.
 *
 * Returns the number of bytes written
 */
unsafe extern "C" fn xmlFdRead(mut context: *mut std::os::raw::c_void,
                               mut buffer: *mut std::os::raw::c_char,
                               mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    ret =
        read(context as ptrdiff_t as std::os::raw::c_int,
             &mut *buffer.offset(0 as std::os::raw::c_int as isize) as
                 *mut std::os::raw::c_char as *mut std::os::raw::c_void, len as size_t) as
            std::os::raw::c_int;
    if ret < 0 as std::os::raw::c_int {
        xmlIOErr(0 as std::os::raw::c_int,
                 b"read()\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ret;
}
/* *
 * xmlFdWrite:
 * @context:  the I/O context
 * @buffer:  where to get data
 * @len:  number of bytes to write
 *
 * Write @len bytes from @buffer to the I/O channel.
 *
 * Returns the number of bytes written
 */
unsafe extern "C" fn xmlFdWrite(mut context: *mut std::os::raw::c_void,
                                mut buffer: *const std::os::raw::c_char,
                                mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if len > 0 as std::os::raw::c_int {
        ret =
            write(context as ptrdiff_t as std::os::raw::c_int,
                  &*buffer.offset(0 as std::os::raw::c_int as isize) as
                      *const std::os::raw::c_char as *const std::os::raw::c_void,
                  len as size_t) as std::os::raw::c_int;
        if ret < 0 as std::os::raw::c_int {
            xmlIOErr(0 as std::os::raw::c_int,
                     b"write()\x00" as *const u8 as *const std::os::raw::c_char);
        }
    }
    return ret;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlFdClose:
 * @context:  the I/O context
 *
 * Close an I/O channel
 *
 * Returns 0 in case of success and error code otherwise
 */
unsafe extern "C" fn xmlFdClose(mut context: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    ret = close(context as ptrdiff_t as std::os::raw::c_int);
    if ret < 0 as std::os::raw::c_int {
        xmlIOErr(0 as std::os::raw::c_int,
                 b"close()\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ret;
}
/* *
 * xmlFileMatch:
 * @filename:  the URI for matching
 *
 * input from FILE *
 *
 * Returns 1 if matches, 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFileMatch(mut filename: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlFileOpen_real:
 * @filename:  the URI for matching
 *
 * input from FILE *, supports compressed input
 * if @filename is " " then the standard input is used
 *
 * Returns an I/O context or NULL in case of error
 */
unsafe extern "C" fn xmlFileOpen_real(mut filename: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    let mut path: *const std::os::raw::c_char = filename;
    let mut fd: *mut FILE = 0 as *mut FILE;
    if filename.is_null() { return 0 as *mut std::os::raw::c_void }
    if strcmp(filename, b"-\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        fd = stdin;
        return fd as *mut std::os::raw::c_void
    }
    if xmlStrncasecmp(filename as *mut xmlChar,
                      b"file://localhost/\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut xmlChar,
                      17 as std::os::raw::c_int) == 0 {
        path =
            &*filename.offset(16 as std::os::raw::c_int as isize) as
                *const std::os::raw::c_char
    } else if xmlStrncasecmp(filename as *mut xmlChar,
                             b"file:///\x00" as *const u8 as
                                 *const std::os::raw::c_char as *mut xmlChar,
                             8 as std::os::raw::c_int) == 0 {
        path =
            &*filename.offset(7 as std::os::raw::c_int as isize) as
                *const std::os::raw::c_char
    } else if xmlStrncasecmp(filename as *mut xmlChar,
                             b"file:/\x00" as *const u8 as *const std::os::raw::c_char
                                 as *mut xmlChar, 6 as std::os::raw::c_int) == 0 {
        /* lots of generators seems to lazy to read RFC 1738 */
        path =
            &*filename.offset(5 as std::os::raw::c_int as isize) as
                *const std::os::raw::c_char
    }
    /* Do not check DDNAME on zOS ! */
    if xmlCheckFilename(path) == 0 { return 0 as *mut std::os::raw::c_void }
    fd = fopen(path, b"r\x00" as *const u8 as *const std::os::raw::c_char);
    /* WIN32 */
    if fd.is_null() { xmlIOErr(0 as std::os::raw::c_int, path); }
    return fd as *mut std::os::raw::c_void;
}
/* *
 * xmlFileOpen:
 * @filename:  the URI for matching
 *
 * Wrapper around xmlFileOpen_real that try it with an unescaped
 * version of @filename, if this fails fallback to @filename
 *
 * Returns a handler or NULL in case or failure
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFileOpen(mut filename: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    let mut unescaped: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut retval: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    retval = xmlFileOpen_real(filename);
    if retval.is_null() {
        unescaped =
            xmlURIUnescapeString(filename, 0 as std::os::raw::c_int,
                                 0 as *mut std::os::raw::c_char);
        if !unescaped.is_null() {
            retval = xmlFileOpen_real(unescaped);
            xmlFree.expect("non-null function pointer")(unescaped as
                                                            *mut std::os::raw::c_void);
        }
    }
    return retval;
}
/* *
 * xmlFileOpenW:
 * @filename:  the URI for matching
 *
 * output to from FILE *,
 * if @filename is "-" then the standard output is used
 *
 * Returns an I/O context or NULL in case of error
 */
unsafe extern "C" fn xmlFileOpenW(mut filename: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut fd: *mut FILE = 0 as *mut FILE;
    if strcmp(filename, b"-\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        fd = stdout;
        return fd as *mut std::os::raw::c_void
    }
    if xmlStrncasecmp(filename as *mut xmlChar,
                      b"file://localhost/\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut xmlChar,
                      17 as std::os::raw::c_int) == 0 {
        path =
            &*filename.offset(16 as std::os::raw::c_int as isize) as
                *const std::os::raw::c_char
    } else if xmlStrncasecmp(filename as *mut xmlChar,
                             b"file:///\x00" as *const u8 as
                                 *const std::os::raw::c_char as *mut xmlChar,
                             8 as std::os::raw::c_int) == 0 {
        path =
            &*filename.offset(7 as std::os::raw::c_int as isize) as
                *const std::os::raw::c_char
    } else { path = filename }
    if path.is_null() { return 0 as *mut std::os::raw::c_void }
    fd = fopen(path, b"wb\x00" as *const u8 as *const std::os::raw::c_char);
    /* WIN32 */
    if fd.is_null() { xmlIOErr(0 as std::os::raw::c_int, path); }
    return fd as *mut std::os::raw::c_void;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlFileRead:
 * @context:  the I/O context
 * @buffer:  where to drop data
 * @len:  number of bytes to write
 *
 * Read @len bytes to @buffer from the I/O channel.
 *
 * Returns the number of bytes written or < 0 in case of failure
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFileRead(mut context: *mut std::os::raw::c_void,
                                     mut buffer: *mut std::os::raw::c_char,
                                     mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    if context.is_null() || buffer.is_null() { return -(1 as std::os::raw::c_int) }
    ret =
        fread(&mut *buffer.offset(0 as std::os::raw::c_int as isize) as
                  *mut std::os::raw::c_char as *mut std::os::raw::c_void,
              1 as std::os::raw::c_int as size_t, len as size_t, context as *mut FILE)
            as std::os::raw::c_int;
    if ret < 0 as std::os::raw::c_int {
        xmlIOErr(0 as std::os::raw::c_int,
                 b"fread()\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ret;
}
/* *
 * xmlFileWrite:
 * @context:  the I/O context
 * @buffer:  where to drop data
 * @len:  number of bytes to write
 *
 * Write @len bytes from @buffer to the I/O channel.
 *
 * Returns the number of bytes written
 */
unsafe extern "C" fn xmlFileWrite(mut context: *mut std::os::raw::c_void,
                                  mut buffer: *const std::os::raw::c_char,
                                  mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut items: std::os::raw::c_int = 0;
    if context.is_null() || buffer.is_null() { return -(1 as std::os::raw::c_int) }
    items =
        fwrite(&*buffer.offset(0 as std::os::raw::c_int as isize) as
                   *const std::os::raw::c_char as *const std::os::raw::c_void, len as size_t,
               1 as std::os::raw::c_int as size_t, context as *mut FILE) as
            std::os::raw::c_int;
    if items == 0 as std::os::raw::c_int && ferror(context as *mut FILE) != 0 {
        xmlIOErr(0 as std::os::raw::c_int,
                 b"fwrite()\x00" as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    return items * len;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlFileClose:
 * @context:  the I/O context
 *
 * Close an I/O channel
 *
 * Returns 0 or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFileClose(mut context: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut fil: *mut FILE = 0 as *mut FILE;
    let mut ret: std::os::raw::c_int = 0;
    if context.is_null() { return -(1 as std::os::raw::c_int) }
    fil = context as *mut FILE;
    if fil == stdout || fil == stderr {
        ret = fflush(fil);
        if ret < 0 as std::os::raw::c_int {
            xmlIOErr(0 as std::os::raw::c_int,
                     b"fflush()\x00" as *const u8 as *const std::os::raw::c_char);
        }
        return 0 as std::os::raw::c_int
    }
    if fil == stdin { return 0 as std::os::raw::c_int }
    ret =
        if fclose(context as *mut FILE) == -(1 as std::os::raw::c_int) {
            -(1 as std::os::raw::c_int)
        } else { 0 as std::os::raw::c_int };
    if ret < 0 as std::os::raw::c_int {
        xmlIOErr(0 as std::os::raw::c_int,
                 b"fclose()\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ret;
}
/* *
 * xmlFileFlush:
 * @context:  the I/O context
 *
 * Flush an I/O channel
 */
unsafe extern "C" fn xmlFileFlush(mut context: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    if context.is_null() { return -(1 as std::os::raw::c_int) }
    ret =
        if fflush(context as *mut FILE) == -(1 as std::os::raw::c_int) {
            -(1 as std::os::raw::c_int)
        } else { 0 as std::os::raw::c_int };
    if ret < 0 as std::os::raw::c_int {
        xmlIOErr(0 as std::os::raw::c_int,
                 b"fflush()\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ret;
}
/* *
 * xmlBufferWrite:
 * @context:  the xmlBuffer
 * @buffer:  the data to write
 * @len:  number of bytes to write
 *
 * Write @len bytes from @buffer to the xml buffer
 *
 * Returns the number of bytes written
 */
unsafe extern "C" fn xmlBufferWrite(mut context: *mut std::os::raw::c_void,
                                    mut buffer: *const std::os::raw::c_char,
                                    mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    ret =
        xmlBufferAdd(context as xmlBufferPtr, buffer as *const xmlChar, len);
    if ret != 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    return len;
}
/* ***********************************************************************
 *									*
 *		I/O for compressed file accesses			*
 *									*
 ************************************************************************/
/* *
 * xmlGzfileMatch:
 * @filename:  the URI for matching
 *
 * input from compressed file test
 *
 * Returns 1 if matches, 0 otherwise
 */
unsafe extern "C" fn xmlGzfileMatch(mut filename: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlGzfileOpen_real:
 * @filename:  the URI for matching
 *
 * input from compressed file open
 * if @filename is " " then the standard input is used
 *
 * Returns an I/O context or NULL in case of error
 */
unsafe extern "C" fn xmlGzfileOpen_real(mut filename: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut fd: gzFile = 0 as *mut gzFile_s;
    if strcmp(filename, b"-\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        let mut duped_fd: std::os::raw::c_int = dup(fileno(stdin));
        fd = gzdopen(duped_fd, b"rb\x00" as *const u8 as *const std::os::raw::c_char);
        if fd.is_null() && duped_fd >= 0 as std::os::raw::c_int {
            close(duped_fd);
            /* gzdOpen() does not close on failure */
        }
        return fd as *mut std::os::raw::c_void
    }
    if xmlStrncasecmp(filename as *mut xmlChar,
                      b"file://localhost/\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut xmlChar,
                      17 as std::os::raw::c_int) == 0 {
        path =
            &*filename.offset(16 as std::os::raw::c_int as isize) as
                *const std::os::raw::c_char
    } else if xmlStrncasecmp(filename as *mut xmlChar,
                             b"file:///\x00" as *const u8 as
                                 *const std::os::raw::c_char as *mut xmlChar,
                             8 as std::os::raw::c_int) == 0 {
        path =
            &*filename.offset(7 as std::os::raw::c_int as isize) as
                *const std::os::raw::c_char
    } else { path = filename }
    if path.is_null() { return 0 as *mut std::os::raw::c_void }
    if xmlCheckFilename(path) == 0 { return 0 as *mut std::os::raw::c_void }
    fd = gzopen64(path, b"rb\x00" as *const u8 as *const std::os::raw::c_char);
    return fd as *mut std::os::raw::c_void;
}
/* *
 * xmlGzfileOpen:
 * @filename:  the URI for matching
 *
 * Wrapper around xmlGzfileOpen if the open fais, it will
 * try to unescape @filename
 */
unsafe extern "C" fn xmlGzfileOpen(mut filename: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    let mut unescaped: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut retval: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    retval = xmlGzfileOpen_real(filename);
    if retval.is_null() {
        unescaped =
            xmlURIUnescapeString(filename, 0 as std::os::raw::c_int,
                                 0 as *mut std::os::raw::c_char);
        if !unescaped.is_null() { retval = xmlGzfileOpen_real(unescaped) }
        xmlFree.expect("non-null function pointer")(unescaped as
                                                        *mut std::os::raw::c_void);
    }
    return retval;
}
/* *
 * xmlGzfileOpenW:
 * @filename:  the URI for matching
 * @compression:  the compression factor (0 - 9 included)
 *
 * input from compressed file open
 * if @filename is " " then the standard input is used
 *
 * Returns an I/O context or NULL in case of error
 */
unsafe extern "C" fn xmlGzfileOpenW(mut filename: *const std::os::raw::c_char,
                                    mut compression: std::os::raw::c_int)
 -> *mut std::os::raw::c_void {
    let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut mode: [std::os::raw::c_char; 15] = [0; 15];
    let mut fd: gzFile = 0 as *mut gzFile_s;
    snprintf(mode.as_mut_ptr(),
             ::std::mem::size_of::<[std::os::raw::c_char; 15]>() as std::os::raw::c_ulong,
             b"wb%d\x00" as *const u8 as *const std::os::raw::c_char, compression);
    if strcmp(filename, b"-\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        let mut duped_fd: std::os::raw::c_int = dup(fileno(stdout));
        fd = gzdopen(duped_fd, b"rb\x00" as *const u8 as *const std::os::raw::c_char);
        if fd.is_null() && duped_fd >= 0 as std::os::raw::c_int {
            close(duped_fd);
            /* gzdOpen() does not close on failure */
        }
        return fd as *mut std::os::raw::c_void
    }
    if xmlStrncasecmp(filename as *mut xmlChar,
                      b"file://localhost/\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut xmlChar,
                      17 as std::os::raw::c_int) == 0 {
        path =
            &*filename.offset(16 as std::os::raw::c_int as isize) as
                *const std::os::raw::c_char
    } else if xmlStrncasecmp(filename as *mut xmlChar,
                             b"file:///\x00" as *const u8 as
                                 *const std::os::raw::c_char as *mut xmlChar,
                             8 as std::os::raw::c_int) == 0 {
        path =
            &*filename.offset(7 as std::os::raw::c_int as isize) as
                *const std::os::raw::c_char
    } else { path = filename }
    if path.is_null() { return 0 as *mut std::os::raw::c_void }
    fd = gzopen64(path, mode.as_mut_ptr());
    return fd as *mut std::os::raw::c_void;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlGzfileRead:
 * @context:  the I/O context
 * @buffer:  where to drop data
 * @len:  number of bytes to write
 *
 * Read @len bytes to @buffer from the compressed I/O channel.
 *
 * Returns the number of bytes read.
 */
unsafe extern "C" fn xmlGzfileRead(mut context: *mut std::os::raw::c_void,
                                   mut buffer: *mut std::os::raw::c_char,
                                   mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    ret =
        gzread(context as gzFile,
               &mut *buffer.offset(0 as std::os::raw::c_int as isize) as
                   *mut std::os::raw::c_char as voidp, len as std::os::raw::c_uint);
    if ret < 0 as std::os::raw::c_int {
        xmlIOErr(0 as std::os::raw::c_int,
                 b"gzread()\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ret;
}
/* *
 * xmlGzfileWrite:
 * @context:  the I/O context
 * @buffer:  where to drop data
 * @len:  number of bytes to write
 *
 * Write @len bytes from @buffer to the compressed I/O channel.
 *
 * Returns the number of bytes written
 */
unsafe extern "C" fn xmlGzfileWrite(mut context: *mut std::os::raw::c_void,
                                    mut buffer: *const std::os::raw::c_char,
                                    mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    ret =
        gzwrite(context as gzFile,
                &*buffer.offset(0 as std::os::raw::c_int as isize) as
                    *const std::os::raw::c_char as *mut std::os::raw::c_char as voidpc,
                len as std::os::raw::c_uint);
    if ret < 0 as std::os::raw::c_int {
        xmlIOErr(0 as std::os::raw::c_int,
                 b"gzwrite()\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ret;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlGzfileClose:
 * @context:  the I/O context
 *
 * Close a compressed I/O channel
 */
unsafe extern "C" fn xmlGzfileClose(mut context: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    ret =
        if gzclose(context as gzFile) == 0 as std::os::raw::c_int {
            0 as std::os::raw::c_int
        } else { -(1 as std::os::raw::c_int) };
    if ret < 0 as std::os::raw::c_int {
        xmlIOErr(0 as std::os::raw::c_int,
                 b"gzclose()\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ret;
}
/* LIBXML_ZLIB_ENABLED */
/* ***********************************************************************
 *									*
 *		I/O for compressed file accesses			*
 *									*
 ************************************************************************/
/* *
 * xmlXzfileMatch:
 * @filename:  the URI for matching
 *
 * input from compressed file test
 *
 * Returns 1 if matches, 0 otherwise
 */
unsafe extern "C" fn xmlXzfileMatch(mut filename: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlXzFileOpen_real:
 * @filename:  the URI for matching
 *
 * input from compressed file open
 * if @filename is " " then the standard input is used
 *
 * Returns an I/O context or NULL in case of error
 */
unsafe extern "C" fn xmlXzfileOpen_real(mut filename: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut fd: xzFile = 0 as *mut std::os::raw::c_void;
    if strcmp(filename, b"-\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        fd =
            __libxml2_xzdopen(dup(fileno(stdin)),
                              b"rb\x00" as *const u8 as *const std::os::raw::c_char);
        return fd
    }
    if xmlStrncasecmp(filename as *mut xmlChar,
                      b"file://localhost/\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut xmlChar,
                      17 as std::os::raw::c_int) == 0 {
        path =
            &*filename.offset(16 as std::os::raw::c_int as isize) as
                *const std::os::raw::c_char
    } else if xmlStrncasecmp(filename as *mut xmlChar,
                             b"file:///\x00" as *const u8 as
                                 *const std::os::raw::c_char as *mut xmlChar,
                             8 as std::os::raw::c_int) == 0 {
        path =
            &*filename.offset(7 as std::os::raw::c_int as isize) as
                *const std::os::raw::c_char
    } else if xmlStrncasecmp(filename as *mut xmlChar,
                             b"file:/\x00" as *const u8 as *const std::os::raw::c_char
                                 as *mut xmlChar, 6 as std::os::raw::c_int) == 0 {
        /* lots of generators seems to lazy to read RFC 1738 */
        path =
            &*filename.offset(5 as std::os::raw::c_int as isize) as
                *const std::os::raw::c_char
    } else { path = filename }
    if path.is_null() { return 0 as *mut std::os::raw::c_void }
    if xmlCheckFilename(path) == 0 { return 0 as *mut std::os::raw::c_void }
    fd =
        __libxml2_xzopen(path, b"rb\x00" as *const u8 as *const std::os::raw::c_char);
    return fd;
}
/* *
 * xmlXzfileOpen:
 * @filename:  the URI for matching
 *
 * Wrapper around xmlXzfileOpen_real that try it with an unescaped
 * version of @filename, if this fails fallback to @filename
 *
 * Returns a handler or NULL in case or failure
 */
unsafe extern "C" fn xmlXzfileOpen(mut filename: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    let mut unescaped: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut retval: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    retval = xmlXzfileOpen_real(filename);
    if retval.is_null() {
        unescaped =
            xmlURIUnescapeString(filename, 0 as std::os::raw::c_int,
                                 0 as *mut std::os::raw::c_char);
        if !unescaped.is_null() { retval = xmlXzfileOpen_real(unescaped) }
        xmlFree.expect("non-null function pointer")(unescaped as
                                                        *mut std::os::raw::c_void);
    }
    return retval;
}
/* *
 * xmlXzfileRead:
 * @context:  the I/O context
 * @buffer:  where to drop data
 * @len:  number of bytes to write
 *
 * Read @len bytes to @buffer from the compressed I/O channel.
 *
 * Returns the number of bytes written
 */
unsafe extern "C" fn xmlXzfileRead(mut context: *mut std::os::raw::c_void,
                                   mut buffer: *mut std::os::raw::c_char,
                                   mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    ret =
        __libxml2_xzread(context,
                         &mut *buffer.offset(0 as std::os::raw::c_int as isize) as
                             *mut std::os::raw::c_char as *mut std::os::raw::c_void,
                         len as std::os::raw::c_uint);
    if ret < 0 as std::os::raw::c_int {
        xmlIOErr(0 as std::os::raw::c_int,
                 b"xzread()\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ret;
}
/* *
 * xmlXzfileClose:
 * @context:  the I/O context
 *
 * Close a compressed I/O channel
 */
unsafe extern "C" fn xmlXzfileClose(mut context: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    ret =
        if __libxml2_xzclose(context) == LZMA_OK as std::os::raw::c_int {
            0 as std::os::raw::c_int
        } else { -(1 as std::os::raw::c_int) };
    if ret < 0 as std::os::raw::c_int {
        xmlIOErr(0 as std::os::raw::c_int,
                 b"xzclose()\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ret;
}
/* *
 * append_reverse_ulong
 * @buff:  Compressed memory buffer
 * @data:  Unsigned long to append
 *
 * Append a unsigned long in reverse byte order to the end of the
 * memory buffer.
 */
unsafe extern "C" fn append_reverse_ulong(mut buff: *mut xmlZMemBuff,
                                          mut data: std::os::raw::c_ulong) {
    let mut idx: std::os::raw::c_int = 0;
    if buff.is_null() { return }
    /*
    **  This is plagiarized from putLong in gzio.c (zlib source) where
    **  the number "4" is hardcoded.  If zlib is ever patched to
    **  support 64 bit file sizes, this code would need to be patched
    **  as well.
    */
    idx = 0 as std::os::raw::c_int;
    while idx < 4 as std::os::raw::c_int {
        *(*buff).zctrl.next_out =
            (data & 0xff as std::os::raw::c_int as std::os::raw::c_ulong) as Bytef;
        data >>= 8 as std::os::raw::c_int;
        (*buff).zctrl.next_out = (*buff).zctrl.next_out.offset(1);
        idx += 1
    };
}
/* *
 *
 * xmlFreeZMemBuff
 * @buff:  The memory buffer context to clear
 *
 * Release all the resources associated with the compressed memory buffer.
 */
unsafe extern "C" fn xmlFreeZMemBuff(mut buff: xmlZMemBuffPtr) {
    if buff.is_null() { return }
    xmlFree.expect("non-null function pointer")((*buff).zbuff as
                                                    *mut std::os::raw::c_void);
    deflateEnd(&mut (*buff).zctrl);
    xmlFree.expect("non-null function pointer")(buff as *mut std::os::raw::c_void);
}
/* *
 * xmlCreateZMemBuff
 *@compression:	Compression value to use
 *
 * Create a memory buffer to hold the compressed XML document.  The
 * compressed document in memory will end up being identical to what
 * would be created if gzopen/gzwrite/gzclose were being used to
 * write the document to disk.  The code for the header/trailer data to
 * the compression is plagiarized from the zlib source files.
 */
unsafe extern "C" fn xmlCreateZMemBuff(mut compression: std::os::raw::c_int)
 -> *mut std::os::raw::c_void {
    let mut z_err: std::os::raw::c_int = 0;
    let mut hdr_lgth: std::os::raw::c_int = 0;
    let mut buff: xmlZMemBuffPtr = 0 as xmlZMemBuffPtr;
    if compression < 1 as std::os::raw::c_int || compression > 9 as std::os::raw::c_int {
        return 0 as *mut std::os::raw::c_void
    }
    /*  Create the control and data areas  */
    buff =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlZMemBuff>()
                                                          as std::os::raw::c_ulong) as
            xmlZMemBuffPtr;
    if buff.is_null() {
        xmlIOErrMemory(b"creating buffer context\x00" as *const u8 as
                           *const std::os::raw::c_char);
        return 0 as *mut std::os::raw::c_void
    }
    memset(buff as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlZMemBuff>() as std::os::raw::c_ulong);
    (*buff).size = 32768 as std::os::raw::c_int as std::os::raw::c_ulong;
    (*buff).zbuff =
        xmlMalloc.expect("non-null function pointer")((*buff).size) as
            *mut std::os::raw::c_uchar;
    if (*buff).zbuff.is_null() {
        xmlFreeZMemBuff(buff);
        xmlIOErrMemory(b"creating buffer\x00" as *const u8 as
                           *const std::os::raw::c_char);
        return 0 as *mut std::os::raw::c_void
    }
    z_err =
        deflateInit2_(&mut (*buff).zctrl, compression, 8 as std::os::raw::c_int,
                      -(15 as std::os::raw::c_int), 8 as std::os::raw::c_int,
                      0 as std::os::raw::c_int,
                      b"1.2.11\x00" as *const u8 as *const std::os::raw::c_char,
                      ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                          std::os::raw::c_int);
    if z_err != 0 as std::os::raw::c_int {
        let mut msg: [xmlChar; 500] = [0; 500];
        xmlFreeZMemBuff(buff);
        buff = 0 as xmlZMemBuffPtr;
        xmlStrPrintf(msg.as_mut_ptr(), 500 as std::os::raw::c_int,
                     b"xmlCreateZMemBuff:  %s %d\n\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     b"Error initializing compression context.  ZLIB error:\x00"
                         as *const u8 as *const std::os::raw::c_char, z_err);
        xmlIOErr(XML_IO_WRITE as std::os::raw::c_int,
                 msg.as_mut_ptr() as *const std::os::raw::c_char);
        return 0 as *mut std::os::raw::c_void
    }
    /*  Set the header data.  The CRC will be needed for the trailer  */
    (*buff).crc =
        crc32(0 as std::os::raw::c_long as uLong, 0 as *const Bytef,
              0 as std::os::raw::c_int as uInt);
    hdr_lgth =
        snprintf((*buff).zbuff as *mut std::os::raw::c_char, (*buff).size,
                 b"%c%c%c%c%c%c%c%c%c%c\x00" as *const u8 as
                     *const std::os::raw::c_char, 0x1f as std::os::raw::c_int,
                 0x8b as std::os::raw::c_int, 8 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                 0 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                 0 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0x3 as std::os::raw::c_int);
    (*buff).zctrl.next_out = (*buff).zbuff.offset(hdr_lgth as isize);
    (*buff).zctrl.avail_out =
        (*buff).size.wrapping_sub(hdr_lgth as std::os::raw::c_ulong) as uInt;
    return buff as *mut std::os::raw::c_void;
}
/* *
 * xmlZMemBuffExtend
 * @buff:  Buffer used to compress and consolidate data.
 * @ext_amt:   Number of bytes to extend the buffer.
 *
 * Extend the internal buffer used to store the compressed data by the
 * specified amount.
 *
 * Returns 0 on success or -1 on failure to extend the buffer.  On failure
 * the original buffer still exists at the original size.
 */
unsafe extern "C" fn xmlZMemBuffExtend(mut buff: xmlZMemBuffPtr,
                                       mut ext_amt: size_t) -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut new_size: size_t = 0;
    let mut cur_used: size_t = 0;
    let mut tmp_ptr: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    if buff.is_null() {
        return -(1 as std::os::raw::c_int)
    } else {
        if ext_amt == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 0 as std::os::raw::c_int
        }
    }
    cur_used =
        (*buff).zctrl.next_out.offset_from((*buff).zbuff) as
            std::os::raw::c_long as size_t;
    new_size = (*buff).size.wrapping_add(ext_amt);
    tmp_ptr =
        xmlRealloc.expect("non-null function pointer")((*buff).zbuff as
                                                           *mut std::os::raw::c_void,
                                                       new_size) as
            *mut std::os::raw::c_uchar;
    if !tmp_ptr.is_null() {
        rc = 0 as std::os::raw::c_int;
        (*buff).size = new_size;
        (*buff).zbuff = tmp_ptr;
        (*buff).zctrl.next_out = tmp_ptr.offset(cur_used as isize);
        (*buff).zctrl.avail_out = new_size.wrapping_sub(cur_used) as uInt
    } else {
        let mut msg: [xmlChar; 500] = [0; 500];
        xmlStrPrintf(msg.as_mut_ptr(), 500 as std::os::raw::c_int,
                     b"xmlZMemBuffExtend:  %s %lu bytes.\n\x00" as *const u8
                         as *const std::os::raw::c_char,
                     b"Allocation failure extending output buffer to\x00" as
                         *const u8 as *const std::os::raw::c_char, new_size);
        xmlIOErr(XML_IO_WRITE as std::os::raw::c_int,
                 msg.as_mut_ptr() as *const std::os::raw::c_char);
    }
    return rc;
}
/* *
 * xmlZMemBuffAppend
 * @buff:  Buffer used to compress and consolidate data
 * @src:   Uncompressed source content to append to buffer
 * @len:   Length of source data to append to buffer
 *
 * Compress and append data to the internal buffer.  The data buffer
 * will be expanded if needed to store the additional data.
 *
 * Returns the number of bytes appended to the buffer or -1 on error.
 */
unsafe extern "C" fn xmlZMemBuffAppend(mut buff: xmlZMemBuffPtr,
                                       mut src: *const std::os::raw::c_char,
                                       mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut z_err: std::os::raw::c_int = 0;
    let mut min_accept: size_t = 0;
    if buff.is_null() || src.is_null() { return -(1 as std::os::raw::c_int) }
    (*buff).zctrl.avail_in = len as uInt;
    (*buff).zctrl.next_in = src as *mut std::os::raw::c_uchar;
    while (*buff).zctrl.avail_in > 0 as std::os::raw::c_int as std::os::raw::c_uint {
        /*
	**  Extend the buffer prior to deflate call if a reasonable amount
	**  of output buffer space is not available.
	*/
        min_accept =
            (*buff).zctrl.avail_in.wrapping_div(5 as std::os::raw::c_int as
                                                    std::os::raw::c_uint) as size_t;
        if (*buff).zctrl.avail_out as std::os::raw::c_ulong <= min_accept {
            if xmlZMemBuffExtend(buff, (*buff).size) == -(1 as std::os::raw::c_int) {
                return -(1 as std::os::raw::c_int)
            }
        }
        z_err = deflate(&mut (*buff).zctrl, 0 as std::os::raw::c_int);
        if z_err != 0 as std::os::raw::c_int {
            let mut msg: [xmlChar; 500] = [0; 500];
            xmlStrPrintf(msg.as_mut_ptr(), 500 as std::os::raw::c_int,
                         b"xmlZMemBuffAppend:  %s %d %s - %d\x00" as *const u8
                             as *const std::os::raw::c_char,
                         b"Compression error while appending\x00" as *const u8
                             as *const std::os::raw::c_char, len,
                         b"bytes to buffer.  ZLIB error\x00" as *const u8 as
                             *const std::os::raw::c_char, z_err);
            xmlIOErr(XML_IO_WRITE as std::os::raw::c_int,
                     msg.as_mut_ptr() as *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    (*buff).crc = crc32((*buff).crc, src as *mut std::os::raw::c_uchar, len as uInt);
    return len;
}
/* *
 * xmlZMemBuffGetContent
 * @buff:  Compressed memory content buffer
 * @data_ref:  Pointer reference to point to compressed content
 *
 * Flushes the compression buffers, appends gzip file trailers and
 * returns the compressed content and length of the compressed data.
 * NOTE:  The gzip trailer code here is plagiarized from zlib source.
 *
 * Returns the length of the compressed data or -1 on error.
 */
unsafe extern "C" fn xmlZMemBuffGetContent(mut buff: xmlZMemBuffPtr,
                                           mut data_ref:
                                               *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut zlgth: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut z_err: std::os::raw::c_int = 0;
    if buff.is_null() || data_ref.is_null() { return -(1 as std::os::raw::c_int) }
    loop 
         /*  Need to loop until compression output buffers are flushed  */
         {
        z_err = deflate(&mut (*buff).zctrl, 4 as std::os::raw::c_int);
        if z_err == 0 as std::os::raw::c_int {
            /*  In this case Z_OK means more buffer space needed  */
            if xmlZMemBuffExtend(buff, (*buff).size) == -(1 as std::os::raw::c_int) {
                return -(1 as std::os::raw::c_int)
            }
        }
        if !(z_err == 0 as std::os::raw::c_int) { break ; }
    }
    /*  If the compression state is not Z_STREAM_END, some error occurred  */
    if z_err == 1 as std::os::raw::c_int {
        /*  Need to append the gzip data trailer  */
        if ((*buff).zctrl.avail_out as std::os::raw::c_ulong) <
               (2 as std::os::raw::c_int as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_ulong>()
                                                    as std::os::raw::c_ulong) {
            if xmlZMemBuffExtend(buff,
                                 (2 as std::os::raw::c_int as
                                      std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_ulong>()
                                                                      as
                                                                      std::os::raw::c_ulong))
                   == -(1 as std::os::raw::c_int) {
                return -(1 as std::os::raw::c_int)
            }
        }
        /*
	**  For whatever reason, the CRC and length data are pushed out
	**  in reverse byte order.  So a memcpy can't be used here.
	*/
        append_reverse_ulong(buff, (*buff).crc);
        append_reverse_ulong(buff, (*buff).zctrl.total_in);
        zlgth =
            (*buff).zctrl.next_out.offset_from((*buff).zbuff) as
                std::os::raw::c_long as std::os::raw::c_int;
        *data_ref = (*buff).zbuff as *mut std::os::raw::c_char
    } else {
        let mut msg: [xmlChar; 500] = [0; 500];
        xmlStrPrintf(msg.as_mut_ptr(), 500 as std::os::raw::c_int,
                     b"xmlZMemBuffGetContent:  %s - %d\n\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     b"Error flushing zlib buffers.  Error code\x00" as
                         *const u8 as *const std::os::raw::c_char, z_err);
        xmlIOErr(XML_IO_WRITE as std::os::raw::c_int,
                 msg.as_mut_ptr() as *const std::os::raw::c_char);
    }
    return zlgth;
}
/* LIBXML_OUTPUT_ENABLED */
/*  LIBXML_ZLIB_ENABLED  */
/* *
 * xmlFreeHTTPWriteCtxt
 * @ctxt:  Context to cleanup
 *
 * Free allocated memory and reclaim system resources.
 *
 * No return value.
 */
unsafe extern "C" fn xmlFreeHTTPWriteCtxt(mut ctxt: xmlIOHTTPWriteCtxtPtr) {
    if !(*ctxt).uri.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).uri as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).doc_buff.is_null() {
        if (*ctxt).compression > 0 as std::os::raw::c_int {
            xmlFreeZMemBuff((*ctxt).doc_buff as xmlZMemBuffPtr);
        } else {
            xmlOutputBufferClose((*ctxt).doc_buff as xmlOutputBufferPtr);
        }
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut std::os::raw::c_void);
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlIOHTTPMatch:
 * @filename:  the URI for matching
 *
 * check if the URI matches an HTTP one
 *
 * Returns 1 if matches, 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIOHTTPMatch(mut filename: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    if xmlStrncasecmp(filename as *mut xmlChar,
                      b"http://\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar, 7 as std::os::raw::c_int) == 0 {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlIOHTTPOpen:
 * @filename:  the URI for matching
 *
 * open an HTTP I/O channel
 *
 * Returns an I/O context or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIOHTTPOpen(mut filename: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    return xmlNanoHTTPOpen(filename, 0 as *mut *mut std::os::raw::c_char);
}
/* *
 * xmlIOHTTPOpenW:
 * @post_uri:  The destination URI for the document
 * @compression:  The compression desired for the document.
 *
 * Open a temporary buffer to collect the document for a subsequent HTTP POST
 * request.  Non-static as is called from the output buffer creation routine.
 *
 * Returns an I/O context or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIOHTTPOpenW(mut post_uri: *const std::os::raw::c_char,
                                        mut compression: std::os::raw::c_int)
 -> *mut std::os::raw::c_void {
    let mut ctxt: xmlIOHTTPWriteCtxtPtr = 0 as xmlIOHTTPWriteCtxtPtr;
    if post_uri.is_null() { return 0 as *mut std::os::raw::c_void }
    ctxt =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlIOHTTPWriteCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlIOHTTPWriteCtxtPtr;
    if ctxt.is_null() {
        xmlIOErrMemory(b"creating HTTP output context\x00" as *const u8 as
                           *const std::os::raw::c_char);
        return 0 as *mut std::os::raw::c_void
    }
    memset(ctxt as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlIOHTTPWriteCtxt>() as std::os::raw::c_ulong);
    (*ctxt).uri = xmlStrdup(post_uri as *const xmlChar) as *mut std::os::raw::c_char;
    if (*ctxt).uri.is_null() {
        xmlIOErrMemory(b"copying URI\x00" as *const u8 as
                           *const std::os::raw::c_char);
        xmlFreeHTTPWriteCtxt(ctxt);
        return 0 as *mut std::os::raw::c_void
    }
    /*
     * **  Since the document length is required for an HTTP post,
     * **  need to put the document into a buffer.  A memory buffer
     * **  is being used to avoid pushing the data to disk and back.
     */
    if compression > 0 as std::os::raw::c_int && compression <= 9 as std::os::raw::c_int {
        (*ctxt).compression = compression;
        (*ctxt).doc_buff = xmlCreateZMemBuff(compression)
    } else {
        /*  Any character conversions should have been done before this  */
        (*ctxt).doc_buff =
            xmlAllocOutputBufferInternal(0 as xmlCharEncodingHandlerPtr) as
                *mut std::os::raw::c_void
    }
    if (*ctxt).doc_buff.is_null() {
        xmlFreeHTTPWriteCtxt(ctxt);
        ctxt = 0 as xmlIOHTTPWriteCtxtPtr
    }
    return ctxt as *mut std::os::raw::c_void;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlIOHTTPDfltOpenW
 * @post_uri:  The destination URI for this document.
 *
 * Calls xmlIOHTTPOpenW with no compression to set up for a subsequent
 * HTTP post command.  This function should generally not be used as
 * the open callback is short circuited in xmlOutputBufferCreateFile.
 *
 * Returns a pointer to the new IO context.
 */
unsafe extern "C" fn xmlIOHTTPDfltOpenW(mut post_uri: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    return xmlIOHTTPOpenW(post_uri, 0 as std::os::raw::c_int);
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlIOHTTPRead:
 * @context:  the I/O context
 * @buffer:  where to drop data
 * @len:  number of bytes to write
 *
 * Read @len bytes to @buffer from the I/O channel.
 *
 * Returns the number of bytes written
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIOHTTPRead(mut context: *mut std::os::raw::c_void,
                                       mut buffer: *mut std::os::raw::c_char,
                                       mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    if buffer.is_null() || len < 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    return xmlNanoHTTPRead(context,
                           &mut *buffer.offset(0 as std::os::raw::c_int as isize) as
                               *mut std::os::raw::c_char as *mut std::os::raw::c_void, len);
}
/* *
 * xmlIOHTTPWrite
 * @context:  previously opened writing context
 * @buffer:   data to output to temporary buffer
 * @len:      bytes to output
 *
 * Collect data from memory buffer into a temporary file for later
 * processing.
 *
 * Returns number of bytes written.
 */
unsafe extern "C" fn xmlIOHTTPWrite(mut context: *mut std::os::raw::c_void,
                                    mut buffer: *const std::os::raw::c_char,
                                    mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut ctxt: xmlIOHTTPWriteCtxtPtr = context as xmlIOHTTPWriteCtxtPtr;
    if ctxt.is_null() || (*ctxt).doc_buff.is_null() || buffer.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if len > 0 as std::os::raw::c_int {
        /*  Use gzwrite or fwrite as previously setup in the open call  */
        if (*ctxt).compression > 0 as std::os::raw::c_int {
            len =
                xmlZMemBuffAppend((*ctxt).doc_buff as xmlZMemBuffPtr, buffer,
                                  len)
        } else {
            len =
                xmlOutputBufferWrite((*ctxt).doc_buff as xmlOutputBufferPtr,
                                     len, buffer)
        }
        if len < 0 as std::os::raw::c_int {
            let mut msg: [xmlChar; 500] = [0; 500];
            xmlStrPrintf(msg.as_mut_ptr(), 500 as std::os::raw::c_int,
                         b"xmlIOHTTPWrite:  %s\n%s \'%s\'.\n\x00" as *const u8
                             as *const std::os::raw::c_char,
                         b"Error appending to internal buffer.\x00" as
                             *const u8 as *const std::os::raw::c_char,
                         b"Error sending document to URI\x00" as *const u8 as
                             *const std::os::raw::c_char, (*ctxt).uri);
            xmlIOErr(XML_IO_WRITE as std::os::raw::c_int,
                     msg.as_mut_ptr() as *const std::os::raw::c_char);
        }
    }
    return len;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlIOHTTPClose:
 * @context:  the I/O context
 *
 * Close an HTTP I/O channel
 *
 * Returns 0
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIOHTTPClose(mut context: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    xmlNanoHTTPClose(context);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlIOHTTCloseWrite
 * @context:  The I/O context
 * @http_mthd: The HTTP method to be used when sending the data
 *
 * Close the transmit HTTP I/O channel and actually send the data.
 */
unsafe extern "C" fn xmlIOHTTPCloseWrite(mut context: *mut std::os::raw::c_void,
                                         mut http_mthd: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut close_rc: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut http_rtn: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut content_lgth: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut ctxt: xmlIOHTTPWriteCtxtPtr = context as xmlIOHTTPWriteCtxtPtr;
    let mut http_content: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut content_encoding: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut content_type: *mut std::os::raw::c_char =
        b"text/xml\x00" as *const u8 as *const std::os::raw::c_char as
            *mut std::os::raw::c_char;
    let mut http_ctxt: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if ctxt.is_null() || http_mthd.is_null() { return -(1 as std::os::raw::c_int) }
    /*  Retrieve the content from the appropriate buffer  */
    if (*ctxt).compression > 0 as std::os::raw::c_int {
        content_lgth =
            xmlZMemBuffGetContent((*ctxt).doc_buff as xmlZMemBuffPtr,
                                  &mut http_content);
        content_encoding =
            b"Content-Encoding: gzip\x00" as *const u8 as *const std::os::raw::c_char
                as *mut std::os::raw::c_char
    } else {
        /*  Pull the data out of the memory output buffer  */
        let mut dctxt: xmlOutputBufferPtr =
            (*ctxt).doc_buff as xmlOutputBufferPtr;
        http_content =
            xmlBufContent((*dctxt).buffer as *const xmlBuf) as
                *mut std::os::raw::c_char;
        content_lgth = xmlBufUse((*dctxt).buffer) as std::os::raw::c_int
    }
    if http_content.is_null() {
        let mut msg: [xmlChar; 500] = [0; 500];
        xmlStrPrintf(msg.as_mut_ptr(), 500 as std::os::raw::c_int,
                     b"xmlIOHTTPCloseWrite:  %s \'%s\' %s \'%s\'.\n\x00" as
                         *const u8 as *const std::os::raw::c_char,
                     b"Error retrieving content.\nUnable to\x00" as *const u8
                         as *const std::os::raw::c_char, http_mthd,
                     b"data to URI\x00" as *const u8 as *const std::os::raw::c_char,
                     (*ctxt).uri);
        xmlIOErr(XML_IO_WRITE as std::os::raw::c_int,
                 msg.as_mut_ptr() as *const std::os::raw::c_char);
    } else {
        http_ctxt =
            xmlNanoHTTPMethod((*ctxt).uri, http_mthd, http_content,
                              &mut content_type, content_encoding,
                              content_lgth);
        if !http_ctxt.is_null() {
            /*  DEBUG_HTTP  */
            http_rtn = xmlNanoHTTPReturnCode(http_ctxt);
            if http_rtn >= 200 as std::os::raw::c_int && http_rtn < 300 as std::os::raw::c_int
               {
                close_rc = 0 as std::os::raw::c_int
            } else {
                let mut msg_0: [xmlChar; 500] = [0; 500];
                xmlStrPrintf(msg_0.as_mut_ptr(), 500 as std::os::raw::c_int,
                             b"xmlIOHTTPCloseWrite: HTTP \'%s\' of %d %s\n\'%s\' %s %d\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             http_mthd, content_lgth,
                             b"bytes to URI\x00" as *const u8 as
                                 *const std::os::raw::c_char, (*ctxt).uri,
                             b"failed.  HTTP return code:\x00" as *const u8 as
                                 *const std::os::raw::c_char, http_rtn);
                xmlIOErr(XML_IO_WRITE as std::os::raw::c_int,
                         msg_0.as_mut_ptr() as *const std::os::raw::c_char);
            }
            xmlNanoHTTPClose(http_ctxt);
            xmlFree.expect("non-null function pointer")(content_type as
                                                            *mut std::os::raw::c_void);
        }
    }
    /*  Final cleanups  */
    xmlFreeHTTPWriteCtxt(ctxt);
    return close_rc;
}
/* *
 * xmlIOHTTPClosePut
 *
 * @context:  The I/O context
 *
 * Close the transmit HTTP I/O channel and actually send data using a PUT
 * HTTP method.
 */
unsafe extern "C" fn xmlIOHTTPClosePut(mut ctxt: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    return xmlIOHTTPCloseWrite(ctxt,
                               b"PUT\x00" as *const u8 as
                                   *const std::os::raw::c_char);
}
/* *
 * xmlIOHTTPClosePost
 *
 * @context:  The I/O context
 *
 * Close the transmit HTTP I/O channel and actually send data using a POST
 * HTTP method.
 */
unsafe extern "C" fn xmlIOHTTPClosePost(mut ctxt: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    return xmlIOHTTPCloseWrite(ctxt,
                               b"POST\x00" as *const u8 as
                                   *const std::os::raw::c_char);
}
/* LIBXML_OUTPUT_ENABLED */
/* LIBXML_HTTP_ENABLED */
/* ***********************************************************************
 *									*
 *			I/O for FTP file accesses			*
 *									*
 ************************************************************************/
/* *
 * xmlIOFTPMatch:
 * @filename:  the URI for matching
 *
 * check if the URI matches an FTP one
 *
 * Returns 1 if matches, 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIOFTPMatch(mut filename: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    if xmlStrncasecmp(filename as *mut xmlChar,
                      b"ftp://\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar, 6 as std::os::raw::c_int) == 0 {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlIOFTPOpen:
 * @filename:  the URI for matching
 *
 * open an FTP I/O channel
 *
 * Returns an I/O context or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIOFTPOpen(mut filename: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    return xmlNanoFTPOpen(filename);
}
/* *
 * xmlIOFTPRead:
 * @context:  the I/O context
 * @buffer:  where to drop data
 * @len:  number of bytes to write
 *
 * Read @len bytes to @buffer from the I/O channel.
 *
 * Returns the number of bytes written
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIOFTPRead(mut context: *mut std::os::raw::c_void,
                                      mut buffer: *mut std::os::raw::c_char,
                                      mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    if buffer.is_null() || len < 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    return xmlNanoFTPRead(context,
                          &mut *buffer.offset(0 as std::os::raw::c_int as isize) as
                              *mut std::os::raw::c_char as *mut std::os::raw::c_void, len);
}
/*
 * xmlNormalizeWindowsPath is obsolete, don't use it.
 * Check xmlCanonicPath in uri.h for a better alternative.
 */
/* *
 * Default 'file://' protocol callbacks
 */
/* *
 * Default 'http://' protocol callbacks
 */
/* LIBXML_OUTPUT_ENABLED */
/* LIBXML_HTTP_ENABLED */
/* *
 * Default 'ftp://' protocol callbacks
 */
/* *
 * xmlIOFTPClose:
 * @context:  the I/O context
 *
 * Close an FTP I/O channel
 *
 * Returns 0
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIOFTPClose(mut context: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    return xmlNanoFTPClose(context);
}
/* LIBXML_FTP_ENABLED */
/* *
 * xmlRegisterInputCallbacks:
 * @matchFunc:  the xmlInputMatchCallback
 * @openFunc:  the xmlInputOpenCallback
 * @readFunc:  the xmlInputReadCallback
 * @closeFunc:  the xmlInputCloseCallback
 *
 * Register a new set of I/O callback for handling parser input.
 *
 * Returns the registered handler number or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterInputCallbacks(mut matchFunc:
                                                       xmlInputMatchCallback,
                                                   mut openFunc:
                                                       xmlInputOpenCallback,
                                                   mut readFunc:
                                                       xmlInputReadCallback,
                                                   mut closeFunc:
                                                       xmlInputCloseCallback)
 -> std::os::raw::c_int {
    if xmlInputCallbackNr >= 15 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    xmlInputCallbackTable[xmlInputCallbackNr as usize].matchcallback =
        matchFunc;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].opencallback =
        openFunc;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].readcallback =
        readFunc;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].closecallback =
        closeFunc;
    xmlInputCallbackInitialized = 1 as std::os::raw::c_int;
    let fresh0 = xmlInputCallbackNr;
    xmlInputCallbackNr = xmlInputCallbackNr + 1;
    return fresh0;
}
/* *
 * xmlRegisterOutputCallbacks:
 * @matchFunc:  the xmlOutputMatchCallback
 * @openFunc:  the xmlOutputOpenCallback
 * @writeFunc:  the xmlOutputWriteCallback
 * @closeFunc:  the xmlOutputCloseCallback
 *
 * Register a new set of I/O callback for handling output.
 *
 * Returns the registered handler number or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterOutputCallbacks(mut matchFunc:
                                                        xmlOutputMatchCallback,
                                                    mut openFunc:
                                                        xmlOutputOpenCallback,
                                                    mut writeFunc:
                                                        xmlOutputWriteCallback,
                                                    mut closeFunc:
                                                        xmlOutputCloseCallback)
 -> std::os::raw::c_int {
    if xmlOutputCallbackNr >= 15 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    xmlOutputCallbackTable[xmlOutputCallbackNr as usize].matchcallback =
        matchFunc;
    xmlOutputCallbackTable[xmlOutputCallbackNr as usize].opencallback =
        openFunc;
    xmlOutputCallbackTable[xmlOutputCallbackNr as usize].writecallback =
        writeFunc;
    xmlOutputCallbackTable[xmlOutputCallbackNr as usize].closecallback =
        closeFunc;
    xmlOutputCallbackInitialized = 1 as std::os::raw::c_int;
    let fresh1 = xmlOutputCallbackNr;
    xmlOutputCallbackNr = xmlOutputCallbackNr + 1;
    return fresh1;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlRegisterDefaultInputCallbacks:
 *
 * Registers the default compiled-in I/O handlers.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterDefaultInputCallbacks() {
    if xmlInputCallbackInitialized != 0 { return }
    xmlRegisterInputCallbacks(Some(xmlFileMatch as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char)
                                           -> std::os::raw::c_int),
                              Some(xmlFileOpen as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char)
                                           -> *mut std::os::raw::c_void),
                              Some(xmlFileRead as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void,
                                                            _:
                                                                *mut std::os::raw::c_char,
                                                            _: std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                              Some(xmlFileClose as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void)
                                           -> std::os::raw::c_int));
    xmlRegisterInputCallbacks(Some(xmlGzfileMatch as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char)
                                           -> std::os::raw::c_int),
                              Some(xmlGzfileOpen as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char)
                                           -> *mut std::os::raw::c_void),
                              Some(xmlGzfileRead as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void,
                                                            _:
                                                                *mut std::os::raw::c_char,
                                                            _: std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                              Some(xmlGzfileClose as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void)
                                           -> std::os::raw::c_int));
    /* LIBXML_ZLIB_ENABLED */
    xmlRegisterInputCallbacks(Some(xmlXzfileMatch as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char)
                                           -> std::os::raw::c_int),
                              Some(xmlXzfileOpen as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char)
                                           -> *mut std::os::raw::c_void),
                              Some(xmlXzfileRead as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void,
                                                            _:
                                                                *mut std::os::raw::c_char,
                                                            _: std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                              Some(xmlXzfileClose as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void)
                                           -> std::os::raw::c_int));
    /* LIBXML_LZMA_ENABLED */
    xmlRegisterInputCallbacks(Some(xmlIOHTTPMatch as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char)
                                           -> std::os::raw::c_int),
                              Some(xmlIOHTTPOpen as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char)
                                           -> *mut std::os::raw::c_void),
                              Some(xmlIOHTTPRead as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void,
                                                            _:
                                                                *mut std::os::raw::c_char,
                                                            _: std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                              Some(xmlIOHTTPClose as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void)
                                           -> std::os::raw::c_int));
    /* LIBXML_HTTP_ENABLED */
    xmlRegisterInputCallbacks(Some(xmlIOFTPMatch as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char)
                                           -> std::os::raw::c_int),
                              Some(xmlIOFTPOpen as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char)
                                           -> *mut std::os::raw::c_void),
                              Some(xmlIOFTPRead as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void,
                                                            _:
                                                                *mut std::os::raw::c_char,
                                                            _: std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                              Some(xmlIOFTPClose as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void)
                                           -> std::os::raw::c_int));
    /* LIBXML_FTP_ENABLED */
    xmlInputCallbackInitialized = 1 as std::os::raw::c_int;
}
/* *
 * xmlRegisterDefaultOutputCallbacks:
 *
 * Registers the default compiled-in I/O handlers.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterDefaultOutputCallbacks() {
    if xmlOutputCallbackInitialized != 0 { return }
    xmlRegisterOutputCallbacks(Some(xmlFileMatch as
                                        unsafe extern "C" fn(_:
                                                                 *const std::os::raw::c_char)
                                            -> std::os::raw::c_int),
                               Some(xmlFileOpenW as
                                        unsafe extern "C" fn(_:
                                                                 *const std::os::raw::c_char)
                                            -> *mut std::os::raw::c_void),
                               Some(xmlFileWrite as
                                        unsafe extern "C" fn(_:
                                                                 *mut std::os::raw::c_void,
                                                             _:
                                                                 *const std::os::raw::c_char,
                                                             _: std::os::raw::c_int)
                                            -> std::os::raw::c_int),
                               Some(xmlFileClose as
                                        unsafe extern "C" fn(_:
                                                                 *mut std::os::raw::c_void)
                                            -> std::os::raw::c_int));
    xmlRegisterOutputCallbacks(Some(xmlIOHTTPMatch as
                                        unsafe extern "C" fn(_:
                                                                 *const std::os::raw::c_char)
                                            -> std::os::raw::c_int),
                               Some(xmlIOHTTPDfltOpenW as
                                        unsafe extern "C" fn(_:
                                                                 *const std::os::raw::c_char)
                                            -> *mut std::os::raw::c_void),
                               Some(xmlIOHTTPWrite as
                                        unsafe extern "C" fn(_:
                                                                 *mut std::os::raw::c_void,
                                                             _:
                                                                 *const std::os::raw::c_char,
                                                             _: std::os::raw::c_int)
                                            -> std::os::raw::c_int),
                               Some(xmlIOHTTPClosePut as
                                        unsafe extern "C" fn(_:
                                                                 *mut std::os::raw::c_void)
                                            -> std::os::raw::c_int));
    /* ********************************
 No way a-priori to distinguish between gzipped files from
 uncompressed ones except opening if existing then closing
 and saving with same compression ratio ... a pain.

#ifdef LIBXML_ZLIB_ENABLED
    xmlRegisterOutputCallbacks(xmlGzfileMatch, xmlGzfileOpen,
	                       xmlGzfileWrite, xmlGzfileClose);
#endif

 Nor FTP PUT ....
#ifdef LIBXML_FTP_ENABLED
    xmlRegisterOutputCallbacks(xmlIOFTPMatch, xmlIOFTPOpen,
	                       xmlIOFTPWrite, xmlIOFTPClose);
#endif
 **********************************/
    xmlOutputCallbackInitialized = 1 as std::os::raw::c_int;
}
/* *
 * xmlRegisterHTTPPostCallbacks:
 *
 * By default, libxml submits HTTP output requests using the "PUT" method.
 * Calling this method changes the HTTP output method to use the "POST"
 * method instead.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterHTTPPostCallbacks() {
    /*  Register defaults if not done previously  */
    if xmlOutputCallbackInitialized == 0 as std::os::raw::c_int {
        xmlRegisterDefaultOutputCallbacks();
    }
    xmlRegisterOutputCallbacks(Some(xmlIOHTTPMatch as
                                        unsafe extern "C" fn(_:
                                                                 *const std::os::raw::c_char)
                                            -> std::os::raw::c_int),
                               Some(xmlIOHTTPDfltOpenW as
                                        unsafe extern "C" fn(_:
                                                                 *const std::os::raw::c_char)
                                            -> *mut std::os::raw::c_void),
                               Some(xmlIOHTTPWrite as
                                        unsafe extern "C" fn(_:
                                                                 *mut std::os::raw::c_void,
                                                             _:
                                                                 *const std::os::raw::c_char,
                                                             _: std::os::raw::c_int)
                                            -> std::os::raw::c_int),
                               Some(xmlIOHTTPClosePost as
                                        unsafe extern "C" fn(_:
                                                                 *mut std::os::raw::c_void)
                                            -> std::os::raw::c_int));
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlAllocParserInputBuffer:
 * @enc:  the charset encoding if known
 *
 * Create a buffered parser input for progressive parsing
 *
 * Returns the new parser input or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAllocParserInputBuffer(mut enc: xmlCharEncoding)
 -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlParserInputBuffer>()
                                                          as std::os::raw::c_ulong) as
            xmlParserInputBufferPtr;
    if ret.is_null() {
        xmlIOErrMemory(b"creating input buffer\x00" as *const u8 as
                           *const std::os::raw::c_char);
        return 0 as xmlParserInputBufferPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlParserInputBuffer>() as std::os::raw::c_ulong);
    (*ret).buffer =
        xmlBufCreateSize((2 as std::os::raw::c_int * *__xmlDefaultBufferSize()) as
                             size_t);
    if (*ret).buffer.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        return 0 as xmlParserInputBufferPtr
    }
    xmlBufSetAllocationScheme((*ret).buffer, XML_BUFFER_ALLOC_DOUBLEIT);
    (*ret).encoder = xmlGetCharEncodingHandler(enc);
    if !(*ret).encoder.is_null() {
        (*ret).raw =
            xmlBufCreateSize((2 as std::os::raw::c_int * *__xmlDefaultBufferSize()) as
                                 size_t)
    } else { (*ret).raw = 0 as xmlBufPtr }
    (*ret).readcallback = None;
    (*ret).closecallback = None;
    (*ret).context = 0 as *mut std::os::raw::c_void;
    (*ret).compressed = -(1 as std::os::raw::c_int);
    (*ret).rawconsumed = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    return ret;
}
/* *
 * xmlAllocOutputBuffer:
 * @encoder:  the encoding converter or NULL
 *
 * Create a buffered parser output
 *
 * Returns the new parser output or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAllocOutputBuffer(mut encoder:
                                                  xmlCharEncodingHandlerPtr)
 -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlOutputBuffer>()
                                                          as std::os::raw::c_ulong) as
            xmlOutputBufferPtr;
    if ret.is_null() {
        xmlIOErrMemory(b"creating output buffer\x00" as *const u8 as
                           *const std::os::raw::c_char);
        return 0 as xmlOutputBufferPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlOutputBuffer>() as std::os::raw::c_ulong);
    (*ret).buffer = xmlBufCreate();
    if (*ret).buffer.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        return 0 as xmlOutputBufferPtr
    }
    /* try to avoid a performance problem with Windows realloc() */
    if xmlBufGetAllocationScheme((*ret).buffer) ==
           XML_BUFFER_ALLOC_EXACT as std::os::raw::c_int {
        xmlBufSetAllocationScheme((*ret).buffer, XML_BUFFER_ALLOC_DOUBLEIT);
    }
    (*ret).encoder = encoder;
    if !encoder.is_null() {
        (*ret).conv = xmlBufCreateSize(4000 as std::os::raw::c_int as size_t);
        if (*ret).conv.is_null() {
            xmlFree.expect("non-null function pointer")(ret as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlOutputBufferPtr
        }
        /*
	 * This call is designed to initiate the encoder state
	 */
        xmlCharEncOutput(ret, 1 as std::os::raw::c_int);
    } else { (*ret).conv = 0 as xmlBufPtr }
    (*ret).writecallback = None;
    (*ret).closecallback = None;
    (*ret).context = 0 as *mut std::os::raw::c_void;
    (*ret).written = 0 as std::os::raw::c_int;
    return ret;
}
/* *
 * xmlAllocOutputBufferInternal:
 * @encoder:  the encoding converter or NULL
 *
 * Create a buffered parser output
 *
 * Returns the new parser output or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAllocOutputBufferInternal(mut encoder:
                                                          xmlCharEncodingHandlerPtr)
 -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlOutputBuffer>()
                                                          as std::os::raw::c_ulong) as
            xmlOutputBufferPtr;
    if ret.is_null() {
        xmlIOErrMemory(b"creating output buffer\x00" as *const u8 as
                           *const std::os::raw::c_char);
        return 0 as xmlOutputBufferPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlOutputBuffer>() as std::os::raw::c_ulong);
    (*ret).buffer = xmlBufCreate();
    if (*ret).buffer.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        return 0 as xmlOutputBufferPtr
    }
    /*
     * For conversion buffers we use the special IO handling
     */
    xmlBufSetAllocationScheme((*ret).buffer, XML_BUFFER_ALLOC_IO);
    (*ret).encoder = encoder;
    if !encoder.is_null() {
        (*ret).conv = xmlBufCreateSize(4000 as std::os::raw::c_int as size_t);
        if (*ret).conv.is_null() {
            xmlFree.expect("non-null function pointer")(ret as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlOutputBufferPtr
        }
        /*
	 * This call is designed to initiate the encoder state
	 */
        xmlCharEncOutput(ret, 1 as std::os::raw::c_int);
    } else { (*ret).conv = 0 as xmlBufPtr }
    (*ret).writecallback = None;
    (*ret).closecallback = None;
    (*ret).context = 0 as *mut std::os::raw::c_void;
    (*ret).written = 0 as std::os::raw::c_int;
    return ret;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlFreeParserInputBuffer:
 * @in:  a buffered parser input
 *
 * Free up the memory used by a buffered parser input
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeParserInputBuffer(mut in_0:
                                                      xmlParserInputBufferPtr) {
    if in_0.is_null() { return }
    if !(*in_0).raw.is_null() {
        xmlBufFree((*in_0).raw);
        (*in_0).raw = 0 as xmlBufPtr
    }
    if !(*in_0).encoder.is_null() { xmlCharEncCloseFunc((*in_0).encoder); }
    if (*in_0).closecallback.is_some() {
        (*in_0).closecallback.expect("non-null function pointer")((*in_0).context);
    }
    if !(*in_0).buffer.is_null() {
        xmlBufFree((*in_0).buffer);
        (*in_0).buffer = 0 as xmlBufPtr
    }
    xmlFree.expect("non-null function pointer")(in_0 as *mut std::os::raw::c_void);
}
/* *
 * xmlOutputBufferClose:
 * @out:  a buffered output
 *
 * flushes and close the output I/O channel
 * and free up all the associated resources
 *
 * Returns the number of byte written or -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferClose(mut out: xmlOutputBufferPtr)
 -> std::os::raw::c_int {
    let mut written: std::os::raw::c_int = 0;
    let mut err_rc: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if out.is_null() { return -(1 as std::os::raw::c_int) }
    if (*out).writecallback.is_some() { xmlOutputBufferFlush(out); }
    if (*out).closecallback.is_some() {
        err_rc =
            (*out).closecallback.expect("non-null function pointer")((*out).context)
    }
    written = (*out).written;
    if !(*out).conv.is_null() {
        xmlBufFree((*out).conv);
        (*out).conv = 0 as xmlBufPtr
    }
    if !(*out).encoder.is_null() { xmlCharEncCloseFunc((*out).encoder); }
    if !(*out).buffer.is_null() {
        xmlBufFree((*out).buffer);
        (*out).buffer = 0 as xmlBufPtr
    }
    if (*out).error != 0 { err_rc = -(1 as std::os::raw::c_int) }
    xmlFree.expect("non-null function pointer")(out as *mut std::os::raw::c_void);
    return if err_rc == 0 as std::os::raw::c_int { written } else { err_rc };
}
/* LIBXML_OUTPUT_ENABLED */
#[no_mangle]
pub unsafe extern "C" fn __xmlParserInputBufferCreateFilename(mut URI:
                                                                  *const std::os::raw::c_char,
                                                              mut enc:
                                                                  xmlCharEncoding)
 -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut context: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if xmlInputCallbackInitialized == 0 as std::os::raw::c_int {
        xmlRegisterDefaultInputCallbacks();
    }
    if URI.is_null() { return 0 as xmlParserInputBufferPtr }
    /*
     * Try to find one of the input accept method accepting that scheme
     * Go in reverse to give precedence to user defined handlers.
     */
    if context.is_null() {
        i = xmlInputCallbackNr - 1 as std::os::raw::c_int;
        while i >= 0 as std::os::raw::c_int {
            if xmlInputCallbackTable[i as usize].matchcallback.is_some() &&
                   xmlInputCallbackTable[i as
                                             usize].matchcallback.expect("non-null function pointer")(URI)
                       != 0 as std::os::raw::c_int {
                context =
                    xmlInputCallbackTable[i as
                                              usize].opencallback.expect("non-null function pointer")(URI);
                if !context.is_null() { break ; }
            }
            i -= 1
        }
    }
    if context.is_null() { return 0 as xmlParserInputBufferPtr }
    /*
     * Allocate the Input buffer front-end.
     */
    ret = xmlAllocParserInputBuffer(enc);
    if !ret.is_null() {
        (*ret).context = context;
        (*ret).readcallback = xmlInputCallbackTable[i as usize].readcallback;
        (*ret).closecallback =
            xmlInputCallbackTable[i as usize].closecallback;
        if xmlInputCallbackTable[i as usize].opencallback ==
               Some(xmlGzfileOpen as
                        unsafe extern "C" fn(_: *const std::os::raw::c_char)
                            -> *mut std::os::raw::c_void) &&
               strcmp(URI, b"-\x00" as *const u8 as *const std::os::raw::c_char) !=
                   0 as std::os::raw::c_int {
            (*ret).compressed =
                (gzdirect(context as gzFile) == 0) as std::os::raw::c_int
        }
        if xmlInputCallbackTable[i as usize].opencallback ==
               Some(xmlXzfileOpen as
                        unsafe extern "C" fn(_: *const std::os::raw::c_char)
                            -> *mut std::os::raw::c_void) &&
               strcmp(URI, b"-\x00" as *const u8 as *const std::os::raw::c_char) !=
                   0 as std::os::raw::c_int {
            (*ret).compressed = __libxml2_xzcompressed(context)
        }
    } else {
        xmlInputCallbackTable[i as
                                  usize].closecallback.expect("non-null function pointer")(context);
    }
    return ret;
}
/* *
 * xmlParserInputBufferCreateFilename:
 * @URI:  a C string containing the URI or filename
 * @enc:  the charset encoding if known
 *
 * Create a buffered parser input for the progressive parsing of a file
 * If filename is "-' then we use stdin as the input.
 * Automatic support for ZLIB/Compress compressed document is provided
 * by default if found at compile-time.
 * Do an encoding check if enc == XML_CHAR_ENCODING_NONE
 *
 * Returns the new parser input or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateFilename(mut URI:
                                                                *const std::os::raw::c_char,
                                                            mut enc:
                                                                xmlCharEncoding)
 -> xmlParserInputBufferPtr {
    if (*__xmlParserInputBufferCreateFilenameValue()).is_some() {
        return (*__xmlParserInputBufferCreateFilenameValue()).expect("non-null function pointer")(URI,
                                                                                                  enc)
    }
    return __xmlParserInputBufferCreateFilename(URI, enc);
}
#[no_mangle]
pub unsafe extern "C" fn __xmlOutputBufferCreateFilename(mut URI:
                                                             *const std::os::raw::c_char,
                                                         mut encoder:
                                                             xmlCharEncodingHandlerPtr,
                                                         mut compression:
                                                             std::os::raw::c_int)
 -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut puri: xmlURIPtr = 0 as *mut xmlURI;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut context: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut unescaped: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut is_file_uri: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if xmlOutputCallbackInitialized == 0 as std::os::raw::c_int {
        xmlRegisterDefaultOutputCallbacks();
    }
    if URI.is_null() { return 0 as xmlOutputBufferPtr }
    puri = xmlParseURI(URI);
    if !puri.is_null() {
        if !(*puri).scheme.is_null() &&
               xmlStrEqual((*puri).scheme as *mut xmlChar,
                           b"file\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar) == 0 {
            is_file_uri = 0 as std::os::raw::c_int
        }
        /*
	 * try to limit the damages of the URI unescaping code.
	 */
        if (*puri).scheme.is_null() ||
               xmlStrEqual((*puri).scheme as *mut xmlChar,
                           b"file\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar) != 0 {
            unescaped =
                xmlURIUnescapeString(URI, 0 as std::os::raw::c_int,
                                     0 as *mut std::os::raw::c_char)
        }
        xmlFreeURI(puri);
    }
    /*
     * Try to find one of the output accept method accepting that scheme
     * Go in reverse to give precedence to user defined handlers.
     * try with an unescaped version of the URI
     */
    if !unescaped.is_null() {
        if compression > 0 as std::os::raw::c_int && compression <= 9 as std::os::raw::c_int
               && is_file_uri == 1 as std::os::raw::c_int {
            context = xmlGzfileOpenW(unescaped, compression);
            if !context.is_null() {
                ret = xmlAllocOutputBufferInternal(encoder);
                if !ret.is_null() {
                    (*ret).context = context;
                    (*ret).writecallback =
                        Some(xmlGzfileWrite as
                                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                      _: *const std::os::raw::c_char,
                                                      _: std::os::raw::c_int)
                                     -> std::os::raw::c_int);
                    (*ret).closecallback =
                        Some(xmlGzfileClose as
                                 unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                     -> std::os::raw::c_int)
                }
                xmlFree.expect("non-null function pointer")(unescaped as
                                                                *mut std::os::raw::c_void);
                return ret
            }
        }
        i = xmlOutputCallbackNr - 1 as std::os::raw::c_int;
        while i >= 0 as std::os::raw::c_int {
            if xmlOutputCallbackTable[i as usize].matchcallback.is_some() &&
                   xmlOutputCallbackTable[i as
                                              usize].matchcallback.expect("non-null function pointer")(unescaped)
                       != 0 as std::os::raw::c_int {
                /*  Need to pass compression parameter into HTTP open calls  */
                if xmlOutputCallbackTable[i as usize].matchcallback ==
                       Some(xmlIOHTTPMatch as
                                unsafe extern "C" fn(_: *const std::os::raw::c_char)
                                    -> std::os::raw::c_int) {
                    context = xmlIOHTTPOpenW(unescaped, compression)
                } else {
                    context =
                        xmlOutputCallbackTable[i as
                                                   usize].opencallback.expect("non-null function pointer")(unescaped)
                }
                if !context.is_null() { break ; }
            }
            i -= 1
        }
        xmlFree.expect("non-null function pointer")(unescaped as
                                                        *mut std::os::raw::c_void);
    }
    /*
     * If this failed try with a non-escaped URI this may be a strange
     * filename
     */
    if context.is_null() {
        if compression > 0 as std::os::raw::c_int && compression <= 9 as std::os::raw::c_int
               && is_file_uri == 1 as std::os::raw::c_int {
            context = xmlGzfileOpenW(URI, compression);
            if !context.is_null() {
                ret = xmlAllocOutputBufferInternal(encoder);
                if !ret.is_null() {
                    (*ret).context = context;
                    (*ret).writecallback =
                        Some(xmlGzfileWrite as
                                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                      _: *const std::os::raw::c_char,
                                                      _: std::os::raw::c_int)
                                     -> std::os::raw::c_int);
                    (*ret).closecallback =
                        Some(xmlGzfileClose as
                                 unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                     -> std::os::raw::c_int)
                }
                return ret
            }
        }
        i = xmlOutputCallbackNr - 1 as std::os::raw::c_int;
        while i >= 0 as std::os::raw::c_int {
            if xmlOutputCallbackTable[i as usize].matchcallback.is_some() &&
                   xmlOutputCallbackTable[i as
                                              usize].matchcallback.expect("non-null function pointer")(URI)
                       != 0 as std::os::raw::c_int {
                /*  Need to pass compression parameter into HTTP open calls  */
                if xmlOutputCallbackTable[i as usize].matchcallback ==
                       Some(xmlIOHTTPMatch as
                                unsafe extern "C" fn(_: *const std::os::raw::c_char)
                                    -> std::os::raw::c_int) {
                    context = xmlIOHTTPOpenW(URI, compression)
                } else {
                    context =
                        xmlOutputCallbackTable[i as
                                                   usize].opencallback.expect("non-null function pointer")(URI)
                }
                if !context.is_null() { break ; }
            }
            i -= 1
        }
    }
    if context.is_null() { return 0 as xmlOutputBufferPtr }
    /*
     * Allocate the Output buffer front-end.
     */
    ret = xmlAllocOutputBufferInternal(encoder);
    if !ret.is_null() {
        (*ret).context = context;
        (*ret).writecallback =
            xmlOutputCallbackTable[i as usize].writecallback;
        (*ret).closecallback =
            xmlOutputCallbackTable[i as usize].closecallback
    }
    return ret;
}
/* *
 * xmlOutputBufferCreateFilename:
 * @URI:  a C string containing the URI or filename
 * @encoder:  the encoding converter or NULL
 * @compression:  the compression ration (0 none, 9 max).
 *
 * Create a buffered  output for the progressive saving of a file
 * If filename is "-' then we use stdout as the output.
 * Automatic support for ZLIB/Compress compressed document is provided
 * by default if found at compile-time.
 * TODO: currently if compression is set, the library only support
 *       writing to a local file.
 *
 * Returns the new output or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateFilename(mut URI:
                                                           *const std::os::raw::c_char,
                                                       mut encoder:
                                                           xmlCharEncodingHandlerPtr,
                                                       mut compression:
                                                           std::os::raw::c_int)
 -> xmlOutputBufferPtr {
    if (*__xmlOutputBufferCreateFilenameValue()).is_some() {
        return (*__xmlOutputBufferCreateFilenameValue()).expect("non-null function pointer")(URI,
                                                                                             encoder,
                                                                                             compression)
    }
    return __xmlOutputBufferCreateFilename(URI, encoder, compression);
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlParserInputBufferCreateFile:
 * @file:  a FILE*
 * @enc:  the charset encoding if known
 *
 * Create a buffered parser input for the progressive parsing of a FILE *
 * buffered C I/O
 *
 * Returns the new parser input or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateFile(mut file: *mut FILE,
                                                        mut enc:
                                                            xmlCharEncoding)
 -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if xmlInputCallbackInitialized == 0 as std::os::raw::c_int {
        xmlRegisterDefaultInputCallbacks();
    }
    if file.is_null() { return 0 as xmlParserInputBufferPtr }
    ret = xmlAllocParserInputBuffer(enc);
    if !ret.is_null() {
        (*ret).context = file as *mut std::os::raw::c_void;
        (*ret).readcallback =
            Some(xmlFileRead as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *mut std::os::raw::c_char,
                                          _: std::os::raw::c_int) -> std::os::raw::c_int);
        (*ret).closecallback =
            Some(xmlFileFlush as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                         -> std::os::raw::c_int)
    }
    return ret;
}
/* *
 * xmlOutputBufferCreateFile:
 * @file:  a FILE*
 * @encoder:  the encoding converter or NULL
 *
 * Create a buffered output for the progressive saving to a FILE *
 * buffered C I/O
 *
 * Returns the new parser output or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateFile(mut file: *mut FILE,
                                                   mut encoder:
                                                       xmlCharEncodingHandlerPtr)
 -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    if xmlOutputCallbackInitialized == 0 as std::os::raw::c_int {
        xmlRegisterDefaultOutputCallbacks();
    }
    if file.is_null() { return 0 as xmlOutputBufferPtr }
    ret = xmlAllocOutputBufferInternal(encoder);
    if !ret.is_null() {
        (*ret).context = file as *mut std::os::raw::c_void;
        (*ret).writecallback =
            Some(xmlFileWrite as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char,
                                          _: std::os::raw::c_int) -> std::os::raw::c_int);
        (*ret).closecallback =
            Some(xmlFileFlush as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                         -> std::os::raw::c_int)
    }
    return ret;
}
/* *
 * xmlOutputBufferCreateBuffer:
 * @buffer:  a xmlBufferPtr
 * @encoder:  the encoding converter or NULL
 *
 * Create a buffered output for the progressive saving to a xmlBuffer
 *
 * Returns the new parser output or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateBuffer(mut buffer: xmlBufferPtr,
                                                     mut encoder:
                                                         xmlCharEncodingHandlerPtr)
 -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    if buffer.is_null() { return 0 as xmlOutputBufferPtr }
    ret =
        xmlOutputBufferCreateIO(Some(xmlBufferWrite as
                                         unsafe extern "C" fn(_:
                                                                  *mut std::os::raw::c_void,
                                                              _:
                                                                  *const std::os::raw::c_char,
                                                              _: std::os::raw::c_int)
                                             -> std::os::raw::c_int), None,
                                buffer as *mut std::os::raw::c_void, encoder);
    return ret;
}
/* *
 * xmlOutputBufferGetContent:
 * @out:  an xmlOutputBufferPtr
 *
 * Gives a pointer to the data currently held in the output buffer
 *
 * Returns a pointer to the data or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferGetContent(mut out:
                                                       xmlOutputBufferPtr)
 -> *const xmlChar {
    if out.is_null() || (*out).buffer.is_null() { return 0 as *const xmlChar }
    return xmlBufContent((*out).buffer as *const xmlBuf);
}
/* *
 * xmlOutputBufferGetSize:
 * @out:  an xmlOutputBufferPtr
 *
 * Gives the length of the data currently held in the output buffer
 *
 * Returns 0 in case or error or no data is held, the size otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferGetSize(mut out: xmlOutputBufferPtr)
 -> size_t {
    if out.is_null() || (*out).buffer.is_null() {
        return 0 as std::os::raw::c_int as size_t
    }
    return xmlBufUse((*out).buffer);
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlParserInputBufferCreateFd:
 * @fd:  a file descriptor number
 * @enc:  the charset encoding if known
 *
 * Create a buffered parser input for the progressive parsing for the input
 * from a file descriptor
 *
 * Returns the new parser input or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateFd(mut fd: std::os::raw::c_int,
                                                      mut enc:
                                                          xmlCharEncoding)
 -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if fd < 0 as std::os::raw::c_int { return 0 as xmlParserInputBufferPtr }
    ret = xmlAllocParserInputBuffer(enc);
    if !ret.is_null() {
        (*ret).context = fd as ptrdiff_t as *mut std::os::raw::c_void;
        (*ret).readcallback =
            Some(xmlFdRead as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *mut std::os::raw::c_char,
                                          _: std::os::raw::c_int) -> std::os::raw::c_int);
        (*ret).closecallback =
            Some(xmlFdClose as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                         -> std::os::raw::c_int)
    }
    return ret;
}
/* *
 * xmlParserInputBufferCreateMem:
 * @mem:  the memory input
 * @size:  the length of the memory block
 * @enc:  the charset encoding if known
 *
 * Create a buffered parser input for the progressive parsing for the input
 * from a memory area.
 *
 * Returns the new parser input or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateMem(mut mem:
                                                           *const std::os::raw::c_char,
                                                       mut size: std::os::raw::c_int,
                                                       mut enc:
                                                           xmlCharEncoding)
 -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut errcode: std::os::raw::c_int = 0;
    if size < 0 as std::os::raw::c_int { return 0 as xmlParserInputBufferPtr }
    if mem.is_null() { return 0 as xmlParserInputBufferPtr }
    ret = xmlAllocParserInputBuffer(enc);
    if !ret.is_null() {
        (*ret).context = mem as *mut std::os::raw::c_void;
        (*ret).readcallback =
            Some(xmlInputReadCallbackNop as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *mut std::os::raw::c_char,
                                          _: std::os::raw::c_int) -> std::os::raw::c_int);
        (*ret).closecallback = None;
        errcode = xmlBufAdd((*ret).buffer, mem as *const xmlChar, size);
        if errcode != 0 as std::os::raw::c_int {
            xmlFree.expect("non-null function pointer")(ret as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlParserInputBufferPtr
        }
    }
    return ret;
}
/* *
 * xmlParserInputBufferCreateStatic:
 * @mem:  the memory input
 * @size:  the length of the memory block
 * @enc:  the charset encoding if known
 *
 * Create a buffered parser input for the progressive parsing for the input
 * from an immutable memory area. This will not copy the memory area to
 * the buffer, but the memory is expected to be available until the end of
 * the parsing, this is useful for example when using mmap'ed file.
 *
 * Returns the new parser input or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateStatic(mut mem:
                                                              *const std::os::raw::c_char,
                                                          mut size:
                                                              std::os::raw::c_int,
                                                          mut enc:
                                                              xmlCharEncoding)
 -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if size < 0 as std::os::raw::c_int { return 0 as xmlParserInputBufferPtr }
    if mem.is_null() { return 0 as xmlParserInputBufferPtr }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlParserInputBuffer>()
                                                          as std::os::raw::c_ulong) as
            xmlParserInputBufferPtr;
    if ret.is_null() {
        xmlIOErrMemory(b"creating input buffer\x00" as *const u8 as
                           *const std::os::raw::c_char);
        return 0 as xmlParserInputBufferPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlParserInputBuffer>() as std::os::raw::c_ulong);
    (*ret).buffer =
        xmlBufCreateStatic(mem as *mut std::os::raw::c_void, size as size_t);
    if (*ret).buffer.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        return 0 as xmlParserInputBufferPtr
    }
    (*ret).encoder = xmlGetCharEncodingHandler(enc);
    if !(*ret).encoder.is_null() {
        (*ret).raw =
            xmlBufCreateSize((2 as std::os::raw::c_int * *__xmlDefaultBufferSize()) as
                                 size_t)
    } else { (*ret).raw = 0 as xmlBufPtr }
    (*ret).compressed = -(1 as std::os::raw::c_int);
    (*ret).context = mem as *mut std::os::raw::c_void;
    (*ret).readcallback = None;
    (*ret).closecallback = None;
    return ret;
}
/* *
 * xmlOutputBufferCreateFd:
 * @fd:  a file descriptor number
 * @encoder:  the encoding converter or NULL
 *
 * Create a buffered output for the progressive saving
 * to a file descriptor
 *
 * Returns the new parser output or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateFd(mut fd: std::os::raw::c_int,
                                                 mut encoder:
                                                     xmlCharEncodingHandlerPtr)
 -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    if fd < 0 as std::os::raw::c_int { return 0 as xmlOutputBufferPtr }
    ret = xmlAllocOutputBufferInternal(encoder);
    if !ret.is_null() {
        (*ret).context = fd as ptrdiff_t as *mut std::os::raw::c_void;
        (*ret).writecallback =
            Some(xmlFdWrite as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char,
                                          _: std::os::raw::c_int) -> std::os::raw::c_int);
        (*ret).closecallback = None
    }
    return ret;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlParserInputBufferCreateIO:
 * @ioread:  an I/O read function
 * @ioclose:  an I/O close function
 * @ioctx:  an I/O handler
 * @enc:  the charset encoding if known
 *
 * Create a buffered parser input for the progressive parsing for the input
 * from an I/O handler
 *
 * Returns the new parser input or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateIO(mut ioread:
                                                          xmlInputReadCallback,
                                                      mut ioclose:
                                                          xmlInputCloseCallback,
                                                      mut ioctx:
                                                          *mut std::os::raw::c_void,
                                                      mut enc:
                                                          xmlCharEncoding)
 -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if ioread.is_none() { return 0 as xmlParserInputBufferPtr }
    ret = xmlAllocParserInputBuffer(enc);
    if !ret.is_null() {
        (*ret).context = ioctx;
        (*ret).readcallback = ioread;
        (*ret).closecallback = ioclose
    }
    return ret;
}
/* *
 * xmlOutputBufferCreateIO:
 * @iowrite:  an I/O write function
 * @ioclose:  an I/O close function
 * @ioctx:  an I/O handler
 * @encoder:  the charset encoding if known
 *
 * Create a buffered output for the progressive saving
 * to an I/O handler
 *
 * Returns the new parser output or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateIO(mut iowrite:
                                                     xmlOutputWriteCallback,
                                                 mut ioclose:
                                                     xmlOutputCloseCallback,
                                                 mut ioctx: *mut std::os::raw::c_void,
                                                 mut encoder:
                                                     xmlCharEncodingHandlerPtr)
 -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    if iowrite.is_none() { return 0 as xmlOutputBufferPtr }
    ret = xmlAllocOutputBufferInternal(encoder);
    if !ret.is_null() {
        (*ret).context = ioctx;
        (*ret).writecallback = iowrite;
        (*ret).closecallback = ioclose
    }
    return ret;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlParserInputBufferCreateFilenameDefault:
 * @func: function pointer to the new ParserInputBufferCreateFilenameFunc
 *
 * Registers a callback for URI input file handling
 *
 * Returns the old value of the registration function
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateFilenameDefault(mut func:
                                                                       xmlParserInputBufferCreateFilenameFunc)
 -> xmlParserInputBufferCreateFilenameFunc {
    let mut old: xmlParserInputBufferCreateFilenameFunc =
        *__xmlParserInputBufferCreateFilenameValue();
    if old.is_none() {
        old =
            Some(__xmlParserInputBufferCreateFilename as
                     unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                          _: xmlCharEncoding)
                         -> xmlParserInputBufferPtr)
    }
    let ref mut fresh2 = *__xmlParserInputBufferCreateFilenameValue();
    *fresh2 = func;
    return old;
}
/* *
 * xmlOutputBufferCreateFilenameDefault:
 * @func: function pointer to the new OutputBufferCreateFilenameFunc
 *
 * Registers a callback for URI output file handling
 *
 * Returns the old value of the registration function
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateFilenameDefault(mut func:
                                                                  xmlOutputBufferCreateFilenameFunc)
 -> xmlOutputBufferCreateFilenameFunc {
    let mut old: xmlOutputBufferCreateFilenameFunc =
        *__xmlOutputBufferCreateFilenameValue();
    if old.is_none() {
        old =
            Some(__xmlOutputBufferCreateFilename as
                     unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                          _: xmlCharEncodingHandlerPtr,
                                          _: std::os::raw::c_int)
                         -> xmlOutputBufferPtr)
    }
    let ref mut fresh3 = *__xmlOutputBufferCreateFilenameValue();
    *fresh3 = func;
    return old;
}
/* *
 * xmlParserInputBufferPush:
 * @in:  a buffered parser input
 * @len:  the size in bytes of the array.
 * @buf:  an char array
 *
 * Push the content of the arry in the input buffer
 * This routine handle the I18N transcoding to internal UTF-8
 * This is used when operating the parser in progressive (push) mode.
 *
 * Returns the number of chars read and stored in the buffer, or -1
 *         in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferPush(mut in_0:
                                                      xmlParserInputBufferPtr,
                                                  mut len: std::os::raw::c_int,
                                                  mut buf:
                                                      *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut nbchars: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut ret: std::os::raw::c_int = 0;
    if len < 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    if in_0.is_null() || (*in_0).error != 0 { return -(1 as std::os::raw::c_int) }
    if !(*in_0).encoder.is_null() {
        let mut use_0: std::os::raw::c_uint = 0;
        /*
	 * Store the data in the incoming raw buffer
	 */
        if (*in_0).raw.is_null() { (*in_0).raw = xmlBufCreate() }
        ret = xmlBufAdd((*in_0).raw, buf as *const xmlChar, len);
        if ret != 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        /*
	 * convert as much as possible to the parser reading buffer.
	 */
        use_0 = xmlBufUse((*in_0).raw) as std::os::raw::c_uint;
        nbchars = xmlCharEncInput(in_0, 1 as std::os::raw::c_int);
        if nbchars < 0 as std::os::raw::c_int {
            xmlIOErr(XML_IO_ENCODER as std::os::raw::c_int, 0 as *const std::os::raw::c_char);
            (*in_0).error = XML_IO_ENCODER as std::os::raw::c_int;
            return -(1 as std::os::raw::c_int)
        }
        (*in_0).rawconsumed =
            (*in_0).rawconsumed.wrapping_add((use_0 as
                                                  std::os::raw::c_ulong).wrapping_sub(xmlBufUse((*in_0).raw)))
    } else {
        nbchars = len;
        ret = xmlBufAdd((*in_0).buffer, buf as *mut xmlChar, nbchars);
        if ret != 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    }
    return nbchars;
}
/* *
 * endOfInput:
 *
 * When reading from an Input channel indicated end of file or error
 * don't reread from it again.
 */
unsafe extern "C" fn endOfInput(mut context: *mut std::os::raw::c_void,
                                mut buffer: *mut std::os::raw::c_char,
                                mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlParserInputBufferGrow:
 * @in:  a buffered parser input
 * @len:  indicative value of the amount of chars to read
 *
 * Grow up the content of the input buffer, the old data are preserved
 * This routine handle the I18N transcoding to internal UTF-8
 * This routine is used when operating the parser in normal (pull) mode
 *
 * TODO: one should be able to remove one extra copy by copying directly
 *       onto in->buffer or in->raw
 *
 * Returns the number of chars read and stored in the buffer, or -1
 *         in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferGrow(mut in_0:
                                                      xmlParserInputBufferPtr,
                                                  mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut buffer: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut res: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut nbchars: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if in_0.is_null() || (*in_0).error != 0 { return -(1 as std::os::raw::c_int) }
    if len <= 4000 as std::os::raw::c_int && len != 4 as std::os::raw::c_int {
        len = 4000 as std::os::raw::c_int
    }
    if xmlBufAvail((*in_0).buffer) <= 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        xmlIOErr(XML_IO_BUFFER_FULL as std::os::raw::c_int, 0 as *const std::os::raw::c_char);
        (*in_0).error = XML_IO_BUFFER_FULL as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    if xmlBufGrow((*in_0).buffer, len + 1 as std::os::raw::c_int) < 0 as std::os::raw::c_int {
        xmlIOErrMemory(b"growing input buffer\x00" as *const u8 as
                           *const std::os::raw::c_char);
        (*in_0).error = XML_ERR_NO_MEMORY as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    buffer = xmlBufEnd((*in_0).buffer) as *mut std::os::raw::c_char;
    /*
     * Call the read method for this I/O type.
     */
    if (*in_0).readcallback.is_some() {
        res =
            (*in_0).readcallback.expect("non-null function pointer")((*in_0).context,
                                                                     &mut *buffer.offset(0
                                                                                             as
                                                                                             std::os::raw::c_int
                                                                                             as
                                                                                             isize),
                                                                     len);
        if res <= 0 as std::os::raw::c_int {
            (*in_0).readcallback =
                Some(endOfInput as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *mut std::os::raw::c_char,
                                              _: std::os::raw::c_int) -> std::os::raw::c_int)
        }
    } else {
        xmlIOErr(XML_IO_NO_INPUT as std::os::raw::c_int, 0 as *const std::os::raw::c_char);
        (*in_0).error = XML_IO_NO_INPUT as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    if res < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    /*
     * try to establish compressed status of input if not done already
     */
    if (*in_0).compressed == -(1 as std::os::raw::c_int) {
        if (*in_0).readcallback ==
               Some(xmlXzfileRead as
                        unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                             _: *mut std::os::raw::c_char,
                                             _: std::os::raw::c_int) -> std::os::raw::c_int) {
            (*in_0).compressed = __libxml2_xzcompressed((*in_0).context)
        }
    }
    len = res;
    if !(*in_0).encoder.is_null() {
        let mut use_0: std::os::raw::c_uint = 0;
        /*
	 * Store the data in the incoming raw buffer
	 */
        if (*in_0).raw.is_null() { (*in_0).raw = xmlBufCreate() }
        res = xmlBufAdd((*in_0).raw, buffer as *const xmlChar, len);
        if res != 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        /*
	 * convert as much as possible to the parser reading buffer.
	 */
        use_0 = xmlBufUse((*in_0).raw) as std::os::raw::c_uint;
        nbchars = xmlCharEncInput(in_0, 1 as std::os::raw::c_int);
        if nbchars < 0 as std::os::raw::c_int {
            xmlIOErr(XML_IO_ENCODER as std::os::raw::c_int, 0 as *const std::os::raw::c_char);
            (*in_0).error = XML_IO_ENCODER as std::os::raw::c_int;
            return -(1 as std::os::raw::c_int)
        }
        (*in_0).rawconsumed =
            (*in_0).rawconsumed.wrapping_add((use_0 as
                                                  std::os::raw::c_ulong).wrapping_sub(xmlBufUse((*in_0).raw)))
    } else { nbchars = len; xmlBufAddLen((*in_0).buffer, nbchars as size_t); }
    return nbchars;
}
/* *
 * xmlParserInputBufferRead:
 * @in:  a buffered parser input
 * @len:  indicative value of the amount of chars to read
 *
 * Refresh the content of the input buffer, the old data are considered
 * consumed
 * This routine handle the I18N transcoding to internal UTF-8
 *
 * Returns the number of chars read and stored in the buffer, or -1
 *         in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferRead(mut in_0:
                                                      xmlParserInputBufferPtr,
                                                  mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if in_0.is_null() || (*in_0).error != 0 { return -(1 as std::os::raw::c_int) }
    if (*in_0).readcallback.is_some() {
        return xmlParserInputBufferGrow(in_0, len)
    } else if xmlBufGetAllocationScheme((*in_0).buffer) ==
                  XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    } else { return -(1 as std::os::raw::c_int) };
}
/* *
 * xmlOutputBufferWrite:
 * @out:  a buffered parser output
 * @len:  the size in bytes of the array.
 * @buf:  an char array
 *
 * Write the content of the array in the output I/O buffer
 * This routine handle the I18N transcoding from internal UTF-8
 * The buffer is lossless, i.e. will store in case of partial
 * or delayed writes.
 *
 * Returns the number of chars immediately written, or -1
 *         in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferWrite(mut out: xmlOutputBufferPtr,
                                              mut len: std::os::raw::c_int,
                                              mut buf: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut nbchars: std::os::raw::c_int =
        0 as std::os::raw::c_int; /* number of chars to output to I/O */
    let mut ret: std::os::raw::c_int = 0; /* return from function call */
    let mut written: std::os::raw::c_int =
        0 as std::os::raw::c_int; /* number of char written to I/O so far */
    let mut chunk: std::os::raw::c_int =
        0; /* number of byte curreent processed from buf */
    if out.is_null() || (*out).error != 0 { return -(1 as std::os::raw::c_int) }
    if len < 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    if (*out).error != 0 { return -(1 as std::os::raw::c_int) }
    loop  {
        chunk = len;
        if chunk > 4 as std::os::raw::c_int * 4000 as std::os::raw::c_int {
            chunk = 4 as std::os::raw::c_int * 4000 as std::os::raw::c_int
        }
        /*
	 * first handle encoding stuff.
	 */
        if !(*out).encoder.is_null() {
            /*
	     * Store the data in the incoming raw buffer
	     */
            if (*out).conv.is_null() { (*out).conv = xmlBufCreate() }
            ret = xmlBufAdd((*out).buffer, buf as *const xmlChar, chunk);
            if ret != 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            if xmlBufUse((*out).buffer) < 4000 as std::os::raw::c_int as std::os::raw::c_ulong
                   && chunk == len {
                break ;
            }
            /*
	     * convert as much as possible to the parser reading buffer.
	     */
            ret = xmlCharEncOutput(out, 0 as std::os::raw::c_int);
            if ret < 0 as std::os::raw::c_int && ret != -(3 as std::os::raw::c_int) {
                xmlIOErr(XML_IO_ENCODER as std::os::raw::c_int,
                         0 as *const std::os::raw::c_char);
                (*out).error = XML_IO_ENCODER as std::os::raw::c_int;
                return -(1 as std::os::raw::c_int)
            }
            nbchars = xmlBufUse((*out).conv) as std::os::raw::c_int
        } else {
            ret = xmlBufAdd((*out).buffer, buf as *const xmlChar, chunk);
            if ret != 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            nbchars = xmlBufUse((*out).buffer) as std::os::raw::c_int
        }
        buf = buf.offset(chunk as isize);
        len -= chunk;
        if nbchars < 4000 as std::os::raw::c_int && len <= 0 as std::os::raw::c_int {
            break ;
        }
        if (*out).writecallback.is_some() {
            /*
	     * second write the stuff to the I/O channel
	     */
            if !(*out).encoder.is_null() {
                ret =
                    (*out).writecallback.expect("non-null function pointer")((*out).context,
                                                                             xmlBufContent((*out).conv
                                                                                               as
                                                                                               *const xmlBuf)
                                                                                 as
                                                                                 *const std::os::raw::c_char,
                                                                             nbchars);
                if ret >= 0 as std::os::raw::c_int {
                    xmlBufShrink((*out).conv, ret as size_t);
                }
            } else {
                ret =
                    (*out).writecallback.expect("non-null function pointer")((*out).context,
                                                                             xmlBufContent((*out).buffer
                                                                                               as
                                                                                               *const xmlBuf)
                                                                                 as
                                                                                 *const std::os::raw::c_char,
                                                                             nbchars);
                if ret >= 0 as std::os::raw::c_int {
                    xmlBufShrink((*out).buffer, ret as size_t);
                }
            }
            if ret < 0 as std::os::raw::c_int {
                xmlIOErr(XML_IO_WRITE as std::os::raw::c_int,
                         0 as *const std::os::raw::c_char);
                (*out).error = XML_IO_WRITE as std::os::raw::c_int;
                return ret
            }
            (*out).written += ret
        }
        written += nbchars;
        if !(len > 0 as std::os::raw::c_int) { break ; }
    }
    return written;
}
/* *
 * xmlEscapeContent:
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @in:  a pointer to an array of unescaped UTF-8 bytes
 * @inlen:  the length of @in
 *
 * Take a block of UTF-8 chars in and escape them.
 * Returns 0 if success, or -1 otherwise
 * The value of @inlen after return is the number of octets consumed
 *     if the return value is positive, else unpredictable.
 * The value of @outlen after return is the number of octets consumed.
 */
unsafe extern "C" fn xmlEscapeContent(mut out: *mut std::os::raw::c_uchar,
                                      mut outlen: *mut std::os::raw::c_int,
                                      mut in_0: *const xmlChar,
                                      mut inlen: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut outstart: *mut std::os::raw::c_uchar = out;
    let mut base: *const std::os::raw::c_uchar = in_0;
    let mut outend: *mut std::os::raw::c_uchar = out.offset(*outlen as isize);
    let mut inend: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
    inend = in_0.offset(*inlen as isize);
    while in_0 < inend && out < outend {
        if *in_0 as std::os::raw::c_int == '<' as i32 {
            if (outend.offset_from(out) as std::os::raw::c_long) <
                   4 as std::os::raw::c_int as std::os::raw::c_long {
                break ;
            }
            let fresh4 = out;
            out = out.offset(1);
            *fresh4 = '&' as i32 as std::os::raw::c_uchar;
            let fresh5 = out;
            out = out.offset(1);
            *fresh5 = 'l' as i32 as std::os::raw::c_uchar;
            let fresh6 = out;
            out = out.offset(1);
            *fresh6 = 't' as i32 as std::os::raw::c_uchar;
            let fresh7 = out;
            out = out.offset(1);
            *fresh7 = ';' as i32 as std::os::raw::c_uchar
        } else if *in_0 as std::os::raw::c_int == '>' as i32 {
            if (outend.offset_from(out) as std::os::raw::c_long) <
                   4 as std::os::raw::c_int as std::os::raw::c_long {
                break ;
            }
            let fresh8 = out;
            out = out.offset(1);
            *fresh8 = '&' as i32 as std::os::raw::c_uchar;
            let fresh9 = out;
            out = out.offset(1);
            *fresh9 = 'g' as i32 as std::os::raw::c_uchar;
            let fresh10 = out;
            out = out.offset(1);
            *fresh10 = 't' as i32 as std::os::raw::c_uchar;
            let fresh11 = out;
            out = out.offset(1);
            *fresh11 = ';' as i32 as std::os::raw::c_uchar
        } else if *in_0 as std::os::raw::c_int == '&' as i32 {
            if (outend.offset_from(out) as std::os::raw::c_long) <
                   5 as std::os::raw::c_int as std::os::raw::c_long {
                break ;
            }
            let fresh12 = out;
            out = out.offset(1);
            *fresh12 = '&' as i32 as std::os::raw::c_uchar;
            let fresh13 = out;
            out = out.offset(1);
            *fresh13 = 'a' as i32 as std::os::raw::c_uchar;
            let fresh14 = out;
            out = out.offset(1);
            *fresh14 = 'm' as i32 as std::os::raw::c_uchar;
            let fresh15 = out;
            out = out.offset(1);
            *fresh15 = 'p' as i32 as std::os::raw::c_uchar;
            let fresh16 = out;
            out = out.offset(1);
            *fresh16 = ';' as i32 as std::os::raw::c_uchar
        } else if *in_0 as std::os::raw::c_int == '\r' as i32 {
            if (outend.offset_from(out) as std::os::raw::c_long) <
                   5 as std::os::raw::c_int as std::os::raw::c_long {
                break ;
            }
            let fresh17 = out;
            out = out.offset(1);
            *fresh17 = '&' as i32 as std::os::raw::c_uchar;
            let fresh18 = out;
            out = out.offset(1);
            *fresh18 = '#' as i32 as std::os::raw::c_uchar;
            let fresh19 = out;
            out = out.offset(1);
            *fresh19 = '1' as i32 as std::os::raw::c_uchar;
            let fresh20 = out;
            out = out.offset(1);
            *fresh20 = '3' as i32 as std::os::raw::c_uchar;
            let fresh21 = out;
            out = out.offset(1);
            *fresh21 = ';' as i32 as std::os::raw::c_uchar
        } else { let fresh22 = out; out = out.offset(1); *fresh22 = *in_0 }
        in_0 = in_0.offset(1)
    }
    *outlen =
        out.offset_from(outstart) as std::os::raw::c_long as std::os::raw::c_int;
    *inlen = in_0.offset_from(base) as std::os::raw::c_long as std::os::raw::c_int;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlOutputBufferWriteEscape:
 * @out:  a buffered parser output
 * @str:  a zero terminated UTF-8 string
 * @escaping:  an optional escaping function (or NULL)
 *
 * Write the content of the string in the output I/O buffer
 * This routine escapes the caracters and then handle the I18N
 * transcoding from internal UTF-8
 * The buffer is lossless, i.e. will store in case of partial
 * or delayed writes.
 *
 * Returns the number of chars immediately written, or -1
 *         in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferWriteEscape(mut out:
                                                        xmlOutputBufferPtr,
                                                    mut str: *const xmlChar,
                                                    mut escaping:
                                                        xmlCharEncodingOutputFunc)
 -> std::os::raw::c_int {
    let mut nbchars: std::os::raw::c_int =
        0 as std::os::raw::c_int; /* number of chars to output to I/O */
    let mut ret: std::os::raw::c_int = 0; /* return from function call */
    let mut written: std::os::raw::c_int =
        0 as std::os::raw::c_int; /* number of char written to I/O so far */
    let mut oldwritten: std::os::raw::c_int = 0 as std::os::raw::c_int; /* loop guard */
    let mut chunk: std::os::raw::c_int =
        0; /* number of byte currently processed from str */
    let mut len: std::os::raw::c_int = 0; /* number of bytes in str */
    let mut cons: std::os::raw::c_int = 0; /* byte from str consumed */
    if out.is_null() || (*out).error != 0 || str.is_null() ||
           (*out).buffer.is_null() ||
           xmlBufGetAllocationScheme((*out).buffer) ==
               XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    len = strlen(str as *const std::os::raw::c_char) as std::os::raw::c_int;
    if len < 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    if (*out).error != 0 { return -(1 as std::os::raw::c_int) }
    if escaping.is_none() {
        escaping =
            Some(xmlEscapeContent as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_uchar,
                                          _: *mut std::os::raw::c_int,
                                          _: *const xmlChar,
                                          _: *mut std::os::raw::c_int) -> std::os::raw::c_int)
    }
    loop  {
        oldwritten = written;
        /*
	 * how many bytes to consume and how many bytes to store.
	 */
        cons = len;
        chunk =
            xmlBufAvail((*out).buffer).wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong) as
                std::os::raw::c_int;
        /*
	 * make sure we have enough room to save first, if this is
	 * not the case force a flush, but make sure we stay in the loop
	 */
        if chunk < 40 as std::os::raw::c_int {
            if xmlBufGrow((*out).buffer, 100 as std::os::raw::c_int) <
                   0 as std::os::raw::c_int {
                return -(1 as std::os::raw::c_int)
            }
            oldwritten = -(1 as std::os::raw::c_int)
        } else {
            /*
	 * first handle encoding stuff.
	 */
            if !(*out).encoder.is_null() {
                /*
	     * Store the data in the incoming raw buffer
	     */
                if (*out).conv.is_null() { (*out).conv = xmlBufCreate() }
                ret =
                    escaping.expect("non-null function pointer")(xmlBufEnd((*out).buffer),
                                                                 &mut chunk,
                                                                 str,
                                                                 &mut cons);
                if ret < 0 as std::os::raw::c_int || chunk == 0 as std::os::raw::c_int {
                    /* chunk==0 => nothing done */
                    return -(1 as std::os::raw::c_int)
                }
                xmlBufAddLen((*out).buffer, chunk as size_t);
                if xmlBufUse((*out).buffer) <
                       4000 as std::os::raw::c_int as std::os::raw::c_ulong && cons == len {
                    break ;
                }
                /*
	     * convert as much as possible to the output buffer.
	     */
                ret = xmlCharEncOutput(out, 0 as std::os::raw::c_int);
                if ret < 0 as std::os::raw::c_int && ret != -(3 as std::os::raw::c_int) {
                    xmlIOErr(XML_IO_ENCODER as std::os::raw::c_int,
                             0 as *const std::os::raw::c_char);
                    (*out).error = XML_IO_ENCODER as std::os::raw::c_int;
                    return -(1 as std::os::raw::c_int)
                }
                nbchars = xmlBufUse((*out).conv) as std::os::raw::c_int
            } else {
                ret =
                    escaping.expect("non-null function pointer")(xmlBufEnd((*out).buffer),
                                                                 &mut chunk,
                                                                 str,
                                                                 &mut cons);
                if ret < 0 as std::os::raw::c_int || chunk == 0 as std::os::raw::c_int {
                    /* chunk==0 => nothing done */
                    return -(1 as std::os::raw::c_int)
                }
                xmlBufAddLen((*out).buffer, chunk as size_t);
                nbchars = xmlBufUse((*out).buffer) as std::os::raw::c_int
            }
            str = str.offset(cons as isize);
            len -= cons;
            if nbchars < 4000 as std::os::raw::c_int && len <= 0 as std::os::raw::c_int {
                break ;
            }
            if (*out).writecallback.is_some() {
                /*
	     * second write the stuff to the I/O channel
	     */
                if !(*out).encoder.is_null() {
                    ret =
                        (*out).writecallback.expect("non-null function pointer")((*out).context,
                                                                                 xmlBufContent((*out).conv
                                                                                                   as
                                                                                                   *const xmlBuf)
                                                                                     as
                                                                                     *const std::os::raw::c_char,
                                                                                 nbchars);
                    if ret >= 0 as std::os::raw::c_int {
                        xmlBufShrink((*out).conv, ret as size_t);
                    }
                } else {
                    ret =
                        (*out).writecallback.expect("non-null function pointer")((*out).context,
                                                                                 xmlBufContent((*out).buffer
                                                                                                   as
                                                                                                   *const xmlBuf)
                                                                                     as
                                                                                     *const std::os::raw::c_char,
                                                                                 nbchars);
                    if ret >= 0 as std::os::raw::c_int {
                        xmlBufShrink((*out).buffer, ret as size_t);
                    }
                }
                if ret < 0 as std::os::raw::c_int {
                    xmlIOErr(XML_IO_WRITE as std::os::raw::c_int,
                             0 as *const std::os::raw::c_char);
                    (*out).error = XML_IO_WRITE as std::os::raw::c_int;
                    return ret
                }
                (*out).written += ret
            } else if xmlBufAvail((*out).buffer) <
                          4000 as std::os::raw::c_int as std::os::raw::c_ulong {
                xmlBufGrow((*out).buffer, 4000 as std::os::raw::c_int);
            }
            written += nbchars
        }
        if !(len > 0 as std::os::raw::c_int && oldwritten != written) { break ; }
    }
    return written;
}
/* *
 * xmlOutputBufferWriteString:
 * @out:  a buffered parser output
 * @str:  a zero terminated C string
 *
 * Write the content of the string in the output I/O buffer
 * This routine handle the I18N transcoding from internal UTF-8
 * The buffer is lossless, i.e. will store in case of partial
 * or delayed writes.
 *
 * Returns the number of chars immediately written, or -1
 *         in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferWriteString(mut out:
                                                        xmlOutputBufferPtr,
                                                    mut str:
                                                        *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut len: std::os::raw::c_int = 0;
    if out.is_null() || (*out).error != 0 { return -(1 as std::os::raw::c_int) }
    if str.is_null() { return -(1 as std::os::raw::c_int) }
    len = strlen(str) as std::os::raw::c_int;
    if len > 0 as std::os::raw::c_int { return xmlOutputBufferWrite(out, len, str) }
    return len;
}
/* *
 * xmlOutputBufferFlush:
 * @out:  a buffered output
 *
 * flushes the output I/O channel
 *
 * Returns the number of byte written or -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferFlush(mut out: xmlOutputBufferPtr)
 -> std::os::raw::c_int {
    let mut nbchars: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if out.is_null() || (*out).error != 0 { return -(1 as std::os::raw::c_int) }
    /*
     * first handle encoding stuff.
     */
    if !(*out).conv.is_null() && !(*out).encoder.is_null() {
        loop 
             /*
	 * convert as much as possible to the parser output buffer.
	 */
             {
            nbchars = xmlCharEncOutput(out, 0 as std::os::raw::c_int);
            if nbchars < 0 as std::os::raw::c_int {
                xmlIOErr(XML_IO_ENCODER as std::os::raw::c_int,
                         0 as *const std::os::raw::c_char);
                (*out).error = XML_IO_ENCODER as std::os::raw::c_int;
                return -(1 as std::os::raw::c_int)
            }
            if !(nbchars != 0) { break ; }
        }
    }
    /*
     * second flush the stuff to the I/O channel
     */
    if !(*out).conv.is_null() && !(*out).encoder.is_null() &&
           (*out).writecallback.is_some() {
        ret =
            (*out).writecallback.expect("non-null function pointer")((*out).context,
                                                                     xmlBufContent((*out).conv
                                                                                       as
                                                                                       *const xmlBuf)
                                                                         as
                                                                         *const std::os::raw::c_char,
                                                                     xmlBufUse((*out).conv)
                                                                         as
                                                                         std::os::raw::c_int);
        if ret >= 0 as std::os::raw::c_int {
            xmlBufShrink((*out).conv, ret as size_t);
        }
    } else if (*out).writecallback.is_some() {
        ret =
            (*out).writecallback.expect("non-null function pointer")((*out).context,
                                                                     xmlBufContent((*out).buffer
                                                                                       as
                                                                                       *const xmlBuf)
                                                                         as
                                                                         *const std::os::raw::c_char,
                                                                     xmlBufUse((*out).buffer)
                                                                         as
                                                                         std::os::raw::c_int);
        if ret >= 0 as std::os::raw::c_int {
            xmlBufShrink((*out).buffer, ret as size_t);
        }
    }
    if ret < 0 as std::os::raw::c_int {
        xmlIOErr(XML_IO_FLUSH as std::os::raw::c_int, 0 as *const std::os::raw::c_char);
        (*out).error = XML_IO_FLUSH as std::os::raw::c_int;
        return ret
    }
    (*out).written += ret;
    return ret;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlParserGetDirectory:
 * @filename:  the path to a file
 *
 * lookup the directory for that file
 *
 * Returns a new allocated string containing the directory, or NULL.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserGetDirectory(mut filename:
                                                   *const std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut ret: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut dir: [std::os::raw::c_char; 1024] = [0; 1024];
    let mut cur: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    /* easy way by now ... wince does not have dirs! */
    if xmlInputCallbackInitialized == 0 as std::os::raw::c_int {
        xmlRegisterDefaultInputCallbacks();
    }
    if filename.is_null() { return 0 as *mut std::os::raw::c_char }
    strncpy(dir.as_mut_ptr(), filename, 1023 as std::os::raw::c_int as std::os::raw::c_ulong);
    dir[1023 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    cur =
        &mut *dir.as_mut_ptr().offset((strlen as
                                           unsafe extern "C" fn(_:
                                                                    *const std::os::raw::c_char)
                                               ->
                                                   std::os::raw::c_ulong)(dir.as_mut_ptr())
                                          as isize) as *mut std::os::raw::c_char;
    while cur > dir.as_mut_ptr() {
        if *cur as std::os::raw::c_int == '/' as i32 { break ; }
        cur = cur.offset(-1)
    }
    if *cur as std::os::raw::c_int == '/' as i32 {
        if cur == dir.as_mut_ptr() {
            dir[1 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char
        } else { *cur = 0 as std::os::raw::c_int as std::os::raw::c_char }
        ret =
            xmlMemStrdup.expect("non-null function pointer")(dir.as_mut_ptr())
    } else if !getcwd(dir.as_mut_ptr(),
                      1024 as std::os::raw::c_int as size_t).is_null() {
        dir[1023 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
        ret =
            xmlMemStrdup.expect("non-null function pointer")(dir.as_mut_ptr())
    }
    return ret;
}
/* ***************************************************************
 *								*
 *		External entities loading			*
 *								*
 ****************************************************************/
/* *
 * xmlCheckHTTPInput:
 * @ctxt: an XML parser context
 * @ret: an XML parser input
 *
 * Check an input in case it was created from an HTTP stream, in that
 * case it will handle encoding and update of the base URL in case of
 * redirection. It also checks for HTTP errors in which case the input
 * is cleanly freed up and an appropriate error is raised in context
 *
 * Returns the input or NULL in case of HTTP error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCheckHTTPInput(mut ctxt: xmlParserCtxtPtr,
                                           mut ret: xmlParserInputPtr)
 -> xmlParserInputPtr {
    if !ret.is_null() && !(*ret).buf.is_null() &&
           (*(*ret).buf).readcallback ==
               Some(xmlIOHTTPRead as
                        unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                             _: *mut std::os::raw::c_char,
                                             _: std::os::raw::c_int) -> std::os::raw::c_int)
           && !(*(*ret).buf).context.is_null() {
        let mut encoding: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
        let mut redir: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
        let mut mime: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
        let mut code: std::os::raw::c_int = 0;
        code = xmlNanoHTTPReturnCode((*(*ret).buf).context);
        if code >= 400 as std::os::raw::c_int {
            /* fatal error */
            if !(*ret).filename.is_null() {
                __xmlLoaderErr(ctxt as *mut std::os::raw::c_void,
                               b"failed to load HTTP resource \"%s\"\n\x00" as
                                   *const u8 as *const std::os::raw::c_char,
                               (*ret).filename);
            } else {
                __xmlLoaderErr(ctxt as *mut std::os::raw::c_void,
                               b"failed to load HTTP resource\n\x00" as
                                   *const u8 as *const std::os::raw::c_char,
                               0 as *const std::os::raw::c_char);
            }
            xmlFreeInputStream(ret);
            ret = 0 as xmlParserInputPtr
        } else {
            mime = xmlNanoHTTPMimeType((*(*ret).buf).context);
            if !xmlStrstr(mime as *mut xmlChar,
                          b"/xml\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar).is_null() ||
                   !xmlStrstr(mime as *mut xmlChar,
                              b"+xml\x00" as *const u8 as *const std::os::raw::c_char
                                  as *mut xmlChar).is_null() {
                encoding = xmlNanoHTTPEncoding((*(*ret).buf).context);
                if !encoding.is_null() {
                    let mut handler: xmlCharEncodingHandlerPtr =
                        0 as *mut xmlCharEncodingHandler;
                    handler = xmlFindCharEncodingHandler(encoding);
                    if !handler.is_null() {
                        xmlSwitchInputEncoding(ctxt, ret, handler);
                    } else {
                        __xmlErrEncoding(ctxt, XML_ERR_UNKNOWN_ENCODING,
                                         b"Unknown encoding %s\x00" as
                                             *const u8 as *const std::os::raw::c_char,
                                         encoding as *mut xmlChar,
                                         0 as *const xmlChar);
                    }
                    if (*ret).encoding.is_null() {
                        (*ret).encoding = xmlStrdup(encoding as *mut xmlChar)
                    }
                }
            }
            redir = xmlNanoHTTPRedir((*(*ret).buf).context);
            if !redir.is_null() {
                if !(*ret).filename.is_null() {
                    xmlFree.expect("non-null function pointer")((*ret).filename
                                                                    as
                                                                    *mut xmlChar
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
                if !(*ret).directory.is_null() {
                    xmlFree.expect("non-null function pointer")((*ret).directory
                                                                    as
                                                                    *mut xmlChar
                                                                    as
                                                                    *mut std::os::raw::c_void);
                    (*ret).directory = 0 as *const std::os::raw::c_char
                }
                (*ret).filename =
                    xmlStrdup(redir as *const xmlChar) as *mut std::os::raw::c_char
            }
        }
    }
    return ret;
}
unsafe extern "C" fn xmlNoNetExists(mut URL: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut path: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if URL.is_null() { return 0 as std::os::raw::c_int }
    if xmlStrncasecmp(URL as *mut xmlChar,
                      b"file://localhost/\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut xmlChar,
                      17 as std::os::raw::c_int) == 0 {
        path = &*URL.offset(16 as std::os::raw::c_int as isize) as *const std::os::raw::c_char
    } else if xmlStrncasecmp(URL as *mut xmlChar,
                             b"file:///\x00" as *const u8 as
                                 *const std::os::raw::c_char as *mut xmlChar,
                             8 as std::os::raw::c_int) == 0 {
        path = &*URL.offset(7 as std::os::raw::c_int as isize) as *const std::os::raw::c_char
    } else { path = URL }
    return xmlCheckFilename(path);
}
/* *
 * xmlResolveResourceFromCatalog:
 * @URL:  the URL for the entity to load
 * @ID:  the System ID for the entity to load
 * @ctxt:  the context in which the entity is called or NULL
 *
 * Resolves the URL and ID against the appropriate catalog.
 * This function is used by xmlDefaultExternalEntityLoader and
 * xmlNoNetExternalEntityLoader.
 *
 * Returns a new allocated URL, or NULL.
 */
unsafe extern "C" fn xmlResolveResourceFromCatalog(mut URL:
                                                       *const std::os::raw::c_char,
                                                   mut ID:
                                                       *const std::os::raw::c_char,
                                                   mut ctxt: xmlParserCtxtPtr)
 -> *mut xmlChar {
    let mut resource: *mut xmlChar = 0 as *mut xmlChar;
    let mut pref: xmlCatalogAllow = XML_CATA_ALLOW_NONE;
    /*
     * If the resource doesn't exists as a file,
     * try to load it from the resource pointed in the catalogs
     */
    pref = xmlCatalogGetDefaults();
    if pref as std::os::raw::c_uint !=
           XML_CATA_ALLOW_NONE as std::os::raw::c_int as std::os::raw::c_uint &&
           xmlNoNetExists(URL) == 0 {
        /*
	 * Do a local lookup
	 */
        if !ctxt.is_null() && !(*ctxt).catalogs.is_null() &&
               (pref as std::os::raw::c_uint ==
                    XML_CATA_ALLOW_ALL as std::os::raw::c_int as std::os::raw::c_uint ||
                    pref as std::os::raw::c_uint ==
                        XML_CATA_ALLOW_DOCUMENT as std::os::raw::c_int as
                            std::os::raw::c_uint) {
            resource =
                xmlCatalogLocalResolve((*ctxt).catalogs, ID as *const xmlChar,
                                       URL as *const xmlChar)
        }
        /*
	 * Try a global lookup
	 */
        if resource.is_null() &&
               (pref as std::os::raw::c_uint ==
                    XML_CATA_ALLOW_ALL as std::os::raw::c_int as std::os::raw::c_uint ||
                    pref as std::os::raw::c_uint ==
                        XML_CATA_ALLOW_GLOBAL as std::os::raw::c_int as std::os::raw::c_uint)
           {
            resource =
                xmlCatalogResolve(ID as *const xmlChar, URL as *const xmlChar)
        }
        if resource.is_null() && !URL.is_null() {
            resource = xmlStrdup(URL as *const xmlChar)
        }
        /*
	 * TODO: do an URI lookup on the reference
	 */
        if !resource.is_null() &&
               xmlNoNetExists(resource as *const std::os::raw::c_char) == 0 {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            if !ctxt.is_null() && !(*ctxt).catalogs.is_null() &&
                   (pref as std::os::raw::c_uint ==
                        XML_CATA_ALLOW_ALL as std::os::raw::c_int as std::os::raw::c_uint ||
                        pref as std::os::raw::c_uint ==
                            XML_CATA_ALLOW_DOCUMENT as std::os::raw::c_int as
                                std::os::raw::c_uint) {
                tmp = xmlCatalogLocalResolveURI((*ctxt).catalogs, resource)
            }
            if tmp.is_null() &&
                   (pref as std::os::raw::c_uint ==
                        XML_CATA_ALLOW_ALL as std::os::raw::c_int as std::os::raw::c_uint ||
                        pref as std::os::raw::c_uint ==
                            XML_CATA_ALLOW_GLOBAL as std::os::raw::c_int as
                                std::os::raw::c_uint) {
                tmp = xmlCatalogResolveURI(resource)
            }
            if !tmp.is_null() {
                xmlFree.expect("non-null function pointer")(resource as
                                                                *mut std::os::raw::c_void);
                resource = tmp
            }
        }
    }
    return resource;
}
/* *
 * xmlDefaultExternalEntityLoader:
 * @URL:  the URL for the entity to load
 * @ID:  the System ID for the entity to load
 * @ctxt:  the context in which the entity is called or NULL
 *
 * By default we don't load external entitites, yet.
 *
 * Returns a new allocated xmlParserInputPtr, or NULL.
 */
unsafe extern "C" fn xmlDefaultExternalEntityLoader(mut URL:
                                                        *const std::os::raw::c_char,
                                                    mut ID:
                                                        *const std::os::raw::c_char,
                                                    mut ctxt:
                                                        xmlParserCtxtPtr)
 -> xmlParserInputPtr {
    let mut ret: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut resource: *mut xmlChar = 0 as *mut xmlChar;
    if !ctxt.is_null() &&
           (*ctxt).options & XML_PARSE_NONET as std::os::raw::c_int != 0 {
        let mut options: std::os::raw::c_int = (*ctxt).options;
        (*ctxt).options -= XML_PARSE_NONET as std::os::raw::c_int;
        ret = xmlNoNetExternalEntityLoader(URL, ID, ctxt);
        (*ctxt).options = options;
        return ret
    }
    resource = xmlResolveResourceFromCatalog(URL, ID, ctxt);
    if resource.is_null() { resource = URL as *mut xmlChar }
    if resource.is_null() {
        if ID.is_null() {
            ID = b"NULL\x00" as *const u8 as *const std::os::raw::c_char
        }
        __xmlLoaderErr(ctxt as *mut std::os::raw::c_void,
                       b"failed to load external entity \"%s\"\n\x00" as
                           *const u8 as *const std::os::raw::c_char, ID);
        return 0 as xmlParserInputPtr
    }
    ret = xmlNewInputFromFile(ctxt, resource as *const std::os::raw::c_char);
    if !resource.is_null() && resource != URL as *mut xmlChar {
        xmlFree.expect("non-null function pointer")(resource as
                                                        *mut std::os::raw::c_void);
    }
    return ret;
}
static mut xmlCurrentExternalEntityLoader: xmlExternalEntityLoader =
    unsafe {
        Some(xmlDefaultExternalEntityLoader as
                 unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                      _: *const std::os::raw::c_char,
                                      _: xmlParserCtxtPtr)
                     -> xmlParserInputPtr)
    };
/* *
 * xmlSetExternalEntityLoader:
 * @f:  the new entity resolver function
 *
 * Changes the defaultexternal entity resolver function for the application
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSetExternalEntityLoader(mut f:
                                                        xmlExternalEntityLoader) {
    xmlCurrentExternalEntityLoader = f;
}
/* *
 * xmlGetExternalEntityLoader:
 *
 * Get the default external entity resolver function for the application
 *
 * Returns the xmlExternalEntityLoader function pointer
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetExternalEntityLoader()
 -> xmlExternalEntityLoader {
    return xmlCurrentExternalEntityLoader;
}
/*
 * Init/Cleanup
 */
/*
 * Input functions
 */
/*
 * Basic parsing Interfaces
 */
/* LIBXML_SAX1_ENABLED */
/*
 * Recovery mode
 */
/* LIBXML_SAX1_ENABLED */
/*
 * Less common routines and SAX interfaces
 */
/* LIBXML_SAX1_ENABLED */
/* LIBXML_VALID_ENABLE */
/* LIBXML_SAX1_ENABLED */
/* LIBXML_SAX1_ENABLED */
/*
 * Parser contexts handling.
 */
/* LIBXML_SAX1_ENABLED */
/*
 * Reading/setting optional parsing features.
 */
/* LIBXML_LEGACY_ENABLED */
/*
 * Interfaces for the Push mode.
 */
/* LIBXML_PUSH_ENABLED */
/*
 * Special I/O mode.
 */
/*
 * Node infos.
 */
/*
 * External entities handling actually implemented in xmlIO.
 */
/* *
 * xmlLoadExternalEntity:
 * @URL:  the URL for the entity to load
 * @ID:  the Public ID for the entity to load
 * @ctxt:  the context in which the entity is called or NULL
 *
 * Load an external entity, note that the use of this function for
 * unparsed entities may generate problems
 *
 * Returns the xmlParserInputPtr or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlLoadExternalEntity(mut URL: *const std::os::raw::c_char,
                                               mut ID: *const std::os::raw::c_char,
                                               mut ctxt: xmlParserCtxtPtr)
 -> xmlParserInputPtr {
    if !URL.is_null() && xmlNoNetExists(URL) == 0 as std::os::raw::c_int {
        let mut canonicFilename: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
        let mut ret: xmlParserInputPtr = 0 as *mut xmlParserInput;
        canonicFilename =
            xmlCanonicPath(URL as *const xmlChar) as *mut std::os::raw::c_char;
        if canonicFilename.is_null() {
            xmlIOErrMemory(b"building canonical path\n\x00" as *const u8 as
                               *const std::os::raw::c_char);
            return 0 as xmlParserInputPtr
        }
        ret =
            xmlCurrentExternalEntityLoader.expect("non-null function pointer")(canonicFilename,
                                                                               ID,
                                                                               ctxt);
        xmlFree.expect("non-null function pointer")(canonicFilename as
                                                        *mut std::os::raw::c_void);
        return ret
    }
    return xmlCurrentExternalEntityLoader.expect("non-null function pointer")(URL,
                                                                              ID,
                                                                              ctxt);
}
/*
 * Summary: interface for the I/O interfaces used by the parser
 * Description: interface for the I/O interfaces used by the parser
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * Those are the functions and datatypes for the parser input
 * I/O structures.
 */
/* *
 * xmlInputMatchCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Input API to detect if the current handler
 * can provide input fonctionnalities for this resource.
 *
 * Returns 1 if yes and 0 if another Input module should be used
 */
/* *
 * xmlInputOpenCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Input API to open the resource
 *
 * Returns an Input context or NULL in case or error
 */
/* *
 * xmlInputReadCallback:
 * @context:  an Input context
 * @buffer:  the buffer to store data read
 * @len:  the length of the buffer in bytes
 *
 * Callback used in the I/O Input API to read the resource
 *
 * Returns the number of bytes read or -1 in case of error
 */
/* *
 * xmlInputCloseCallback:
 * @context:  an Input context
 *
 * Callback used in the I/O Input API to close the resource
 *
 * Returns 0 or -1 in case of error
 */
/*
 * Those are the functions and datatypes for the library output
 * I/O structures.
 */
/* *
 * xmlOutputMatchCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Output API to detect if the current handler
 * can provide output fonctionnalities for this resource.
 *
 * Returns 1 if yes and 0 if another Output module should be used
 */
/* *
 * xmlOutputOpenCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Output API to open the resource
 *
 * Returns an Output context or NULL in case or error
 */
/* *
 * xmlOutputWriteCallback:
 * @context:  an Output context
 * @buffer:  the buffer of data to write
 * @len:  the length of the buffer in bytes
 *
 * Callback used in the I/O Output API to write to the resource
 *
 * Returns the number of bytes written or -1 in case of error
 */
/* *
 * xmlOutputCloseCallback:
 * @context:  an Output context
 *
 * Callback used in the I/O Output API to close the resource
 *
 * Returns 0 or -1 in case of error
 */
/* LIBXML_OUTPUT_ENABLED */
/* I18N conversions to UTF-8 */
/* Local buffer encoded in UTF-8 */
/* if encoder != NULL buffer for raw input */
/* -1=unknown, 0=not compressed, 1=compressed */
/* amount consumed from raw */
/* I18N conversions to UTF-8 */
/* Local buffer encoded in UTF-8 or ISOLatin */
/* if encoder != NULL buffer for output */
/* total number of byte written */
/* LIBXML_OUTPUT_ENABLED */
/*
 * Interfaces for input
 */
/*
 * Interfaces for output
 */
/* Couple of APIs to get the output without digging into the buffers */
/*  This function only exists if HTTP support built into the library  */
/* LIBXML_HTTP_ENABLED */
/* LIBXML_OUTPUT_ENABLED */
/*
 * A predefined entity loader disabling network accesses
 */
/* ***********************************************************************
 *									*
 *		Disabling Network access				*
 *									*
 ************************************************************************/
/* *
 * xmlNoNetExternalEntityLoader:
 * @URL:  the URL for the entity to load
 * @ID:  the System ID for the entity to load
 * @ctxt:  the context in which the entity is called or NULL
 *
 * A specific entity loader disabling network accesses, though still
 * allowing local catalog accesses for resolution.
 *
 * Returns a new allocated xmlParserInputPtr, or NULL.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNoNetExternalEntityLoader(mut URL:
                                                          *const std::os::raw::c_char,
                                                      mut ID:
                                                          *const std::os::raw::c_char,
                                                      mut ctxt:
                                                          xmlParserCtxtPtr)
 -> xmlParserInputPtr {
    let mut input: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut resource: *mut xmlChar = 0 as *mut xmlChar;
    resource = xmlResolveResourceFromCatalog(URL, ID, ctxt);
    if resource.is_null() { resource = URL as *mut xmlChar }
    if !resource.is_null() {
        if xmlStrncasecmp(resource,
                          b"ftp://\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar, 6 as std::os::raw::c_int) == 0 ||
               xmlStrncasecmp(resource,
                              b"http://\x00" as *const u8 as
                                  *const std::os::raw::c_char as *mut xmlChar,
                              7 as std::os::raw::c_int) == 0 {
            xmlIOErr(XML_IO_NETWORK_ATTEMPT as std::os::raw::c_int,
                     resource as *const std::os::raw::c_char);
            if resource != URL as *mut xmlChar {
                xmlFree.expect("non-null function pointer")(resource as
                                                                *mut std::os::raw::c_void);
            }
            return 0 as xmlParserInputPtr
        }
    }
    input =
        xmlDefaultExternalEntityLoader(resource as *const std::os::raw::c_char, ID,
                                       ctxt);
    if resource != URL as *mut xmlChar {
        xmlFree.expect("non-null function pointer")(resource as
                                                        *mut std::os::raw::c_void);
    }
    return input;
}
/* __INCLUDE_ELFGCCHACK */
