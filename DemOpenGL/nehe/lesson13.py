#!/usr/bin/env python3

from __future__ import annotations

import sys
import math
from typing import Any, Optional, Union, cast

import OpenGL.GL as gl
import OpenGL.GLU as glu
import OpenGL.GLUT as glut


class Lesson13:
    """NeHe Lesson 13 - Bitmap Fonts (ported from the C/GLUT tutorial code)."""

    ESCAPE = b"\033"

    def __init__(self) -> None:
        self.window: int = 0

        self.base_list: Optional[int] = None
        self.cnt1: float = 0.0
        self.cnt2: float = 0.0

        # PyOpenGL exposes GLUT bitmap font pointers at runtime, but type stubs often
        # don't declare these attributes. Using getattr keeps Pylance happy.
        font = getattr(glut, "GLUT_BITMAP_HELVETICA_18", None)
        if font is None:
            font = getattr(glut, "GLUT_BITMAP_8_BY_13", None)
        if font is None:
            raise RuntimeError("No GLUT bitmap font available in this environment")
        self._font: Any = font

    def build_font(self) -> None:
        base_raw = gl.glGenLists(96)
        if base_raw is None or int(base_raw) == 0:
            raise RuntimeError("glGenLists failed while building font")
        base = int(base_raw)
        self.base_list = base

        for i in range(96):
            gl.glNewList(base + i, gl.GL_COMPILE)
            glut.glutBitmapCharacter(self._font, 32 + i)
            gl.glEndList()

    def kill_font(self) -> None:
        if self.base_list is None:
            return

        gl.glDeleteLists(int(self.base_list), 96)
        self.base_list = None

    def gl_print(self, text: str) -> None:
        if not text or self.base_list is None:
            return

        base = int(self.base_list)
        for ch in text:
            code = ord(ch)
            if 32 <= code < 128:
                gl.glCallList(base + (code - 32))

    def init_gl(self, width: int, height: int) -> None:
        gl.glClearColor(0.0, 0.0, 0.0, 0.0)
        gl.glClearDepth(1.0)
        gl.glDepthFunc(gl.GL_LESS)
        gl.glEnable(gl.GL_DEPTH_TEST)
        gl.glShadeModel(gl.GL_SMOOTH)

        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)

        gl.glMatrixMode(gl.GL_MODELVIEW)

        self.build_font()

    def resize_scene(self, width: int, height: int) -> None:
        if height == 0:
            height = 1

        gl.glViewport(0, 0, width, height)
        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

    def draw_scene(self) -> None:
        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
        gl.glLoadIdentity()
        gl.glTranslatef(0.0, 0.0, -1.0)

        gl.glColor3f(
            float(math.cos(self.cnt1)),
            float(math.sin(self.cnt2)),
            float(1.0 - 0.5 * math.cos(self.cnt1 + self.cnt2)),
        )

        gl.glRasterPos2f(
            float(-0.2 + 0.35 * math.cos(self.cnt1)),
            float(0.35 * math.sin(self.cnt2)),
        )

        self.gl_print("OpenGL With NeHe")

        self.cnt1 += 0.01
        self.cnt2 += 0.0081

        glut.glutSwapBuffers()

    def key_pressed(self, key: Union[bytes, int], *_args) -> None:
        if key != self.ESCAPE:
            return

        self.kill_font()
        if self.window:
            glut.glutDestroyWindow(int(self.window))

    def _register_callbacks(self) -> None:
        glut.glutDisplayFunc(self.draw_scene)
        glut.glutIdleFunc(self.draw_scene)
        glut.glutReshapeFunc(self.resize_scene)
        glut.glutKeyboardFunc(self.key_pressed)

    def run(self) -> None:
        glut.glutInit(sys.argv)
        glut.glutInitDisplayMode(
            int(glut.GLUT_RGBA)
            | int(glut.GLUT_DOUBLE)
            | int(glut.GLUT_ALPHA)
            | int(glut.GLUT_DEPTH)
        )
        glut.glutInitWindowSize(640, 480)
        glut.glutInitWindowPosition(0, 0)

        window_raw = glut.glutCreateWindow(
            "Jeff Molofee's GL Code Tutorial ... NeHe '99"
        )
        # Some stubs declare this as Optional[int]; guard before casting.
        if window_raw is None or int(cast(Any, window_raw)) == 0:
            raise RuntimeError("glutCreateWindow failed")
        self.window = int(cast(Any, window_raw))
        self._register_callbacks()

        glut.glutFullScreen()
        self.init_gl(640, 480)
        glut.glutMainLoop()


def main() -> None:
    Lesson13().run()


if __name__ == "__main__":
    main()
