# Graphical Editor

### Description
Graphical editors such as Photoshop allow us to alter bit-mapped images in the same way that text editors allow us to modify documents. Images are represented as *M x N* array of pixels, where each pixel has a given color.

Your task is to write a program which simulates a simple interactive graphical editor. 

### Input
The input consists of sequence of editor commands, one per line. Each command is represented by one capital letter placed as the first character of the line. If the command needs parameters, they will be given on the same line separated by spaces.

Pixel coordinates are represented by two integers, a column number between *1...M* and a row number between *1...N*, where 1 <= M, N <= 250. the origin sits in the upper-left corner of the table. Colors are specified by capital letters.

The editor accepts the following commands:

| Commands        | Definition                                                                                                                                                                                                                   |
|-----------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| I M N           | Create a new *M x N* image with all pixels initially colored white (O).                                                                                                                                                      |
| C               | Clear the table by setting all pixels white (O). The size remains unchanged.                                                                                                                                                 |
| L X Y C         | Colors the pixel (X, Y) in color (C).                                                                                                                                                                                        |
| V X Y1 Y2 C     | Draw a vertical segment of color (C) in column X, between the rows Y1 and Y2 inclusive.                                                                                                                                      |
| H X1 X2 Y C     | Draw a horizontal segment of color (C) in the row Y, between the columns X1 and X2 inclusive.                                                                                                                                |
| K X1 Y1 X2 Y2 C | Draw a filled rectangle of color C, where (X1, Y1) is the upper-left and (X2, Y2) the lower right corner.                                                                                                                    |
| F X Y C         | Fill the region R with the color C, where R is defined as follows. Pixel (X, Y) belongs to R. Any other pixel which is the same color as pixel (X, Y) and shares a common side with any pixel in R also belongs to this region. |
| S Name          | Write the file name is MSDOS 8.3 format followed by the contents of the current image.                                                                                                                                       |
| X               | Terminate the session.                                                                                                                                                                                                       

### Output
On every command **S Name**, print the filename *Name* and contents of the current image. Each row is represented by the color contents of each pixel. See the sample output.

Ignore the entire line of any command defined by a character other than I, C, L, V, H, K, F, S, or X, and pass on to the next command. In case of other errors, the program behaviour is unpredictable.

<div style="display: flex; column-gap: 50px;">
<div>
<h3>Sample Input</h3>
<pre>
I 5 6
L 2 3 A
S one.bmp
G 2 3 J
F 3 3 J
V 2 3 4 W
H 3 4 2 Z
S two.bmp
X
</pre>
</div>

<div>
<h3>Sample Output</h3>
<pre>
one.bmp
OOOOO
OOOOO
OAOOO
OOOOO
OOOOO
two.bmp
JJJJJ
JJZZJ
JWJJJ
JWJJJ
JJJJJ
JJJJJ
</pre></div>
</div>

