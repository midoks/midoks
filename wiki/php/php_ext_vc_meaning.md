### PHP关于VC14, VC11 , VC9, VC6以及Thread Safe和Non Thread Safe版本选择的问题
- 网上整理(备忘)

- PHP关于VC14, VC11 , VC9, VC6
- http://wiki.php.net/internals/windows/stepbystepbuild
```

Visual C++ 9.0 (Visual Studio 2008 or Visual C++ 2008) for PHP 5.4.
Visual C++ 11.0 (Visual Studio 2012) for PHP 5.5 or 5.6.
Visual C++ 14.0 (Visual Studio 2015) for PHP 7.0+.

```

- Thread Safe和Non Thread Safe版本选择的问题
```

	Windows版的PHP从版本5.2.1开始有ThreadSafe(线程安全)和None Thread Safe(NTS，非线程安全)之分，这两者不同在于何处？
到底应该用哪种？这里做一个简单的介绍。从2000年10月20日发布的第一个Windows版的PHP3.0.17开始的都是线程安全的版本，这是由于
与Linux/Unix系统是采用多进程的工作方式不同的是Windows系统是采用多线程的工作方式。如果在IIS下以CGI方式运行PHP会非常慢，这
是由于CGI模式是建立在多进程的基础之上的，而非多线程。一般我们会把PHP配置成以ISAPI的方式来运行，ISAPI是多线程的方式，这样就
快多了。但存在一个问题，很多常用的PHP扩展是以Linux/Unix的多进程思想来开发的，这些扩展在ISAPI的方式运行时就会出错搞垮IIS。
因此在IIS下CGI模式才是 PHP运行的最安全方式，但CGI模式对于每个HTTP请求都需要重新加载和卸载整个PHP环境，其消耗是巨大的。

	为了兼顾IIS下PHP的效率和安全，微软给出了FastCGI的解决方案。FastCGI可以让PHP的进程重复利用而不是每一个新的请求就重开一
个进程。同时FastCGI也可以允许几个进程同时执行。这样既解决了CGI进程模式消耗太大的问题，又利用上了CGI进程模式不存在线程安全问
题的优势。 先从字面意思上理解，ThreadSafe是线程安全，执行时会进行线程（Thread）安全检查，以防止有新要求就启动新线程的CGI执
行方式而耗尽系统资源。Non Thread Safe是非线程安全，在执行时不进行线程（Thread）安全检查。因此，如果是使用ISAPI的方式来运行
PHP就必须用ThreadSafe(线程安全)的版本；而用FastCGI模式运行PHP的话就没有必要用线程安全检查了，用None Thread Safe(NTS，非
线程安全)的版本能够更好的提高效率。

```

- PHP的两种执行方式：ISAPI和FastCGI。
```
ISAPI执行方式是以DLL动态库的形式使用，可以在被用户请求后执行，在处理完一个用户请求后不会马上消失，所以需要进行线程安全检查，这
样来提高程序的执行效率，所以如果是以ISAPI来执行PHP，建议选择Thread Safe版本。而FastCGI执行方式是以单一线程来执行操作，所以
不需要进行线程的安全检查，除去线程安全检查的防护反而可以提高执行效率，所以，如果是以FastCGI来执行PHP，建议选择
Non Thread Safe版本。
```

- 如有错误及不足,欢迎指正