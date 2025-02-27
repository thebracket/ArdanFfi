#include <thread>
#include <vector>
#include <stdio.h>

extern "C" {
    void start_generator();
    void async_generator(void (*callback)(int, int), int n);
}

void thread_function(int i) {
    async_generator([](int generated, int thread_id) {
        printf("[%d] Called with %d\n", thread_id, generated);
    }, i);
}

int main() {
    // Launch the async Rust
    start_generator();

    // Create and start threads
    std::thread t1(thread_function, 1);
    std::thread t2(thread_function, 2);

    // Wait for all threads to finish
    t1.join();
    t2.join();
    
    return 0;
}