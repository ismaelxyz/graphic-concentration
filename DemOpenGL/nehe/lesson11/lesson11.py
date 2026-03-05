#!/usr/bin/env python3

from __future__ import annotations

import math
import sys
from pathlib import Path
from typing import Union

import OpenGL.GL as gl
import OpenGL.GLU as glu
import OpenGL.GLUT as glut
from PIL import Image


GRID_SIZE = 45


class Lesson11:
    """NeHe Lesson 11 - Waving flag (ported from the C/GLUT tutorial code)."""

    ESCAPE = b"\033"

    def __init__(self) -> None:
        self.window = 0

        self.xrot = 0.0
        self.yrot = 0.0
        self.zrot = 0.0

        self.wiggle_count = 0

        self.texture_id: int | None = None
        self.points: list[list[list[float]]] = [
            [[0.0, 0.0, 0.0] for _ in range(GRID_SIZE)] for _ in range(GRID_SIZE)
        ]

    def _assets_dir(self) -> Path:
        return Path(__file__).resolve().parent

    def _texture_path(self) -> Path:
        # Repo asset lives next to this script.
        local = self._assets_dir() / "tim.bmp"
        if local.exists():
            return local

        # Keep a fallback compatible with the original C path.
        fallback = self._assets_dir() / "Data" / "lesson11" / "tim.bmp"
        return fallback

    def _init_points(self) -> None:
        # Mirror the C loops: float_x/float_y in [0, 9) step 0.2 => indices 0..44.
        for x_index in range(GRID_SIZE):
            for y_index in range(GRID_SIZE):
                self.points[x_index][y_index][0] = (x_index * 0.2) - 4.4
                self.points[x_index][y_index][1] = (y_index * 0.2) - 4.4
                self.points[x_index][y_index][2] = float(
                    math.sin(math.radians(x_index * 8.0))
                )

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
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_LINEAR)
        gl.glTexImage2D(
            gl.GL_TEXTURE_2D,
            0,
            gl.GL_RGB,
            width,
            height,
            0,
            gl.GL_RGB,
            gl.GL_UNSIGNED_BYTE,
            image_data,
        )

    def init_gl(self, width: int, height: int) -> None:
        self.load_textures()
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

        self._init_points()

    def resize_scene(self, width: int, height: int) -> None:
        if height == 0:
            height = 1

        gl.glViewport(0, 0, width, height)
        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

    def _update_wave(self) -> None:
        if self.wiggle_count == 2:
            # Cycle the sine values across the grid (same as C).
            for y in range(GRID_SIZE):
                self.points[GRID_SIZE - 1][y][2] = self.points[0][y][2]

            for x in range(GRID_SIZE - 1):
                for y in range(GRID_SIZE):
                    self.points[x][y][2] = self.points[x + 1][y][2]

            self.wiggle_count = 0
        self.wiggle_count += 1

        # These are incremented in the C code (even though they aren't used for transforms there).
        self.xrot += 0.3
        self.yrot += 0.2
        self.zrot += 0.4

    def draw_scene(self) -> None:
        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
        gl.glLoadIdentity()
        gl.glTranslatef(0.0, 0.0, -12.0)

        if self.texture_id is not None:
            gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.texture_id))

        gl.glPolygonMode(gl.GL_BACK, gl.GL_FILL)
        gl.glPolygonMode(gl.GL_FRONT, gl.GL_LINE)

        gl.glBegin(gl.GL_QUADS)
        for x in range(GRID_SIZE - 1):
            for y in range(GRID_SIZE - 1):
                tx = x / 44.0
                ty = y / 44.0
                txb = (x + 1) / 44.0
                tyb = (y + 1) / 44.0

                gl.glTexCoord2f(float(tx), float(ty))
                gl.glVertex3f(
                    float(self.points[x][y][0]),
                    float(self.points[x][y][1]),
                    float(self.points[x][y][2]),
                )

                gl.glTexCoord2f(float(tx), float(tyb))
                gl.glVertex3f(
                    float(self.points[x][y + 1][0]),
                    float(self.points[x][y + 1][1]),
                    float(self.points[x][y + 1][2]),
                )

                gl.glTexCoord2f(float(txb), float(tyb))
                gl.glVertex3f(
                    float(self.points[x + 1][y + 1][0]),
                    float(self.points[x + 1][y + 1][1]),
                    float(self.points[x + 1][y + 1][2]),
                )

                gl.glTexCoord2f(float(txb), float(ty))
                gl.glVertex3f(
                    float(self.points[x + 1][y][0]),
                    float(self.points[x + 1][y][1]),
                    float(self.points[x + 1][y][2]),
                )
        gl.glEnd()

        self._update_wave()
        glut.glutSwapBuffers()

    def key_pressed(self, key: Union[bytes, int], *_args) -> None:
        if key == self.ESCAPE:
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

        self.window = glut.glutCreateWindow(
            "Jeff Molofee's GL Code Tutorial ... NeHe '99"
        )
        self._register_callbacks()

        glut.glutFullScreen()
        self.init_gl(640, 480)
        glut.glutMainLoop()


def main() -> None:
    Lesson11().run()


if __name__ == "__main__":
    main()
