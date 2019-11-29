#define FFI_SCOPE "TESTPHP"
#define FFI_LIB "target/debug/libtest_php_ffi.so"


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t x;
  int32_t y;
} Point;

void hello_from_rust(Point p);
