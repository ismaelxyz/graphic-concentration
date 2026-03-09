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
    """NeHe Lesson 16 - Quadrics (ported from the Linux C/GLUT version).

    Keyboard controls (matches the C version):
    - `L`: toggle lighting
    - `F`: cycle texture filtering (nearest/linear/mipmapped)
    - `B`: toggle blending
    - `Space`: cycle object
    - `PageUp`/`PageDown`: move in/out (z)
    - Arrow keys: adjust rotation speeds
    - `ESC` / `Q`: quit
    """

    ESCAPE = b"\033"

    def __init__(self) -> None:
        self.window: int = 0

        self.light_enabled: bool = False
        self.blend_enabled: bool = False

        self.filter_index: int = 0
        self.textures: list[int] = []

        self.object_index: int = 0

        self.x_rotation: float = 0.0
        self.y_rotation: float = 0.0
        self.x_speed: float = 0.0
        self.y_speed: float = 0.0
        self.z: float = -5.0

        self.part1: int = 0
        self.part2: int = 0
        self.p1: int = 0
        self.p2: int = 1

        self.quadratic: Any = None

    def _assets_dir(self) -> Path:
        return Path(__file__).resolve().parent

    def _texture_path(self) -> Path:
        texture = self._assets_dir() / "crate.bmp"

        return texture

    def _gen_textures(self, count: int) -> list[int]:
        raw = gl.glGenTextures(count)
        if raw is None:
            raise RuntimeError("glGenTextures returned None")

        if count == 1:
            return [int(cast(Any, raw))]
        return [int(v) for v in cast(Any, raw)]

    def load_textures(self) -> None:
        image_path = self._texture_path()
        image = Image.open(image_path).convert("RGB")
        width, height = image.size
        image_data = image.tobytes("raw", "RGB", 0, -1)

        self.textures = self._gen_textures(3)

        # Texture 1: nearest
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

        # Texture 2: linear
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

        # Texture 3: mipmapped
        gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.textures[2]))
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_LINEAR)
        gl.glTexParameteri(
            gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_LINEAR_MIPMAP_NEAREST
        )
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
        glu.gluBuild2DMipmaps(
            gl.GL_TEXTURE_2D,
            3,
            int(width),
            int(height),
            gl.GL_RGB,
            gl.GL_UNSIGNED_BYTE,
            image_data,
        )

    def _apply_lighting_state(self) -> None:
        if self.light_enabled:
            gl.glEnable(gl.GL_LIGHTING)
        else:
            gl.glDisable(gl.GL_LIGHTING)

    def _apply_blend_state(self) -> None:
        if self.blend_enabled:
            gl.glEnable(gl.GL_BLEND)
            gl.glDisable(gl.GL_DEPTH_TEST)
        else:
            gl.glDisable(gl.GL_BLEND)
            gl.glEnable(gl.GL_DEPTH_TEST)

    def init_gl(self, width: int, height: int) -> None:
        self.load_textures()
        gl.glEnable(gl.GL_TEXTURE_2D)

        gl.glClearColor(0.0, 0.0, 0.0, 0.0)
        gl.glClearDepth(1.0)
        gl.glDepthFunc(gl.GL_LESS)
        gl.glEnable(gl.GL_DEPTH_TEST)
        gl.glShadeModel(gl.GL_SMOOTH)

        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

        # Light 1 setup (GL_LIGHTING itself is toggled by key)
        light_ambient = (0.5, 0.5, 0.5, 1.0)
        light_diffuse = (1.0, 1.0, 1.0, 1.0)
        light_position = (0.0, 0.0, 2.0, 1.0)
        gl.glLightfv(gl.GL_LIGHT1, gl.GL_AMBIENT, light_ambient)
        gl.glLightfv(gl.GL_LIGHT1, gl.GL_DIFFUSE, light_diffuse)
        gl.glLightfv(gl.GL_LIGHT1, gl.GL_POSITION, light_position)
        gl.glEnable(gl.GL_LIGHT1)

        # Blending
        gl.glBlendFunc(gl.GL_SRC_ALPHA, gl.GL_ONE)
        gl.glColor4f(1.0, 1.0, 1.0, 0.5)

        self.quadratic = glu.gluNewQuadric()
        glu.gluQuadricNormals(self.quadratic, glu.GLU_SMOOTH)
        glu.gluQuadricTexture(self.quadratic, int(gl.GL_TRUE))

        self.light_enabled = False
        self.blend_enabled = False
        self._apply_lighting_state()
        self._apply_blend_state()

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

    def draw_cube(self) -> None:
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

    def _animate_partial_disk(self) -> None:
        self.part1 += self.p1
        self.part2 += self.p2

        if self.part1 > 359:
            self.p1 = 0
            self.part1 = 0
            self.p2 = 1
            self.part2 = 0

        if self.part2 > 359:
            self.p1 = 1
            self.p2 = 0

    def draw_scene(self) -> None:
        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
        gl.glLoadIdentity()

        gl.glTranslatef(0.0, 0.0, float(self.z))
        gl.glRotatef(float(self.x_rotation), 1.0, 0.0, 0.0)
        gl.glRotatef(float(self.y_rotation), 0.0, 1.0, 0.0)

        self._bind_active_texture()

        if self.object_index == 0:
            self.draw_cube()
        elif self.object_index == 1:
            gl.glTranslatef(0.0, 0.0, -1.5)
            glu.gluCylinder(self.quadratic, 1.0, 1.0, 3.0, 32, 32)
        elif self.object_index == 2:
            glu.gluDisk(self.quadratic, 0.5, 1.5, 32, 32)
        elif self.object_index == 3:
            glu.gluSphere(self.quadratic, 1.3, 32, 32)
        elif self.object_index == 4:
            gl.glTranslatef(0.0, 0.0, -1.5)
            glu.gluCylinder(self.quadratic, 1.0, 0.2, 3.0, 32, 32)
        elif self.object_index == 5:
            self._animate_partial_disk()
            sweep = self.part2 - self.part1
            glu.gluPartialDisk(self.quadratic, 0.5, 1.5, 32, 32, self.part1, sweep)

        self.x_rotation += self.x_speed
        self.y_rotation += self.y_speed

        glut.glutSwapBuffers()

    def key_pressed(self, key: Union[bytes, int], *_args) -> None:
        if key == self.ESCAPE or key in (b"q", b"Q"):
            if self.window:
                glut.glutDestroyWindow(int(self.window))
            sys.exit(0)

        if isinstance(key, int):
            return

        if key in (b"l", b"L"):
            self.light_enabled = not self.light_enabled
            self._apply_lighting_state()
            return

        if key in (b"f", b"F"):
            self.filter_index = (self.filter_index + 1) % 3
            return

        if key in (b"b", b"B"):
            self.blend_enabled = not self.blend_enabled
            self._apply_blend_state()
            return

        if key == b" ":
            self.object_index = (self.object_index + 1) % 6
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
            int(glut.GLUT_RGB) | int(glut.GLUT_DOUBLE) | int(glut.GLUT_DEPTH)
        )
        glut.glutInitWindowSize(640, 480)
        glut.glutInitWindowPosition(0, 0)

        window_raw = glut.glutCreateWindow("My GL Tutorial")
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
