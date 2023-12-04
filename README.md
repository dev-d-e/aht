# aht

It's another hypertext format to describe GUI for native application.

## Usage

```
<aht>
    <head>
        <title></title>
    </head>
    <body column="[100,100]" row="[100,100],2">
        <a href=""></a>
        <area class="" id="" width="1000" height="100" column="2" row=""></area>
    </body>
 </aht>
```
 
* "head" element is a collection of metadata for the document.There is only one "head" element in conforming documents.

* "body" element is grid layout. There is only one "body" element in conforming documents.

set "column" attribute and "row" attribute with segments or number, child elements can be located in body, these position are calculated by segments array or column number.

* "area" element is grid layout. it has "class","id","width","height","column","row"... attributes.

"class" attribute value has a set of space-separated tokens representing the various classes that the element belongs to.

"id" attribute value must be unique in all emelemts.

"width" attribute is horizontal dimension.

"height" attribute is vertical dimension.

set "column" attribute and "row" attribute with segments or number, child elements can be located in body, these position are calculated by segments array or column number.

* "a" element has an "href" attribute,it represents a hyperlink.
