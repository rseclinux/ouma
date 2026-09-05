# ouma

> You banned me from Twitter, God bans you from Heaven.

A slow-burn, bone-chilling, spine-tingling, genre-redefining hardened libc implementation for [GNU/Linux](https://www.gnu.org/gnu/gnu-linux-faq.html#linuxalone) operating systems.

## Why writing one from scratch?
There are sure lots of userspace ISO/IEC 9899 and IEEE 1003.1 implementations for GNU/Linux such as [glibc](https://www.gnu.org/software/libc/) or [musl libc](https://musl.libc.org/). However, they're not the best fit for people who are creating hardened environments (such as servers, network appliances, hardened desktops, containers...). Why? For example glibc takes lot of time to adapt new hardening features or they do not implement it at all, has big and hard to audit code that increases attack surface, and has a history of severe security holes and vulnerabilities. On the other hand, musl libc is worse than glibc: it [implements basic features badly](https://git.musl-libc.org/cgit/musl/tree/src/unistd/getlogin.c), [lacks hardening for atexit and setjmp/longjmp](https://www.dustri.org/b/security-features-of-musl.html) (through glibc isn't always better in that regard), and [has made poor architectural decisions over time](https://news.ycombinator.com/item?id=22692344). There are other issues of course, those above are just the tip of the iceberg. Aside from poor implementations, none of them have support for LLVM CFI, Cross-DSO CFI, or Safestack. Patching glibc or musl against them would result in ABI breakage, so it's better to define our own ABI regardless. And what about relibc? Well... [it's awful](https://gist.github.com/keepitupkitty/43effb8c8fadecf2101b6c0fc4de8790). They have also been using many unsafe methods when they could have avoided that.

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
