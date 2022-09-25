from cgi import test
import pytex2pdf

testtex = '''
\\documentclass{article}
\\begin{document}
Hello, world!
\\end{document}
'''

output = pytex2pdf.tex2pdf(testtex)

with open("test.pdf", "wb") as f:
    f.write(output)
