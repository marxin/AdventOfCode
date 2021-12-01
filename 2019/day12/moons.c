#define N 4
#define D 3

/*
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>

int positions[N][D] = {{-1, 0, 2}, {2, -10, -7}, {4, -8, 8}, {3, 5, -1}};

<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>

int positions[N][D] = {{-8, -10, 0}, {5, 5, 10}, {2, -7, 3}, {9, -8, -3}};

<x=-4, y=-14, z=8>
<x=1, y=-8, z=10>
<x=-15, y=2, z=1>
<x=-17, y=-17, z=16>

*/

int positions[N][D] = {{-4, -14, 8}, {1, -8, 10}, {-15, 2, 1}, {-17, -17, 16}};
int velocity[N][D] = {};

int positions_begin[N][D];
int velocity_begin[N][D];;

void print()
{
  for (unsigned i = 0; i < N; i++)
  {
    __builtin_printf("p[%d]: [%d,%d,%d], [%d,%d,%d]\n", i,
        positions[i][0], positions[i][1], positions[i][2],
        velocity[i][0], velocity[i][1], velocity[i][2]);
  }
}

int main()
{
  for (unsigned i = 0; i < N; i++)
    for (unsigned j = 0; j < D; j++)
    {
      positions_begin[i][j] = positions[i][j];
      velocity_begin[i][j] = velocity[i][j];
    }

  unsigned long step = 0;
  while (1)
  {
    step++;
    if (step % 1000000 == 0)
      __builtin_printf ("Step #%lld\n", step);
    // print();
    int diff[N][D] = {};
    for (unsigned i = 0; i < N; i++)
      for (unsigned j = 0; j < N; j++)
        for (unsigned k = 0; k < D; k++)
        {
          int d = positions[i][k] - positions[j][k];
          if (d != 0)
            diff[i][k] += d < 0 ? 1 : -1;
        }

    for (unsigned i = 0; i < N; i++)
      for (unsigned j = 0; j < D; j++)
        velocity[i][j] += diff[i][j];

    for (unsigned i = 0; i < N; i++)
      for (unsigned j = 0; j < D; j++)
        positions[i][j] += velocity[i][j];

    if (__builtin_memcmp (positions, positions_begin, N * D * sizeof(int)) == 0
        && __builtin_memcmp (velocity, velocity_begin, N * D * sizeof(int)) == 0)
      break;
  }

  __builtin_printf ("Done: %d\n", step);
}
