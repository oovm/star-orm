---
zhihu-url: https://www.zhihu.com/question/267165403
---

做个小调研吧, 针对 TeX 的一些痛点进行了改进.

认可的人多的话我就写个 demo.

## 移除宏语言

不要做成宏语言, 也不要追求图灵完备.

转义符越多反转义越复杂, 就一个 `\` 足以.

复杂运算定义一套稳定 abi 交给外部就行.

## 词捕获

lexer 不再按照 char 来切词, 直接按 letter 和 number 切.

比如说 `a^100` 自动变成 $a^{100}$, TeX 里会变成 $a^100$

## `( )` 语块

代替原来的 `{ }`, 比如 `\frac{a}{b}` 需写成 `\frac(a, b)`

## `{ }` 语块

用于替代 `\begin` 和 `\end`

比如

```latex
\begin{aligned}
    a &= b + c \\
    d &= e + f \\
\end{aligned}
```

需要写成

```latex
\align(untagged) {
    a &= b + c \br
    d &= e + f \br
}
```

## [待定]数字单位

数字可以带单位, 比如 `1cm`, `1pt` 之类的

## 精细空格和精细换行

空格支持用 `\s(length: -0.33)` 设置长度, 不再支持 `\ `, `\,` 之类的

换行不再笼统的使用 `\\`, 需区分 `\r`, `\br`, `\wbr`, 后面同样可以带单位

## [待定]字符类型

在函数内时用于阻止进一步解析

在函数外则渲染为普通的 text

## `_` 和 `^` 运算符

将 _ 和 ^ 作为运算符, 优先级最低, 且右结合

| 表达式         | 解糖                    | LaTeX     |
|-------------|-----------------------|-----------|
| `a_b`       | `\sub(a, b)`          | $a_b$     |
| `a_b_c`     | `\sub(a, sub(b, c))`  | $a_{b_c}$ |
| `a_{b_c}`   | `\sub(a, sub(b, c))`  | $a_{b_c}$ |
| `{a_b}_c`   | `\sub(\sub(a, b), c)` | ${a_b}_c$ |
| `a_^{b, c}` | `\subsup(a, b, c)`    | $a_b^c$   |
| `a_b^c`     | `\sup(a, sub(b, c))`  | $a_{b^c}$ |
| `a_{b^c}`   | `\sub(a, sup(b, c))`  | $a_{b^c}$ |
| `{a_b}^c`   | `\sup(\sub(a, b), c)` | ${a_b}^c$ |

## pair 函数

可以少写点 `\left`, `\right`

| 表达式                  | 解糖                                   | LaTeX                         |
|----------------------|--------------------------------------|-------------------------------|
| `\abs(a)`            | `\pair(\|, \|, a)`                   | $\left\|a\right\|$            |
| `\abs(a/b, inline)`  | `\pair(\|, \|, a/b, display: false)` | $\left\|\frac{a}{b}\right\|$  |
| `\abs(a/b, display)` | `\pair(\|, \|, a/b, display: true)`  | $\left\|\dfrac{a}{b}\right\|$ |
| `\norm(a)`           | `\pair(∥, ∥, a)`                     | $\left\lVert a \right\rVert$  |
| `\ceil(a)`           | `\pair(⌈, ⌉, a)`                     | $\left\lceil a\right\rceil$   |
| `\floor(a)`          | `\pair(⌊, ⌋, a)`                     | $\left\lfloor a\right\rfloor$ |

## curry 函数

如果函数参数未满, 则返回一个新函数, 该函数接受剩余参数

比如定义 `\let(norm) {\pair(∥, ∥)}`, 那么 `\norm(a)` 就等于 `\pair(∥, ∥)(a)` 等于 `\pair(∥, ∥, a)`

## ligature 宏

某些特殊符号直接拼, 不需要写名字, 增强可读性

| 表达式    | 解糖                   | LaTeX                |
|--------|----------------------|----------------------|
| `!=`   | `\neq`               | $\neq$               |
| `<=`   | `\leqslant`          | $\leqslant$          |
| `>=`   | `\geqslant`          | $\geqslant$          |
| `+-`   | `\pm`                | $\pm$                |
| `-+`   | `\mp`                | $\mp$                |
| `->`   | `\rightarrow`        | $\rightarrow$        |
| `=>`   | `\Rightarrow`        | $\Rightarrow$        |
| `->>`  | `\twoheadrightarrow` | $\twoheadrightarrow$ |
| `>->`  | `\rightarrowtail`    | $\rightarrowtail$    |
| `\|->` | `\mapsto`            | $\mapsto$            |
| `~>`   | `\leadsto`           | $\leadsto$           |
| `~>>`  | `\rightsquigarrow`   | $\rightsquigarrow$   |
| `~`    | `\sim`               | $\sim$               |
| `~~`   | `\approx`            | $\approx$            |

## dot 链

一切皆对象, 所有对象带一个 display 方法.

然后就可以用 dot 链来写了, 最后调用 display 派发到正确的渲染函数.

| 表达式                       | LaTeX                     |
|---------------------------|---------------------------|
| `\gt`                     | $\gt$                     |   
| `\gt.eq`                  | $\geqslant$               |
| `\gt.not`                 | $\ngtr$                   |
| `\gt.eq.not`              | $\ngeqslant$              |
| `\gt.not.eq`              | $\ngeqslant$              |
| `{a / b}.inline`          | $\frac{a}{b}$             |
| `{a / b}.display`         | $\dfrac{a}{b}$            |
| `{a.phantom / b}.display` | $\dfrac{\phantom {a}}{b}$ |
