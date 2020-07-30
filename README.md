# Arjan Parser
This repository is an effort to unify and standardize all of the string manipulation methods used in Arjan into a single module. For maximum performance we plan to re-write most of the methods in rust.

## Background
Currently there’s various different functions dedicated to parsing html strings that are spread throughout the different modules of Arjan Tools. Many of the methods have been copy pasted from one module to another or have been re-written differently to achieve the same result. This is bad practice and hard to maintain. 

The idea of the arjan parser is to manipulate HTML without having to create (and re-create) an abstract syntax tree (AST) for the HTML as creating the AST is often the most time-consuming operation. Its true that once the AST has been generated sequential operations on the DOM will be quicker but whenever there's a change in the html document the DOM has to be re-constructed so more often than not the operation can end up taking longer.

**An update with an AST:**
1. Create the AST from the HTML
2. Update the AST with the desired changes
3. Rebuild the HTML from the AST
4. save the HTML 

**As opposed to:**
1. Update the HTML with desired changes
2. Save the HTML

# API
A lot of the methods replicate standard DOM methods like getElementById on html-strings.

## methods
1. **splitElements(html)** -> receives an html document as a string returns an array of elements.
2. **containsText(element)** → returns true if the element has text 
3. **getTextContent(element)** → returns the text content of an element
4. **replaceTextContent(element, content)** → replaces the text content of an element
5. **insertId(element)** → if the element doesnt have an id create one and return it else return its existing id
6. **getElementById(id)** → returns the element
7. **getAttributes(element)** → return an array with the attributes of the element
8. **getAtttribute(element, attribute)** → returns the value of an attribute
9. **getElementsByType(html, type)** → returns an array with all elements by type
10. **replaceElement(element, newElement)**
11. **removeElement(element)**
12. **createTree(html) → r**eturns an abstract syntax tree
13. **createHtml(Obj)** → returns an HTML string
