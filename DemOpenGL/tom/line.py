#!/usr/bin/python

import OpenGL.GL as gl
import OpenGL.Tk as tk


def redraw(o):
    gl.glClearColor(0.5, 0.5, 0.5, 0)
    gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
    gl.glOrtho(0, 1, 0, 1, 0, 1)
    gl.glDisable(gl.GL_LIGHTING)
    gl.glBegin(gl.GL_LINES)
    gl.glColor3f(1, 1, 0)
    gl.glVertex2f(0, 0)
    gl.glColor3f(1, 0, 1)
    gl.glVertex2f(1, 1)
    gl.glColor3f(1, 0, 0)
    gl.glVertex2f(1, 0)
    gl.glColor3f(0, 0, 1)
    gl.glVertex2f(0, 1)
    gl.glEnd()  # type: ignore[call-arg]
    gl.glEnable(gl.GL_LIGHTING)


def main():
    gltk = tk.Opengl(width=400, height=400, double=1)
    gltk.redraw = redraw
    gltk.pack(side="top", expand=1, fill="both")
    gltk.mainloop()


if __name__ == "__main__":
    main()
