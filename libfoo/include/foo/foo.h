//! @author Nikita Guzhva (nik_finger@mail.ru)
//! @file   libfoo/include/foo/foo.h
//! @date   05/10/2021

#ifndef FOO_H
#define FOO_H

#include <cstddef>

extern "C" {
  struct Tensor {
    void* data;
    size_t size;
  };

  void allocateTensor(Tensor* t, const void* data, size_t size, size_t typeSize);
  void freeTensor(Tensor* t);
}

#endif // FOO_H

