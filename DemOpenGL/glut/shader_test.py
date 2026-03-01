#!/usr/bin/env python3
"""Minimal GLUT demo using ARB shader objects (PyOpenGL).

Refactor notes:
- Avoids wildcard imports (better for linters, readability, and tooling).
- Uses a small class instead of module-level globals.
"""


from dataclasses import dataclass
from typing import Optional, Sequence, Union

import sys

from OpenGL import GL, GLU, GLUT
from OpenGL.GL.ARB import fragment_shader, shader_objects, vertex_shader


ShaderSource = Union[str, Sequence[str]]


VERTEX_SHADER_SOURCE = """\
varying vec3 normal;
void main() {
    normal = gl_NormalMatrix * gl_Normal;
    gl_Position = gl_ModelViewProjectionMatrix * gl_Vertex;
}
"""


FRAGMENT_SHADER_SOURCE = """\
varying vec3 normal;
void main() {
    float intensity;
    vec4 color;
    vec3 n = normalize(normal);
    vec3 l = normalize(gl_LightSource[0].position).xyz;

    // quantize to 5 steps (0, .25, .5, .75 and 1)
    intensity = (floor(dot(l, n) * 4.0) + 1.0) / 4.0;
    color = vec4(intensity * 1.0, intensity * 0.5, intensity * 0.5, intensity * 1.0);

    gl_FragColor = color;
}
"""


def _as_source(source: ShaderSource) -> ShaderSource:
    """Normalize shader source for PyOpenGL.

    `glShaderSourceARB` accepts either a string or a sequence of strings.
    This keeps compatibility with older patterns without forcing callers to care.
    """

    if isinstance(source, (list, tuple)):
        return [str(part) for part in source]
    return str(source)


def compile_shader(source: ShaderSource, shader_type: int) -> int:
    shader = shader_objects.glCreateShaderObjectARB(shader_type)
    shader_objects.glShaderSourceARB(shader, _as_source(source))
    shader_objects.glCompileShaderARB(shader)

    assert type(shader) == int, "Expected shader object to be an integer handle"
    return shader


def compile_program(
    *,
    vertex_source: Optional[ShaderSource] = None,
    fragment_source: Optional[ShaderSource] = None,
) -> int:
    program = shader_objects.glCreateProgramObjectARB()

    vertex_shader_obj: Optional[int] = None
    fragment_shader_obj: Optional[int] = None
    try:
        if vertex_source:
            vertex_shader_obj = compile_shader(
                vertex_source, vertex_shader.GL_VERTEX_SHADER_ARB
            )
            shader_objects.glAttachObjectARB(program, vertex_shader_obj)
        if fragment_source:
            fragment_shader_obj = compile_shader(
                fragment_source, fragment_shader.GL_FRAGMENT_SHADER_ARB
            )
            shader_objects.glAttachObjectARB(program, fragment_shader_obj)

        shader_objects.glLinkProgramARB(program)
        shader_objects.glValidateProgramARB(program)

        assert type(program) == int, "Expected program object to be an integer handle"
        return program
    finally:
        if vertex_shader_obj:
            shader_objects.glDeleteObjectARB(vertex_shader_obj)
        if fragment_shader_obj:
            shader_objects.glDeleteObjectARB(fragment_shader_obj)


@dataclass
class ShaderDemo:
    width: int = 640
    height: int = 480
    title: str = "ARB Shader Objects Demo"

    window: Optional[int] = None
    program: Optional[int] = None

    def init_gl(self) -> None:
        GL.glClearColor(0.0, 0.0, 0.0, 0.0)
        GL.glClearDepth(1.0)
        GL.glDepthFunc(GL.GL_LESS)
        GL.glEnable(GL.GL_DEPTH_TEST)
        GL.glShadeModel(GL.GL_SMOOTH)

        GL.glMatrixMode(GL.GL_PROJECTION)
        GL.glLoadIdentity()
        GLU.gluPerspective(45.0, float(self.width) / float(self.height), 0.1, 100.0)
        GL.glMatrixMode(GL.GL_MODELVIEW)

        if not shader_objects.glInitShaderObjectsARB():
            raise RuntimeError("Missing ARB_shader_objects support")
        if not vertex_shader.glInitVertexShaderARB():
            raise RuntimeError("Missing ARB_vertex_shader support")
        if not fragment_shader.glInitFragmentShaderARB():
            raise RuntimeError("Missing ARB_fragment_shader support")

        self.program = compile_program(
            vertex_source=VERTEX_SHADER_SOURCE,
            fragment_source=FRAGMENT_SHADER_SOURCE,
        )

    def reshape(self, width: int, height: int) -> None:
        self.width = int(width)
        self.height = int(height) or 1

        GL.glViewport(0, 0, self.width, self.height)
        GL.glMatrixMode(GL.GL_PROJECTION)
        GL.glLoadIdentity()
        GLU.gluPerspective(45.0, float(self.width) / float(self.height), 0.1, 100.0)
        GL.glMatrixMode(GL.GL_MODELVIEW)

    def display(self) -> None:
        GL.glClear(int(GL.GL_COLOR_BUFFER_BIT) | int(GL.GL_DEPTH_BUFFER_BIT))
        GL.glLoadIdentity()

        GL.glTranslatef(-1.5, 0.0, -6.0)
        if self.program:
            shader_objects.glUseProgramObjectARB(self.program)

        GLUT.glutSolidSphere(1.0, 32, 32)
        GL.glTranslatef(1.0, 0.0, 2.0)
        GLUT.glutSolidCube(1.0)
        GLUT.glutSwapBuffers()

    def keyboard(self, key: bytes, _x: int, _y: int) -> None:
        if key == b"\x1b" and self.window is not None:
            GLUT.glutDestroyWindow(self.window)

    def run(self) -> None:
        GLUT.glutInit(sys.argv)
        GLUT.glutInitDisplayMode(
            int(GLUT.GLUT_RGBA) | int(GLUT.GLUT_DOUBLE) | int(GLUT.GLUT_DEPTH)
        )
        GLUT.glutInitWindowSize(self.width, self.height)
        GLUT.glutInitWindowPosition(0, 0)

        self.window = GLUT.glutCreateWindow(self.title)

        GLUT.glutDisplayFunc(self.display)
        GLUT.glutIdleFunc(self.display)
        GLUT.glutReshapeFunc(self.reshape)
        GLUT.glutKeyboardFunc(self.keyboard)

        self.init_gl()
        GLUT.glutMainLoop()


def main() -> None:
    print("Hit ESC key to quit.")
    ShaderDemo(title="Jeff Molofee's GL Code Tutorial ... NeHe '99").run()


if __name__ == "__main__":
    main()
