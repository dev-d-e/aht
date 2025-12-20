# aht

It's another hypertext format to describe GUI for native application.

The components build on plane rectangular coordinates.

## Usage

Use `Element` or `Page` to parse.

```
<aht>
    <head>
        <title></title>
    </head>
    <body>
        <inp name="" value="" readonly required>input</inp>
        <button href="" class=a>button</button>
        <area id="b"></area>
    </body>
    <style>
    body {
        column:[100,100];
        row:[100,100],2;
    }
    inp {
        ordinal:0;
        height:100;
    }
    .a {
        position=100,100;
    }
    .id = b {
        width:1000;
        height:100;
        column:2;
        row:1;
    }
    </style>
    <script>
    </script>
 </aht>
```
or use single mark format
```
<aht>
    < head>
        < title>
    <:body>
        < inp name="" value="" readonly required>input
        <button href="" class=a>button
        <area class="" id="b">
    <:style>
    body {
        column:[100,100];
        row:[100,100],2;
    }
    inp {
        ordinal:0;
        height:100;
    }
    .a {
        position=100,100;
    }
    .id = b {
        width:1000;
        height:100;
        column:2;
        row:1;
    }
    <script>
```
 
* "head" element is a collection of metadata for the document.There is only one "head" element in conforming documents.

* "body" element is grid/coordinates layout. There is only one "body" element in conforming documents.

set "column" attribute and "row" attribute with number or points or segments, child elements can be located in body.

* "area" element is grid/coordinates layout. it has "class","id","width","height","column","row"... attributes.

"class" attribute value has a set of space-separated tokens representing the various classes that the element belongs to.

"id" attribute value must be unique in all emelemts.

"width" attribute is horizontal dimension.

"height" attribute is vertical dimension.

set "column" attribute and "row" attribute with number or points or segments, child elements can be located in body.

* "pt" element represents a plain text.

* "inp" element represents input.

* "button" element represents a button.

* "video" element represents video.

* "style" element represents style sheet, which supports mark searching and attribute searching.

* "script" element represents script.
