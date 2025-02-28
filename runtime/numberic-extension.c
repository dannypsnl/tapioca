#include <stdint.h>
#include <stdio.h>

uint16_t u16_plus(int size, uint16_t *manyu16) {
  uint16_t out = 0;
  for (int i = 0; i < size; i++) {
    out += manyu16[i];
  }
  return out;
}

uint16_t u16_sub(int size, uint16_t init, uint16_t *manyu16) {
  if (size == 0) {
    return -init;
  }
  uint16_t out = init;
  for (int i = 0; i < size; i++) {
    out -= manyu16[i];
  }
  return out;
}

uint16_t u16_multiply(int size, uint16_t *manyu16) {
  uint16_t out = 1;
  for (int i = 0; i < size; i++) {
    out *= manyu16[i];
  }
  return out;
}

uint16_t u16_div(int size, uint16_t init, uint16_t *manyu16) {
  uint16_t out = init;
  for (int i = 0; i < size; i++) {
    out /= manyu16[i];
  }
  return out;
}

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
