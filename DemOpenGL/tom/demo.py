#!/usr/bin/python

from OpenGL.GL import *
from OpenGL.Tk import *
from OpenGL.Tk import _default_root


class Demo:

    def __init__(self):
        self.bar = Frame(_default_root, relief=RAISED, borderwidth=2)
        self.bar.pack(fill=X)
        menubar = self.make_menu()
        _default_root.config(menu=menubar)
        self.gl = Opengl(self.bar, width=300, height=300, double=1, depth=1)
        self.gl.redraw = self.draw_lines
        self.gl.set_centerpoint(30, 0, 0)
        self.gl.set_eyepoint(140)
        self.gl.pack(side='top', expand=1, fill='both')
        self.gl.grob = -1

    def make_menu(self):
        menu = Menu(_default_root)
        demos = Menu(menu, tearoff=0)

        demos.add_command(label='Blue', underline=0, command=self.set_blue)
        demos.add_command(label='Lines', underline=0, command=self.set_lines)
        demos.add_command(label='Text', underline=0, command=self.set_text)

        menu.add_cascade(label="Demos", menu=demos)
        menu.add_command(label='Quit', underline=0, background='red',
                         activebackground='green', command=_default_root.quit)

        return menu

    def draw_lines(self, gl):
        glClearColor(0, 0, 0, 0)
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
        glDisable(GL_LIGHTING)
        glBegin(GL_LINES)
        glColor3f(1, 1, 0)
        glVertex2f(0, -30)
        glColor3f(1, 0, 1)
        glVertex2f(60, 30)
        glColor3f(1, 0, 0)
        glVertex2f(60, -30)
        glColor3f(0, 0, 1)
        glVertex2f(0, 30)
        glEnd()
        glEnable(GL_LIGHTING)

    def set_lines(self):
        self.gl.redraw = self.draw_lines
        self.gl.tkRedraw()

    def draw_blue(self, gl):
        glClearColor(0, 0, 1, 0)
        glClear(GL_COLOR_BUFFER_BIT)

    def set_blue(self):
        self.gl.redraw = self.draw_blue
        self.gl.tkRedraw()

    def draw_text(self, gl):
        glClearColor(0, 0, 0.5, 0)
        glClear(GL_COLOR_BUFFER_BIT)
        if gl.grob == -1:
            try:
                from logo import define_logo
            except:
                from .logo import define_logo

            gl.grob = glGenLists(1)
            glNewList(gl.grob, GL_COMPILE_AND_EXECUTE)
            glMaterialfv(GL_FRONT, GL_DIFFUSE, [1, 0, 0, 0])
            define_logo()
            glEndList()
        else:
            glCallList(gl.grob)

    def set_text(self):
        self.gl.redraw = self.draw_text
        self.gl.tkRedraw()


def main():
    demo = Demo()
    _default_root.mainloop()


if __name__ == '__main__':
    main()
