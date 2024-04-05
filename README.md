# E

my editor啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊1啊啊啊啊222啊啊啊啊啊啊啊啊啊啊啊啊啊啊

## Installation

```bash
cargo install e
```

## Usage

```c
void editorDrawRows(struct abuf *ab) {
  int y;
  for (y = 0; y < E.screenrows; y++) {
    int filerow = y + E.rowoff;
    if (filerow >= E.numrows) {
      if (E.numrows == 0 && y == E.screenrows / 3) {
        char welcome[80];
        int welcomelen = snprintf(welcome, sizeof(welcome),
          "Kilo editor -- version %s", KILO_VERSION);
        if (welcomelen > E.screencols) welcomelen = E.screencols;
        int padding = (E.screencols - welcomelen) / 2;
        if (padding) {
          abAppend(ab, "~", 1);
          padding--;
        }
        while (padding--) abAppend(ab, " ", 1);
        abAppend(ab, welcome, welcomelen);
      } else {
        abAppend(ab, "~", 1);
      }
    } else {
      int len = E.row[filerow].size;
      if (len > E.screencols) len = E.screencols;
      abAppend(ab, E.row[filerow].chars, len);
    }
    abAppend(ab, "\x1b[K", 3);
    if (y < E.screenrows - 1) {
      abAppend(ab, "\r\n", 2);
    }
  }
}
```

To get the row of the file that we want to display at each y position,

we add E.rowoff to the y position. So we define a new variable filerow that contains that value, and use that as the
index into E.row.

Now where do we set the value of E.rowoff? Our strategy will be to check if the cursor has moved

outside of the visible window, and if so, adjust E.rowoff so that the cursor is just inside the visible window.

We’ll put this logic in a function called editorScroll(), and call it right before we refresh the screen.
