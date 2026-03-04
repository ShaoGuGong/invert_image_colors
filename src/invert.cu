#include <stdint.h>
#include <cuda_runtime.h>

extern "C" __global__ void invert_colors(uint8_t *const pixels, int size) {
  int tid = blockIdx.x * blockDim.x + threadIdx.x;
  uint32_t *p = ((uint32_t *) pixels);
  int num_ints = size >> 2;

  if (tid < num_ints) {
    p[tid] = ~p[tid];
  }

  if (tid < 4) {
    int left_bit = (num_ints << 2) + tid;
    if (left_bit < size) {
      pixels[left_bit] = ~pixels[left_bit];
    }
  }
}

extern "C" float launch_invert_colors(uint8_t *pixels, int size) {
  uint8_t * deviceBuffer;
  int block_size = 256;
  int grid_size = ((size >> 2) + block_size - 1) / block_size;
  cudaEvent_t start, stop;
  cudaEventCreate(&start);
  cudaEventCreate(&stop);
  
  cudaMalloc(&deviceBuffer, sizeof(uint8_t) * size);
  cudaEventRecord(start);
  cudaMemcpy(deviceBuffer, pixels, sizeof(uint8_t) * size, cudaMemcpyHostToDevice);
  invert_colors<<<grid_size, block_size>>>(deviceBuffer, size);
  cudaMemcpy(pixels, deviceBuffer, sizeof(uint8_t) * size, cudaMemcpyDeviceToHost);
  cudaEventRecord(stop);
  cudaEventSynchronize(stop);

  cudaFree(deviceBuffer);
  float duration = 0;
  cudaEventElapsedTime(&duration, start, stop);

  cudaEventDestroy(start);
  cudaEventDestroy(stop);
  return duration;
}
