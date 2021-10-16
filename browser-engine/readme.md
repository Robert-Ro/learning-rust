# Build your Own Toy Browser Engine

## your are build what?

browser engine is the portion of a web browser that works "under the hood" to

- fetch a web page from the internet,
- and translate its contents into forms your are read, watch, hear, etc

Blink, Gecko, Webkit and Trident are browser engines.

A browser engines includes many sub-components:

- an HTTP client,
- an HTML parser,
- a CSS parser,
- a javascript engine(itself composed of parsers, interpreters, and compilers),
- and much more

Those components involved in parsing web formats like HTML and CSS and translating them into what you see on-screen are sometimes called the **layout engine** or **rendering engine**

## Why a "toy" rendering engine?

A full-featured browser engine is **hugely complex**. Not the easiest thing for a newcomer to comprehend!

Making a toy system is a **useful tool for learning how the real thing works**. Even if you never build a real-world compiler or kernel, **understanding how they work can help you make better use of them when writing your own programs**.

## Reference

- [Let's build a browser engine!](https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html)
- [robinson](https://github.com/mbrubeck/robinson), a toy web rendering engine
- 理解上，参考前端训练营中的相关部分内容
- [git submodule](https://git-scm.com/book/zh/v2/Git-%E5%B7%A5%E5%85%B7-%E5%AD%90%E6%A8%A1%E5%9D%97)
- [How Browsers Work](http://www.html5rocks.com/en/tutorials/internals/howbrowserswork/)

open source web rendering engines:

- [CSSBox](https://github.com/philborlin/CSSBox) (Java)
- [Cocktail](https://github.com/silexlabs/Cocktail) (Haxe)
- [gngr](https://gngr.info/) (Java)
- [litehtml](https://github.com/tordex/litehtml) (C++)
- [LURE](https://github.com/admin36/LURE) (Lua)
- [NetSurf](http://www.netsurf-browser.org/) (C)
- [Servo](https://github.com/servo/servo/) (Rust)
- [Simple San Simon](http://hsbrowser.wordpress.com/3s-functional-web-browser/) (Haskell)
- [WeasyPrint](https://github.com/Kozea/WeasyPrint) (Python)
- [WebWhirr](https://github.com/reesmichael1/WebWhirr) (C++)
