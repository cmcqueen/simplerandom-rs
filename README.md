# Simple Random

Simple pseudo-random number generators.

## Intro

This project provides `simplerandom`, simple pseudo-random number
generators.

Features:

* Main API functions:
    * Seed
    * Generate "next" random value
    * "Jump-ahead" (also known as "discard" in C++) to skip the generator
      ahead by 'n' samples.
* Simple algorithms that are easily ported to different languages.
* Safe seeding. Many generators have some "bad" state values that must
  be avoided. The seed functions for all generators ensure that any
  "bad" state values are avoided, and replaced by a suitable
  alternative initial state.
* These random number generators have been implemented in the following
  languages:
    * C
    * Python
    * Rust
* Same numeric output in each supported language. It can be useful
  to be able to implement the identical algorithm on muliple
  platforms and/or languages.
* Simple algorithms and state size appropriate for limited RAM and ROM
  (e.g. in embedded systems).
* Decent cross-platform support.
    * Various OS.
    * Various processors, 8- to 64-bit.
* Implement target language's API idioms and/or existing random number
  generator API.
* Reasonable statistical properties of pseudo-random output (though
  not for all generators provided).

## Algorithms

Most algorithms were obtained from two newsgroup posts by George
Marsaglia [[mars1]](#mars1) [[mars2]](#mars2). However, some
modifications have been made. From [[rose1]](#rose1), it seems that the
SHR3 algorithm defined in [[mars1]](#mars1) is flawed and should not be
used. It doesn't actually have a period of 2<sup>32</sup>-1 as expected, but has
64 different cycles, some with very short periods. The SHR3 in the 2003
post is very similar, but with two shift values swapped. It has a
period of 2<sup>32</sup>-1 as expected.

We still find KISS from [[mars1]](#mars1) useful mainly because it uses
32-bit calculations for MWC, which can be more suitable for small
embedded systems. So we define KISS that uses a MWC based on
[[mars1]](#mars1), but the Cong and SHR3 from [[mars2]](#mars2).

From Pierre L'Ecuyer [[lecuyer1]](#lecuyer1) [[lecuyer2]](#lecuyer2),
the Combined LFSR (Tausworthe) LFSR113 algorithm
[[lecuyer3]](#lecuyer3) and LFSR88 (aka Taus88) have been implemented.

### Random Number Generators Provided

The following pseudo-random number generators are provided:

| Generator   | Notes
| ----------- | --------------------------------------------------------------------------------------------------------------
| `MWC1`      | Two 32-bit MWCs combined. From [[mars1]](#mars1).
| `MWC2`      | Very similar to `MWC1`, but slightly modified to improve its statistical properties.
| `Cong`      | From [[mars2]](#mars2).
| `SHR3`      | From [[mars2]](#mars2).
| `MWC64`     | A single 64-bit multiply-with-carry calculation. From [[mars2]](#mars2).
| `KISS`      | Combination of MWC2, Cong and SHR3. Based on [[mars1]](#mars1) but using Cong and SHR3 from [[mars2]](#mars2), and the modified MWC.
| `KISS2`     | Combination of MWC64, Cong and SHR3. From [[mars2]](#mars2).
| `LFSR113`   | Combined LFSR (Tausworthe) random number generator by L'Ecuyer. From [[lecuyer1]](#lecuyer1) [[lecuyer3]](#lecuyer3).
| `LFSR88`    | Combined LFSR (Tausworthe) random number generator by L'Ecuyer. From [[lecuyer2]](#lecuyer2).



## License

The code is released under the MIT license. See LICENSE.txt for details.

## References

<a name="mars1"></a>
\[mars1\]  
[Random Numbers for C: End, at last?](http://www.cse.yorku.ca/~oz/marsaglia-rng.html)  
George Marsaglia  
Newsgroup post, sci.stat.math and others, Thu, 21 Jan 1999

<a name="mars2"></a>
\[mars2\]  
[RNGs](http://groups.google.com/group/sci.math/msg/9959175f66dd138f)  
George Marsaglia  
Newsgroup post, sci.math, 26 Feb 2003

<a name="rose1"></a>
\[rose1\]  
[KISS: A Bit Too Simple](http://eprint.iacr.org/2011/007.pdf)  
Greg Rose  
Qualcomm Inc.

<a name="lecuyer1"></a>
\[lecuyer1\]  
[Tables of Maximally-Equidistributed Combined LFSR Generators](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.43.3639)  
Pierre L'Ecuyer  
Mathematics of Computation, 68, 225 (1999), 261–269.

<a name="lecuyer2"></a>
\[lecuyer2\]  
[Maximally Equidistributed Combined Tausworthe Generators](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.43.4155)  
P. L'Ecuyer  
Mathematics of Computation, 65, 213 (1996), 203–213.

<a name="lecuyer3"></a>
\[lecuyer3\]  
[LFSR113 C double implementation](http://www.iro.umontreal.ca/~simardr/rng/lfsr113.c)  
Pierre L'Ecuyer
