/*
    png\work\src\main.h
    Q@khaa.pk
 */

#include <stdio.h>
#include <stdlib.h>

__declspec( dllimport ) void called_from_c (void);

#define TOKEN_OF_CHANGE_CLI_OPTION "change"
#define PNG_SIGNATURE_SIZE  8

typedef struct chunk 
{
    unsigned char length[4];
    unsigned char type[4];
    unsigned char* data;
    unsigned char crc[4];

    struct chunk* prev;
    struct chunk* next;
}PNG_CHUNK;

typedef PNG_CHUNK* PTR_PNG_CHUNK;

typedef struct png
{
    char* f;
    unsigned char signature[PNG_SIGNATURE_SIZE];
    unsigned int n; // Number of chunks
    PTR_PNG_CHUNK ptr; // Pointer to first chunk

    struct png* prev;
    struct png* next;
}PNG;

typedef PNG* PTR_PNG;