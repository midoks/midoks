# Solr分词
```
managed-schema.xml

<fieldType name="textComplex" class="solr.TextField" positionIncrementGap="100" >  
     <analyzer>  
        <tokenizer class="com.chenlb.mmseg4j.solr.MMSegTokenizerFactory" mode="complex" dicPath="dic"/>  
        <filter class="solr.LowerCaseFilterFactory"/>  
    </analyzer>  
</fieldType>  

<fieldType name="textMaxWord" class="solr.TextField" positionIncrementGap="100" >  
    <analyzer>  
        <tokenizer class="com.chenlb.mmseg4j.solr.MMSegTokenizerFactory" mode="max-word" dicPath="dic"/>  
        <filter class="solr.LowerCaseFilterFactory"/>  
    </analyzer>  
</fieldType>

<fieldType name="textSimple" class="solr.TextField" positionIncrementGap="100" >  
   	<analyzer>  
        <tokenizer class="com.chenlb.mmseg4j.solr.MMSegTokenizerFactory" mode="simple" dicPath="dic"/>  
        <filter class="solr.LowerCaseFilterFactory"/>  
    </analyzer>  
</fieldType> 

下载 mmseg-solr-2.4.0.jar mmseg-core-1.1.0.jar

curl 'http://127.0.0.1:8983/solr/article/dataimport?_=1490424730223&indent=on&wt=json'  
--data 'command=delta-import&verbose=false&clean=false&commit=true&optimize=false&debug=true&core=article&entity=article&name=dataimport'

mmseg 中文分词
```