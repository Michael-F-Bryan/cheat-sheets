# LaTeX Snippets #

## Inserting Images

```tex
\usepackage{graphicx}
...

\begin{figure}[h]
\centering
\includegraphics[width=0.8\textwidth]{./q1_model.png}
\caption{Question 1 System Model}\label{fig:q1_model}
\end{figure}
```

Sometimes you'll get those really annoying images who won't stay to their
designated sections. You can use the `placeins` package to keep them under
control.

```tex
\usepackage[section]{placeins}
```


## Equations

First you'll need the `amsmath` package:

```tex
\usepackage{amsmath}  
```

Then you can enter math like normal, separating each line with `\\`. You can
use the `&` symbol to align each line. LaTeX will try to line up all the
ampersands.

```tex
\begin{align}
\frac{D(x) - D_i}{x} &= \frac{D_o - D_i}{1} \\
D(x) &= x \times (D_o - D_i) + D_i  \\
D(x) &= x \times 0.5 + 0.25 \label{eq:q2_diameter}
\end{align}
```

If you need to embed text in your equations, use the `intertext` command.

```tex
\begin{align}
  P_i + \frac{1}{2} \rho V_i^2 &= P(x) + \frac{1}{2} \rho \Big[ V(x) \Big]^2 \\
  \intertext{Let $P_i$ = 0, so P(x) is now gauge pressure (relative to inlet)} \\
  P(x) &= \frac{1}{2} \rho V_i^2 - \frac{1}{2} \rho \Big[ V(x) \Big]^2 \\
\end{align}
```

Slanted fractions can be accessed via the `xfrac` package.

```tex
\usepackage{xfrac}
...

\begin{align}
  \sfrac{1}{2} - \sfrac{1}{3} &= \sfrac{1}{6}
\end{align}
```

When using `Python` and `Pandas` for numerical analysis, it is almost trivial
to set things up so that the results are exported to a file, then included in
a LaTeX report automatically.

On the `Python` side:

```python
with open('data_table.tex', 'w') as f:
    table = data.to_latex(index=False, escape=False)
    f.write(table)
```

And in LaTeX:

```tex
\usepackage{booktabs}           
...

\begin{table}[h]
  \caption{Calculated and Measured Data}\label{tbl:data_table}
  \input{./data_table.tex}
\end{table}
```


## Appendices

To start off the appendices section requires just one command:

```tex
\appendix
```

Thereafter, any `section` is added as an appendix instead of a normal section.

To embed a PDF as one of your appendices, you can use the `pdfpages` package:

```tex
\usepackage{pdfpages}

% Note, I added the front page separately so you can add a \section{} command
% to make the appendix appear in the ToC. You also scale the document down a
% tad to make it fit more nicely
\includepdf[pagecommand=\section{Numerical Analysis}, scale=0.75, pages=1]{./Numerical_Analysis.pdf}
\includepdf[scale=0.75, pages=2-]{./Numerical_Analysis.pdf}
```


## Footnotes

Adding footnotes is pretty straightforward.

```tex
I'm writing something here to test \footnote{footnotes working fine}
several features.
```

Or if you have a particularly long footnote, you can insert a `\footnotemark`
and then write the footnote itself at the bottom of your paragraph.

```tex
I'm writing something here to test \footnote[10]{footnotes working fine}
several features. You can write the footnote text\footnotemark in its
own line.
\footnotetext{Second footnote}
```


## Nomenclature

A nomenclature works pretty much the same way as index entries.

First you need to import the package and let it do some setup stuff:

```tex
\usepackage{nomencl}
\makenomenclature
```

Then you'll define some symbols:

```tex
\nomenclature{$c$}{Speed of light in a vacuum inertial frame}
\nomenclature{$h$}{Planck constant}
```

And finally tell LaTeX to print the nomenclature:

```tex
\printnomenclature
```


## Referencing

Referencing in LaTeX is designed to be fairly painless and efficient to do.

To cite various sources throughout your work, just add in a `\cite{foo}`, where
`foo` would be a name for some reference in your `*.bib` file.

The easiest way to manage your references is with a `bib` file that sits next
to your other `tex` files. It's just a basic text file with references and
their various bits of metadata listed;

```tex
@misc{chapter_6_notes,
    author =       "Ramesh Narayanaswamy",
    title =        "Flow Over Immersed Bodies: Concepts of Lift and Drag - Chapter 6",
    year =         "2016",
    URL =          "https://lms.curtin.edu.au/bbcswebdav/pid-4268432-dt-content-rid-23946938_1/courses/308810-CU-061-01-Sxx-x2/2016/LectureNotes/MCEN3002_2016_Chapter_06.pdf"
}
```

It's pretty much identical to what you'd normally need to enter in when using
`Word`, except instead of filling in random boxes, it's entered into a text
file that anything (not just M$ products) can read. Plus it's easy to put into
version control.

Once you've written up your reference database, all you need to do is tell LaTeX
to print a bibliography using your `foo.bib` file, and also the referencing
style to use.


```tex
\bibliographystyle{apalike}
\bibliography{references}
```

Check out [this page][refs] for a list of the various bibliography and citation
styles, and [this page][dot_bib] for the various fields and entry types for your
`*.bib` file.


## Templates

I've got a pair of templates I use for pretty much everything.

* [General report template](https://github.com/Michael-F-Bryan/cheat-sheets/blob/master/src/lab_report_template.tex
)
* [Cover Sheet](https://github.com/Michael-F-Bryan/cheat-sheets/blob/master/src/cover_page_template.tex)

## Code Listings

To insert code snippets you'll need the `listings` package.

```tex
\usepackage{listings}
```

Then you can use the `lstlisting` environment for your code.

```tex
\begin{lstlisting}
def foo(x):
    """Do something interesting"""
    return x**x
\end{lstlisting}
```

Alternatively, if you've got a file already that you want include, you can input
it. Optionally specifying line numbers and language.

```tex
\lstinputlisting[language=Octave, firstline=2, lastline=12]{BitXorMatrix.m}
```

With the `lstlisting` environment, you can also add a caption or language.

```tex
\begin{lstlisting}[language=Python, caption=Python example]
...

\end{lstlisting}
```

And then you can chuck a *list of listings* (similar to list of figures, etc) at
the front of your document.

```tex
\lstlistoflistings
```


## Custom Floats

By default, LaTeX comes with `figure` and `table` as the only floats available,
although it's fairly trivial to add your own custom ones.

First you need to import the `float` package,

```tex
\usepackage{float}
```

Then define your custom float.

```tex
\floatstyle{ruled}
\newfloat{program}{thp}{lop}
\floatname{program}{Program}
```

And use it,

```tex
\begin{program}
  \begin{verbatim}

class HelloWorldApp {
  public static void main(String[] args) {
    //Display the string
    System.out.println("Hello World!");
  }
}
\end{verbatim}
  \caption{The Hello World! program in Java.}
\end{program}
```

From the LaTeX wikibook:

1. Add `\usepackage{float}` to the preamble of your document
2. Declare your new float using: `\newfloat{type}{placement}{ext}[outer counter]`,
   where:
    * **type** - the new name you wish to call your float, in this instance, 'program'.
    * **placement** - t, b, p, or h (as previously described in Placement), where
      letters enumerate permitted placements.
    * **ext** - the file name extension of an auxiliary file for the list of
      figures (or whatever). Latex writes the captions to this file.
    * **outer counter** - the presence of this parameter indicates that the
      counter associated with this new float should depend on outer counter, for
      example 'chapter'.
3. The default name that appears at the start of the caption is the type. If you
   wish to alter this, use `\floatname{type}{floatname}`.
4. Changing float style can be issued with `\floatstyle{style}` (Works on all
   subsequent `\newfloat` commands, therefore, must be inserted before
   `\newfloat` to be effective).
    * **plain** - the normal style for Latex floats, but the caption is always
      below the content.
    * **plaintop** - the normal style for Latex floats, but the caption is
      always above the content.
    * **boxed** - a box is drawn that surrounds the float, and the caption is
      printed below.
    * **ruled** - the caption appears above the float, with rules immediately
      above and below. Then the float contents, followed by a final horizontal rule.




[refs]: http://sites.stat.psu.edu/~surajit/present/bib.htm
[dot_bib]: https://www.andy-roberts.net/res/writing/latex/bibentries.pdf
