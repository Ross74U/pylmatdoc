# PyLMATDoc
A Rust-based python module for parsing and generating .DOCX from 
LMAT-markdown - a Markdown derived markup language for Word Documents 
```
# Centered Text
*Italic*
**Bold**
***Bold and Italic***
_Underlined_
~Bullet point layer 1
~~Bullet point layer 2
```

## Usage
``` python
import lmatdoc

teststring : str = """
# *hello world* 
This is a new line of some ***stuff***
*This is a new line* of some ***stuff***
~this is a new line of some ***stuff***
~~_This is a new line of some ***stuff***
~This is a new line of some_ ***stuff***
""" 

doc = lmatdoc.LmatDocProcessor(teststring) # 25Mb/s throughput
b: bytes = doc.make_docx()

with open("test.docx","wb") as f:
    f.write(b)
```

the module may be installed via Maturin

