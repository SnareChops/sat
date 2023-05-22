if at top level:
    
    starts with `test`: parse line as Test(name, block)
    // line with `=` as first Special: parse line as assignment
    //    - json may span multiple lines
    
if inside a block:
    starts with assert: parse line as Assert(expression)
    contains `=` as first Special: parse line as assignment
    contains `==` as first Special: parse line as equality
    