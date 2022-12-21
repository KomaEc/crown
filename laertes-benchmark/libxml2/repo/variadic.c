#define IN_LIBXML
#include "libxml.h"

#include <libxml/xmlexports.h>
#include <libxml/xmlversion.h>
#include <libxml/xmlerror.h>
#include <libxml/xmlreader.h>
#include <libxml/xmlwriter.h>
#include <libxml/xmlschemas.h>
#include <stdlib.h>
#include <string.h>

#ifdef HAVE_SYS_SELECT_H
#include <sys/select.h>
#endif

#include "variadic.h"

// from xmllint.c:
int xmllint_callbacks;
char xmllint_buffer[50000];
typedef enum {
    XMLLINT_RETURN_OK = 0,	/* No error */
    XMLLINT_ERR_UNCLASS = 1,	/* Unclassified */
    XMLLINT_ERR_DTD = 2,	/* Error in DTD */
    XMLLINT_ERR_VALID = 3,	/* Validation error */
    XMLLINT_ERR_RDFILE = 4,	/* CtxtReadFile error */
    XMLLINT_ERR_SCHEMACOMP = 5,	/* Schema compilation */
    XMLLINT_ERR_OUT = 6,	/* Error writing output */
    XMLLINT_ERR_SCHEMAPAT = 7,	/* Error in schema pattern */
    XMLLINT_ERR_RDREGIS = 8,	/* Error in Reader registration */
    XMLLINT_ERR_MEM = 9,	/* Out of memory error */
    XMLLINT_ERR_XPATH = 10	/* XPath evaluation error */
} xmllintReturnCode;
xmllintReturnCode xmllint_progresult = XMLLINT_RETURN_OK;
int xmllint_noout = 0;

// from xmllint.c
#if defined(HAVE_GETTIMEOFDAY)
struct timeval xmllint_begin, xmllint_end;
void XMLCDECL LIBXML_ATTR_FORMAT(1,2)
xmllint_endTimer(const char *fmt, ...)
{
    long msec;
    va_list ap;

    gettimeofday(&xmllint_end, NULL);
    msec = xmllint_end.tv_sec - xmllint_begin.tv_sec;
    msec *= 1000;
    msec += (xmllint_end.tv_usec - xmllint_begin.tv_usec) / 1000;

#ifndef HAVE_STDARG_H
#error "endTimer required stdarg functions"
#endif
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);

    fprintf(stderr, " took %ld ms\n", msec);
}
#elif defined(HAVE_TIME_H)
void XMLCDECL LIBXML_ATTR_FORMAT(1,2)
xmllint_endTimer(const char *fmt, ...)
{
    long msec;
    va_list ap;

    xmllint_end = clock();
    msec = ((xmllint_end - xmllint_begin) * 1000) / CLOCKS_PER_SEC;

#ifndef HAVE_STDARG_H
#error "xmllint_endTimer required stdarg functions"
#endif
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    fprintf(stderr, " took %ld ms\n", msec);
}
#else
void XMLCDECL LIBXML_ATTR_FORMAT(1,2)
xmllint_endTimer(char *format, ...)
{
    /*
     * We cannot do anything because we don't have a timing function
     */
#ifdef HAVE_STDARG_H
    va_list ap;
    va_start(ap, format);
    vfprintf(stderr, format, ap);
    va_end(ap);
    fprintf(stderr, " was not timed\n");
#else
    /* We don't have gettimeofday, time or stdarg.h, what crazy world is
     * this ?!
     */
#endif
}
#endif

void
xmlHTMLPrintFileContext(xmlParserInputPtr input) {
    const xmlChar *cur, *base;
    int len;
    int n;

    if (input == NULL) return;
    xmlGenericError(xmlGenericErrorContext, "<pre>\n");
    cur = input->cur;
    base = input->base;
    while ((cur > base) && ((*cur == '\n') || (*cur == '\r'))) {
	cur--;
    }
    n = 0;
    while ((n++ < 80) && (cur > base) && (*cur != '\n') && (*cur != '\r'))
        cur--;
    if ((*cur == '\n') || (*cur == '\r')) cur++;
    base = cur;
    n = 0;
    while ((*cur != 0) && (*cur != '\n') && (*cur != '\r') && (n < 79)) {
	len = strlen(xmllint_buffer);
        snprintf(&xmllint_buffer[len], sizeof(xmllint_buffer) - len, "%c",
		    (unsigned char) *cur++);
	n++;
    }
    len = strlen(xmllint_buffer);
    snprintf(&xmllint_buffer[len], sizeof(xmllint_buffer) - len, "\n");
    cur = input->cur;
    while ((*cur == '\n') || (*cur == '\r'))
	cur--;
    n = 0;
    while ((cur != base) && (n++ < 80)) {
	len = strlen(xmllint_buffer);
        snprintf(&xmllint_buffer[len], sizeof(xmllint_buffer) - len, " ");
        base++;
    }
    len = strlen(xmllint_buffer);
    snprintf(&xmllint_buffer[len], sizeof(xmllint_buffer) - len, "^\n");
    xmlHTMLEncodeSend();
    xmlGenericError(xmlGenericErrorContext, "</pre>");
}
void
xmlHTMLEncodeSend(void) {
    char *result;

    result = (char *) xmlEncodeEntitiesReentrant(NULL, BAD_CAST xmllint_buffer);
    if (result) {
	xmlGenericError(xmlGenericErrorContext, "%s", result);
	xmlFree(result);
    }
    xmllint_buffer[0] = 0;
}
void
xmlHTMLPrintFileInfo(xmlParserInputPtr input) {
    int len;
    xmlGenericError(xmlGenericErrorContext, "<p>");

    len = strlen(xmllint_buffer);
    if (input != NULL) {
	if (input->filename) {
	    snprintf(&xmllint_buffer[len], sizeof(xmllint_buffer) - len, "%s:%d: ", input->filename,
		    input->line);
	} else {
	    snprintf(&xmllint_buffer[len], sizeof(xmllint_buffer) - len, "Entity: line %d: ", input->line);
	}
    }
    xmlHTMLEncodeSend();
}
void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
xmlHTMLError(void *ctx, const char *msg, ...)
{
    xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx;
    xmlParserInputPtr input;
    va_list args;
    int len;

    xmllint_buffer[0] = 0;
    input = ctxt->input;
    if ((input != NULL) && (input->filename == NULL) && (ctxt->inputNr > 1)) {
        input = ctxt->inputTab[ctxt->inputNr - 2];
    }

    xmlHTMLPrintFileInfo(input);

    xmlGenericError(xmlGenericErrorContext, "<b>error</b>: ");
    va_start(args, msg);
    len = strlen(xmllint_buffer);
    vsnprintf(&xmllint_buffer[len],  sizeof(xmllint_buffer) - len, msg, args);
    va_end(args);
    xmlHTMLEncodeSend();
    xmlGenericError(xmlGenericErrorContext, "</p>\n");

    xmlHTMLPrintFileContext(input);
    xmlHTMLEncodeSend();
}

void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
xmlHTMLWarning(void *ctx, const char *msg, ...)
{
    xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx;
    xmlParserInputPtr input;
    va_list args;
    int len;

    xmllint_buffer[0] = 0;
    input = ctxt->input;
    if ((input != NULL) && (input->filename == NULL) && (ctxt->inputNr > 1)) {
        input = ctxt->inputTab[ctxt->inputNr - 2];
    }

    xmlHTMLPrintFileInfo(input);

    xmlGenericError(xmlGenericErrorContext, "<b>warning</b>: ");
    va_start(args, msg);
    len = strlen(xmllint_buffer);
    vsnprintf(&xmllint_buffer[len],  sizeof(xmllint_buffer) - len, msg, args);
    va_end(args);
    xmlHTMLEncodeSend();
    xmlGenericError(xmlGenericErrorContext, "</p>\n");

    xmlHTMLPrintFileContext(input);
    xmlHTMLEncodeSend();
}

void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
xmlHTMLValidityError(void *ctx, const char *msg, ...)
{
    xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx;
    xmlParserInputPtr input;
    va_list args;
    int len;

    xmllint_buffer[0] = 0;
    input = ctxt->input;
    if ((input->filename == NULL) && (ctxt->inputNr > 1))
        input = ctxt->inputTab[ctxt->inputNr - 2];

    xmlHTMLPrintFileInfo(input);

    xmlGenericError(xmlGenericErrorContext, "<b>validity error</b>: ");
    len = strlen(xmllint_buffer);
    va_start(args, msg);
    vsnprintf(&xmllint_buffer[len],  sizeof(xmllint_buffer) - len, msg, args);
    va_end(args);
    xmlHTMLEncodeSend();
    xmlGenericError(xmlGenericErrorContext, "</p>\n");

    xmlHTMLPrintFileContext(input);
    xmlHTMLEncodeSend();
    xmllint_progresult = XMLLINT_ERR_VALID;
}

void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
xmlHTMLValidityWarning(void *ctx, const char *msg, ...)
{
    xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx;
    xmlParserInputPtr input;
    va_list args;
    int len;

    xmllint_buffer[0] = 0;
    input = ctxt->input;
    if ((input->filename == NULL) && (ctxt->inputNr > 1))
        input = ctxt->inputTab[ctxt->inputNr - 2];

    xmlHTMLPrintFileInfo(input);

    xmlGenericError(xmlGenericErrorContext, "<b>validity warning</b>: ");
    va_start(args, msg);
    len = strlen(xmllint_buffer);
    vsnprintf(&xmllint_buffer[len],  sizeof(xmllint_buffer) - len, msg, args);
    va_end(args);
    xmlHTMLEncodeSend();
    xmlGenericError(xmlGenericErrorContext, "</p>\n");

    xmlHTMLPrintFileContext(input);
    xmlHTMLEncodeSend();
}

void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
xmllint_warningDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
{
    va_list args;

    xmllint_callbacks++;
    if (xmllint_noout)
	return;
    va_start(args, msg);
    fprintf(stdout, "SAX.warning: ");
    vfprintf(stdout, msg, args);
    va_end(args);
}

void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
xmllint_errorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
{
    va_list args;

    xmllint_callbacks++;
    if (xmllint_noout)
	return;
    va_start(args, msg);
    fprintf(stdout, "SAX.error: ");
    vfprintf(stdout, msg, args);
    va_end(args);
}

void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
xmllint_fatalErrorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
{
    va_list args;

    xmllint_callbacks++;
    if (xmllint_noout)
	return;
    va_start(args, msg);
    fprintf(stdout, "SAX.fatalError: ");
    vfprintf(stdout, msg, args);
    va_end(args);
}


// from error.c:
void XMLCDECL
xmlGenericErrorDefaultFunc(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...) {
    va_list args;

    if (xmlGenericErrorContext == NULL)
	    xmlGenericErrorContext = (void *) stderr;

    va_start(args, msg);
    vfprintf((FILE *)xmlGenericErrorContext, msg, args);
    va_end(args);
}

void XMLCDECL
__xmlRaiseError(xmlStructuredErrorFunc schannel,
              xmlGenericErrorFunc channel, void *data, void *ctx,
              void *nod, int domain, int code, xmlErrorLevel level,
              const char *file, int line, const char *str1,
              const char *str2, const char *str3, int int1, int col,
	      const char *msg, ...)
{
    xmlParserCtxtPtr ctxt = NULL;
    xmlNodePtr node = (xmlNodePtr) nod;
    char *str = NULL;
    xmlParserInputPtr input = NULL;
    xmlErrorPtr to = &xmlLastError;
    xmlNodePtr baseptr = NULL;

    if (code == XML_ERR_OK)
        return;
    if ((xmlGetWarningsDefaultValue == 0) && (level == XML_ERR_WARNING))
        return;
    if ((domain == XML_FROM_PARSER) || (domain == XML_FROM_HTML) ||
        (domain == XML_FROM_DTD) || (domain == XML_FROM_NAMESPACE) ||
	    (domain == XML_FROM_IO) || (domain == XML_FROM_VALID)) {
        ctxt = (xmlParserCtxtPtr) ctx;
        if ((schannel == NULL) && (ctxt != NULL) && (ctxt->sax != NULL) &&
            (ctxt->sax->initialized == XML_SAX2_MAGIC) &&
            (ctxt->sax->serror != NULL)) {
            schannel = ctxt->sax->serror;
            data = ctxt->userData;
        }
    }
    /*
     * Check if structured error handler set
     */
    if (schannel == NULL) {
	schannel = xmlStructuredError;
	/*
	 * if user has defined handler, change data ptr to user's choice
	 */
	if (schannel != NULL)
	    data = xmlStructuredErrorContext;
    }
    /*
     * Formatting the message
     */
    if (msg == NULL) {
        str = (char *) xmlStrdup(BAD_CAST "No error message provided");
    } else {
        XML_GET_VAR_STR(msg, str);
    }

    /*
     * specific processing if a parser context is provided
     */
    if (ctxt != NULL) {
        if (file == NULL) {
            input = ctxt->input;
            if ((input != NULL) && (input->filename == NULL) &&
                (ctxt->inputNr > 1)) {
                input = ctxt->inputTab[ctxt->inputNr - 2];
            }
            if (input != NULL) {
                file = input->filename;
                line = input->line;
                col = input->col;
            }
        }
        to = &ctxt->lastError;
    } else if ((node != NULL) && (file == NULL)) {
	int i;

	if ((node->doc != NULL) && (node->doc->URL != NULL)) {
	    baseptr = node;
/*	    file = (const char *) node->doc->URL; */
	}
	for (i = 0;
	     ((i < 10) && (node != NULL) && (node->type != XML_ELEMENT_NODE));
	     i++)
	     node = node->parent;
        if ((baseptr == NULL) && (node != NULL) &&
	    (node->doc != NULL) && (node->doc->URL != NULL))
	    baseptr = node;

	if ((node != NULL) && (node->type == XML_ELEMENT_NODE))
	    line = node->line;
	if ((line == 0) || (line == 65535))
	    line = xmlGetLineNo(node);
    }

    /*
     * Save the information about the error
     */
    xmlResetError(to);
    to->domain = domain;
    to->code = code;
    to->message = str;
    to->level = level;
    if (file != NULL)
        to->file = (char *) xmlStrdup((const xmlChar *) file);
    else if (baseptr != NULL) {
#ifdef LIBXML_XINCLUDE_ENABLED
	/*
	 * We check if the error is within an XInclude section and,
	 * if so, attempt to print out the href of the XInclude instead
	 * of the usual "base" (doc->URL) for the node (bug 152623).
	 */
        xmlNodePtr prev = baseptr;
	int inclcount = 0;
	while (prev != NULL) {
	    if (prev->prev == NULL)
	        prev = prev->parent;
	    else {
	        prev = prev->prev;
		if (prev->type == XML_XINCLUDE_START) {
		    if (--inclcount < 0)
		        break;
		} else if (prev->type == XML_XINCLUDE_END)
		    inclcount++;
	    }
	}
	if (prev != NULL) {
	    if (prev->type == XML_XINCLUDE_START) {
            prev->type = XML_ELEMENT_NODE;
            to->file = (char *) xmlGetProp(prev, BAD_CAST "href");
            prev->type = XML_XINCLUDE_START;
	    } else {
		    to->file = (char *) xmlGetProp(prev, BAD_CAST "href");
	    }
	} else
#endif
	    to->file = (char *) xmlStrdup(baseptr->doc->URL);
        if ((to->file == NULL) && (node != NULL) && (node->doc != NULL)) {
            to->file = (char *) xmlStrdup(node->doc->URL);
        }
    }
    to->line = line;
    if (str1 != NULL)
        to->str1 = (char *) xmlStrdup((const xmlChar *) str1);
    if (str2 != NULL)
        to->str2 = (char *) xmlStrdup((const xmlChar *) str2);
    if (str3 != NULL)
        to->str3 = (char *) xmlStrdup((const xmlChar *) str3);
    to->int1 = int1;
    to->int2 = col;
    to->node = node;
    to->ctxt = ctx;

    if (to != &xmlLastError)
        xmlCopyError(to,&xmlLastError);

    if (schannel != NULL) {
	    schannel(data, to);
	return;
    }

    /*
     * Find the callback channel if channel param is NULL
     */
    if ((ctxt != NULL) && (channel == NULL) &&
        (xmlStructuredError == NULL) && (ctxt->sax != NULL)) {
        if (level == XML_ERR_WARNING)
	        channel = ctxt->sax->warning;
        else
	        channel = ctxt->sax->error;
	    data = ctxt->userData;
    } else if (channel == NULL) {
	    channel = xmlGenericError;
        if (ctxt != NULL) {
            data = ctxt;
        } else {
            data = xmlGenericErrorContext;
        }
    }
    if (channel == NULL)
        return;

    if ((channel == xmlParserError) ||
        (channel == xmlParserWarning) ||
	(channel == xmlParserValidityError) ||
	(channel == xmlParserValidityWarning))
	    xmlReportError(to, ctxt, str, NULL, NULL);
    else if ((channel == (xmlGenericErrorFunc) fprintf) ||
             (channel == xmlGenericErrorDefaultFunc))
	    xmlReportError(to, ctxt, str, channel, data);
    else
	    channel(data, "%s", str);
}

void XMLCDECL
xmlParserError(void *ctx, const char *msg, ...)
{
    xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx;
    xmlParserInputPtr input = NULL;
    xmlParserInputPtr cur = NULL;
    char * str;

    if (ctxt != NULL) {
        input = ctxt->input;
        if ((input != NULL) && (input->filename == NULL) &&
            (ctxt->inputNr > 1)) {
            cur = input;
            input = ctxt->inputTab[ctxt->inputNr - 2];
        }
        xmlParserPrintFileInfo(input);
    }

    xmlGenericError(xmlGenericErrorContext, "error: ");
    XML_GET_VAR_STR(msg, str);
    xmlGenericError(xmlGenericErrorContext, "%s", str);
    if (str != NULL)
	    xmlFree(str);

    if (ctxt != NULL) {
        xmlParserPrintFileContext(input);
        if (cur != NULL) {
            xmlParserPrintFileInfo(cur);
            xmlGenericError(xmlGenericErrorContext, "\n");
            xmlParserPrintFileContext(cur);
        }
    }
}

void XMLCDECL
xmlParserWarning(void *ctx, const char *msg, ...)
{
    xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx;
    xmlParserInputPtr input = NULL;
    xmlParserInputPtr cur = NULL;
    char * str;

    if (ctxt != NULL) {
        input = ctxt->input;
        if ((input != NULL) && (input->filename == NULL) &&
            (ctxt->inputNr > 1)) {
            cur = input;
            input = ctxt->inputTab[ctxt->inputNr - 2];
        }
        xmlParserPrintFileInfo(input);
    }

    xmlGenericError(xmlGenericErrorContext, "warning: ");
    XML_GET_VAR_STR(msg, str);
    xmlGenericError(xmlGenericErrorContext, "%s", str);
    if (str != NULL)
	    xmlFree(str);

    if (ctxt != NULL) {
        xmlParserPrintFileContext(input);
        if (cur != NULL) {
            xmlParserPrintFileInfo(cur);
            xmlGenericError(xmlGenericErrorContext, "\n");
            xmlParserPrintFileContext(cur);
        }
    }
}

void XMLCDECL
xmlParserValidityError(void *ctx, const char *msg, ...)
{
    xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx;
    xmlParserInputPtr input = NULL;
    char * str;
    int len = xmlStrlen((const xmlChar *) msg);
    static int had_info = 0;

    if ((len > 1) && (msg[len - 2] != ':')) {
        if (ctxt != NULL) {
            input = ctxt->input;
            if ((input->filename == NULL) && (ctxt->inputNr > 1))
            input = ctxt->inputTab[ctxt->inputNr - 2];

            if (had_info == 0) {
            xmlParserPrintFileInfo(input);
            }
        }
        xmlGenericError(xmlGenericErrorContext, "validity error: ");
        had_info = 0;
    } else {
	    had_info = 1;
    }

    XML_GET_VAR_STR(msg, str);
    xmlGenericError(xmlGenericErrorContext, "%s", str);
    if (str != NULL)
	    xmlFree(str);

    if ((ctxt != NULL) && (input != NULL)) {
	    xmlParserPrintFileContext(input);
    }
}

void XMLCDECL
xmlParserValidityWarning(void *ctx, const char *msg, ...)
{
    xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx;
    xmlParserInputPtr input = NULL;
    char * str;
    int len = xmlStrlen((const xmlChar *) msg);

    if ((ctxt != NULL) && (len != 0) && (msg[len - 1] != ':')) {
        input = ctxt->input;
        if ((input->filename == NULL) && (ctxt->inputNr > 1))
            input = ctxt->inputTab[ctxt->inputNr - 2];

        xmlParserPrintFileInfo(input);
    }

    xmlGenericError(xmlGenericErrorContext, "validity warning: ");
    XML_GET_VAR_STR(msg, str);
    xmlGenericError(xmlGenericErrorContext, "%s", str);
    if (str != NULL)
	    xmlFree(str);

    if (ctxt != NULL) {
	    xmlParserPrintFileContext(input);
    }
}

// from valid.c:
void XMLCDECL xmlNoValidityErr(void *ctx ATTRIBUTE_UNUSED,
                               const char *msg ATTRIBUTE_UNUSED, ...) {
    return;
}

// from xmlreader.c:
#define MAX_ERR_MSG_SIZE 64000
#define HAVE_VA_COPY
#ifndef VA_COPY
  #ifdef HAVE_VA_COPY
    #define VA_COPY(dest, src) va_copy(dest, src)
  #else
    #ifdef HAVE___VA_COPY
      #define VA_COPY(dest,src) __va_copy(dest, src)
    #else
      #ifndef VA_LIST_IS_ARRAY
        #define VA_COPY(dest,src) (dest) = (src)
      #else
        #include <string.h>
        #define VA_COPY(dest,src) memcpy((char *)(dest),(char *)(src),sizeof(va_list))
      #endif
    #endif
  #endif
#endif

void XMLCDECL
xmlTextReaderValidityErrorRelay(void *ctx, const char *msg, ...)
{
    xmlTextReaderPtr reader = (xmlTextReaderPtr) ctx;

    char *str;

    va_list ap;

    va_start(ap, msg);
    str = xmlTextReaderBuildMessage(msg, ap);
    if (!reader->errorFunc) {
        xmlTextReaderValidityError(ctx, "%s", str);
    } else {
        reader->errorFunc(reader->errorFuncArg, str,
                          XML_PARSER_SEVERITY_VALIDITY_ERROR,
                          NULL /* locator */ );
    }
    if (str != NULL)
        xmlFree(str);
    va_end(ap);
}

void XMLCDECL
xmlTextReaderValidityWarningRelay(void *ctx, const char *msg, ...)
{
    xmlTextReaderPtr reader = (xmlTextReaderPtr) ctx;

    char *str;

    va_list ap;

    va_start(ap, msg);
    str = xmlTextReaderBuildMessage(msg, ap);
    if (!reader->errorFunc) {
        xmlTextReaderValidityWarning(ctx, "%s", str);
    } else {
        reader->errorFunc(reader->errorFuncArg, str,
                          XML_PARSER_SEVERITY_VALIDITY_WARNING,
                          NULL /* locator */ );
    }
    if (str != NULL)
        xmlFree(str);
    va_end(ap);
}

void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
xmlTextReaderError(void *ctxt, const char *msg, ...)
{
    va_list ap;

    va_start(ap, msg);
    xmlTextReaderGenericError(ctxt,
                              XML_PARSER_SEVERITY_ERROR,
                              xmlTextReaderBuildMessage(msg, ap));
    va_end(ap);

}

void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
xmlTextReaderWarning(void *ctxt, const char *msg, ...)
{
    va_list ap;

    va_start(ap, msg);
    xmlTextReaderGenericError(ctxt,
                              XML_PARSER_SEVERITY_WARNING,
                              xmlTextReaderBuildMessage(msg, ap));
    va_end(ap);
}

void XMLCDECL
xmlTextReaderValidityError(void *ctxt, const char *msg, ...)
{
    va_list ap;

    int len = xmlStrlen((const xmlChar *) msg);

    if ((len > 1) && (msg[len - 2] != ':')) {
        /*
         * some callbacks only report locator information:
         * skip them (mimicking behaviour in error.c)
         */
        va_start(ap, msg);
        xmlTextReaderGenericError(ctxt,
                                  XML_PARSER_SEVERITY_VALIDITY_ERROR,
                                  xmlTextReaderBuildMessage(msg, ap));
        va_end(ap);
    }
}

void XMLCDECL
xmlTextReaderValidityWarning(void *ctxt, const char *msg, ...)
{
    va_list ap;

    int len = xmlStrlen((const xmlChar *) msg);

    if ((len != 0) && (msg[len - 1] != ':')) {
        /*
         * some callbacks only report locator information:
         * skip them (mimicking behaviour in error.c)
         */
        va_start(ap, msg);
        xmlTextReaderGenericError(ctxt,
                                  XML_PARSER_SEVERITY_VALIDITY_WARNING,
                                  xmlTextReaderBuildMessage(msg, ap));
        va_end(ap);
    }
}
char *
xmlTextReaderBuildMessage(const char *msg, va_list ap) {
    int size = 0;
    int chars;
    char *larger;
    char *str = NULL;
    va_list aq;

    while (1) {
        VA_COPY(aq, ap);
        chars = vsnprintf(str, size, msg, aq);
        va_end(aq);
        if (chars < 0) {
	    xmlGenericError(xmlGenericErrorContext, "vsnprintf failed !\n");
	    if (str)
		xmlFree(str);
	    return NULL;
	}
	if ((chars < size) || (size == MAX_ERR_MSG_SIZE))
            break;
        if (chars < MAX_ERR_MSG_SIZE)
	size = chars + 1;
	else
		size = MAX_ERR_MSG_SIZE;
        if ((larger = (char *) xmlRealloc(str, size)) == NULL) {
	    xmlGenericError(xmlGenericErrorContext, "xmlRealloc failed !\n");
	    if (str)
                xmlFree(str);
            return NULL;
        }
        str = larger;
    }

    return str;
}

// from xmlchemas.c:
#define TODO								\
    xmlGenericError(xmlGenericErrorContext,				\
	    "Unimplemented block at %s:%d\n",				\
            __FILE__, __LINE__);
void XMLCDECL
warningSplit(void *ctx, const char *msg ATTRIBUTE_UNUSED, ...) {
    xmlSchemaSAXPlugPtr ctxt = (xmlSchemaSAXPlugPtr) ctx;
    if ((ctxt != NULL) && (ctxt->user_sax != NULL) &&
        (ctxt->user_sax->warning != NULL)) {
	TODO
    }
}
void XMLCDECL
errorSplit(void *ctx, const char *msg ATTRIBUTE_UNUSED, ...) {
    xmlSchemaSAXPlugPtr ctxt = (xmlSchemaSAXPlugPtr) ctx;
    if ((ctxt != NULL) && (ctxt->user_sax != NULL) &&
        (ctxt->user_sax->error != NULL)) {
	TODO
    }
}
void XMLCDECL
fatalErrorSplit(void *ctx, const char *msg ATTRIBUTE_UNUSED, ...) {
    xmlSchemaSAXPlugPtr ctxt = (xmlSchemaSAXPlugPtr) ctx;
    if ((ctxt != NULL) && (ctxt->user_sax != NULL) &&
        (ctxt->user_sax->fatalError != NULL)) {
	TODO
    }
}

// from xmlstring.c:
int XMLCDECL
xmlStrPrintf(xmlChar *buf, int len, const char *msg, ...) {
    va_list args;
    int ret;

    if((buf == NULL) || (msg == NULL)) {
        return(-1);
    }

    va_start(args, msg);
    ret = vsnprintf((char *) buf, len, (const char *) msg, args);
    va_end(args);
    buf[len - 1] = 0; /* be safe ! */

    return(ret);
}

// from xmlwriter.c:
int XMLCDECL
xmlTextWriterWriteFormatComment(xmlTextWriterPtr writer,
                                const char *format, ...)
{
    int rc;
    va_list ap;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatComment(writer, format, ap);

    va_end(ap);
    return rc;
}
int XMLCDECL
xmlTextWriterWriteFormatRaw(xmlTextWriterPtr writer, const char *format,
                            ...)
{
    int rc;
    va_list ap;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatRaw(writer, format, ap);

    va_end(ap);
    return rc;
}
int XMLCDECL
xmlTextWriterWriteFormatString(xmlTextWriterPtr writer, const char *format,
                               ...)
{
    int rc;
    va_list ap;

    if ((writer == NULL) || (format == NULL))
        return -1;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatString(writer, format, ap);

    va_end(ap);
    return rc;
}
int XMLCDECL
xmlTextWriterWriteFormatAttribute(xmlTextWriterPtr writer,
                                  const xmlChar * name, const char *format,
                                  ...)
{
    int rc;
    va_list ap;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatAttribute(writer, name, format, ap);

    va_end(ap);
    return rc;
}
int XMLCDECL
xmlTextWriterWriteFormatAttributeNS(xmlTextWriterPtr writer,
                                    const xmlChar * prefix,
                                    const xmlChar * name,
                                    const xmlChar * namespaceURI,
                                    const char *format, ...)
{
    int rc;
    va_list ap;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatAttributeNS(writer, prefix, name,
                                              namespaceURI, format, ap);

    va_end(ap);
    return rc;
}
int XMLCDECL
xmlTextWriterWriteFormatElement(xmlTextWriterPtr writer,
                                const xmlChar * name, const char *format,
                                ...)
{
    int rc;
    va_list ap;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatElement(writer, name, format, ap);

    va_end(ap);
    return rc;
}
int XMLCDECL
xmlTextWriterWriteFormatElementNS(xmlTextWriterPtr writer,
                                  const xmlChar * prefix,
                                  const xmlChar * name,
                                  const xmlChar * namespaceURI,
                                  const char *format, ...)
{
    int rc;
    va_list ap;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatElementNS(writer, prefix, name,
                                            namespaceURI, format, ap);

    va_end(ap);
    return rc;
}
int XMLCDECL
xmlTextWriterWriteFormatPI(xmlTextWriterPtr writer, const xmlChar * target,
                           const char *format, ...)
{
    int rc;
    va_list ap;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatPI(writer, target, format, ap);

    va_end(ap);
    return rc;
}
int XMLCDECL
xmlTextWriterWriteFormatCDATA(xmlTextWriterPtr writer, const char *format,
                              ...)
{
    int rc;
    va_list ap;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatCDATA(writer, format, ap);

    va_end(ap);
    return rc;
}
int XMLCDECL
xmlTextWriterWriteFormatDTD(xmlTextWriterPtr writer,
                            const xmlChar * name,
                            const xmlChar * pubid,
                            const xmlChar * sysid, const char *format, ...)
{
    int rc;
    va_list ap;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatDTD(writer, name, pubid, sysid, format,
                                      ap);

    va_end(ap);
    return rc;
}
int XMLCDECL
xmlTextWriterWriteFormatDTDElement(xmlTextWriterPtr writer,
                                   const xmlChar * name,
                                   const char *format, ...)
{
    int rc;
    va_list ap;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatDTDElement(writer, name, format, ap);

    va_end(ap);
    return rc;
}
int XMLCDECL
xmlTextWriterWriteFormatDTDAttlist(xmlTextWriterPtr writer,
                                   const xmlChar * name,
                                   const char *format, ...)
{
    int rc;
    va_list ap;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatDTDAttlist(writer, name, format, ap);

    va_end(ap);
    return rc;
}
int XMLCDECL
xmlTextWriterWriteFormatDTDInternalEntity(xmlTextWriterPtr writer,
                                          int pe,
                                          const xmlChar * name,
                                          const char *format, ...)
{
    int rc;
    va_list ap;

    va_start(ap, format);

    rc = xmlTextWriterWriteVFormatDTDInternalEntity(writer, pe, name,
                                                    format, ap);

    va_end(ap);
    return rc;
}
xmlChar *
xmlTextWriterVSprintf(const char *format, va_list argptr)
{
    int size;
    int count;
    xmlChar *buf;
    va_list locarg;

    size = BUFSIZ;
    buf = (xmlChar *) xmlMalloc(size);
    if (buf == NULL) {
        xmlWriterErrMsg(NULL, XML_ERR_NO_MEMORY,
                        "xmlTextWriterVSprintf : out of memory!\n");
        return NULL;
    }

    VA_COPY(locarg, argptr);
    while (((count = vsnprintf((char *) buf, size, format, locarg)) < 0)
           || (count == size - 1) || (count == size) || (count > size)) {
	va_end(locarg);
        xmlFree(buf);
        size += BUFSIZ;
        buf = (xmlChar *) xmlMalloc(size);
        if (buf == NULL) {
            xmlWriterErrMsg(NULL, XML_ERR_NO_MEMORY,
                            "xmlTextWriterVSprintf : out of memory!\n");
            return NULL;
        }
	VA_COPY(locarg, argptr);
    }
    va_end(locarg);

    return buf;
}

// from nanoftp.c:
// not really variadic, but generates ASM:
int
xmlNanoFTPCloseConnection(void *ctx) {
    xmlNanoFTPCtxtPtr ctxt = (xmlNanoFTPCtxtPtr) ctx;
    int res;
    fd_set rfd, efd;
    struct timeval tv;

    if ((ctxt == NULL) || (ctxt->controlFd == INVALID_SOCKET)) return(-1);

    closesocket(ctxt->dataFd); ctxt->dataFd = INVALID_SOCKET;
    tv.tv_sec = 15;
    tv.tv_usec = 0;
    FD_ZERO(&rfd);
    FD_SET(ctxt->controlFd, &rfd);
    FD_ZERO(&efd);
    FD_SET(ctxt->controlFd, &efd);
    res = select(ctxt->controlFd + 1, &rfd, NULL, &efd, &tv);
    if (res < 0) {
#ifdef DEBUG_FTP
	perror("select");
#endif
	closesocket(ctxt->controlFd); ctxt->controlFd = INVALID_SOCKET;
	return(-1);
    }
    if (res == 0) {
#ifdef DEBUG_FTP
	xmlGenericError(xmlGenericErrorContext,
		"xmlNanoFTPCloseConnection: timeout\n");
#endif
	closesocket(ctxt->controlFd); ctxt->controlFd = INVALID_SOCKET;
    } else {
	res = xmlNanoFTPGetResponse(ctxt);
	if (res != 2) {
	    closesocket(ctxt->controlFd); ctxt->controlFd = INVALID_SOCKET;
	    return(-1);
	}
    }
    return(0);
}

/*
 * Trapping the error messages at the generic level to grab the equivalent of
 * stderr messages on CLI tools.
 */
static char testErrors[32769];
static int testErrorsSize = 0;
unsigned long callbacks_testlimits = 0;

void XMLCDECL
channel_testlimits(void *ctx  ATTRIBUTE_UNUSED, const char *msg, ...) {
    va_list args;
    int res;

    if (testErrorsSize >= 32768)
        return;
    va_start(args, msg);
    res = vsnprintf(&testErrors[testErrorsSize],
                    32768 - testErrorsSize,
		    msg, args);
    va_end(args);
    if (testErrorsSize + res >= 32768) {
        /* buffer is full */
	testErrorsSize = 32768;
	testErrors[testErrorsSize] = 0;
    } else {
        testErrorsSize += res;
    }
    testErrors[testErrorsSize] = 0;
}

/**
 * warningCallback:
 * @ctxt:  An XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a warning messages, gives file, line, position and
 * extra parameters.
 */
void XMLCDECL
warningCallback_testlimits(void *ctx ATTRIBUTE_UNUSED,
                const char *msg ATTRIBUTE_UNUSED, ...)
{
    callbacks_testlimits++;
    return;
}

/**
 * errorCallback:
 * @ctxt:  An XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a error messages, gives file, line, position and
 * extra parameters.
 */
void XMLCDECL
errorCallback_testlimits(void *ctx ATTRIBUTE_UNUSED, const char *msg ATTRIBUTE_UNUSED,
              ...)
{
    callbacks_testlimits++;
    return;
}

/**
 * fatalErrorCallback:
 * @ctxt:  An XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a fatalError messages, gives file, line, position and
 * extra parameters.
 */
void XMLCDECL
fatalErrorCallback_testlimits(void *ctx ATTRIBUTE_UNUSED,
                   const char *msg ATTRIBUTE_UNUSED, ...)
{
    return;
}

/**
 * xmlNanoFTPCheckResponse:
 * @ctx:  an FTP context
 *
 * Check if there is a response from the FTP server after a command.
 * Returns the code number, or 0
 */

int
xmlNanoFTPCheckResponse(void *ctx) {
    xmlNanoFTPCtxtPtr ctxt = (xmlNanoFTPCtxtPtr) ctx;
    fd_set rfd;
    struct timeval tv;

    if ((ctxt == NULL) || (ctxt->controlFd == INVALID_SOCKET)) return(-1);
    tv.tv_sec = 0;
    tv.tv_usec = 0;
    FD_ZERO(&rfd);
    FD_SET(ctxt->controlFd, &rfd);
    switch(select(ctxt->controlFd + 1, &rfd, NULL, NULL, &tv)) {
	case 0:
	    return(0);
	case -1:
	    __xmlIOErr(XML_FROM_FTP, 0, "select");
	    return(-1);

    }

    return(xmlNanoFTPReadResponse(ctx));
}

// from testrecurse.c:
char testErrors_recurse[32769];
int testErrorsSize_recurse = 0;
/*
 * Trapping the error messages at the generic level to grab the equivalent of
 * stderr messages on CLI tools.
 */

void XMLCDECL
channel_testrecurse(void *ctx  ATTRIBUTE_UNUSED, const char *msg, ...) {
    va_list args;
    int res;

    if (testErrorsSize_recurse >= 32768)
        return;
    va_start(args, msg);
    res = vsnprintf(&testErrors_recurse[testErrorsSize_recurse],
                    32768 - testErrorsSize_recurse,
		    msg, args);
    va_end(args);
    if (testErrorsSize_recurse + res >= 32768) {
        /* buffer is full */
	testErrorsSize_recurse = 32768;
	testErrors_recurse[testErrorsSize_recurse] = 0;
    } else {
        testErrorsSize_recurse += res;
    }
    testErrors_recurse[testErrorsSize_recurse] = 0;
}

// from testHTML.c:
void XMLCDECL
testHTML_warningDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
{
    va_list args;

    va_start(args, msg);
    fprintf(stdout, "SAX.warning: ");
    vfprintf(stdout, msg, args);
    va_end(args);
}
void XMLCDECL
testHTML_errorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
{
    va_list args;

    va_start(args, msg);
    fprintf(stdout, "SAX.error: ");
    vfprintf(stdout, msg, args);
    va_end(args);
}
void XMLCDECL
testHTML_fatalErrorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
{
    va_list args;

    va_start(args, msg);
    fprintf(stdout, "SAX.fatalError: ");
    vfprintf(stdout, msg, args);
    va_end(args);
}

// from testSAX.c:
int testSAX_callbacks = 0;
int testSAX_quiet = 0;
void XMLCDECL
testSAX_warningDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
{
    va_list args;

    testSAX_callbacks++;
    if (testSAX_quiet)
	return;
    va_start(args, msg);
    fprintf(stdout, "SAX.warning: ");
    vfprintf(stdout, msg, args);
    va_end(args);
}
void XMLCDECL
testSAX_errorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
{
    va_list args;

    testSAX_callbacks++;
    if (testSAX_quiet)
	return;
    va_start(args, msg);
    fprintf(stdout, "SAX.error: ");
    vfprintf(stdout, msg, args);
    va_end(args);
}
void XMLCDECL
testSAX_fatalErrorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
{
    va_list args;

    testSAX_callbacks++;
    if (testSAX_quiet)
	return;
    va_start(args, msg);
    fprintf(stdout, "SAX.fatalError: ");
    vfprintf(stdout, msg, args);
    va_end(args);
}
#if defined(HAVE_GETTIMEOFDAY)
static struct timeval testSAX_begin, testSAX_end;

/*
 * testSAX_startTimer: call where you want to start timing
 */
void
testSAX_startTimer(void)
{
    gettimeofday(&testSAX_begin, NULL);
}

/*
 * testSAX_endTimer: call where you want to stop timing and to print out a
 *           message about the timing performed; format is a printf
 *           type argument
 */
void XMLCDECL
testSAX_endTimer(const char *fmt, ...)
{
    long msec;
    va_list ap;

    gettimeofday(&testSAX_end, NULL);
    msec = testSAX_end.tv_sec - testSAX_begin.tv_sec;
    msec *= 1000;
    msec += (testSAX_end.tv_usec - testSAX_begin.tv_usec) / 1000;

#ifndef HAVE_STDARG_H
#error "testSAX_endTimer required stdarg functions"
#endif
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);

    fprintf(stderr, " took %ld ms\n", msec);
}
#elif defined(HAVE_TIME_H)
/*
 * No gettimeofday function, so we have to make do with calling clock.
 * This is obviously less accurate, but there's little we can do about
 * that.
 */
#ifndef CLOCKS_PER_SEC
#define CLOCKS_PER_SEC 100
#endif

static clock_t testSAX_begin, testSAX_end;
void
testSAX_startTimer(void)
{
    testSAX_begin = clock();
}
void XMLCDECL
testSAX_endTimer(const char *fmt, ...)
{
    long msec;
    va_list ap;

    testSAX_end = clock();
    msec = ((testSAX_end - testSAX_begin) * 1000) / CLOCKS_PER_SEC;

#ifndef HAVE_STDARG_H
#error "testSAX_endTimer required stdarg functions"
#endif
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    fprintf(stderr, " took %ld ms\n", msec);
}
#else

/*
 * We don't have a gettimeofday or time.h, so we just don't do timing
 */
void
testSAX_startTimer(void)
{
    /*
     * Do nothing
     */
}
void XMLCDECL
testSAX_endTimer(const char *format, ...)
{
    /*
     * We cannot do anything because we don't have a timing function
     */
#ifdef HAVE_STDARG_H
    va_start(ap, format);
    vfprintf(stderr, format, ap);
    va_end(ap);
    fprintf(stderr, " was not timed\n", msec);
#else
    /* We don't have gettimeofday, time or stdarg.h, what crazy world is
     * this ?!
     */
#endif
}
#endif

/*
 * Trapping the error messages at the generic level to grab the equivalent of
 * stderr messages on CLI tools.
 */
char testErrors_runtest[32769];
int testErrorsSize_runtest = 0;

void XMLCDECL
testErrorHandler_runtest(void *ctx  ATTRIBUTE_UNUSED, const char *msg, ...) {
    va_list args;
    int res;

    if (testErrorsSize_runtest >= 32768)
        return;
    va_start(args, msg);
    res = vsnprintf(&testErrors_runtest[testErrorsSize_runtest],
                    32768 - testErrorsSize_runtest,
		    msg, args);
    va_end(args);
    if (testErrorsSize_runtest + res >= 32768) {
        /* buffer is full */
	testErrorsSize_runtest = 32768;
	testErrors_runtest[testErrorsSize_runtest] = 0;
    } else {
        testErrorsSize_runtest += res;
    }
    testErrors_runtest[testErrorsSize_runtest] = 0;
}

void XMLCDECL
channel_runtest(void *ctx  ATTRIBUTE_UNUSED, const char *msg, ...) {
    va_list args;
    int res;

    if (testErrorsSize_runtest >= 32768)
        return;
    va_start(args, msg);
    res = vsnprintf(&testErrors_runtest[testErrorsSize_runtest],
                    32768 - testErrorsSize_runtest,
		    msg, args);
    va_end(args);
    if (testErrorsSize_runtest + res >= 32768) {
        /* buffer is full */
	testErrorsSize_runtest = 32768;
	testErrors_runtest[testErrorsSize_runtest] = 0;
    } else {
        testErrorsSize_runtest += res;
    }
    testErrors_runtest[testErrorsSize_runtest] = 0;
}



int callbacks_runtest = 0;
int quiet_runtest = 0;
FILE *SAXdebug_runtest = NULL;

/**
 * warningDebug:
 * @ctxt:  An XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a warning messages, gives file, line, position and
 * extra parameters.
 */
void XMLCDECL
warningDebug_runtest(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
{
    va_list args;

    callbacks_runtest++;
    if (quiet_runtest)
	return;
    va_start(args, msg);
    fprintf(SAXdebug_runtest, "SAX.warning: ");
    vfprintf(SAXdebug_runtest, msg, args);
    va_end(args);
}

/**
 * errorDebug:
 * @ctxt:  An XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a error messages, gives file, line, position and
 * extra parameters.
 */
void XMLCDECL
errorDebug_runtest(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
{
    va_list args;

    callbacks_runtest++;
    if (quiet_runtest)
	return;
    va_start(args, msg);
    fprintf(SAXdebug_runtest, "SAX.error: ");
    vfprintf(SAXdebug_runtest, msg, args);
    va_end(args);
}

/**
 * fatalErrorDebug:
 * @ctxt:  An XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a fatalError messages, gives file, line, position and
 * extra parameters.
 */
void XMLCDECL
fatalErrorDebug_runtest(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
{
    va_list args;

    callbacks_runtest++;
    if (quiet_runtest)
	return;
    va_start(args, msg);
    fprintf(SAXdebug_runtest, "SAX.fatalError: ");
    vfprintf(SAXdebug_runtest, msg, args);
    va_end(args);
}

void
ignoreGenericError_runtest(void *ctx ATTRIBUTE_UNUSED,
        const char *msg ATTRIBUTE_UNUSED, ...) {
}
