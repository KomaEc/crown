
pub use crate::blocksort::Int32;
/*-------------------------------------------------------------*/
/*--- Table for randomising repetitive blocks               ---*/
/*---                                           randtable.c ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
   This file is part of bzip2/libbzip2, a program and library for
   lossless, block-sorting data compression.

   bzip2/libbzip2 version 1.0.8 of 13 July 2019
   Copyright (C) 1996-2019 Julian Seward <jseward@acm.org>

   Please read the WARNING, DISCLAIMER and PATENTS sections in the 
   README file.

   This program is released under the terms of the license contained
   in the file LICENSE.
   ------------------------------------------------------------------ */
/*---------------------------------------------*/
#[no_mangle]
pub static mut BZ2_rNums: [Int32; 512] =
    [619 as std::os::raw::c_int, 720 as std::os::raw::c_int, 127 as std::os::raw::c_int,
     481 as std::os::raw::c_int, 931 as std::os::raw::c_int, 816 as std::os::raw::c_int,
     813 as std::os::raw::c_int, 233 as std::os::raw::c_int, 566 as std::os::raw::c_int,
     247 as std::os::raw::c_int, 985 as std::os::raw::c_int, 724 as std::os::raw::c_int,
     205 as std::os::raw::c_int, 454 as std::os::raw::c_int, 863 as std::os::raw::c_int,
     491 as std::os::raw::c_int, 741 as std::os::raw::c_int, 242 as std::os::raw::c_int,
     949 as std::os::raw::c_int, 214 as std::os::raw::c_int, 733 as std::os::raw::c_int,
     859 as std::os::raw::c_int, 335 as std::os::raw::c_int, 708 as std::os::raw::c_int,
     621 as std::os::raw::c_int, 574 as std::os::raw::c_int, 73 as std::os::raw::c_int,
     654 as std::os::raw::c_int, 730 as std::os::raw::c_int, 472 as std::os::raw::c_int,
     419 as std::os::raw::c_int, 436 as std::os::raw::c_int, 278 as std::os::raw::c_int,
     496 as std::os::raw::c_int, 867 as std::os::raw::c_int, 210 as std::os::raw::c_int,
     399 as std::os::raw::c_int, 680 as std::os::raw::c_int, 480 as std::os::raw::c_int,
     51 as std::os::raw::c_int, 878 as std::os::raw::c_int, 465 as std::os::raw::c_int,
     811 as std::os::raw::c_int, 169 as std::os::raw::c_int, 869 as std::os::raw::c_int,
     675 as std::os::raw::c_int, 611 as std::os::raw::c_int, 697 as std::os::raw::c_int,
     867 as std::os::raw::c_int, 561 as std::os::raw::c_int, 862 as std::os::raw::c_int,
     687 as std::os::raw::c_int, 507 as std::os::raw::c_int, 283 as std::os::raw::c_int,
     482 as std::os::raw::c_int, 129 as std::os::raw::c_int, 807 as std::os::raw::c_int,
     591 as std::os::raw::c_int, 733 as std::os::raw::c_int, 623 as std::os::raw::c_int,
     150 as std::os::raw::c_int, 238 as std::os::raw::c_int, 59 as std::os::raw::c_int,
     379 as std::os::raw::c_int, 684 as std::os::raw::c_int, 877 as std::os::raw::c_int,
     625 as std::os::raw::c_int, 169 as std::os::raw::c_int, 643 as std::os::raw::c_int,
     105 as std::os::raw::c_int, 170 as std::os::raw::c_int, 607 as std::os::raw::c_int,
     520 as std::os::raw::c_int, 932 as std::os::raw::c_int, 727 as std::os::raw::c_int,
     476 as std::os::raw::c_int, 693 as std::os::raw::c_int, 425 as std::os::raw::c_int,
     174 as std::os::raw::c_int, 647 as std::os::raw::c_int, 73 as std::os::raw::c_int,
     122 as std::os::raw::c_int, 335 as std::os::raw::c_int, 530 as std::os::raw::c_int,
     442 as std::os::raw::c_int, 853 as std::os::raw::c_int, 695 as std::os::raw::c_int,
     249 as std::os::raw::c_int, 445 as std::os::raw::c_int, 515 as std::os::raw::c_int,
     909 as std::os::raw::c_int, 545 as std::os::raw::c_int, 703 as std::os::raw::c_int,
     919 as std::os::raw::c_int, 874 as std::os::raw::c_int, 474 as std::os::raw::c_int,
     882 as std::os::raw::c_int, 500 as std::os::raw::c_int, 594 as std::os::raw::c_int,
     612 as std::os::raw::c_int, 641 as std::os::raw::c_int, 801 as std::os::raw::c_int,
     220 as std::os::raw::c_int, 162 as std::os::raw::c_int, 819 as std::os::raw::c_int,
     984 as std::os::raw::c_int, 589 as std::os::raw::c_int, 513 as std::os::raw::c_int,
     495 as std::os::raw::c_int, 799 as std::os::raw::c_int, 161 as std::os::raw::c_int,
     604 as std::os::raw::c_int, 958 as std::os::raw::c_int, 533 as std::os::raw::c_int,
     221 as std::os::raw::c_int, 400 as std::os::raw::c_int, 386 as std::os::raw::c_int,
     867 as std::os::raw::c_int, 600 as std::os::raw::c_int, 782 as std::os::raw::c_int,
     382 as std::os::raw::c_int, 596 as std::os::raw::c_int, 414 as std::os::raw::c_int,
     171 as std::os::raw::c_int, 516 as std::os::raw::c_int, 375 as std::os::raw::c_int,
     682 as std::os::raw::c_int, 485 as std::os::raw::c_int, 911 as std::os::raw::c_int,
     276 as std::os::raw::c_int, 98 as std::os::raw::c_int, 553 as std::os::raw::c_int,
     163 as std::os::raw::c_int, 354 as std::os::raw::c_int, 666 as std::os::raw::c_int,
     933 as std::os::raw::c_int, 424 as std::os::raw::c_int, 341 as std::os::raw::c_int,
     533 as std::os::raw::c_int, 870 as std::os::raw::c_int, 227 as std::os::raw::c_int,
     730 as std::os::raw::c_int, 475 as std::os::raw::c_int, 186 as std::os::raw::c_int,
     263 as std::os::raw::c_int, 647 as std::os::raw::c_int, 537 as std::os::raw::c_int,
     686 as std::os::raw::c_int, 600 as std::os::raw::c_int, 224 as std::os::raw::c_int,
     469 as std::os::raw::c_int, 68 as std::os::raw::c_int, 770 as std::os::raw::c_int,
     919 as std::os::raw::c_int, 190 as std::os::raw::c_int, 373 as std::os::raw::c_int,
     294 as std::os::raw::c_int, 822 as std::os::raw::c_int, 808 as std::os::raw::c_int,
     206 as std::os::raw::c_int, 184 as std::os::raw::c_int, 943 as std::os::raw::c_int,
     795 as std::os::raw::c_int, 384 as std::os::raw::c_int, 383 as std::os::raw::c_int,
     461 as std::os::raw::c_int, 404 as std::os::raw::c_int, 758 as std::os::raw::c_int,
     839 as std::os::raw::c_int, 887 as std::os::raw::c_int, 715 as std::os::raw::c_int,
     67 as std::os::raw::c_int, 618 as std::os::raw::c_int, 276 as std::os::raw::c_int,
     204 as std::os::raw::c_int, 918 as std::os::raw::c_int, 873 as std::os::raw::c_int,
     777 as std::os::raw::c_int, 604 as std::os::raw::c_int, 560 as std::os::raw::c_int,
     951 as std::os::raw::c_int, 160 as std::os::raw::c_int, 578 as std::os::raw::c_int,
     722 as std::os::raw::c_int, 79 as std::os::raw::c_int, 804 as std::os::raw::c_int,
     96 as std::os::raw::c_int, 409 as std::os::raw::c_int, 713 as std::os::raw::c_int,
     940 as std::os::raw::c_int, 652 as std::os::raw::c_int, 934 as std::os::raw::c_int,
     970 as std::os::raw::c_int, 447 as std::os::raw::c_int, 318 as std::os::raw::c_int,
     353 as std::os::raw::c_int, 859 as std::os::raw::c_int, 672 as std::os::raw::c_int,
     112 as std::os::raw::c_int, 785 as std::os::raw::c_int, 645 as std::os::raw::c_int,
     863 as std::os::raw::c_int, 803 as std::os::raw::c_int, 350 as std::os::raw::c_int,
     139 as std::os::raw::c_int, 93 as std::os::raw::c_int, 354 as std::os::raw::c_int,
     99 as std::os::raw::c_int, 820 as std::os::raw::c_int, 908 as std::os::raw::c_int,
     609 as std::os::raw::c_int, 772 as std::os::raw::c_int, 154 as std::os::raw::c_int,
     274 as std::os::raw::c_int, 580 as std::os::raw::c_int, 184 as std::os::raw::c_int,
     79 as std::os::raw::c_int, 626 as std::os::raw::c_int, 630 as std::os::raw::c_int,
     742 as std::os::raw::c_int, 653 as std::os::raw::c_int, 282 as std::os::raw::c_int,
     762 as std::os::raw::c_int, 623 as std::os::raw::c_int, 680 as std::os::raw::c_int,
     81 as std::os::raw::c_int, 927 as std::os::raw::c_int, 626 as std::os::raw::c_int,
     789 as std::os::raw::c_int, 125 as std::os::raw::c_int, 411 as std::os::raw::c_int,
     521 as std::os::raw::c_int, 938 as std::os::raw::c_int, 300 as std::os::raw::c_int,
     821 as std::os::raw::c_int, 78 as std::os::raw::c_int, 343 as std::os::raw::c_int,
     175 as std::os::raw::c_int, 128 as std::os::raw::c_int, 250 as std::os::raw::c_int,
     170 as std::os::raw::c_int, 774 as std::os::raw::c_int, 972 as std::os::raw::c_int,
     275 as std::os::raw::c_int, 999 as std::os::raw::c_int, 639 as std::os::raw::c_int,
     495 as std::os::raw::c_int, 78 as std::os::raw::c_int, 352 as std::os::raw::c_int,
     126 as std::os::raw::c_int, 857 as std::os::raw::c_int, 956 as std::os::raw::c_int,
     358 as std::os::raw::c_int, 619 as std::os::raw::c_int, 580 as std::os::raw::c_int,
     124 as std::os::raw::c_int, 737 as std::os::raw::c_int, 594 as std::os::raw::c_int,
     701 as std::os::raw::c_int, 612 as std::os::raw::c_int, 669 as std::os::raw::c_int,
     112 as std::os::raw::c_int, 134 as std::os::raw::c_int, 694 as std::os::raw::c_int,
     363 as std::os::raw::c_int, 992 as std::os::raw::c_int, 809 as std::os::raw::c_int,
     743 as std::os::raw::c_int, 168 as std::os::raw::c_int, 974 as std::os::raw::c_int,
     944 as std::os::raw::c_int, 375 as std::os::raw::c_int, 748 as std::os::raw::c_int,
     52 as std::os::raw::c_int, 600 as std::os::raw::c_int, 747 as std::os::raw::c_int,
     642 as std::os::raw::c_int, 182 as std::os::raw::c_int, 862 as std::os::raw::c_int,
     81 as std::os::raw::c_int, 344 as std::os::raw::c_int, 805 as std::os::raw::c_int,
     988 as std::os::raw::c_int, 739 as std::os::raw::c_int, 511 as std::os::raw::c_int,
     655 as std::os::raw::c_int, 814 as std::os::raw::c_int, 334 as std::os::raw::c_int,
     249 as std::os::raw::c_int, 515 as std::os::raw::c_int, 897 as std::os::raw::c_int,
     955 as std::os::raw::c_int, 664 as std::os::raw::c_int, 981 as std::os::raw::c_int,
     649 as std::os::raw::c_int, 113 as std::os::raw::c_int, 974 as std::os::raw::c_int,
     459 as std::os::raw::c_int, 893 as std::os::raw::c_int, 228 as std::os::raw::c_int,
     433 as std::os::raw::c_int, 837 as std::os::raw::c_int, 553 as std::os::raw::c_int,
     268 as std::os::raw::c_int, 926 as std::os::raw::c_int, 240 as std::os::raw::c_int,
     102 as std::os::raw::c_int, 654 as std::os::raw::c_int, 459 as std::os::raw::c_int,
     51 as std::os::raw::c_int, 686 as std::os::raw::c_int, 754 as std::os::raw::c_int,
     806 as std::os::raw::c_int, 760 as std::os::raw::c_int, 493 as std::os::raw::c_int,
     403 as std::os::raw::c_int, 415 as std::os::raw::c_int, 394 as std::os::raw::c_int,
     687 as std::os::raw::c_int, 700 as std::os::raw::c_int, 946 as std::os::raw::c_int,
     670 as std::os::raw::c_int, 656 as std::os::raw::c_int, 610 as std::os::raw::c_int,
     738 as std::os::raw::c_int, 392 as std::os::raw::c_int, 760 as std::os::raw::c_int,
     799 as std::os::raw::c_int, 887 as std::os::raw::c_int, 653 as std::os::raw::c_int,
     978 as std::os::raw::c_int, 321 as std::os::raw::c_int, 576 as std::os::raw::c_int,
     617 as std::os::raw::c_int, 626 as std::os::raw::c_int, 502 as std::os::raw::c_int,
     894 as std::os::raw::c_int, 679 as std::os::raw::c_int, 243 as std::os::raw::c_int,
     440 as std::os::raw::c_int, 680 as std::os::raw::c_int, 879 as std::os::raw::c_int,
     194 as std::os::raw::c_int, 572 as std::os::raw::c_int, 640 as std::os::raw::c_int,
     724 as std::os::raw::c_int, 926 as std::os::raw::c_int, 56 as std::os::raw::c_int,
     204 as std::os::raw::c_int, 700 as std::os::raw::c_int, 707 as std::os::raw::c_int,
     151 as std::os::raw::c_int, 457 as std::os::raw::c_int, 449 as std::os::raw::c_int,
     797 as std::os::raw::c_int, 195 as std::os::raw::c_int, 791 as std::os::raw::c_int,
     558 as std::os::raw::c_int, 945 as std::os::raw::c_int, 679 as std::os::raw::c_int,
     297 as std::os::raw::c_int, 59 as std::os::raw::c_int, 87 as std::os::raw::c_int,
     824 as std::os::raw::c_int, 713 as std::os::raw::c_int, 663 as std::os::raw::c_int,
     412 as std::os::raw::c_int, 693 as std::os::raw::c_int, 342 as std::os::raw::c_int,
     606 as std::os::raw::c_int, 134 as std::os::raw::c_int, 108 as std::os::raw::c_int,
     571 as std::os::raw::c_int, 364 as std::os::raw::c_int, 631 as std::os::raw::c_int,
     212 as std::os::raw::c_int, 174 as std::os::raw::c_int, 643 as std::os::raw::c_int,
     304 as std::os::raw::c_int, 329 as std::os::raw::c_int, 343 as std::os::raw::c_int,
     97 as std::os::raw::c_int, 430 as std::os::raw::c_int, 751 as std::os::raw::c_int,
     497 as std::os::raw::c_int, 314 as std::os::raw::c_int, 983 as std::os::raw::c_int,
     374 as std::os::raw::c_int, 822 as std::os::raw::c_int, 928 as std::os::raw::c_int,
     140 as std::os::raw::c_int, 206 as std::os::raw::c_int, 73 as std::os::raw::c_int,
     263 as std::os::raw::c_int, 980 as std::os::raw::c_int, 736 as std::os::raw::c_int,
     876 as std::os::raw::c_int, 478 as std::os::raw::c_int, 430 as std::os::raw::c_int,
     305 as std::os::raw::c_int, 170 as std::os::raw::c_int, 514 as std::os::raw::c_int,
     364 as std::os::raw::c_int, 692 as std::os::raw::c_int, 829 as std::os::raw::c_int,
     82 as std::os::raw::c_int, 855 as std::os::raw::c_int, 953 as std::os::raw::c_int,
     676 as std::os::raw::c_int, 246 as std::os::raw::c_int, 369 as std::os::raw::c_int,
     970 as std::os::raw::c_int, 294 as std::os::raw::c_int, 750 as std::os::raw::c_int,
     807 as std::os::raw::c_int, 827 as std::os::raw::c_int, 150 as std::os::raw::c_int,
     790 as std::os::raw::c_int, 288 as std::os::raw::c_int, 923 as std::os::raw::c_int,
     804 as std::os::raw::c_int, 378 as std::os::raw::c_int, 215 as std::os::raw::c_int,
     828 as std::os::raw::c_int, 592 as std::os::raw::c_int, 281 as std::os::raw::c_int,
     565 as std::os::raw::c_int, 555 as std::os::raw::c_int, 710 as std::os::raw::c_int,
     82 as std::os::raw::c_int, 896 as std::os::raw::c_int, 831 as std::os::raw::c_int,
     547 as std::os::raw::c_int, 261 as std::os::raw::c_int, 524 as std::os::raw::c_int,
     462 as std::os::raw::c_int, 293 as std::os::raw::c_int, 465 as std::os::raw::c_int,
     502 as std::os::raw::c_int, 56 as std::os::raw::c_int, 661 as std::os::raw::c_int,
     821 as std::os::raw::c_int, 976 as std::os::raw::c_int, 991 as std::os::raw::c_int,
     658 as std::os::raw::c_int, 869 as std::os::raw::c_int, 905 as std::os::raw::c_int,
     758 as std::os::raw::c_int, 745 as std::os::raw::c_int, 193 as std::os::raw::c_int,
     768 as std::os::raw::c_int, 550 as std::os::raw::c_int, 608 as std::os::raw::c_int,
     933 as std::os::raw::c_int, 378 as std::os::raw::c_int, 286 as std::os::raw::c_int,
     215 as std::os::raw::c_int, 979 as std::os::raw::c_int, 792 as std::os::raw::c_int,
     961 as std::os::raw::c_int, 61 as std::os::raw::c_int, 688 as std::os::raw::c_int,
     793 as std::os::raw::c_int, 644 as std::os::raw::c_int, 986 as std::os::raw::c_int,
     403 as std::os::raw::c_int, 106 as std::os::raw::c_int, 366 as std::os::raw::c_int,
     905 as std::os::raw::c_int, 644 as std::os::raw::c_int, 372 as std::os::raw::c_int,
     567 as std::os::raw::c_int, 466 as std::os::raw::c_int, 434 as std::os::raw::c_int,
     645 as std::os::raw::c_int, 210 as std::os::raw::c_int, 389 as std::os::raw::c_int,
     550 as std::os::raw::c_int, 919 as std::os::raw::c_int, 135 as std::os::raw::c_int,
     780 as std::os::raw::c_int, 773 as std::os::raw::c_int, 635 as std::os::raw::c_int,
     389 as std::os::raw::c_int, 707 as std::os::raw::c_int, 100 as std::os::raw::c_int,
     626 as std::os::raw::c_int, 958 as std::os::raw::c_int, 165 as std::os::raw::c_int,
     504 as std::os::raw::c_int, 920 as std::os::raw::c_int, 176 as std::os::raw::c_int,
     193 as std::os::raw::c_int, 713 as std::os::raw::c_int, 857 as std::os::raw::c_int,
     265 as std::os::raw::c_int, 203 as std::os::raw::c_int, 50 as std::os::raw::c_int,
     668 as std::os::raw::c_int, 108 as std::os::raw::c_int, 645 as std::os::raw::c_int,
     990 as std::os::raw::c_int, 626 as std::os::raw::c_int, 197 as std::os::raw::c_int,
     510 as std::os::raw::c_int, 357 as std::os::raw::c_int, 358 as std::os::raw::c_int,
     850 as std::os::raw::c_int, 858 as std::os::raw::c_int, 364 as std::os::raw::c_int,
     936 as std::os::raw::c_int, 638 as std::os::raw::c_int];
pub fn borrow<'a, 'b: 'a, T>(p: &'a Option<&'b mut T>) -> Option<&'a T> {
    p.as_ref().map(|x| &**x)
}

pub fn borrow_mut<'a, 'b : 'a, T>(p: &'a mut Option<&'b mut T>) -> Option<&'a mut T> {
    p.as_mut().map(|x| &mut **x)
}

pub fn owned_as_ref<'a, T>(p: &'a Option<Box<T>>) -> Option<&'a T> {
    p.as_ref().map(|x| x.as_ref())
}

pub fn owned_as_mut<'a, T>(p: &'a mut Option<Box<T>>) -> Option<&'a mut T> {
    p.as_mut().map(|x| x.as_mut())
}

pub fn option_to_raw<T>(p: Option<&T>) -> * const T {
    p.map_or(core::ptr::null(), |p| p as * const T)
}

pub fn _ref_eq<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) == option_to_raw(q)
}

pub fn _ref_ne<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) != option_to_raw(q)
}

/*-------------------------------------------------------------*/
/*--- end                                       randtable.c ---*/
/*-------------------------------------------------------------*/
