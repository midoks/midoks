# Solr分词
```
managed-schema.xml

<fieldtype name="textComplex" class="solr.TextField" positionIncrementGap="100">
<analyzer>
<tokenizer class="com.chenlb.mmseg4j.solr.MMSegTokenizerFactory" mode="complex" dicPath="dic"/>
</analyzer>
</fieldtype>
<fieldtype name="textMaxWord" class="solr.TextField" positionIncrementGap="100">
<analyzer>
<tokenizer class="com.chenlb.mmseg4j.solr.MMSegTokenizerFactory" mode="max-word" />
</analyzer>
</fieldtype>
<fieldtype name="textSimple" class="solr.TextField" positionIncrementGap="100">
<analyzer>
<tokenizer class="com.chenlb.mmseg4j.solr.MMSegTokenizerFactory" mode="simple" dicPath="n:/custom/path/to/my_dic" />
</analyzer>
</fieldtype>

下载 mmseg-solr-2.4.0.jar mmseg-core-1.1.0.jar

curl 'http://127.0.0.1:8983/solr/article/dataimport?_=1490424730223&indent=on&wt=json'--data 'command=delta-import&verbose=false&clean=false&commit=true&optimize=false&debug=true&core=article&entity=article&name=dataimport'

mmseg 中文分词
```