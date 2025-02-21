#include <stdint.h>

uint8_t u8_plus(int size, uint8_t *manyu8) {
  uint8_t out = 0;
  for (int i = 0; i < size; i++) {
    out += manyu8[i];
  }
  return out;
}

uint8_t u8_sub(int size, uint8_t init, uint8_t *manyu8) {
  if (size == 0) {
    return -init;
  }
  uint8_t out = init;
  for (int i = 0; i < size; i++) {
    out -= manyu8[i];
  }
  return out;
}

uint8_t u8_multiply(int size, uint8_t *manyu8) {
  uint8_t out = 1;
  for (int i = 0; i < size; i++) {
    out *= manyu8[i];
  }
  return out;
}

uint8_t u8_div(int size, uint8_t init, uint8_t *manyu8) {
  uint8_t out = init;
  for (int i = 0; i < size; i++) {
    out /= manyu8[i];
  }
  return out;
}
