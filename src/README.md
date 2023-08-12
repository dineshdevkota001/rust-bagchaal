basically there are 3 main components here.

1. board:
it is a array of pieces
responsible for 
- management of pieces
- replace piece with another piece
- manage a lobby of killed pieces
- displaying the board and call the pieces to display

2. pieces:
it is a piece
responsible for
- handles property of piece
- color of piece, killed state of the piece
- state of movement, if is moved or not
- handle displaying your piece
- handles movement of piece

4. game manager
responsible for
- game loop
- game logic handling
- player turn state
- has all the component

5. input manager
responsible for
- taking input
- formatting input
- convert input to position
