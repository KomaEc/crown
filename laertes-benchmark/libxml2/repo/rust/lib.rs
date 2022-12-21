#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(main)]
#![feature(ptr_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]


#[macro_use]
extern crate c2rust_bitfields;#[macro_use]
extern crate c2rust_asm_casts;




pub mod src {
pub mod HTMLparser;
pub mod HTMLtree;
pub mod SAX;
pub mod SAX2;
pub mod buf;
pub mod c14n;
pub mod catalog;
pub mod chvalid;
pub mod debugXML;
pub mod dict;
pub mod doc {
pub mod examples {
pub mod io1;
pub mod io2;
pub mod parse1;
pub mod parse2;
pub mod parse3;
pub mod parse4;
pub mod reader1;
pub mod reader2;
pub mod reader3;
pub mod reader4;
pub mod testWriter;
pub mod tree1;
pub mod tree2;
pub mod xpath1;
pub mod xpath2;
} // mod examples
} // mod doc
pub mod encoding;
pub mod entities;
pub mod error;
pub mod example {
pub mod gjobread;
} // mod example
pub mod globals;
pub mod hash;
pub mod legacy;
pub mod list;
pub mod nanoftp;
pub mod nanohttp;
pub mod parser;
pub mod parserInternals;
pub mod pattern;
pub mod python {
pub mod libxml;
pub mod libxml2_py;
pub mod types;
} // mod python
pub mod relaxng;
pub mod runsuite;
pub mod runtest;
pub mod runxmlconf;
pub mod schematron;
pub mod testAutomata;
pub mod testC14N;
pub mod testHTML;
pub mod testModule;
pub mod testReader;
pub mod testRegexp;
pub mod testRelax;
pub mod testSAX;
pub mod testSchemas;
pub mod testThreads;
pub mod testURI;
pub mod testXPath;
pub mod testapi;
pub mod testchar;
pub mod testdict;
pub mod testdso;
pub mod testlimits;
pub mod testrecurse;
pub mod threads;
pub mod tree;
pub mod uri;
pub mod valid;
pub mod variadic;
pub mod xchecks {
pub mod xchecks;
} // mod xchecks
pub mod xinclude;
pub mod xlink;
pub mod xmlIO;
pub mod xmlcatalog;
pub mod xmllint;
pub mod xmlmemory;
pub mod xmlmodule;
pub mod xmlreader;
pub mod xmlregexp;
pub mod xmlsave;
pub mod xmlschemas;
pub mod xmlschemastypes;
pub mod xmlstring;
pub mod xmlunicode;
pub mod xmlwriter;
pub mod xpath;
pub mod xpointer;
pub mod xzlib;
} // mod src

