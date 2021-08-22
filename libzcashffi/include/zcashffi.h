#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Response {
  const char *transaction_id;
  const char *raw;
} Response;

typedef struct UTXO {
  const char *transaction_hash;
  unsigned int index;
  unsigned long amount;
  const char *private_key;
} UTXO;

struct Response *build_transaction(uint32_t input_length,
                                   struct UTXO *inputs_ptr,
                                   const char *to,
                                   uint64_t amount,
                                   const char *change,
                                   uint32_t height);

void sapling(const uint8_t *output, uint32_t len);
