gem list jekyll #展示插件列表
gem source -a http://gems.github.com -r  https://rubygems.org/
gem source -a https://gems.ruby-china.org/ -r  https://rubygems.org/

# 添加源

# 安装必要的插件
gem install bundler
gem install jekyll-paginate
gem install pygments.rb
gem install wdm

```
$ jekyll serve
# => 一个开发服务器将会运行在 http://localhost:4000/

$ jekyll serve --detach
# => 功能和`jekyll serve`命令相同，但是会脱离终端在后台运行。
#    如果你想关闭服务器，可以使用`kill -9 1234`命令，"1234" 是进程号（PID）。
#    如果你找不到进程号，那么就用`ps aux | grep jekyll`命令来查看，然后关闭服务器。[更多](http://unixhelp.ed.ac.uk/shell/jobz5.html).

$ jekyll serve --watch
# => 和`jekyll serve`相同，但是会查看变更并且自动再生成。

<<<<<<< Updated upstream
$增量更新
jekyll serve --watch --incremental
=======
$ jekyll serve --incremental
>>>>>>> Stashed changes
```
