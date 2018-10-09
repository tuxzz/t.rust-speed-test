#include <vector>
#include <chrono>
#include <cstdio>
#include <cassert>

// bad implemention
void method_loop_1(const float * __restrict x, const size_t n_x, float * __restrict out) {
  for(size_t i = 0; i < n_x; i += 2)
    out[i] = x[i + 1] * 123.0f;
  for (size_t i = 1; i < n_x; i += 2)
    out[i] = x[i - 1] * 456.0f;
}

// good implemention
void method_loop_2(const float * __restrict x, const size_t n_x, float * __restrict out) {
  for (size_t h_i = 0; h_i < n_x / 2; ++h_i) {
    auto i = h_i * 2;
    out[i] = x[i + 1] * 123.0f;
    out[i + 1] = x[i] * 456.0f;
  }
}

void do_test(const char *prefix) {
  std::vector<float> out_a(65536);
  {
    std::vector<float> x(65536);
    for (size_t i = 0; i < 65536; ++i)
      x[i] = static_cast<float>(i);
    auto start = std::chrono::steady_clock::now();
    for (size_t ct = 0; ct < 8192; ++ct) {
      method_loop_1(x.data(), x.size(), out_a.data());
    }
    auto duration = static_cast<double>(std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now() - start).count()) / 1000.0;
    printf("%smethod_loop bad: %lf\n", prefix, duration);
  }
  std::vector<float> out_b(65536);
  {
    std::vector<float> x(65536);
    for (size_t i = 0; i < 65536; ++i)
      x[i] = static_cast<float>(i);
    auto start = std::chrono::steady_clock::now();
    for (size_t ct = 0; ct < 8192; ++ct) {
      method_loop_2(x.data(), x.size(), out_b.data());
    }
    auto duration = static_cast<double>(std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now() - start).count()) / 1000.0;
    printf("%smethod_loop good: %lf\n", prefix, duration);
  }
  assert(out_a == out_b);
}

int main() {
  do_test("Warm up ");
  printf("*************\n");
  do_test("");
  printf("Finished.");
}