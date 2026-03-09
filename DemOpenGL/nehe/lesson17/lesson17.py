#!/usr/bin/env python3

from __future__ import annotations

import math
import sys
from pathlib import Path
from typing import Any, Optional, Union, cast

import OpenGL.GL as gl
import OpenGL.GLU as glu
import OpenGL.GLUT as glut
from PIL import Image


class Lesson17:
    """NeHe Lesson 17 - 2D Texture Font (ported from the Linux GLX C version).

    Keyboard controls (adapted to GLUT):
    - `ESC`: quit
    - `F1`: toggle fullscreen

    Notes:
    - The original C version uses GLX/XF86VidMode for fullscreen; this port uses GLUT.
    - Font rendering is done via display lists over a texture atlas, matching the C logic.
    """

    ESCAPE = b"\033"

    def __init__(self) -> None:
        self.window: int = 0
        self.is_fullscreen: bool = True
        self.windowed_size: tuple[int, int] = (640, 480)
        self.windowed_pos: tuple[int, int] = (0, 0)

        self.base_list: Optional[int] = None
        self.textures: list[int] = []  # [font_tex, bumps_tex]

        self.cnt1: float = 0.0
        self.cnt2: float = 0.0

    def _assets_dir(self) -> Path:
        return Path(__file__).resolve().parent

    def _font_path(self) -> Path:
        font = self._assets_dir() / "Font.bmp"
        return font

    def _bumps_path(self) -> Path:
        bumps = self._assets_dir() / "Bumps.bmp"
        return bumps

    def _gen_textures(self, count: int) -> list[int]:
        raw = gl.glGenTextures(count)
        if raw is None:
            raise RuntimeError("glGenTextures returned None")

        if count == 1:
            return [int(cast(Any, raw))]
        return [int(v) for v in cast(Any, raw)]

    def _load_bmp_rgb(self, path: Path) -> tuple[int, int, bytes]:
        image = Image.open(path).convert("RGB")
        width, height = image.size
        data = image.tobytes("raw", "RGB", 0, -1)
        return int(width), int(height), data

    def load_gl_textures(self) -> None:
        font_w, font_h, font_data = self._load_bmp_rgb(self._font_path())
        bumps_w, bumps_h, bumps_data = self._load_bmp_rgb(self._bumps_path())

        self.textures = self._gen_textures(2)

        for idx, (w, h, data) in enumerate(
            ((font_w, font_h, font_data), (bumps_w, bumps_h, bumps_data))
        ):
            gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.textures[idx]))
            gl.glTexImage2D(
                gl.GL_TEXTURE_2D,
                0,
                3,
                int(w),
                int(h),
                0,
                gl.GL_RGB,
                gl.GL_UNSIGNED_BYTE,
                data,
            )
            gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_LINEAR)
            gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_LINEAR)

    def build_font(self) -> None:
        if not self.textures:
            raise RuntimeError("Textures not loaded")

        base_raw = gl.glGenLists(256)
        if base_raw is None or int(base_raw) == 0:
            raise RuntimeError("glGenLists failed while building font")

        base = int(base_raw)
        self.base_list = base

        gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.textures[0]))

        for i in range(256):
            cx = float(i % 16) / 16.0
            cy = float(i // 16) / 16.0

            gl.glNewList(base + i, gl.GL_COMPILE)
            gl.glBegin(gl.GL_QUADS)
            gl.glTexCoord2f(cx, 1.0 - cy - 0.0625)
            gl.glVertex2i(0, 0)
            gl.glTexCoord2f(cx + 0.0625, 1.0 - cy - 0.0625)
            gl.glVertex2i(16, 0)
            gl.glTexCoord2f(cx + 0.0625, 1.0 - cy)
            gl.glVertex2i(16, 16)
            gl.glTexCoord2f(cx, 1.0 - cy)
            gl.glVertex2i(0, 16)
            gl.glEnd()
            gl.glTranslated(10.0, 0.0, 0.0)
            gl.glEndList()

    def kill_font(self) -> None:
        if self.base_list is None:
            return
        gl.glDeleteLists(int(self.base_list), 256)
        self.base_list = None

    def print_gl(self, x: int, y: int, text: str, set_index: int) -> None:
        if not text or self.base_list is None or not self.textures:
            return

        if set_index > 1:
            set_index = 1

        gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.textures[0]))
        gl.glDisable(gl.GL_DEPTH_TEST)

        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glPushMatrix()
        gl.glLoadIdentity()
        gl.glOrtho(0, 640, 0, 480, -1, 1)

        gl.glMatrixMode(gl.GL_MODELVIEW)
        gl.glPushMatrix()
        gl.glLoadIdentity()
        gl.glTranslated(float(x), float(y), 0.0)

        gl.glListBase(int(self.base_list) - 32 + (128 * int(set_index)))
        for ch in text:
            gl.glCallList(int(self.base_list) + (ord(ch) & 0xFF))

        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glPopMatrix()
        gl.glMatrixMode(gl.GL_MODELVIEW)
        gl.glPopMatrix()

        gl.glEnable(gl.GL_DEPTH_TEST)

    def init_gl(self, width: int, height: int) -> None:
        self.load_gl_textures()
        self.build_font()

        gl.glShadeModel(gl.GL_SMOOTH)
        gl.glClearColor(0.0, 0.0, 0.0, 0.0)
        gl.glClearDepth(1.0)
        gl.glEnable(gl.GL_DEPTH_TEST)
        gl.glDepthFunc(gl.GL_LEQUAL)
        gl.glBlendFunc(gl.GL_SRC_ALPHA, gl.GL_ONE)
        gl.glEnable(gl.GL_TEXTURE_2D)

        self.resize_scene(width, height)
        gl.glFlush()

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

        if self.textures:
            gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.textures[1]))

        gl.glTranslatef(0.0, 0.0, -5.0)
        gl.glRotatef(45.0, 0.0, 0.0, 1.0)
        gl.glRotatef(float(self.cnt1 * 30.0), 1.0, 1.0, 0.0)

        gl.glDisable(gl.GL_BLEND)
        gl.glColor3f(1.0, 1.0, 1.0)

        gl.glBegin(gl.GL_QUADS)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex2f(-1.0, 1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex2f(1.0, 1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex2f(1.0, -1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex2f(-1.0, -1.0)
        gl.glEnd()

        gl.glRotatef(90.0, 1.0, 1.0, 0.0)
        gl.glBegin(gl.GL_QUADS)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex2f(-1.0, 1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex2f(1.0, 1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex2f(1.0, -1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex2f(-1.0, -1.0)
        gl.glEnd()

        gl.glEnable(gl.GL_BLEND)
        gl.glLoadIdentity()

        gl.glColor3f(
            float(math.cos(self.cnt1)),
            float(math.sin(self.cnt2)),
            float(1.0 - 0.5 * math.cos(self.cnt1 + self.cnt2)),
        )
        self.print_gl(
            int(280 + 250 * math.cos(self.cnt1)),
            int(235 + 200 * math.sin(self.cnt2)),
            "NeHe",
            0,
        )

        gl.glColor3f(
            float(math.sin(self.cnt2)),
            float(1.0 - 0.5 * math.cos(self.cnt1 + self.cnt2)),
            float(math.cos(self.cnt1)),
        )
        self.print_gl(
            int(280 + 230 * math.cos(self.cnt2)),
            int(235 + 200 * math.sin(self.cnt1)),
            "OpenGL",
            1,
        )

        gl.glColor3f(0.0, 0.0, 1.0)
        self.print_gl(
            int(240 + 200 * math.cos((self.cnt2 + self.cnt1) / 5.0)),
            2,
            "Giuseppe D'Agata",
            0,
        )
        gl.glColor3f(1.0, 1.0, 1.0)
        self.print_gl(
            int(242 + 200 * math.cos((self.cnt2 + self.cnt1) / 5.0)),
            2,
            "Giuseppe D'Agata",
            0,
        )

        self.cnt1 += 0.01
        self.cnt2 += 0.0081

        glut.glutSwapBuffers()

    def key_pressed(self, key: Union[bytes, int], *_args) -> None:
        if key == self.ESCAPE:
            self.kill_font()

            if self.window:
                glut.glutDestroyWindow(int(self.window))

    def special_key_pressed(self, key: Union[bytes, int], *_args) -> None:
        if key == glut.GLUT_KEY_F1:
            self.toggle_fullscreen()

    def toggle_fullscreen(self) -> None:
        self.is_fullscreen = not self.is_fullscreen
        if self.is_fullscreen:
            glut.glutFullScreen()
        else:
            w, h = self.windowed_size
            x, y = self.windowed_pos
            glut.glutReshapeWindow(int(w), int(h))
            glut.glutPositionWindow(int(x), int(y))

    def _register_callbacks(self) -> None:
        glut.glutDisplayFunc(self.draw_scene)
        glut.glutIdleFunc(self.draw_scene)
        glut.glutReshapeFunc(self.resize_scene)
        glut.glutKeyboardFunc(self.key_pressed)
        glut.glutSpecialFunc(self.special_key_pressed)

    def run(self) -> None:
        glut.glutInit(sys.argv)
        glut.glutInitDisplayMode(
            int(glut.GLUT_RGBA) | int(glut.GLUT_DOUBLE) | int(glut.GLUT_DEPTH)
        )
        glut.glutInitWindowSize(640, 480)
        glut.glutInitWindowPosition(0, 0)

        window_raw = glut.glutCreateWindow("NeHe & Giuseppe D'Agata's 2D Font Tutorial")
        if window_raw is None or int(cast(Any, window_raw)) == 0:
            raise RuntimeError("glutCreateWindow failed")
        self.window = int(cast(Any, window_raw))

        self._register_callbacks()

        self.init_gl(640, 480)
        glut.glutFullScreen()
        self.is_fullscreen = True

        glut.glutMainLoop()


def main() -> None:
    Lesson17().run()


if __name__ == "__main__":
    main()
