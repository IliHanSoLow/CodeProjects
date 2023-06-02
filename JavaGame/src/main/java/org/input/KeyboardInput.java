package org.input;

import org.main.Panel;

import java.awt.event.KeyEvent;
import java.awt.event.KeyListener;

public class KeyboardInput implements KeyListener {

    private Panel gamepanel;

    public KeyboardInput(Panel p) {
        gamepanel = p;
    }

    @Override
    public void keyTyped(KeyEvent e) {

    }

    @Override
    public void keyPressed(KeyEvent e) {
        switch (e.getKeyCode()) {
            case KeyEvent.VK_W -> gamepanel.changeYDelta(-5);
            case KeyEvent.VK_A -> gamepanel.changeXDelta(-5);
            case KeyEvent.VK_S -> gamepanel.changeYDelta(5);
            case KeyEvent.VK_D -> gamepanel.changeXDelta(5);
        }
    }


    @Override
    public void keyReleased(KeyEvent e) {

    }
}
