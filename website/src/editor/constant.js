const defaultCode = `
# Welcome into the Tiny Mathematic Language online editor

# You can perform simple operations
1+1
1+2+3+4+5+6+7+8+9
3*3
(3/8) * (32+4) + 34
2+2
8/(2*(2+2))
4.5*10^-3*35*10^6

# Use constants
PI
cos(PI)
cos(PI/2)
sin(23*PI)

# Use mathematical function
cos(5)
sin(4/8)
log2(1000)
exp(ln(7))
10 / 3
trunc(10/3)
fract(10/3)

# And print text

"J'aime le chocolat"
"Full: "1/3"   Truncated: "trunc(1/3)"    Decimal: "fract(1/3)

# And now somethings more interesting
a = 12
b = 32
"a = "a" & b = "b
"hypotenuse = sqrt(a*a+b*b)"
"hypotenuse = "sqrt(a*a+b*b)

`;

export { defaultCode };
