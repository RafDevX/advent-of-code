I'd never used C++ before so all the code here is admittedly terrible. The solutions are the first things I came up with upon reading the problem, implemented using the first methods I found online.

In due time I'd like to do these properly, but unfortunately I had very little availability during this AoC, so I just slammed things together to get those precious golden stars.

To complement the cursed code, I have a semi-cursed way of running it: for example,

```sh
function car { g++ -o $1.out $1.cpp && ./$1.out $2; }; car part1 input.txt
```

Eventually, the above should just be turned into a Makefile so a bit of sanity can be returned to the world.
