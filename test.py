import lmatdoc
teststring : str = """
# *hello world* 
This is a new line of some ***stuff***
*This is a new line* of some ***stuff***
~this is a new line of some ***stuff***
~~_This is a new line of some ***stuff***
~This is a new line of some_ ***stuff***
""" 
doc_writer = lmatdoc.LmatDocProcessor(teststring)
b: bytes = doc_writer.make_docx()

with open("test.docx","wb") as f:
    f.write(b)

print("wrote to test.docx!")
