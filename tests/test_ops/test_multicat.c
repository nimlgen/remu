#include <hip/hip_common.h>
#define INFINITY (__builtin_inff())
#define NAN (__builtin_nanf(""))
  typedef float float8 __attribute__((ext_vector_type(8)));
  __device__ float8 make_float8(float x, float y, float z, float w, float a, float b, float c, float d) { return {x, y, z, w, a, b, c, d}; }
  extern "C" __global__
  void __launch_bounds__ (1, 1) E_45_195(float* data0, const float* data1, const float* data2, const float* data3) {
  int gidx0 = blockIdx.y; /* 45 */
  int gidx1 = blockIdx.x; /* 195 */
  int alu0 = ((gidx0*65)+gidx1);
  float val0 = ((gidx1<65))?(*(data1+alu0)):0.0f;
  int alu1 = (gidx1*(-1));
  float val1 = (((alu1<(-64))*(gidx1<130)))?(*(data2+alu0+(-65))):0.0f;
  float val2 = ((alu1<(-129)))?(*(data3+alu0+(-130))):0.0f;
  *(data0+(gidx0*195)+gidx1) = (val0+val1+val2);
}