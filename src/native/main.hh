/*
    src/native/main.hh
    Q@khaa.pk
 */

/*
    lib/C/main.hh
 */

#include <stdio.h>

#ifndef CLAP_HH
#define CLAP_HH

#ifdef EXPORT_IMPORT
#undef EXPORT_IMPORT
#endif

#ifdef __cplusplus
#define EXPORT_IMPORT __declspec( dllexport )
#else
#define EXPORT_IMPORT __declspec( dllimport )
#endif

#ifdef __cplusplus
extern "C" {  // only need to export C interface if
              // used by C++ source code      
#endif

EXPORT_IMPORT void called_from_c (void);

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus

#endif

#endif
