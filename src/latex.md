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
