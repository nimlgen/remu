#include <hip/hip_common.h>
#define INFINITY (__builtin_inff())
#define NAN (__builtin_nanf(""))
  typedef float float8 __attribute__((ext_vector_type(8)));
  __device__ float8 make_float8(float x, float y, float z, float w, float a, float b, float c, float d) { return {x, y, z, w, a, b, c, d}; }
  extern "C" __global__
  void __launch_bounds__ (1, 1) E_2_3_5_7_8n1(float* data0, const float* data1, const float* data2) {
  int gidx0 = blockIdx.z; /* 2 */
  int gidx1 = blockIdx.y; /* 3 */
  int gidx2 = blockIdx.x; /* 280 */
  int alu0 = ((gidx2/8)%7);
  float val0 = *(data1+(gidx1*7)+alu0);
  int alu1 = (gidx2/56);
  int alu2 = (gidx2%8);
  float val1 = *(data2+(gidx0*40)+(alu1*8)+alu2);
  *(data0+(gidx0*840)+(gidx1*280)+(alu1*56)+(alu0*8)+alu2) = (val0-val1);
}