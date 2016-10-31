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
