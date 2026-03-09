#!/usr/bin/env python3

from __future__ import annotations

import sys
from pathlib import Path
from typing import Any, Union, cast

import OpenGL.GL as gl
import OpenGL.GLU as glu
import OpenGL.GLUT as glut
from PIL import Image


class Lesson16:
    """NeHe Lesson 16 - Fog (ported from the C tutorial code).

    Keyboard controls (matches the C version):
    - `L`: toggle lighting
    - `F`: cycle texture filtering (nearest/linear/mipmapped)
    - `G`: cycle fog mode (EXP/EXP2/LINEAR)
    - `PageUp`/`PageDown`: move in/out (z)
    - Arrow keys: adjust rotation speeds
    - `ESC`: quit
    """

    ESCAPE = b"\033"

    def __init__(self) -> None:
        self.window: int = 0

        self.x_rotation: float = 0.0
        self.y_rotation: float = 0.0
        self.x_speed: float = 0.0
        self.y_speed: float = 0.0
        self.z: float = -5.0

        self.filter_index: int = 0
        self.light_enabled: bool = False

        self.fog_modes: list[int] = [int(gl.GL_EXP), int(gl.GL_EXP2), int(gl.GL_LINEAR)]
        self.fog_filter: int = 0
        self.fog_color: tuple[float, float, float, float] = (0.5, 0.5, 0.5, 1.0)

        self.textures: list[int] = []

    def _assets_dir(self) -> Path:
        return Path(__file__).resolve().parent

    def _texture_path(self) -> Path:
        texture = self._assets_dir() / "crate.bmp"

        return texture

    def _gen_textures(self, count: int) -> list[int]:
        tex_raw = gl.glGenTextures(count)
        if tex_raw is None:
            raise RuntimeError("glGenTextures returned None")

        if count == 1:
            return [int(cast(Any, tex_raw))]
        return [int(t) for t in cast(Any, tex_raw)]

    def load_textures(self) -> None:
        image_path = self._texture_path()
        image = Image.open(image_path).convert("RGB")
        width, height = image.size
        image_data = image.tobytes("raw", "RGB", 0, -1)

        self.textures = self._gen_textures(3)

        gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.textures[0]))
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_NEAREST)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_NEAREST)
        gl.glTexImage2D(
            gl.GL_TEXTURE_2D,
            0,
            3,
            int(width),
            int(height),
            0,
            gl.GL_RGB,
            gl.GL_UNSIGNED_BYTE,
            image_data,
        )

        gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.textures[1]))
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_LINEAR)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_LINEAR)
        gl.glTexImage2D(
            gl.GL_TEXTURE_2D,
            0,
            3,
            int(width),
            int(height),
            0,
            gl.GL_RGB,
            gl.GL_UNSIGNED_BYTE,
            image_data,
        )

        gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.textures[2]))
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_LINEAR)
        gl.glTexParameteri(
            gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_LINEAR_MIPMAP_NEAREST
        )
        glu.gluBuild2DMipmaps(
            gl.GL_TEXTURE_2D,
            3,
            int(width),
            int(height),
            gl.GL_RGB,
            gl.GL_UNSIGNED_BYTE,
            image_data,
        )

    def init_gl(self, width: int, height: int) -> None:
        self.load_textures()

        gl.glEnable(gl.GL_TEXTURE_2D)
        gl.glShadeModel(gl.GL_SMOOTH)

        gl.glClearColor(0.5, 0.5, 0.5, 1.0)
        gl.glClearDepth(1.0)
        gl.glEnable(gl.GL_DEPTH_TEST)
        gl.glDepthFunc(gl.GL_LEQUAL)
        gl.glHint(gl.GL_PERSPECTIVE_CORRECTION_HINT, gl.GL_NICEST)

        light_ambient = (0.5, 0.5, 0.5, 1.0)
        light_diffuse = (1.0, 1.0, 1.0, 1.0)
        light_position = (0.0, 0.0, 2.0, 1.0)
        gl.glLightfv(gl.GL_LIGHT1, gl.GL_AMBIENT, light_ambient)
        gl.glLightfv(gl.GL_LIGHT1, gl.GL_DIFFUSE, light_diffuse)
        gl.glLightfv(gl.GL_LIGHT1, gl.GL_POSITION, light_position)
        gl.glEnable(gl.GL_LIGHT1)
        gl.glDisable(gl.GL_LIGHTING)
        self.light_enabled = False

        gl.glFogi(gl.GL_FOG_MODE, int(self.fog_modes[self.fog_filter]))
        gl.glFogfv(gl.GL_FOG_COLOR, self.fog_color)
        gl.glFogf(gl.GL_FOG_DENSITY, 0.35)
        gl.glHint(gl.GL_FOG_HINT, gl.GL_DONT_CARE)
        gl.glFogf(gl.GL_FOG_START, 1.0)
        gl.glFogf(gl.GL_FOG_END, 5.0)
        gl.glEnable(gl.GL_FOG)

        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

    def resize_scene(self, width: int, height: int) -> None:
        if height == 0:
            height = 1
        gl.glViewport(0, 0, width, height)
        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

    def _bind_active_texture(self) -> None:
        if not self.textures:
            return
        gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.textures[self.filter_index]))

    def draw_scene(self) -> None:
        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
        gl.glLoadIdentity()
        gl.glTranslatef(0.0, 0.0, float(self.z))

        gl.glRotatef(float(self.x_rotation), 1.0, 0.0, 0.0)
        gl.glRotatef(float(self.y_rotation), 0.0, 1.0, 0.0)

        self._bind_active_texture()

        gl.glBegin(gl.GL_QUADS)

        gl.glNormal3f(0.0, 0.0, 1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, 1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)

        gl.glNormal3f(0.0, 0.0, -1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, -1.0)

        gl.glNormal3f(0.0, 1.0, 0.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(1.0, 1.0, 1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)

        gl.glNormal3f(0.0, -1.0, 0.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(1.0, -1.0, -1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)

        gl.glNormal3f(1.0, 0.0, 0.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(1.0, -1.0, -1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(1.0, 1.0, 1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)

        gl.glNormal3f(-1.0, 0.0, 0.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)

        gl.glEnd()

        self.x_rotation += self.x_speed
        self.y_rotation += self.y_speed

        glut.glutSwapBuffers()

    def _toggle_lighting(self) -> None:
        self.light_enabled = not self.light_enabled
        if self.light_enabled:
            gl.glEnable(gl.GL_LIGHTING)
        else:
            gl.glDisable(gl.GL_LIGHTING)

    def _cycle_filter(self) -> None:
        self.filter_index = (self.filter_index + 1) % 3

    def _cycle_fog(self) -> None:
        self.fog_filter = (self.fog_filter + 1) % 3
        gl.glFogi(gl.GL_FOG_MODE, int(self.fog_modes[self.fog_filter]))

    def key_pressed(self, key: Union[bytes, int], *_args) -> None:
        if key == self.ESCAPE:
            if self.window:
                glut.glutDestroyWindow(int(self.window))

        if isinstance(key, int):
            return

        if key in (b"l", b"L"):
            self._toggle_lighting()
            return

        if key in (b"f", b"F"):
            self._cycle_filter()
            return

        if key in (b"g", b"G"):
            self._cycle_fog()
            return

    def special_key_pressed(self, key: Union[bytes, int], *_args) -> None:
        if key == glut.GLUT_KEY_PAGE_UP:
            self.z -= 0.02
            return
        if key == glut.GLUT_KEY_PAGE_DOWN:
            self.z += 0.02
            return

        if key == glut.GLUT_KEY_UP:
            self.x_speed -= 0.01
            return
        if key == glut.GLUT_KEY_DOWN:
            self.x_speed += 0.01
            return
        if key == glut.GLUT_KEY_LEFT:
            self.y_speed -= 0.01
            return
        if key == glut.GLUT_KEY_RIGHT:
            self.y_speed += 0.01
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
            int(glut.GLUT_RGBA) | int(glut.GLUT_DOUBLE) | int(glut.GLUT_DEPTH)
        )
        glut.glutInitWindowSize(640, 480)
        glut.glutInitWindowPosition(0, 0)

        window_raw = glut.glutCreateWindow("Daniel Davis's Fog Tutorial ... NeHe '99")
        if window_raw is None or int(cast(Any, window_raw)) == 0:
            raise RuntimeError("glutCreateWindow failed")
        self.window = int(cast(Any, window_raw))

        self._register_callbacks()
        self.init_gl(640, 480)
        glut.glutMainLoop()


def main() -> None:
    Lesson16().run()


if __name__ == "__main__":
    main()
