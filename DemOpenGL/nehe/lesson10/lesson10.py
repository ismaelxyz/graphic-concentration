#!/usr/bin/env python3

from __future__ import annotations

import sys
from dataclasses import dataclass
import math
from pathlib import Path
from typing import Union

import OpenGL.GL as gl
import OpenGL.GLU as glu
import OpenGL.GLUT as glut
from PIL import Image


PIOVER180 = 0.0174532925


@dataclass(slots=True)
class Vertex:
    x: float
    y: float
    z: float
    u: float
    v: float


@dataclass(slots=True)
class Triangle:
    v0: Vertex
    v1: Vertex
    v2: Vertex


@dataclass(slots=True)
class Sector:
    triangles: list[Triangle]


class Lesson10:
    """NeHe Lesson 10 - Loading and moving through a 3D world."""

    ESCAPE = b"\033"

    def __init__(self) -> None:
        self.window = 0

        self.light = False
        self.blend = False
        self.filter_index = 0

        self.xrot = 0.0
        self.yrot = 0.0
        self.xspeed = 0.0
        self.yspeed = 0.0

        self.walkbias = 0.0
        self.walkbiasangle = 0.0
        self.lookupdown = 0.0

        self.xpos = 0.0
        self.zpos = 0.0

        self.texture_ids: list[int] = []
        self.sector = Sector(triangles=[])

        self.light_ambient = (0.5, 0.5, 0.5, 1.0)
        self.light_diffuse = (1.0, 1.0, 1.0, 1.0)
        self.light_position = (0.0, 0.0, 2.0, 1.0)

    def _assets_dir(self) -> Path:
        return Path(__file__).resolve().parent

    def _world_path(self) -> Path:
        return self._assets_dir() / "world.txt"

    def _texture_path(self) -> Path:
        return self._assets_dir() / "mud.bmp"

    @staticmethod
    def _rad(angle_degrees: float) -> float:
        return angle_degrees * PIOVER180

    def load_world(self) -> None:
        """Load world geometry from world.txt (same format as the C tutorial)."""

        world_path = self._world_path()
        text = world_path.read_text(encoding="utf-8", errors="replace")

        # Filter comment and blank lines (C version treats '/' at column 0 as a comment)
        lines: list[str] = []
        for raw in text.splitlines():
            line = raw.strip()
            if not line:
                continue
            if line.startswith("/"):
                continue
            lines.append(line)

        if not lines:
            raise RuntimeError(f"World file is empty: {world_path}")

        header = lines.pop(0)
        # Format: NUMPOLLIES <int>
        parts = header.split()
        if len(parts) != 2 or parts[0] != "NUMPOLLIES":
            raise RuntimeError(
                f"Invalid world header in {world_path}: expected 'NUMPOLLIES <n>', got: {header!r}"
            )
        try:
            num_triangles = int(parts[1])
        except ValueError as exc:
            raise RuntimeError(
                f"Invalid triangle count in {world_path}: {parts[1]!r}"
            ) from exc

        needed = num_triangles * 3
        if len(lines) < needed:
            raise RuntimeError(
                f"World file {world_path} has {len(lines)} vertex lines, expected {needed}"
            )

        triangles: list[Triangle] = []
        idx = 0
        for _ in range(num_triangles):
            vtx: list[Vertex] = []
            for _vert in range(3):
                x_s, y_s, z_s, u_s, v_s = lines[idx].split()
                idx += 1
                vtx.append(
                    Vertex(
                        x=float(x_s),
                        y=float(y_s),
                        z=float(z_s),
                        u=float(u_s),
                        v=float(v_s),
                    )
                )
            triangles.append(Triangle(v0=vtx[0], v1=vtx[1], v2=vtx[2]))

        self.sector = Sector(triangles=triangles)

    def _create_texture_variants(self, image: Image.Image) -> list[int]:
        image = image.convert("RGB")
        width, height = image.size
        image_data = image.tobytes("raw", "RGB", 0, -1)

        ids = [int(x) for x in gl.glGenTextures(3)]

        # 0: nearest
        gl.glBindTexture(gl.GL_TEXTURE_2D, ids[0])
        gl.glPixelStorei(gl.GL_UNPACK_ALIGNMENT, 1)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_NEAREST)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_NEAREST)
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

        # 1: linear
        gl.glBindTexture(gl.GL_TEXTURE_2D, ids[1])
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

        # 2: mipmapped (match C: GL_LINEAR + GL_LINEAR_MIPMAP_NEAREST)
        gl.glBindTexture(gl.GL_TEXTURE_2D, ids[2])
        gl.glPixelStorei(gl.GL_UNPACK_ALIGNMENT, 1)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_LINEAR)
        gl.glTexParameteri(
            gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_LINEAR_MIPMAP_NEAREST
        )
        # Use gluBuild2DMipmaps to mirror tutorial behavior.
        glu.gluBuild2DMipmaps(
            gl.GL_TEXTURE_2D,
            gl.GL_RGB,
            width,
            height,
            gl.GL_RGB,
            gl.GL_UNSIGNED_BYTE,
            image_data,
        )

        return ids

    def load_textures(self) -> None:
        texture_path = self._texture_path()
        image = Image.open(texture_path)
        self.texture_ids = self._create_texture_variants(image)

    def init_gl(self, width: int, height: int) -> None:
        self.load_world()
        self.load_textures()

        gl.glEnable(gl.GL_TEXTURE_2D)
        gl.glBlendFunc(gl.GL_SRC_ALPHA, gl.GL_ONE)

        gl.glClearColor(0.0, 0.0, 0.0, 0.0)
        gl.glClearDepth(1.0)
        gl.glDepthFunc(gl.GL_LESS)
        gl.glEnable(gl.GL_DEPTH_TEST)
        gl.glShadeModel(gl.GL_SMOOTH)

        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

        # Lights (GL_LIGHT1)
        gl.glLightfv(gl.GL_LIGHT1, gl.GL_AMBIENT, self.light_ambient)
        gl.glLightfv(gl.GL_LIGHT1, gl.GL_DIFFUSE, self.light_diffuse)
        gl.glLightfv(gl.GL_LIGHT1, gl.GL_POSITION, self.light_position)
        gl.glEnable(gl.GL_LIGHT1)

        # Start with lighting disabled like the tutorial toggles.
        gl.glDisable(gl.GL_LIGHTING)

    def resize_scene(self, width: int, height: int) -> None:
        if height == 0:
            height = 1

        gl.glViewport(0, 0, width, height)
        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

    def draw_scene(self) -> None:
        # Calculate translations and rotations (same idea as C version)
        xtrans = -self.xpos
        ztrans = -self.zpos
        ytrans = -self.walkbias - 0.25
        sceneroty = 360.0 - self.yrot

        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
        gl.glLoadIdentity()

        gl.glRotatef(float(self.lookupdown), 1.0, 0.0, 0.0)
        gl.glRotatef(float(sceneroty), 0.0, 1.0, 0.0)
        gl.glTranslatef(float(xtrans), float(ytrans), float(ztrans))

        if self.texture_ids:
            gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.texture_ids[self.filter_index]))

        for tri in self.sector.triangles:
            gl.glBegin(gl.GL_TRIANGLES)
            gl.glNormal3f(0.0, 0.0, 1.0)

            gl.glTexCoord2f(float(tri.v0.u), float(tri.v0.v))
            gl.glVertex3f(float(tri.v0.x), float(tri.v0.y), float(tri.v0.z))

            gl.glTexCoord2f(float(tri.v1.u), float(tri.v1.v))
            gl.glVertex3f(float(tri.v1.x), float(tri.v1.y), float(tri.v1.z))

            gl.glTexCoord2f(float(tri.v2.u), float(tri.v2.v))
            gl.glVertex3f(float(tri.v2.x), float(tri.v2.y), float(tri.v2.z))

            gl.glEnd()

        glut.glutSwapBuffers()

    def key_pressed(self, key: Union[bytes, int], *_args) -> None:
        if key == self.ESCAPE:
            glut.glutDestroyWindow(int(self.window))
            return

        if key in (b"b", b"B"):
            self.blend = not self.blend
            if self.blend:
                gl.glEnable(gl.GL_BLEND)
                gl.glDisable(gl.GL_DEPTH_TEST)
            else:
                gl.glDisable(gl.GL_BLEND)
                gl.glEnable(gl.GL_DEPTH_TEST)
            return

        if key in (b"f", b"F"):
            self.filter_index = (self.filter_index + 1) % 3
            return

        if key in (b"l", b"L"):
            self.light = not self.light
            if self.light:
                gl.glEnable(gl.GL_LIGHTING)
            else:
                gl.glDisable(gl.GL_LIGHTING)
            return

    def special_key_pressed(self, key: Union[bytes, int], *_args) -> None:
        if key == glut.GLUT_KEY_PAGE_UP:
            self.lookupdown -= 0.2
            return

        if key == glut.GLUT_KEY_PAGE_DOWN:
            self.lookupdown += 1.0
            return

        if key == glut.GLUT_KEY_UP:
            self.xpos -= float(math.sin(self._rad(self.yrot))) * 0.05
            self.zpos -= float(math.cos(self._rad(self.yrot))) * 0.05
            if self.walkbiasangle >= 359.0:
                self.walkbiasangle = 0.0
            else:
                self.walkbiasangle += 10.0
            self.walkbias = float(math.sin(self._rad(self.walkbiasangle))) / 20.0
            return

        if key == glut.GLUT_KEY_DOWN:
            self.xpos += float(math.sin(self._rad(self.yrot))) * 0.05
            self.zpos += float(math.cos(self._rad(self.yrot))) * 0.05
            if self.walkbiasangle <= 1.0:
                self.walkbiasangle = 359.0
            else:
                self.walkbiasangle -= 10.0
            self.walkbias = float(math.sin(self._rad(self.walkbiasangle))) / 20.0
            return

        if key == glut.GLUT_KEY_LEFT:
            self.yrot += 1.5
            return

        if key == glut.GLUT_KEY_RIGHT:
            self.yrot -= 1.5
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
            | int(glut.GLUT_DEPTH)
            | int(glut.GLUT_ALPHA)
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
    Lesson10().run()


if __name__ == "__main__":
    main()
