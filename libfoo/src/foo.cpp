#include <foo/foo.h>

#include <cmath>
#include <cstring>


void allocateTensor(Tensor* t, const void* data, size_t size, size_t typeSize) {
  t->data = new double[size]();
  memcpy(t->data, data, size * typeSize);
  t->size = size;
}

void freeTensor(Tensor* t) {
  delete[] static_cast<double*>(t->data);  //!< HARDCODED TYPE
  t = {};
}
