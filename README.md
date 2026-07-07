# ouma

> You banned me from Twitter, God bans you from Heaven.

A slow-burn, bone-chilling, spine-tingling, genre-redefining hardened libc implementation for [GNU/Linux](https://www.gnu.org/gnu/gnu-linux-faq.html#linuxalone) operating systems.

## Why writing one from scratch?
There are sure lots of userspace ISO/IEC 9899 and IEEE 1003.1 implementations for GNU/Linux such as [glibc](https://www.gnu.org/software/libc/), [musl libc](https://musl.libc.org/) but they are not the best fit for people who are creating hardened environments (such as servers, network appliances, hardened desktops, containers). Why? For example glibc takes lot of time to adapt new hardening features or they do not implement it at all, glibc has big and hard to audit code that increases attack surface, has history of security holes and vulnerabilities. musl libc is way worse than glibc in this aspect, first the code is [notoriosly obfuscated and hard to read](https://git.musl-libc.org/cgit/musl/tree/src/thread/pthread_mutex_timedlock.c) and by extension it is hard to audit, musl is [full of](https://git.musl-libc.org/cgit/musl/tree/src/unistd/getlogin.c) [poorly defined stubs](https://git.musl-libc.org/cgit/musl/tree/src/passwd/getspent.c), [hilarious bugs](https://git.musl-libc.org/cgit/musl/commit/?id=0ccaf0572e9cccda2cced0f7ee659af4c1c6679a) (you can notice [the usage of VLA](https://jorenar.com/blog/vla-pitfalls)), [absence of basic hardening](https://isopenbsdsecu.re/mitigations/atexit_hardening/), [poor (like conflicting with Linux headers despite stating this is libc was made for Linux)](https://news.ycombinator.com/item?id=22693015) [architectural decisions](https://news.ycombinator.com/item?id=22692344). There are numerous issues of course, those above are just tip of the iceberg. Aside from poor implementation, none of the following libcs implement support for LLVM CFI, Cross-DSO CFI, Safestack, patching glibc or musl against them would result in ABI breakage and it's better to define our own ABI whatsoever. And what about relibc? Well.. it's awful. They have been using [various implementations from ouma](https://gist.github.com/keepitupkitty/43effb8c8fadecf2101b6c0fc4de8790), their code is full of usage of unsafe methods when they could have avoided that.

## Why Rust?
We could of course write this in C or C++, Rust is more explicit about usage of unsafe semantics such as pointer arithmetic, pointer dereferencing, usage of assembly, usage of methods that may give incorrect result, monadic error handling and much much more!
ouma utilizes such semantics, makes code to use safe `slice` types, using monadic error handling for correct C FFI interop and handling errors appropriately, minimizes use of raw pointers and pointer arithmetic (in practice ouma uses unsafe for converting pointers to slices, assembly, raw pointer arithmetic in C string routines such as `strlen`, usage of platform-specific implementations such as extraction of `long double` bytes from va_list, va_list manipulation, mutating global variables such as `errno`, `optarg`).
Another nice feature of Rust is it's standard library, the `core` crate is rich, it has good portion of data structures, methods, containers which eases the development and helping preserving correctness of libc overall. And don't let me begin with the notorious borrow checker and explicit lifetime declarations.

## Why AGPLv3?
Lots of userspace libc implementations have been using non-free licenses such as MIT that allow to keep modifications in private, abusing copyright, parasitizing on free and libre software, [locking down consumer devices using free and libre software](https://en.wikipedia.org/wiki/Tivoization). GPLv3 and AGPLv3 have been created to fight such cases and keep free software to be free as freedom and not as free as beer in the bar. rsec GNU/Linux-libre opposes any usage of it's components in proprietary software, locked down hardware that restrict user's freedom, AI.

## The state of the project
As of July 2026, ouma has complete locale support, complete floating point abstractions to help implementing higher-level methods such as `strtod`, complete and fast implementations of various algorithms (such as [Ryu floating point to string](https://github.com/ulfjack/ryu), Dragon4 algorithm, [Eisel-Lemire string to floating point conversion](https://lemire.me/blog/2020/03/10/fast-float-parsing-in-practice/)) which are enough for making a complete implementations of `fprintf`, `sprintf`, `fscanf`, `sscanf` routines, `strtoll`, `strtoull`, `strtof`, `strtod` and `strtold` routines, full support of multibyte routines such as `mbrtowc`, `wcrtomb` and routines listed in `uchar.h` with UTF-8 and ASCII support (it can be extended, thanks to modular design of locale engine). libc can be built with Address and Memory sanitizers and tested using them.

## How can I help rsec GNU/Linux-libre's ouma?
You can either contribute code to the project or make a small donation to the [main developer](mailto:theexanori@gmail.com)

USDT TRC-20 address:
```
TEjkDNLknThmhM1dPsLJiSR6M3nGuM3FgV
```

BTC address:
```
bc1qpjgnefrz40vqm235mrccj5p8jnaz86vpt3zzpq
```

ETH address:
```
0x5907925669EDA3a48f49844243C8fD4218ddF64e
```
