#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>
#include <pthread.h>
#include <unistd.h>     // For sysconf (on POSIX systems)

#define ARR_SIZE 1000

bool is_prime_slow_c(int32_t n) {
    if (n < 2) return false;
    for (int32_t i = 2; i < n / 2; i++) {
        if (n % i == 0) return false;
    }
    return true;
}

// Structure to hold thread-specific data
typedef struct {
    int32_t *arr;       // Pointer to the array
    int start;          // Start index (inclusive)
    int end;            // End index (exclusive)
    long partial_sum;   // This thread's count of primes
} ThreadData;

// Thread function: count how many numbers in [start, end) are prime
void *thread_func(void *arg) {
    ThreadData *data = (ThreadData *)arg;
    long local_count = 0;

    for (int i = data->start; i < data->end; i++) {
        if (is_prime_slow_c(data->arr[i])) {
            local_count++;
        }
    }

    // Store result in the struct
    data->partial_sum = local_count;
    return NULL;
}

int main() {
    // Start timing
    time_t start_time = time(NULL);

    // Allocate memory for the array
    printf("Allocating memory for the array...\n");
    int32_t *arr = (int32_t *)malloc(ARR_SIZE * sizeof(int32_t));

    // Populate the array with random numbers
    printf("Populating the array with random numbers...\n");
    for (int32_t i = 0; i < ARR_SIZE; i++) {
        arr[i] = rand();
    }

    // -----------------------------------------------------------
    // Pthread-based parallel prime counting
    // -----------------------------------------------------------

    // 1. Determine the number of CPUs
    int num_cpus = (int)sysconf(_SC_NPROCESSORS_ONLN);
    if (num_cpus < 1) {
        fprintf(stderr, "Could not determine number of CPUs; defaulting to 1.\n");
        num_cpus = 1;
    }

    // 2. Create arrays to hold thread data and pthread handles
    ThreadData *thread_data = (ThreadData *)malloc(num_cpus * sizeof(ThreadData));
    pthread_t *threads = (pthread_t *)malloc(num_cpus * sizeof(pthread_t));

    // Calculate how many elements per thread
    int chunk_size = ARR_SIZE / num_cpus;

    // 3. Initialize per-thread data and create threads
    for (int i = 0; i < num_cpus; i++) {
        thread_data[i].arr = arr;
        thread_data[i].start = i * chunk_size;
        // Last chunk might take the "remainder" if ARR_SIZE not perfectly divisible
        thread_data[i].end = (i == num_cpus - 1) ? ARR_SIZE : (i + 1) * chunk_size;
        thread_data[i].partial_sum = 0;

        pthread_create(&threads[i], NULL, thread_func, &thread_data[i]);
    }

    // 4. Join threads and accumulate partial sums
    long sum = 0;
    for (int i = 0; i < num_cpus; i++) {
        pthread_join(threads[i], NULL);
        sum += thread_data[i].partial_sum;
    }

    // 5. Clean up
    free(thread_data);
    free(threads);

    // End timing
    time_t end_time = time(NULL);
    printf("Count (from C, Parallel): %ld. Seconds: %ld\n", sum, (long)(end_time - start_time));

    // Free the allocated memory
    free(arr);

    return 0;
}
