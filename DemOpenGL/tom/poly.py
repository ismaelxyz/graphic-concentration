#!/usr/bin/python

import OpenGL.GL as gl
import OpenGL.Tk as tk
import numpy as np

n = 50
a = np.arange(0, n)
vertices = np.transpose(
    np.reshape(
        np.array(
            (np.cos(2 * np.pi * a / float(n)), np.sin(3 * 2 * np.pi * a / float(n)))
        ),
        (2, n),
    )
)

colors = np.ones((n, 3))
colors[0] = [1, 0, 0]
colors[25] = [1, 1, 0]
colors.shape = (n, 3)


def redraw(_):
    global n
    gl.glClearColor(0.5, 0.5, 0.5, 0)
    gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
    gl.glOrtho(-1, 1, -1, 1, -1, 1)
    gl.glDisable(gl.GL_LIGHTING)
    gl.glDrawArrays(gl.GL_LINE_LOOP, 0, n)
    gl.glEnable(gl.GL_LIGHTING)


def main():
    global n, colors, vertices
    gltk = tk.Opengl(width=400, height=400, double=1)
    gltk.redraw = redraw
    gltk.autospin_allowed = 1

    gl.glVertexPointerd(vertices)
    gl.glColorPointerd(colors)
    gl.glEnableClientState(gl.GL_VERTEX_ARRAY)
    gl.glEnableClientState(gl.GL_COLOR_ARRAY)

    gltk.pack(side="top", expand=1, fill="both")
    gltk.mainloop()


if __name__ == "__main__":
    main()
