#!/usr/bin/python

import OpenGL.GL as gl
import OpenGL.Tk as tk


class Demo:

    def __init__(self):
        self.bar = tk.Frame(tk._default_root, relief=tk.RAISED, borderwidth=2)
        self.bar.pack(fill=tk.X)
        menubar = self.make_menu()
        tk._default_root.config(menu=menubar)
        self.gltk = tk.Opengl(self.bar, width=300, height=300, double=1, depth=1)
        self.gltk.redraw = self.draw_lines
        self.gltk.set_centerpoint(30, 0, 0)
        self.gltk.set_eyepoint(140)
        self.gltk.pack(side="top", expand=1, fill="both")
        self.gltk.grob = -1

    def make_menu(self):
        menu = tk.Menu(tk._default_root)
        demos = tk.Menu(menu, tearoff=0)

        demos.add_command(label="Blue", underline=0, command=self.set_blue)
        demos.add_command(label="Lines", underline=0, command=self.set_lines)
        demos.add_command(label="Text", underline=0, command=self.set_text)

        menu.add_cascade(label="Demos", menu=demos)
        menu.add_command(
            label="Quit",
            underline=0,
            background="red",
            activebackground="green",
            command=tk._default_root.quit,
        )

        return menu

    def draw_lines(self, gltk):
        gl.glClearColor(0, 0, 0, 0)
        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
        gl.glDisable(gl.GL_LIGHTING)
        gl.glBegin(gl.GL_LINES)
        gl.glColor3f(1, 1, 0)
        gl.glVertex2f(0, -30)
        gl.glColor3f(1, 0, 1)
        gl.glVertex2f(60, 30)
        gl.glColor3f(1, 0, 0)
        gl.glVertex2f(60, -30)
        gl.glColor3f(0, 0, 1)
        gl.glVertex2f(0, 30)
        gl.glEnd()  # type: ignore[call-arg]
        gl.glEnable(gl.GL_LIGHTING)

    def set_lines(self):
        self.gltk.redraw = self.draw_lines
        self.gltk.tkRedraw()

    def draw_blue(self, gltk):
        gl.glClearColor(0, 0, 1, 0)
        gl.glClear(gl.GL_COLOR_BUFFER_BIT)

    def set_blue(self):
        self.gltk.redraw = self.draw_blue
        self.gltk.tkRedraw()

    def draw_text(self, gltk):
        gl.glClearColor(0, 0, 0.5, 0)
        gl.glClear(gl.GL_COLOR_BUFFER_BIT)
        if gltk.grob == -1:
            try:
                from logo import define_logo
            except:
                from .logo import define_logo

            gltk.grob = gl.glGenLists(1)
            gl.glNewList(gltk.grob, gl.GL_COMPILE_AND_EXECUTE)
            gl.glMaterialfv(gl.GL_FRONT, gl.GL_DIFFUSE, [1, 0, 0, 0])
            define_logo()
            gl.glEndList()
        else:
            gl.glCallList(gltk.grob)

    def set_text(self):
        self.gltk.redraw = self.draw_text
        self.gltk.tkRedraw()


def main():
    demo = Demo()
    tk._default_root.mainloop()


if __name__ == "__main__":
    main()
