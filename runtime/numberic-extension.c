#include <stdint.h>

int id(int x)
{
  return x;
}

uint8_t u8_plus(int size, uint8_t *manyu8)
{
  uint8_t out = 0;
  for (int i = 0; i < size; i++)
  {
    out += manyu8[i];
  }
  return out;
}
