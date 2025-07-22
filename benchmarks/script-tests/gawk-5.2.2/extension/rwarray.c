/*
 * rwarray.c - Builtin functions to binary read / write arrays to a file.
 *
 * Arnold Robbins
 * May 2009
 * Redone June 2012
 * Improved September 2017
 * GMP/MPFR support added November 2021
 */

/*
 * Copyright (C) 2009-2014, 2017, 2018, 2020-2022
 * the Free Software Foundation, Inc.
 *
 * This file is part of GAWK, the GNU implementation of the
 * AWK Programming Language.
 *
 * GAWK is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * GAWK is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <stdio.h>
#include <assert.h>
#include <errno.h>
#include <fcntl.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#ifdef __MINGW32__
#include <winsock2.h>
#include <stdint.h>
#else
#include <arpa/inet.h>
#endif
#include <sys/types.h>
#include <sys/stat.h>

#ifdef HAVE_MPFR
#include <gmp.h>
#include <mpfr.h>
#endif

#include "gawkapi.h"

#include "gettext.h"
#define _(msgid)  gettext(msgid)
#define N_(msgid) msgid

#define MAGIC "awkrulz\n"
#define MAJOR 4
#define MINOR 1

static const gawk_api_t *api;	/* for convenience macros to work */
static awk_ext_id_t ext_id;
static const char *ext_version = "rwarray extension: version 2.1";
static awk_bool_t (*init_func)(void) = NULL;

int plugin_is_GPL_compatible;

static awk_bool_t write_array(FILE *fp, awk_array_t array);
static awk_bool_t write_elem(FILE *fp, awk_element_t *element);
static awk_bool_t write_value(FILE *fp, awk_value_t *val);
static awk_bool_t write_number(FILE *fp, awk_value_t *val);

#ifdef HAVE_MPFR
typedef union {
	mpz_t mpz_val;
	mpfr_t mpfr_val;
} value_storage;
#else
typedef int value_storage;	// should not be used
#endif /* HAVE_MPFR */

static awk_bool_t read_array(FILE *fp, awk_array_t array);
static awk_bool_t read_elem(FILE *fp, awk_element_t *element, value_storage *);
static awk_bool_t read_value(FILE *fp, awk_value_t *value, awk_value_t *idx, value_storage *vs);
static awk_bool_t read_number(FILE *fp, awk_value_t *value, uint32_t code, value_storage *);

/*
 * Format of array info:
 *
 * MAGIC		8 bytes
 * Major version	4 bytes - network order
 * Minor version	4 bytes - network order
 * Element count	4 bytes - network order
 * Elements
 *
 * For each element:
 * Length of index val:	4 bytes - network order
 * Index val as characters (N bytes)
 * Value type		4 bytes (see list below)
 * IF string:
 * 	Length of value	4 bytes
 * 	Value as characters (N bytes)
 * ELSE IF number:
 * 	8 bytes as native double
 * ELSE
 * 	Element count
 * 	Elements
 * END IF
 */

#define VT_STRING	1
#define VT_NUMBER	2
#define VT_GMP		3
#define VT_MPFR		4
#define VT_ARRAY	5
#define VT_REGEX	6
#define VT_STRNUM	7
#define VT_BOOL		8
#define VT_UNDEFINED	20

/* write_backend --- write an array */

static awk_value_t *
write_backend(awk_value_t *result, awk_array_t array, const char *name)
{
	awk_value_t filename;
	FILE *fp = NULL;
	uint32_t major = MAJOR;
	uint32_t minor = MINOR;

	assert(result != NULL);
	make_number(0.0, result);

	/* filename is first arg */
	if (! get_argument(0, AWK_STRING, & filename)) {
		warning(ext_id, _("%s: first argument is not a string"), name);
		errno = EINVAL;
		goto done1;
	}

	/* open the file, if error, set ERRNO and return */
	fp = fopen(filename.str_value.str, "wb");
	if (fp == NULL)
		goto done1;

	if (fwrite(MAGIC, 1, strlen(MAGIC), fp) != strlen(MAGIC))
		goto done1;

	major = htonl(major);
	if (fwrite(& major, 1, sizeof(major), fp) != sizeof(major))
		goto done1;

	minor = htonl(minor);
	if (fwrite(& minor, 1, sizeof(minor), fp) != sizeof(minor))
		goto done1;

	if (write_array(fp, array)) {
		make_number(1.0, result);
		fclose(fp);
		return result;
	}

done1:
	update_ERRNO_int(errno);
	if (fp != NULL) {
		fclose(fp);
		unlink(filename.str_value.str);
	}
	return result;
}

/* do_writea --- write an array */

static awk_value_t *
do_writea(int nargs, awk_value_t *result, struct awk_ext_func *unused)
{
	awk_value_t array;

	if (! get_argument(1, AWK_ARRAY, & array)) {
		warning(ext_id, _("writea: second argument is not an array"));
		errno = EINVAL;
		update_ERRNO_int(errno);
		make_number(0.0, result);
		return result;
	}
	return write_backend(result, array.array_cookie, "writea");
}

/* do_writeall --- write out SYMTAB */

static awk_value_t *
do_writeall(int nargs, awk_value_t *result, struct awk_ext_func *unused)
{
	awk_value_t array;

	if (! sym_lookup("SYMTAB", AWK_ARRAY, & array)) {
		warning(ext_id, _("writeall: unable to find SYMTAB array"));
		errno = EINVAL;
		update_ERRNO_int(errno);
		make_number(0.0, result);
		return result;
	}
	return write_backend(result, array.array_cookie, "writeall");
}


/* write_array --- write out an array or a sub-array */

static awk_bool_t
write_array(FILE *fp, awk_array_t array)
{
	uint32_t i;
	uint32_t count;
	awk_flat_array_t *flat_array;

	if (! flatten_array(array, & flat_array)) {
		warning(ext_id, _("write_array: could not flatten array"));
		return awk_false;
	}

	count = htonl(flat_array->count);
	if (fwrite(& count, 1, sizeof(count), fp) != sizeof(count))
		return awk_false;

	for (i = 0; i < flat_array->count; i++) {
		if (! write_elem(fp, & flat_array->elements[i])) {
			(void) release_flattened_array(array, flat_array);
			return awk_false;
		}
	}

	if (! release_flattened_array(array, flat_array)) {
		warning(ext_id, _("write_array: could not release flattened array"));
		return awk_false;
	}

	return awk_true;
}

/* write_elem --- write out a single element */

static awk_bool_t
write_elem(FILE *fp, awk_element_t *element)
{
	uint32_t indexval_len;
	ssize_t write_count;

	indexval_len = htonl(element->index.str_value.len);
	if (fwrite(& indexval_len, 1, sizeof(indexval_len), fp) != sizeof(indexval_len))
		return awk_false;

	if (element->index.str_value.len > 0) {
		write_count = fwrite(element->index.str_value.str,
				1, element->index.str_value.len, fp);
		if (write_count != (ssize_t) element->index.str_value.len)
			return awk_false;
	}

	return write_value(fp, & element->value);
}

/* write_value --- write a number or a string or a strnum or a regex or an array */

static awk_bool_t
write_value(FILE *fp, awk_value_t *val)
{
	uint32_t code, len;

	if (val->val_type == AWK_ARRAY) {
		code = htonl(VT_ARRAY);
		if (fwrite(& code, 1, sizeof(code), fp) != sizeof(code))
			return awk_false;
		return write_array(fp, val->array_cookie);
	}

	if (val->val_type == AWK_NUMBER)
		return write_number(fp, val);

	switch (val->val_type) {
	case AWK_STRING:
		code = htonl(VT_STRING);
		break;
	case AWK_STRNUM:
		code = htonl(VT_STRNUM);
		break;
	case AWK_REGEX:
		code = htonl(VT_REGEX);
		break;
	case AWK_BOOL:
		code = htonl(VT_BOOL);
		break;
	case AWK_UNDEFINED:
		code = htonl(VT_UNDEFINED);
		break;
	default:
		/* XXX can this happen? */
		code = htonl(VT_UNDEFINED);
		warning(ext_id, _("array value has unknown type %d"), val->val_type);
		break;
	}

	if (fwrite(& code, 1, sizeof(code), fp) != sizeof(code))
		return awk_false;

	if (code == ntohl(VT_BOOL)) {
		len = (val->bool_value == awk_true ? 4 : 5);
		len = htonl(len);
		const char *s = (val->bool_value == awk_true ? "TRUE" : "FALSE");

		if (fwrite(& len, 1, sizeof(len), fp) != sizeof(len))
			return awk_false;

		if (fwrite(s, 1, strlen(s), fp) != (ssize_t) strlen(s))
			return awk_false;
	} else {
		len = htonl(val->str_value.len);
		if (fwrite(& len, 1, sizeof(len), fp) != sizeof(len))
			return awk_false;

		if (fwrite(val->str_value.str, 1, val->str_value.len, fp)
				!= (ssize_t) val->str_value.len)
			return awk_false;
	}
	return awk_true;
}

/* write_number --- write a double, GMP or MPFR number */

static awk_bool_t
write_number(FILE *fp, awk_value_t *val)
{
	uint32_t len, code;
	char buffer[BUFSIZ];

	if (val->num_type == AWK_NUMBER_TYPE_DOUBLE) {
		uint32_t network_order_len;

		code = htonl(VT_NUMBER);
		if (fwrite(& code, 1, sizeof(code), fp) != sizeof(code))
			return awk_false;

		// for portability, save double precision number as a string
		sprintf(buffer, "%.17g", val->num_value);
		len = strlen(buffer) + 1;	// get trailing '\0' too...
		network_order_len = htonl(len);

		if (fwrite(& network_order_len, 1, sizeof(len), fp) != sizeof(len))
			return awk_false;

		if (fwrite(buffer, 1, len, fp) != len)
			return awk_false;
	} else {
#ifdef HAVE_MPFR
		if (val->num_type == AWK_NUMBER_TYPE_MPFR) {
			code = htonl(VT_MPFR);
			if (fwrite(& code, 1, sizeof(code), fp) != sizeof(code))
				return awk_false;

#ifdef USE_MPFR_FPIF
			/*
			 * This would be preferable, but it is not available
			 * on older platforms with mpfr 3.x. It's also marked
			 * experimental in mpfr 4.1, so perhaps not ready for
			 * production use yet.
			 */
			if (mpfr_fpif_export(fp, val->num_ptr) != 0)
#else
#define MPFR_STR_BASE	62	   /* maximize base to minimize string len */
#define MPFR_STR_ROUND	mpfr_get_default_rounding_mode()
			/*
			 * Does the choice of rounding mode matter, given
			 * that the precision is 0, so we should be rendering
			 * in full precision?
			 */
			// We need to write a terminating space, since
			// mpfr_inp_str reads until it hits a space or EOF
			if ((mpfr_out_str(fp, MPFR_STR_BASE, 0, val->num_ptr, MPFR_STR_ROUND) == 0) || (putc(' ', fp) == EOF))
#endif
				return awk_false;
		} else {
			code = htonl(VT_GMP);
			if (fwrite(& code, 1, sizeof(code), fp) != sizeof(code))
				return awk_false;

			if (mpz_out_raw(fp, val->num_ptr) == 0)
				return awk_false;
		}
#else
		fatal(ext_id, _("rwarray extension: received GMP/MPFR value but compiled without GMP/MPFR support."));
#endif
	}
	// all the OK cases fall through to here
	return awk_true;
}

/* free_value --- release memory for ignored global variables */

static void
free_value(awk_value_t *v)
{
	switch (v->val_type) {
	case AWK_ARRAY:
		destroy_array(v->array_cookie);
		break;
	case AWK_STRING:
	case AWK_REGEX:
	case AWK_STRNUM:
	case AWK_UNDEFINED:
		gawk_free(v->str_value.str);
		break;
	case AWK_BOOL:
		/* no memory allocated */
		break;
	case AWK_NUMBER:
		switch (v->num_type) {
		case AWK_NUMBER_TYPE_DOUBLE:
			/* no memory allocated */
			break;
#ifdef HAVE_MPFR
		case AWK_NUMBER_TYPE_MPZ:
			mpz_clear(v->num_ptr);
			break;
		case AWK_NUMBER_TYPE_MPFR:
			mpfr_clear(v->num_ptr);
			break;
#endif /* HAVE_MPFR */
		default:
			warning(ext_id, _("cannot free number with unknown type %d"), v->num_type);
			break;
		}
		break;
	default:
		warning(ext_id, _("cannot free value with unhandled type %d"), v->val_type);
		break;
	}
}

/* do_poke --- create a global variable */

static awk_bool_t
do_poke(awk_element_t *e)
{
	awk_value_t t;

	if (e->index.val_type != AWK_STRING)
		return awk_false;
	/*
	 * So this is a bit tricky. If the program refers to the variable,
	 * then it will already exist in an undefined state after parsing.
	 * If the program never refers to it, then the lookup fails.
	 * We still need to create it in case the program accesses it via
	 * indirection through the SYMTAB table.
	 */
	// it's even trickier, we need to handle foo::bar as well
	char *p = strstr(e->index.str_value.str, "::");
	char *ns, *ident;
	if (p != NULL) {
		ns = e->index.str_value.str;
		ident = p + 2;
		*p = '\0';
	} else {
		ns = "";
		ident = e->index.str_value.str;
	}

	if (sym_lookup_ns(ns, ident, AWK_UNDEFINED, & t)
	    && (t.val_type != AWK_UNDEFINED))
		return awk_false;

	if (! sym_update_ns(ns, ident, & e->value)) {
		if (ns[0])
			warning(ext_id, _("readall: unable to set %s::%s"), ns, ident);
		else
			warning(ext_id, _("readall: unable to set %s"), ident);
		return awk_false;
	}
	return awk_true;
}

/* read_global --- read top-level variables dumped from SYMTAB */

static awk_bool_t
read_global(FILE *fp, awk_array_t unused)
{
	uint32_t i;
	uint32_t count;
	awk_element_t new_elem;
	value_storage vs;

	if (fread(& count, 1, sizeof(count), fp) != sizeof(count))
		return awk_false;

	count = ntohl(count);

	for (i = 0; i < count; i++) {
		if (read_elem(fp, & new_elem, &vs)) {
			if (! do_poke(& new_elem))
				free_value(& new_elem.value);
			if (new_elem.index.str_value.len)
				/* free string allocated by make_const_string */
				gawk_free(new_elem.index.str_value.str);
		} else
			return awk_false;
	}

	return awk_true;
}

/* read_one --- read one array */

static awk_bool_t
read_one(FILE *fp, awk_array_t array)
{
	if (! clear_array(array)) {
		errno = ENOMEM;
		warning(ext_id, _("reada: clear_array failed"));
		return awk_false;
	}

	return read_array(fp, array);
}

/* read_backend --- common code for reada and readall */

static awk_value_t *
read_backend(awk_value_t *result, awk_array_t array, const char *name, awk_bool_t (*func)(FILE *, awk_array_t))
{
	awk_value_t filename;
	FILE *fp = NULL;
	uint32_t major;
	uint32_t minor;
	char magic_buf[30];

	assert(result != NULL);
	make_number(0.0, result);

	/* filename is first arg */
	if (! get_argument(0, AWK_STRING, & filename)) {
		warning(ext_id, _("%s: first argument is not a string"), name);
		errno = EINVAL;
		goto done1;
	}

	fp = fopen(filename.str_value.str, "rb");
	if (fp == NULL)
		goto done1;

	memset(magic_buf, '\0', sizeof(magic_buf));
	if (fread(magic_buf, 1, strlen(MAGIC), fp) != strlen(MAGIC)) {
		errno = EBADF;
		goto done1;
	}

	if (strcmp(magic_buf, MAGIC) != 0) {
		errno = EBADF;
		goto done1;
	}

	if (fread(& major, 1, sizeof(major), fp) != sizeof(major)) {
		errno = EBADF;
		goto done1;
	}
	major = ntohl(major);

	if (major != MAJOR) {
		errno = EBADF;
		goto done1;
	}

	if (fread(& minor, 1, sizeof(minor), fp) != sizeof(minor)) {
		/* read() sets errno */
		goto done1;
	}

	minor = ntohl(minor);
	if (minor != MINOR) {
		errno = EBADF;
		goto done1;
	}

	if ((*func)(fp, array)) {
		make_number(1.0, result);
		goto done0;
	}

done1:
	update_ERRNO_int(errno);
done0:
	if (fp != NULL)
		fclose(fp);
	return result;
}

/* do_reada --- read an array */

static awk_value_t *
do_reada(int nargs, awk_value_t *result, struct awk_ext_func *unused)
{
	awk_value_t array;

	if (! get_argument(1, AWK_ARRAY, & array)) {
		warning(ext_id, _("reada: second argument is not an array"));
		errno = EINVAL;
		update_ERRNO_int(errno);
		make_number(0.0, result);
		return result;
	}
	return read_backend(result, array.array_cookie, "read", read_one);
}

/* do_readall --- read top-level variables */

static awk_value_t *
do_readall(int nargs, awk_value_t *result, struct awk_ext_func *unused)
{
	return read_backend(result, NULL, "readall", read_global);
}


/* read_array --- read in an array or sub-array */

static awk_bool_t
read_array(FILE *fp, awk_array_t array)
{
	uint32_t i;
	uint32_t count;
	awk_element_t new_elem;
	value_storage vs;

	if (fread(& count, 1, sizeof(count), fp) != sizeof(count))
		return awk_false;

	count = ntohl(count);

	for (i = 0; i < count; i++) {
		if (read_elem(fp, & new_elem, &vs)) {
			/* add to array */
			if (! set_array_element_by_elem(array, & new_elem)) {
				warning(ext_id, _("read_array: set_array_element failed"));
				return awk_false;
			}
		} else
			break;
	}

	if (i != count)
		return awk_false;

	return awk_true;
}

/* read_elem --- read in a single element */

static awk_bool_t
read_elem(FILE *fp, awk_element_t *element, value_storage *vs)
{
	uint32_t index_len;
	static char *buffer;
	static uint32_t buflen;
	ssize_t ret;

	if ((ret = fread(& index_len, 1, sizeof(index_len), fp)) != sizeof(index_len)) {
		return awk_false;
	}
	index_len = ntohl(index_len);

	memset(element, 0, sizeof(*element));

	if (index_len > 0) {
		if (buffer == NULL) {
			/* allocate buffer */
			emalloc(buffer, char *, index_len, "read_elem");
			buflen = index_len;
		} else if (buflen < index_len) {
			/* reallocate buffer */
			char *cp = gawk_realloc(buffer, index_len);

			if (cp == NULL)
				return awk_false;

			buffer = cp;
			buflen = index_len;
		}

		if (fread(buffer, 1, index_len, fp) != (ssize_t) index_len) {
			return awk_false;
		}
		make_const_string(buffer, index_len, & element->index);
	} else {
		make_null_string(& element->index);
	}

	if (! read_value(fp, & element->value, & element->index, vs))
		return awk_false;

	return awk_true;
}

/* read_value --- read a number or a string */

static awk_bool_t
read_value(FILE *fp, awk_value_t *value, awk_value_t *idx, value_storage *vs)
{
	uint32_t code, len;

	if (fread(& code, 1, sizeof(code), fp) != sizeof(code))
		return awk_false;

	code = ntohl(code);

	if (code == VT_ARRAY) {
		awk_array_t array = create_array();

		if (! read_array(fp, array))
			return awk_false;

		/* hook into value */
		value->val_type = AWK_ARRAY;
		value->array_cookie = array;
	} else if (code == VT_NUMBER
		   || code == VT_GMP
		   || code == VT_MPFR) {
		return read_number(fp, value, code, vs);
	} else {
		if (fread(& len, 1, sizeof(len), fp) != sizeof(len)) {
			return awk_false;
		}
		len = ntohl(len);
		switch (code) {
		case VT_STRING:
			value->val_type = AWK_STRING;
			break;
		case VT_REGEX:
			value->val_type = AWK_REGEX;
			break;
		case VT_STRNUM:
			value->val_type = AWK_STRNUM;
			break;
		case VT_UNDEFINED:
			value->val_type = AWK_UNDEFINED;
			break;
		case VT_BOOL:
			value->val_type = AWK_BOOL;
			break;
		default:
			/* this cannot happen! */
			warning(ext_id, _("treating recovered value with unknown type code %d as a string"), code);
			value->val_type = AWK_STRING;
			break;
		}
		value->str_value.len = len;
		value->str_value.str = gawk_malloc(len + 1);

		if (fread(value->str_value.str, 1, len, fp) != (ssize_t) len) {
			gawk_free(value->str_value.str);
			return awk_false;
		}
		value->str_value.str[len] = '\0';
		value->str_value.len = len;

		if (code == VT_BOOL) {
			bool val = (strcmp(value->str_value.str, "TRUE") == 0);

			gawk_free(value->str_value.str);
			value->str_value.str = NULL;
			value->bool_value = val ? awk_true : awk_false;
		}
	}

	return awk_true;
}

/* read_number --- read a double, GMP, or MPFR number */

static awk_bool_t
read_number(FILE *fp, awk_value_t *value, uint32_t code, value_storage *vs)
{
	uint32_t len;

	if (code == VT_NUMBER) {
		char buffer[BUFSIZ];
		double d;

		if (fread(& len, 1, sizeof(len), fp) != sizeof(len))
			return awk_false;

		len = ntohl(len);
		if (fread(buffer, 1, len, fp) != len)
			return awk_false;

		(void) sscanf(buffer, "%lg", & d);

		/* hook into value */
		value = make_number(d, value);
	} else {
#ifdef HAVE_MPFR
		if (code == VT_GMP) {
			mpz_init(vs->mpz_val);
    			if (mpz_inp_raw(vs->mpz_val, fp) == 0)
				return awk_false;

			value = make_number_mpz(vs->mpz_val, value);
		} else {
			mpfr_init(vs->mpfr_val);
#ifdef USE_MPFR_FPIF
			/* preferable if widely available and stable */
			if (mpfr_fpif_import(vs->mpfr_val, fp) != 0)
#else
			// N.B. need to consume the terminating space we wrote
			// after mpfr_out_str
			if ((mpfr_inp_str(vs->mpfr_val, fp, MPFR_STR_BASE, MPFR_STR_ROUND) == 0) || (getc(fp) != ' '))
#endif
				return awk_false;

			value = make_number_mpfr(vs->mpfr_val, value);
		}
#else
		fatal(ext_id, _("rwarray extension: GMP/MPFR value in file but compiled without GMP/MPFR support."));
#endif
	}

	return awk_true;
}

static awk_ext_func_t func_table[] = {
	{ "writea", do_writea, 2, 2, awk_false, NULL },
	{ "reada", do_reada, 2, 2, awk_false, NULL },
	{ "writeall", do_writeall, 1, 1, awk_false, NULL },
	{ "readall", do_readall, 1, 1, awk_false, NULL },
};


/* define the dl_load function using the boilerplate macro */

dl_load_func(func_table, rwarray, "")
