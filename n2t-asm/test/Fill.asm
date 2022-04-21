// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

(ELOOP)

    @KBD
    D=M

    @KY
    D;JGT

    @NOKY
    1; JGT

(KY)

    @8192
    D=A // initialize counter to "end" of screen buffer
    (SET)
        @SCREEN
        D=D-1 // advance to next iteration
        A=A+D // jump ptr to pixel in current iteration
        M=-1  // color current string
     
        // condition: The page clear has visited the last string (0)
        @ELOOP
        D;JEQ
    
        @SET
        0;JMP

(NOKY)

    @SCREEN
    M=0

    @8192
    D=A // initialize counter to "end" of screen buffer
    (UNSET)
        @SCREEN
        D=D-1 // advance to next iteration
        A=A+D // jump ptr to pixel in current iteration
        M=0  // color current string
     
        // condition: The page clear has visited the last string (0)
        @ELOOP
        D;JEQ
    
        @UNSET
        0;JMP

