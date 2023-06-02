package org.main;

import javax.swing.*;

public class Window extends JFrame {
    public Window(Panel gamepanel) {
        this.setSize(400, 400);
        this.setDefaultCloseOperation(this.EXIT_ON_CLOSE);
        this.add(gamepanel);
        this.setLocationRelativeTo(null);
        this.setVisible(true);
    }
}
