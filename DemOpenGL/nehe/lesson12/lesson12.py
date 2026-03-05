#!/usr/bin/env python3

from __future__ import annotations

import sys
from pathlib import Path
from typing import Union

import OpenGL.GL as gl
import OpenGL.GLU as glu
import OpenGL.GLUT as glut
from PIL import Image


class Lesson12:
    """NeHe Lesson 12 - Display Lists (ported from the C/GLUT tutorial code)."""

    ESCAPE = b"\033"

    _BOX_COLORS: list[tuple[float, float, float]] = [
        (1.0, 0.0, 0.0),
        (1.0, 0.5, 0.0),
        (1.0, 1.0, 0.0),
        (0.0, 1.0, 0.0),
        (0.0, 1.0, 1.0),
    ]

    _TOP_COLORS: list[tuple[float, float, float]] = [
        (0.5, 0.0, 0.0),
        (0.5, 0.25, 0.0),
        (0.5, 0.5, 0.0),
        (0.0, 0.5, 0.0),
        (0.0, 0.5, 0.5),
    ]

    def __init__(self) -> None:
        self.window = 0

        self.xrot = 0.0
        self.yrot = 0.0

        self.texture_id: int | None = None
        self.cube_list: int | None = None
        self.top_list: int | None = None

    def _assets_dir(self) -> Path:
        return Path(__file__).resolve().parent

    def _texture_path(self) -> Path:
        local = self._assets_dir() / "cube.bmp"
        if local.exists():
            return local

        # Keep a fallback compatible with the original C relative path.
        fallback = self._assets_dir() / "Data" / "lesson12" / "cube.bmp"
        return fallback

    def load_textures(self) -> None:
        image_path = self._texture_path()
        image = Image.open(image_path).convert("RGB")

        width, height = image.size
        image_data = image.tobytes("raw", "RGB", 0, -1)

        texture_id = int(gl.glGenTextures(1))
        self.texture_id = texture_id

        gl.glBindTexture(gl.GL_TEXTURE_2D, texture_id)
        gl.glPixelStorei(gl.GL_UNPACK_ALIGNMENT, 1)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_LINEAR)
        gl.glTexParameteri(
            gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_LINEAR_MIPMAP_NEAREST
        )
        glu.gluBuild2DMipmaps(
            gl.GL_TEXTURE_2D,
            3,
            width,
            height,
            gl.GL_RGB,
            gl.GL_UNSIGNED_BYTE,
            image_data,
        )

    def build_lists(self) -> None:
        cube_list = int(gl.glGenLists(2))  # type: ignore[assignment]
        self.cube_list = cube_list

        gl.glNewList(cube_list, gl.GL_COMPILE)
        gl.glBegin(gl.GL_QUADS)

        # Bottom Face
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(1.0, -1.0, -1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)

        # Front Face
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, 1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)

        # Back Face
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, -1.0)

        # Right Face
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(1.0, -1.0, -1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(1.0, 1.0, 1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)

        # Left Face
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)

        gl.glEnd()
        gl.glEndList()

        top_list = cube_list + 1
        self.top_list = top_list

        gl.glNewList(top_list, gl.GL_COMPILE)
        gl.glBegin(gl.GL_QUADS)

        # Top Face
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(1.0, 1.0, 1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)

        gl.glEnd()
        gl.glEndList()

    def init_gl(self, width: int, height: int) -> None:
        self.load_textures()
        self.build_lists()

        gl.glEnable(gl.GL_TEXTURE_2D)
        gl.glClearColor(0.0, 0.0, 1.0, 0.0)
        gl.glClearDepth(1.0)
        gl.glDepthFunc(gl.GL_LESS)
        gl.glEnable(gl.GL_DEPTH_TEST)
        gl.glShadeModel(gl.GL_SMOOTH)

        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

        gl.glEnable(gl.GL_LIGHT0)
        gl.glEnable(gl.GL_LIGHTING)
        gl.glEnable(gl.GL_COLOR_MATERIAL)

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

        if self.texture_id is not None:
            gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.texture_id))

        if self.cube_list is None or self.top_list is None:
            glut.glutSwapBuffers()
            return

        for yloop in range(1, 6):  # 5 rows of cubes.
            for xloop in range(0, yloop):
                gl.glLoadIdentity()

                gl.glTranslatef(
                    1.4 + (float(xloop) * 2.8) - (float(yloop) * 1.4),
                    ((6.0 - float(yloop)) * 2.4) - 7.0,
                    -20.0,
                )

                gl.glRotatef(
                    45.0 - (2.0 * float(yloop)) + float(self.xrot), 1.0, 0.0, 0.0
                )
                gl.glRotatef(45.0 + float(self.yrot), 0.0, 1.0, 0.0)

                gl.glColor3fv(self._BOX_COLORS[yloop - 1])
                gl.glCallList(int(self.cube_list))

                gl.glColor3fv(self._TOP_COLORS[yloop - 1])
                gl.glCallList(int(self.top_list))

        glut.glutSwapBuffers()

    def key_pressed(self, key: Union[bytes, int], *_args) -> None:
        if key == self.ESCAPE:
            glut.glutDestroyWindow(int(self.window))

    def special_key_pressed(self, key: Union[bytes, int], *_args) -> None:
        if key == glut.GLUT_KEY_UP:
            self.xrot -= 0.2
            return

        if key == glut.GLUT_KEY_DOWN:
            self.xrot += 0.2
            return

        if key == glut.GLUT_KEY_LEFT:
            self.yrot += 0.2
            return

        if key == glut.GLUT_KEY_RIGHT:
            self.yrot -= 0.2
            return

    def _register_callbacks(self) -> None:
        glut.glutDisplayFunc(self.draw_scene)
        glut.glutIdleFunc(self.draw_scene)
        glut.glutReshapeFunc(self.resize_scene)
        glut.glutKeyboardFunc(self.key_pressed)
        glut.glutSpecialFunc(self.special_key_pressed)

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

        self.window = glut.glutCreateWindow(
            "Jeff Molofee's GL Code Tutorial ... NeHe '99"
        )
        self._register_callbacks()

        glut.glutFullScreen()
        self.init_gl(640, 480)
        glut.glutMainLoop()


def main() -> None:
    Lesson12().run()


if __name__ == "__main__":
    main()
