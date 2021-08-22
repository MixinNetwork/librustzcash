#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Response {
  const char *transaction_id;
  const char *raw;
  uint64_t output_index;
  uint64_t output_amount;
  uint64_t change_index;
  uint64_t change_amount;
} Response;

typedef struct UTXO {
  const char *transaction_hash;
  uint32_t index;
  uint64_t amount;
  const char *private_key;
} UTXO;

struct Response *build_transaction(struct UTXO *inputs_ptr,
                                   uint32_t input_length,
                                   const char *to,
                                   uint64_t amount,
                                   const char *change,
                                   uint32_t height,
                                   const uint8_t *spend_params,
                                   uint32_t spend_params_len,
                                   const uint8_t *output_params,
                                   uint32_t output_params_len);