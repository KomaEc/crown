#ifndef __LIBXML2_VARIADIC_H
#define __LIBXML2_VARIADIC_H

#include <libxml/xmlreader.h>
#include <libxml/nanoftp.h>
// #include <libxml/xmlwriter.h>

#include <netinet/in.h>
#include <sys/socket.h>
#ifdef LIBXML_XINCLUDE_ENABLED
#include <libxml/xinclude.h>
#endif
#ifdef LIBXML_PATTERN_ENABLED
#include <libxml/pattern.h>
#endif

#ifndef _WINSOCKAPI_
#if !defined(__BEOS__) || defined(__HAIKU__)
#include <sys/socket.h>
#define closesocket(s) close(s)
#endif
#endif

// from xmllint.c:
void xmlHTMLEncodeSend(void);
#if defined(HAVE_GETTIMEOFDAY)
void XMLCDECL LIBXML_ATTR_FORMAT(1,2) xmllint_endTimer(const char *fmt, ...);
#elif defined(HAVE_TIME_H)
void XMLCDECL LIBXML_ATTR_FORMAT(1,2) xmllint_endTimer(const char *fmt, ...);
#else
void XMLCDECL LIBXML_ATTR_FORMAT(1,2) xmllint_endTimer(char *format, ...);
#endif
void XMLCDECL LIBXML_ATTR_FORMAT(2,3) xmlHTMLError(void *ctx, const char *msg, ...);
void XMLCDECL LIBXML_ATTR_FORMAT(2,3) xmlHTMLWarning(void *ctx, const char *msg, ...);
void XMLCDECL LIBXML_ATTR_FORMAT(2,3) xmlHTMLValidityError(void *ctx, const char *msg, ...);
void XMLCDECL LIBXML_ATTR_FORMAT(2,3) xmlHTMLValidityWarning(void *ctx, const char *msg, ...);
void XMLCDECL LIBXML_ATTR_FORMAT(2,3) xmllint_warningDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...);
void XMLCDECL LIBXML_ATTR_FORMAT(2,3) xmllint_errorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...);
void XMLCDECL LIBXML_ATTR_FORMAT(2,3) xmllint_fatalErrorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...);

// from xmlreader.c:
typedef enum {
    XML_TEXTREADER_NONE = -1,
    XML_TEXTREADER_START= 0,
    XML_TEXTREADER_ELEMENT= 1,
    XML_TEXTREADER_END= 2,
    XML_TEXTREADER_EMPTY= 3,
    XML_TEXTREADER_BACKTRACK= 4,
    XML_TEXTREADER_DONE= 5,
    XML_TEXTREADER_ERROR= 6
} xmlTextReaderState;

typedef enum {
    XML_TEXTREADER_NOT_VALIDATE = 0,
    XML_TEXTREADER_VALIDATE_DTD = 1,
    XML_TEXTREADER_VALIDATE_RNG = 2,
    XML_TEXTREADER_VALIDATE_XSD = 4
} xmlTextReaderValidate;
struct _xmlTextReader {
    int				mode;	/* the parsing mode */
    xmlDocPtr			doc;    /* when walking an existing doc */
    xmlTextReaderValidate       validate;/* is there any validation */
    int				allocs;	/* what structure were deallocated */
    xmlTextReaderState		state;
    xmlParserCtxtPtr		ctxt;	/* the parser context */
    xmlSAXHandlerPtr		sax;	/* the parser SAX callbacks */
    xmlParserInputBufferPtr	input;	/* the input */
    startElementSAXFunc		startElement;/* initial SAX callbacks */
    endElementSAXFunc		endElement;  /* idem */
    startElementNsSAX2Func	startElementNs;/* idem */
    endElementNsSAX2Func	endElementNs;  /* idem */
    charactersSAXFunc		characters;
    cdataBlockSAXFunc		cdataBlock;
    unsigned int		base;	/* base of the segment in the input */
    unsigned int		cur;	/* current position in the input */
    xmlNodePtr			node;	/* current node */
    xmlNodePtr			curnode;/* current attribute node */
    int				depth;  /* depth of the current node */
    xmlNodePtr			faketext;/* fake xmlNs chld */
    int				preserve;/* preserve the resulting document */
    xmlBufPtr		        buffer; /* used to return const xmlChar * */
    xmlDictPtr			dict;	/* the context dictionary */

    /* entity stack when traversing entities content */
    xmlNodePtr         ent;          /* Current Entity Ref Node */
    int                entNr;        /* Depth of the entities stack */
    int                entMax;       /* Max depth of the entities stack */
    xmlNodePtr        *entTab;       /* array of entities */

    /* error handling */
    xmlTextReaderErrorFunc errorFunc;    /* callback function */
    void                  *errorFuncArg; /* callback function user argument */

#ifdef LIBXML_SCHEMAS_ENABLED
    /* Handling of RelaxNG validation */
    xmlRelaxNGPtr          rngSchemas;	/* The Relax NG schemas */
    xmlRelaxNGValidCtxtPtr rngValidCtxt;/* The Relax NG validation context */
    int                    rngPreserveCtxt; /* 1 if the context was provided by the user */
    int                    rngValidErrors;/* The number of errors detected */
    xmlNodePtr             rngFullNode;	/* the node if RNG not progressive */
    /* Handling of Schemas validation */
    xmlSchemaPtr          xsdSchemas;	/* The Schemas schemas */
    xmlSchemaValidCtxtPtr xsdValidCtxt;/* The Schemas validation context */
    int                   xsdPreserveCtxt; /* 1 if the context was provided by the user */
    int                   xsdValidErrors;/* The number of errors detected */
    xmlSchemaSAXPlugPtr   xsdPlug;	/* the schemas plug in SAX pipeline */
#endif
#ifdef LIBXML_XINCLUDE_ENABLED
    /* Handling of XInclude processing */
    int                xinclude;	/* is xinclude asked for */
    const xmlChar *    xinclude_name;	/* the xinclude name from dict */
    xmlXIncludeCtxtPtr xincctxt;	/* the xinclude context */
    int                in_xinclude;	/* counts for xinclude */
#endif
#ifdef LIBXML_PATTERN_ENABLED
    int                patternNr;       /* number of preserve patterns */
    int                patternMax;      /* max preserve patterns */
    xmlPatternPtr     *patternTab;      /* array of preserve patterns */
#endif
    int                preserves;	/* level of preserves */
    int                parserFlags;	/* the set of options set */
    /* Structured error handling */
    xmlStructuredErrorFunc sErrorFunc;  /* callback function */
};

// from xmlschemas.c:
struct _xmlSchemaSAXPlug {
    unsigned int magic;

    /* the original callbacks informations */
    xmlSAXHandlerPtr     *user_sax_ptr;
    xmlSAXHandlerPtr      user_sax;
    void                **user_data_ptr;
    void                 *user_data;

    /* the block plugged back and validation informations */
    xmlSAXHandler         schemas_sax;
    xmlSchemaValidCtxtPtr ctxt;
};

// from nanoftp.c:
#define FTP_BUF_SIZE		1024
typedef struct xmlNanoFTPCtxt {
    char *protocol;	/* the protocol name */
    char *hostname;	/* the host name */
    int port;		/* the port */
    char *path;		/* the path within the URL */
    char *user;		/* user string */
    char *passwd;	/* passwd string */
#ifdef SUPPORT_IP6
    struct sockaddr_storage ftpAddr; /* this is large enough to hold IPv6 address*/
#else
    struct sockaddr_in ftpAddr; /* the socket address struct */
#endif
    int passive;	/* currently we support only passive !!! */
    SOCKET controlFd;	/* the file descriptor for the control socket */
    SOCKET dataFd;	/* the file descriptor for the data socket */
    int state;		/* WRITE / READ / CLOSED */
    int returnValue;	/* the protocol return value */
    /* buffer for data received from the control connection */
    char controlBuf[FTP_BUF_SIZE + 1];
    int controlBufIndex;
    int controlBufUsed;
    int controlBufAnswer;
} xmlNanoFTPCtxt, *xmlNanoFTPCtxtPtr;

// from error.c:
void xmlReportError(xmlErrorPtr err, xmlParserCtxtPtr ctxt, const char *str, xmlGenericErrorFunc channel, void *data);
#define XML_GET_VAR_STR(msg, str) {				\
    int       size, prev_size = -1;				\
    int       chars;						\
    char      *larger;						\
    va_list   ap;						\
								\
    str = (char *) xmlMalloc(150);				\
    if (str != NULL) {						\
								\
    size = 150;							\
								\
    while (size < 64000) {					\
	va_start(ap, msg);					\
	chars = vsnprintf(str, size, msg, ap);			\
	va_end(ap);						\
	if ((chars > -1) && (chars < size)) {			\
	    if (prev_size == chars) {				\
		break;						\
	    } else {						\
		prev_size = chars;				\
	    }							\
	}							\
	if (chars > -1)						\
	    size += chars + 1;					\
	else							\
	    size += 100;					\
	if ((larger = (char *) xmlRealloc(str, size)) == NULL) {\
	    break;						\
	}							\
	str = larger;						\
    }}								\
}
void XMLCDECL xmlGenericErrorDefaultFunc (void *ctx ATTRIBUTE_UNUSED, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
// already in include/libxml2/xmlerror.h?:
// void XMLCDECL __xmlRaiseError(xmlStructuredErrorFunc schannel, xmlGenericErrorFunc channel, void *data, void *ctx,
//                               void *nod, int domain, int code, xmlErrorLevel level, const char *file, int line,
//                               const char *str1, const char *str2, const char *str3, int int1, int col, const char *msg, ...);
// void XMLCDECL xmlParserError(void *ctx, const char *msg, ...);
// void XMLCDECL xmlParserWarning(void *ctx, const char *msg, ...);
// void XMLCDECL xmlParserValidityError(void *ctx, const char *msg, ...);
// void XMLCDECL xmlParserValidityWarning(void *ctx, const char *msg, ...);


// from valid.c:
void XMLCDECL xmlNoValidityErr(void *ctx ATTRIBUTE_UNUSED, const char *msg ATTRIBUTE_UNUSED, ...);

// from xmlreader.c:
void xmlTextReaderGenericError(void *ctxt, xmlParserSeverities severity, char *str);
void XMLCDECL xmlTextReaderValidityErrorRelay(void *ctx, const char *msg, ...);
void XMLCDECL xmlTextReaderValidityWarningRelay(void *ctx, const char *msg, ...);
void XMLCDECL LIBXML_ATTR_FORMAT(2,3) xmlTextReaderError(void *ctxt, const char *msg, ...);
void XMLCDECL LIBXML_ATTR_FORMAT(2,3) xmlTextReaderWarning(void *ctxt, const char *msg, ...);
void XMLCDECL xmlTextReaderValidityError(void *ctxt, const char *msg, ...);
void XMLCDECL xmlTextReaderValidityWarning(void *ctxt, const char *msg, ...);
char *xmlTextReaderBuildMessage(const char *msg, va_list ap);

// from xmlchemas.c:
void XMLCDECL warningSplit(void *ctx, const char *msg ATTRIBUTE_UNUSED, ...);
void XMLCDECL errorSplit(void *ctx, const char *msg ATTRIBUTE_UNUSED, ...);
void XMLCDECL fatalErrorSplit(void *ctx, const char *msg ATTRIBUTE_UNUSED, ...);

// from xmlstring.c:
// int XMLCDECL xmlStrPrintf(xmlChar *buf, int len, const char *msg, ...);

// from xmlwriter.c:
// already in include/libxml/xmlwriter.h?:
// int XMLCDECL xmlTextWriterWriteFormatComment(xmlTextWriterPtr writer, const char *format, ...);
// int XMLCDECL xmlTextWriterWriteFormatRaw(xmlTextWriterPtr writer, const char *format, ...);
// int XMLCDECL xmlTextWriterWriteFormatString(xmlTextWriterPtr writer, const char *format, ...);
// int XMLCDECL xmlTextWriterWriteFormatAttribute(xmlTextWriterPtr writer, const xmlChar * name, const char *format, ...);
// int XMLCDECL xmlTextWriterWriteFormatAttributeNS(xmlTextWriterPtr writer, const xmlChar * prefix, const xmlChar * name, const xmlChar * namespaceURI, const char *format, ...);
// int XMLCDECL xmlTextWriterWriteFormatElement(xmlTextWriterPtr writer, const xmlChar * name, const char *format, ...);
// int XMLCDECL xmlTextWriterWriteFormatElementNS(xmlTextWriterPtr writer, const xmlChar * prefix, const xmlChar * name, const xmlChar * namespaceURI, const char *format, ...);
// int XMLCDECL xmlTextWriterWriteFormatPI(xmlTextWriterPtr writer, const xmlChar * target, const char *format, ...);
// int XMLCDECL xmlTextWriterWriteFormatCDATA(xmlTextWriterPtr writer, const char *format, ...);
// int XMLCDECL xmlTextWriterWriteFormatDTD(xmlTextWriterPtr writer, const xmlChar * name, const xmlChar * pubid, const xmlChar * sysid, const char *format, ...);
// int XMLCDECL xmlTextWriterWriteFormatDTDElement(xmlTextWriterPtr writer, const xmlChar * name, const char *format, ...);
// int XMLCDECL xmlTextWriterWriteFormatDTDAttlist(xmlTextWriterPtr writer, const xmlChar * name, const char *format, ...);
// int XMLCDECL xmlTextWriterWriteFormatDTDInternalEntity(xmlTextWriterPtr writer, int pe, const xmlChar * name, const char *format, ...);

/*
 * testlimits.c
 * changed from `func` to `func_testlimits`
 */
void XMLCDECL channel_testlimits(void *ctx  ATTRIBUTE_UNUSED, const char *msg, ...);
void XMLCDECL warningCallback_testlimits(void *ctx ATTRIBUTE_UNUSED, const char *msg ATTRIBUTE_UNUSED, ...);
void XMLCDECL errorCallback_testlimits(void *ctx ATTRIBUTE_UNUSED, const char *msg ATTRIBUTE_UNUSED, ...);
void XMLCDECL fatalErrorCallback_testlimits(void *ctx ATTRIBUTE_UNUSED, const char *msg ATTRIBUTE_UNUSED, ...);

/*
 * nanoftp.c
 * xmlNanoFTPCheckResponse uses asm, so linking against it is possibly the best choice
 */
extern int xmlNanoFTPReadResponse(void *ctx);
// Already included in another header afaict:
// int xmlNanoFTPCheckResponse(void *ctx);

/*
 * testapi.c
 * moved the `channel` function as well as some variables, and renamed them.
 */
void XMLCDECL channel_testrecurse(void *ctx  ATTRIBUTE_UNUSED, const char *msg, ...);

// from testHTML.c:
void XMLCDECL testHTML_warningDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...);
void XMLCDECL testHTML_errorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...);
void XMLCDECL testHTML_fatalErrorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...);

// from testSAX.c:
void XMLCDECL testSAX_warningDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...);
void XMLCDECL testSAX_errorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...);
void XMLCDECL testSAX_fatalErrorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...);
void testSAX_startTimer(void);
void XMLCDECL testSAX_endTimer(const char *format, ...);

// from runtest.c:
void XMLCDECL channel_runtest(void *ctx  ATTRIBUTE_UNUSED, const char *msg, ...);
void XMLCDECL testErrorHandler_runtest(void *ctx  ATTRIBUTE_UNUSED, const char *msg, ...);
void XMLCDECL warningDebug_runtest(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...);
void XMLCDECL errorDebug_runtest(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...);
void XMLCDECL fatalErrorDebug_runtest(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...);
void ignoreGenericError_runtest(void *ctx ATTRIBUTE_UNUSED, const char *msg ATTRIBUTE_UNUSED, ...);

#endif
