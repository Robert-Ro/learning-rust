# 一个白学家眼里的 WebAssembly

> WASM 运行时性能在原理上就是受限的，甚至 JS 都可以和编译到 WASM 的 Rust 一较高下。加上工具链的高度侵入性，它并不太适合作为前端背景同学**allin**的方向，但对于原生应用的跨平台分发则非常有潜力。

## WASM==汇编级性能？

这显然不对，WASM 里的 Assembly 并不意味着真正的汇编码，而**只是种新约定的字节码，也是需要解释器运行的**。这种解释器肯定比 JS 解释器快得多，但自然也达不到真正的原生机器码水平。一个可供参考的数据指标，是 JS 上了 JIT 后整体性能大致是机器码 1/20 的水平，而 WASM 则可以跑到机器码 1/3 的量级（视场景不同很不好说，仅供参考）。相当于即便你写的是 C++和 Rust 级的语言，得到的其实也只是 Java 和 C#级的性能。这也可以解释为什么 WASM 并不能在所有应用场景都显示出压倒性的性能优势：**只要你懂得如何让 JS 引擎走在 HappyPath 上，那么在浏览器里，JS 就敢和 Rust 五五开**。

### 案例

- MozillaHacks 发表了一篇名为[用 Rust 和 WASM 优化 SourceMap 性能](https://link.juejin.cn/?target=https%3A%2F%2Fhacks.mozilla.org%2F2018%2F01%2Foxidizing-source-maps-with-rust-and-webassembly%2F)的博文，将 source-map 这个 JS 包的性能优化了五倍。
- V8 核心开发 VyacheslavEgorov 回应了名为[你也许不需要用 Rust 和 WASM 来优化 JS](https://link.juejin.cn/?target=https%3A%2F%2Fmrale.ph%2Fblog%2F2018%2F02%2F03%2Fmaybe-you-dont-need-rust-to-speed-up-your-js.html)的博文，用纯 JS 实现了速度比 Rust 更快的惊人优化。
- 原文作者以[无需魔法的速度](https://link.juejin.cn/?target=https%3A%2F%2Ffitzgeraldnick.com%2F2018%2F02%2F26%2Fspeed-without-wizardry.html)为名展开了进一步讨论，并用 Rust 做出了新的性能优化。

![](../assets/image/16fa30fd43d46796_tplv-t2oaga2asx-watermark.awebp)

另外，Milo Yip 大大做过的不同语言光线追踪性能测试（修罗场），也能侧面印证带 VM 语言与机器码之间的性能对比结论。C++、Java 和 JS 在未经特别优化的前提下，可以分别代表三个典型的性能档次：
[C++/C#/F#/Java/JS/Lua/Python/Ruby 渲染比试](https://link.juejin.cn/?target=https%3A%2F%2Fwww.cnblogs.com%2Fmiloyip%2Farchive%2F2010%2F07%2F07%2Flanguages_brawl_GI.html)

## WASM 比 JS 快，所以计算密集型应用就该用它？

这有点偏颇，WASM 同样是 CPU 上的计算。对于可以高度并行化的任务，使用 WebGL 来做 GPU 加速往往更快。譬如我在 [实用 WebGL 图像处理入门](https://link.juejin.cn/?target=https%3A%2F%2Fzhuanlan.zhihu.com%2Fp%2F100388037) 这篇文章里介绍的图像处理算法，比起 JS 里 for 循环遍历 Canvas 像素就可以很轻松地快个几十倍。而这种套两层 for 循环的苦力活，用现在的 WASM 重写能快几倍就非常不错了。至于浏览器内 AI 计算的性能方面，社区的评测结论也是 WebGL 和 WebMetal 具备最高的性能水平，然后才是 WASM。参见这里：[浏览器内的 AI 评测](https://link.juejin.cn/?target=https%3A%2F%2Fblog.logrocket.com%2Fai-in-browsers-comparing-tensorflow-onnx-and-webdnn-for-image-classification%2F)

不过，WebGL 的加速存在精度问题。相关讨论参见这里：[Issue #114 · nodeca/pica](https://link.juejin.cn/?target=https%3A%2F%2Fgithub.com%2Fnodeca%2Fpica%2Fissues%2F114)

所以对计算密集型任务，WASM 并不是前端唯一的救星，而是给大家多了一种在性能、开发成本和效果之间权衡的选择。在我个人印象里，前端在图形渲染外需要算力的场景说实话并不太多，像加密、压缩、挖矿这种，都难说是高频刚需。至于未来可能相当重要的 AI 应用，长期而言我还是看好 **WebGPU** 这种更能发挥出 GPU 潜力的下一代标准，当然 WASM 也已经是个不错的可选项了。

## 只要嵌入 WASM 函数到 JS 就能提高性能？

这还真不一定，因为现代浏览器内的 JS 引擎都标配了一种东西，那就是 **JIT**。

像在 [JS 和 WASM 之间的调用终于变快了](https://link.juejin.cn/?target=https%3A%2F%2Fhacks.mozilla.org%2F2018%2F10%2Fcalls-between-javascript-and-webassembly-are-finally-fast-%25F0%259F%258E%2589%2F) 这篇文章中，Lin Clark 非常精彩地论述了整个优化过程，最终使得 JS 和 WASM 间的函数调用，比非内联的 JS 函数间调用要快。不过，至于和被 JIT 内联掉的 JS 函数调用相比起来如何，这篇文章就没有提及了。

## 在 JS 里调 WASM，就像 Python 里调 C 那样简单？

- 在安卓的 `Java class` 里调用 `C++`
- 在 `Flutter` 的 `Dart` 里调用 `C`
- 在 `QuickJS` 这种嵌入式 `JS` 引擎里调用 `C`

在引擎里新建原生对象，并将它以传引用的方式直接交给`C/C++`函数调用，并用引擎的 GC 来管理对象的生命周期。这种方式一般称为 `FFI`（Foreign Function Interface 外部函数接口），可以把原生代码嵌入到语言 Runtime 中。

WASM 的线性内存空间可以随便用 JS 读写，并没有深拷贝的困扰。不过，WASM 只有 int 和 float 之流的数据类型，连 string 都没有，因此对于稍复杂一点的对象，都很难手写出 JS 和 WASM 两边各自的结构。现在这件脏活是交由 wasm-bindgen 等轮子来做的。但毕竟这个过程并不是直接在 JS 的 Runtime 里嵌入 C/ C++ 函数，和传统编译到机器码的 FFI 还是挺不一样的。

## 前端框架迟早会用 WASM 重写？

对于主流的前端应用来说，它们都是 IO 密集而不是计算密集型的，这时 WASM 增加的算力很难成为瓶颈，反而会增加许多工程上的维护成本。

这方面的一个论据，是 Google 的 JIT-less V8 介绍。V8 在关闭 JIT 后峰值性能降低到了不到原先十分之一的级别（见 QuickJS Benchmark），却也几乎不影响刷 YouTube 这种轻度应用的性能表现， 在模拟重度 Web 应用负载的 Speedometer 标准下，其跑分也有原先的 60% 左右，只有在 Webpack 打包类型的任务上出现了数量级的差异。你觉得迁移到 WASM 后，峰值算力就算比现在再翻两倍，能在事件驱动、IO 密集的 GUI 场景中表现出颠覆性的突破吗？能说服框架作者们完全放弃现有的 JS 代码库，选用另一种语言来彻底重写框架吗？况且 WASM 从长期来看，可都要依赖不少体积足以影响首屏性能的 JS 胶水代码和 polyfill 呢。

## WASM 属于前端生态？

一个 WASM 应用，其编译工具链和依赖库生态，基本完全不涉及 JS。

### js 走出去的案例

- [随着 TypeScript 继续普及，会不会出现直接跑 TypeScript 的运行时？](https://link.juejin.cn/?target=https%3A%2F%2Fwww.zhihu.com%2Fquestion%2F363807522%2Fanswer%2F961295958)
- [将 React 渲染到嵌入式液晶屏](https://link.juejin.cn/?target=https%3A%2F%2Fzhuanlan.zhihu.com%2Fp%2F89574235)

### 小结

JS 和 WASM 的人设大概各自是这样的：

- JS：我先来的，哪里有浏览器，哪里就是我的主场。虽然有人不喜欢我的脾气，但我到哪都是一头黑长直的字符串脚本。追我的引擎有的是，但我始终首先是浏览器的忠犬。
- WASM：我是高岭之花，浏览器内外大家都欢迎我，而且谁都能编译到我，所以欢迎大家都来用我的二进制格式吧。我虽然很想和 JS 和浏览器三个人永远在一起，但最希望以后跨平台只要靠我的努力就可以永远幸福下去了。

## 后记

WASM 当然是个革命性的技术，代表了一种跨平台的全新方向，尤其对原生应用开发者来说具备巨大的商业价值。*但它对前端来说其实就是个浏览器内置的字节码虚拟机*，不是一切性能问题的灵丹妙药。目前网上不少对它的赞美，**在我看来多少有些过誉了**。所以建议大家不要盲目跟风，还是从白学，啊不计算机科学的基础出发，去判断一个技术的适用场景和价值在哪吧。

## Reference

- [原文链接](https://juejin.cn/post/6844904047648964616)
