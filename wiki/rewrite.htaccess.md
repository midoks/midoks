# Apache Rewrite 例子
```
# BEGIN
<IfModule mod_rewrite.c>
RewriteEngine On
RewriteBase /


RewriteRule ^(\w+)/(\w+)$ 		/index.php?_c=$1&_a=$2&%{QUERY_STRING}
RewriteRule ^(\w+)$ 			/index.php?_c=$1&%{QUERY_STRING}

RewriteRule ^index\.php$ - [L]
RewriteCond %{REQUEST_FILENAME} !-f
RewriteCond %{REQUEST_FILENAME} !-d
RewriteRule . /index.php?%{QUERY_STRING} [L]
</IfModule>

# END
```