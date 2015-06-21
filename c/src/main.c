#include <stdio.h>
#include <gsl/gsl_cblas.h>
#include <time.h>
#include <stdlib.h>

#define EXAMPLES 100000

int main(char** args) {

  unsigned long times[EXAMPLES];
  double dots[EXAMPLES];
  struct timespec start, end;

  for (int i = 0; i < EXAMPLES; ++i) {
    double v[] = {
      drand48(),
      drand48(),
      drand48(),
      drand48(),
      drand48(),
      drand48(),
      drand48(),
      drand48(),
      drand48(),
      drand48(),
    };
    double w[] = {
      drand48(),
      drand48(),
      drand48(),
      drand48(),
      drand48(),
      drand48(),
      drand48(),
      drand48(),
      drand48(),
      drand48(),
    };

    clock_gettime(CLOCK_MONOTONIC, &start);
    dots[i] = cblas_ddot(10, v, 1, w, 1);
    clock_gettime(CLOCK_MONOTONIC, &end);

    times[i] = end.tv_nsec - start.tv_nsec;
  }

  unsigned long sum = 0;

  for (int i = 0; i < EXAMPLES; ++i) {
    sum += times[i];
  }

  unsigned long avg = sum / EXAMPLES;

  printf("average dot product computation: %luns\n", avg);
  printf("first dot: %lf, first time: %luns\n", dots[0], times[0]);
  printf("mid dot: %lf, mid time: %luns\n", dots[EXAMPLES/2],
      times[EXAMPLES/2]);
  printf("last dot: %lf, last time: %luns\n", dots[EXAMPLES-1],
      times[EXAMPLES-1]);

  return 0;
}
