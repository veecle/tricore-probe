/** \file mcd_types.h
    \brief The Multi-core Debug (MCD) API basic types

    This is the definition of basic scalar types used by the SPRINT MCD API 1.1.
    In contrast to the SPRINT MCD API version 1.0 boolean types and char types are here defined with
    a size, which is fixed for the same platform.

    New host platforms may require changes in this file.
*/

#ifndef __mcd_types_h
#define __mcd_types_h


#if defined(_MSC_VER) && (_MSC_VER<1600)
  typedef __int8            int8_t;
  typedef __int16           int16_t;
  typedef __int32           int32_t;
  typedef __int64           int64_t;
  typedef unsigned __int8   uint8_t;
  typedef unsigned __int16  uint16_t;
  typedef unsigned __int32  uint32_t;
  typedef unsigned __int64  uint64_t;
#else
# if defined(__SUNPRO_C) || defined(__SUNPRO_CC) || defined(__hpux)
#   include <inttypes.h>
# else
#   include <stdint.h>
# endif
#endif


/* Definition of character type (mcd_char_t):
 * Characters are only used for strings.
 * We assume here that "char" has the same size for all compilers on a platform.
 * The width of a character should correspond to the size of a minimum addressable unit.
 * So for byte addressable machines the mcd_char_t should have the width of 8-bits.
 */
typedef char mcd_char_t;


/* Definition of the boolean type (mcd_bool_t):
 * The boolean types must have always the same size with any compiler on the same platform.
 * Booleans are here defined with 32-bits to have fairly good compatibility with version 1.0 of the API.
 * (Note, that both C and C++ standard define their boolean type (bool or _Bool) not with a fixed size.)
 * If all bits of a MCD boolean (mcd_bool_t) are zero the boolean represents the value FALSE, otherwise it represents TRUE.
 * An MCD variable should only be set to TRUE by writing 1 to the variable.
 */
typedef uint32_t mcd_bool_t;

#ifndef FALSE
# define FALSE 0
# define TRUE  1  /* Use just for setting not for checking! */
#endif

#if FALSE != 0
# error Definition of FALSE is not compliant to MCD API
#endif


#endif /* __mcd_types_h */

