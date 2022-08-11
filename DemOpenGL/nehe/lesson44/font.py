# glFont.py -- SImple font class that uses a single texture represent 16x16 tiles
# for a font.
# // Code writen by: Vic Hollis 09/07/2003
# // I don't mind if you use this class in your own code. All I ask is
# // that you give me and Giuseppe D'Agata credit for it if you do.
# // And plug NeHe while your at it! :P  Thanks go to Giuseppe D'Agata
# // for the code that this class is based off of. Thanks Enjoy.
# //////////////////////////////////////////////////////////////////////
# // glFont.cpp: implementation of the glFont class.
# //////////////////////////////////////////////////////////////////////

from OpenGL.GL import *


class glFont:
    def __init__(self):
        self.m_FontTexture = 0
        self.m_ListBase = 0

    def __del__(self):
        self.release()
        return

    def release(self):
        """ We've made a separate resoruce-deallocation method
            so that the client can return system resources when the
            want to explcitly. Python will eventually garbage collect,
            once all refs to the object go away. This method
            allows the client to retain the object, and yet free up the
            gl resources. 
        """
        if (self.m_FontTexture != 0):
            glDeleteTextures(self.m_FontTexture)
        if (self.m_ListBase != 0):
            glDeleteLists(self.m_ListBase, 256)
        return

    def SetFontTexture(self, tex):
        if (tex != 0):                                                # // If the texture is valid
            # // Set the font texture
            self.m_FontTexture = tex
        else:
            # Client should not pass an invalid texture.
            raise RuntimeError(
                "SetFontTexture passed invalid texture (ID == 0)")
        return

    def BuildFont(self, Scale):
        # // Creating 256 Display Lists
        self.m_ListBase = glGenLists(256)
        if (self.m_FontTexture != 0):
            # // Select Our Font Texture
            glBindTexture(GL_TEXTURE_2D, self.m_FontTexture)
            # // Loop Through All 256 Lists
            for loop in range(256):
                # // X Position Of Current Character
                cx = float(loop % 16)/16.0
                # // Y Position Of Current Character
                cy = float(loop/16)/16.0

                # // Start Building A List
                glNewList(self.m_ListBase+loop, GL_COMPILE)
                # List start
                # // Use A Quad For Each Character
                glBegin(GL_QUADS)
                # // Texture Coord (Bottom Left)
                glTexCoord2f(cx, 1 - cy - 0.0625)
                # // Vertex Coord (Bottom Left)
                glVertex2f(0, 0)
                # // Texture Coord (Bottom Right)
                glTexCoord2f(cx + 0.0625, 1 - cy - 0.0625)
                # // Vertex Coord (Bottom Right)
                glVertex2f(16 * Scale, 0)
                # // Texture Coord (Top Right)
                glTexCoord2f(cx + 0.0625, 1 - cy)
                # // Vertex Coord (Top Right)
                glVertex2f(16 * Scale, 16 * Scale)
                # // Texture Coord (Top Left)
                glTexCoord2f(cx, 1 - cy)
                # // Vertex Coord (Top Left)
                glVertex2f(0, 16 * Scale)
                # // Done Building Our Quad (Character)
                glEnd()
                # // Move To The Right Of The Character
                glTranslated(10*Scale, 0, 0)
                # // Done Building The Display List
                glEndList()
                # List end

    def glPrintf(self, x, y, set, text):
        # // Enable 2d Textures
        glEnable(GL_TEXTURE_2D)
        # // Enable Blending
        glEnable(GL_BLEND)
        glBlendFunc(GL_SRC_COLOR, GL_ONE_MINUS_SRC_COLOR)
        # // Select Our Font Texture
        glBindTexture(GL_TEXTURE_2D, self.m_FontTexture)
        # // Disables Depth Testing
        glDisable(GL_DEPTH_TEST)
        # // Select The Projection Matrix
        glMatrixMode(GL_PROJECTION)
        # // Store The Projection Matrix
        glPushMatrix()
        # // Reset The Projection Matrix
        glLoadIdentity()
        # // Set Up An Ortho Screen
        glOrtho(0, self.m_WindowWidth, 0, self.m_WindowHeight, -1, 1)
        # // Select The Modelview Matrix
        glMatrixMode(GL_MODELVIEW)
        # // Store The Modelview Matrix
        glPushMatrix()
        # // Reset The Modelview Matrix
        glLoadIdentity()
        # // Position The Text (0,0 - Bottom Left)
        glTranslated(x, y, 0)
        # // Choose The Font Set (0 or 1)
        glListBase(self.m_ListBase-32+(128*set))
        # glCallLists(len(text),GL_BYTE,text);                        # // Write The Text To The Screen
        # function can figure out the count and TYP
        # // Write The Text To The Screen
        glCallLists(text)
        # // Select The Projection Matrix
        glMatrixMode(GL_PROJECTION)
        # // Restore The Old Projection Matrix
        glPopMatrix()
        # // Select The Modelview Matrix
        glMatrixMode(GL_MODELVIEW)
        # // Restore The Old Projection Matrix
        glPopMatrix()
        glEnable(GL_DEPTH_TEST)
        glDisable(GL_BLEND)
        glDisable(GL_TEXTURE_2D)
        return

    def SetWindowSize(self, width, height):
        self.m_WindowWidth = width
        self.m_WindowHeight = height
        return

    def GetTexture(self):
        return self.m_FontTexture

    def GetListBase(self):
        return self.m_ListBase
