#include <cuda_runtime.h>
#include <stdint.h>

extern "C" __global__ void invert_colors_vectorized(uint8_t *const pixels,
                                                    int size) {
  int tid = blockIdx.x * blockDim.x + threadIdx.x;

  // 使用 uint4 一次處理 16 bytes (128 bits)
  uint4 *p4 = (uint4 *)pixels;
  int num_uint4 = size >> 4;

  if (tid < num_uint4) {
    uint4 val = p4[tid];
    val.x = ~val.x;
    val.y = ~val.y;
    val.z = ~val.z;
    val.w = ~val.w;
    p4[tid] = val;
  }

  // 處理最後不足 16 bytes 的部分
  int tail_id = (num_uint4 << 4) + tid;
  if (tail_id < size) {
    pixels[tail_id] = ~pixels[tail_id];
  }
}

extern "C" float launch_invert_colors(uint8_t *pixels, int size) {
  uint8_t *deviceBuffer;
  int block_size = 256;
  // 改用 uint4 後，grid 大小也要相應調整
  int num_uint4 = size >> 4;
  int grid_size = (num_uint4 + block_size - 1) / block_size;
  if (grid_size == 0)
    grid_size = 1;

  cudaEvent_t start, stop;
  cudaEventCreate(&start);
  cudaEventCreate(&stop);

  // 優化點：使用 cudaHostRegister 鎖定現有記憶體，提升 DMA 傳輸效率
  cudaHostRegister(pixels, size, cudaHostRegisterDefault);

  cudaMalloc(&deviceBuffer, size);

  cudaEventRecord(start);

  // 非同步傳輸 (配合 Pinned Memory)
  cudaMemcpy(deviceBuffer, pixels, size, cudaMemcpyHostToDevice);

  invert_colors_vectorized<<<grid_size, block_size>>>(deviceBuffer, size);

  cudaMemcpy(pixels, deviceBuffer, size, cudaMemcpyDeviceToHost);

  cudaEventRecord(stop);
  cudaEventSynchronize(stop);

  float duration = 0;
  cudaEventElapsedTime(&duration, start, stop);

  cudaFree(deviceBuffer);
  cudaHostUnregister(pixels);
  cudaEventDestroy(start);
  cudaEventDestroy(stop);

  return duration;
}
