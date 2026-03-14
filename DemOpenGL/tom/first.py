#!/usr/bin/python

import OpenGL.GL as gl
import OpenGL.Tk as tk


def redraw(o):
    gl.glClearColor(0, 0, 1, 0)
    gl.glClear(gl.GL_COLOR_BUFFER_BIT)


def main():
    gltk = tk.Opengl(width=400, height=400, double=1)
    gltk.redraw = redraw
    gltk.pack(side="top", expand=1, fill="both")
    gltk.mainloop()


if __name__ == "__main__":
    main()
