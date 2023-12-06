# LCD Display

### Description
A friend of yours has just bought a new computer. before this, the most powerful machine he ever used was a pocket calculator. he is a little disappointed because he liked the LCD display of his calculator more than the screen on his new computer! To make him happy, write a program that prints numbers in LCD display style. 

### Input
The input file contains several lines one for each number to be displayed. Each line contains integers *s* and *n* is the number to be displayed (0 <= n <= 99,999,999) and *s* is the size in which it shall be displayed (1 <= s <= 10). The input will be terminated by a line containing two zeros, which should not be processed.

### Output
Print the numbers specified in the input file in an LCD display-style using *s* "-" signs for the horizontal segments and *s* "|" signs for the vertical ones. Each digit occupies exactly *s+2* columns and *2s+3* rows. Be sure to fill all the white space occupied by the digits which blanks, including the last digit. There must be exactly one column of blanks between two digits. 

Output a blank line after each number. you will find an example of each digit in the sample output below.

<div style="display: flex; column-gap: 50px;">
<div>
<h3>Sample Input</h3>
<pre>
2 12345
0 0
</pre>
</div>

<div>
<h3>Sample Output</h3>
<pre>
        --    --           --
   |      |     |  |  |   |   
   |      |     |  |  |   |
        --    --    --     --
   |   |        |     |      |
   |   |        |     |      |
        --    --           --
</pre></div>
</div>

