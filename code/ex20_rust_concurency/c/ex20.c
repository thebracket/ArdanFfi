#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>

#define ARR_SIZE 1000

bool is_prime_slow(int32_t n);
int64_t count_primes(int32_t *arr, size_t len);

bool is_prime_slow_c(int32_t n) {
    if (n < 2) return false;
    for (int32_t i = 2; i < n/2; i++) {
        if (n % i == 0) return false;
    }
    return true;
}

int main() {
    time_t start, end;

    // Allocate memory for the array
    printf("Allocating memory for the array...\n");
    int32_t *arr = (int32_t *)malloc(ARR_SIZE * sizeof(int32_t));

    // Populate the array with random numbers
    printf("Populating the array with random numbers...\n");
    for (int32_t i = 0; i < ARR_SIZE; i++) {
        arr[i] = rand();
    }

    // Count the primes in the array with Rust, cheating we're using Rayon
    start = time(NULL);
    int64_t sum_rust = count_primes(arr, ARR_SIZE);
    end = time(NULL);
    printf("Count (from Rust, Parallel): %ld. Seconds: %ld\n", sum_rust, end - start);

    // Count the primes in the array with a C loop
    printf("Calculating the sum of the array in C...\n");
    start = time(NULL);
    int64_t sum = 0;
    for (int32_t i = 0; i < ARR_SIZE; i++) {
        if (is_prime_slow(arr[i])) sum++;
    }
    end = time(NULL);
    printf("Count (from C): %ld. Seconds: %ld\n", sum, end - start);

    // Count the primes in the array with a C loop using native code
    printf("Calculating the sum of the array in C (native)...\n");
    start = time(NULL);
    sum = 0;
    for (int32_t i = 0; i < ARR_SIZE; i++) {
        if (is_prime_slow_c(arr[i])) sum++;
    }
    end = time(NULL);
    printf("Count (from native C): %ld. Seconds: %ld\n", sum, end - start);

    // Free the allocated memory
    free(arr);
    return 0;
}